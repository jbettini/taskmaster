use std::collections::HashMap;
use std::fs::File;
use std::process::Stdio;
use std::time::{Duration, SystemTime};
use std::{thread, process};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};

pub mod server;
pub mod command;
pub mod initconfig;

const SOCK_PATH: &'static str = "/home/ramzi/Desktop/Taskmaster/confs/mysocket.sock";
const LOGFILE: &'static str = "/home/ramzi/Desktop/Taskmaster/confs/logfile";

use initconfig::parsing::ProgramConfig;
use initconfig::{get_config, Procs, Status};
use server::logfile::SaveLog;
use command::Command;
use server::bidirmsg::BidirectionalMessage;
use fork::{daemon, fork, Fork};
use chrono::{DateTime, Local, TimeZone, Utc};


fn stop_program_internal(args: Vec<String>, procs: &mut Procs) -> String {
    let mut response = String::new();
    let processes = procs.processes.clone();

    for arg in args {
        let statuses: Vec<_> = {
            let processes_guard = processes.lock().unwrap();
            processes_guard
                .iter()
                .filter(|(key, _)| key.starts_with(&arg))
                .map(|(key, status)| (key.clone(), status.clone()))
                .collect()
        };

        if statuses.is_empty() {
            response.push_str(&format!("Program {} is not running\n", arg));
            continue;
        }

        for (instance_name, status) in statuses {
            let child_opt = {
                let mut status_guard = status.lock().unwrap();
                let child_opt = status_guard.child.take();
                status_guard.state = String::from("STOPPED");
                status_guard.start_time = Some(system_time(SystemTime::now()));
                child_opt
            };

            if let Some(child_arc) = child_opt {
                let mut attempts = 0;
                let max_attempts = 10;
                let mut locked = false;

                while attempts < max_attempts {
                    match child_arc.try_lock() {
                        Ok(mut child) => {
                            match child.kill() {
                                Ok(_) => {
                                    let STOPPED = child.wait().is_ok();
                                    if STOPPED {
                                        response.push_str(&format!("Program {} stopped.\n", instance_name));
                                    } else {
                                        response.push_str(&format!("Failed to stop program {}: still running.\n", instance_name));
                                    }
                                    locked = true;
                                    break;
                                },
                                Err(e) => {
                                    response.push_str(&format!("Failed to stop program {}: {}\n", instance_name, e));
                                    locked = true;
                                    break;
                                },
                            }
                        },
                        Err(_) => {
                            attempts += 1;
                            thread::sleep(Duration::from_millis(100));
                        },
                    }
                }

                if !locked {
                    response.push_str(&format!("Failed to acquire lock to stop program {}\n", instance_name));
                }
            } else {
                response.push_str(&format!("Program {} is not running\n", instance_name));
            }
        }
    }
    response
}




fn handle_stop(args: Vec<String>, channel: BidirectionalMessage, procs: &mut Procs) {
    if args.is_empty() {
        shutdown_daemon(channel, procs);
        return;
    }
    
    let response = stop_program_internal(args, procs);
    channel.answer(response).unwrap();
}


fn shutdown_daemon(channel: BidirectionalMessage, procs: &mut Procs) {
    let exit_msg = "Daemon shutting down...";
    channel.answer(String::from("Quit")).expect("Error when channel.answer is used");
    exit_msg.logs(LOGFILE, "Daemon");
    println!("{}", exit_msg);
    let processes = procs.processes.clone();
    let processes_guard = processes.lock().unwrap();

    for (name, status) in processes_guard.iter() {
        let child_opt = {
            let status_guard = status.lock().unwrap();
            status_guard.child.clone()
        };
        if let Some(child_arc) = child_opt {
            let mut attempts = 0;
            let max_attempts = 10;
            let mut locked = false;

            while attempts < max_attempts {
                match child_arc.try_lock() {
                    Ok(mut child) => {
                        //println!("Successfully locked child_arc for program: {}", name);
                        let _ = child.kill();
                        let _ = child.wait();
                        locked = true;
                        break;
                    },
                    Err(_) => {
                        attempts += 1;
                        thread::sleep(Duration::from_millis(100));
                    },
                }
            }
            if !locked {
                eprintln!("Failed to acquire lock to stop program {}", name);
            }
        }
    }

    if std::fs::metadata(SOCK_PATH).is_ok() {
        std::fs::remove_file(SOCK_PATH).unwrap();
    }
    thread::sleep(Duration::from_secs(2));
    println!("Daemon Exit");
    process::exit(0);
}

fn handle_start(args: Vec<String>, channel: BidirectionalMessage, procs: &mut Procs) {
    if args.is_empty() {
        if let Err(e) = channel.answer("No program specified to start".to_string()) {
            eprintln!("Failed to send start response: {:?}", e);
        }
        return;
    }

    let mut response = String::new();
    for arg in args {
        if let Some(program) = procs.config.programs.get(&arg) {
            if is_program_running(arg.clone(), procs) {
                response.push_str(&format!("Program {} is already running.\n", arg));
            } else {
                let status = Arc::new(Mutex::new(Status::new(arg.clone(), String::from("starting"))));
                procs.status.retain(|s| s.lock().unwrap().name != arg);  // Remove any existing status with the same name
                procs.status.push(status.clone());
                if let Err(e) = start_process(arg.clone(), program.clone(), status.clone(), procs.processes.clone()) {
                    response.push_str(&format!("Failed to start program {}: {:?}\n", arg, e));
                } else {
                    response.push_str(&format!("Program {} started\n", arg));
                }
            }
        } else {
            response.push_str(&format!("Program {} not found in configuration\n", arg));
        }
    }
    if let Err(e) = channel.answer(response) {
        eprintln!("Failed to send start response: {:?}", e);
    }
}




fn handle_restart(args: Vec<String>, channel: BidirectionalMessage, procs: &mut Procs) {
    if args.is_empty() {
        channel.answer("No program specified to restart".to_string()).unwrap();
        return;
    }

    let mut response = String::new();

    // First, stop the programs and capture the list of successfully STOPPED programs
    for arg in args.clone() {
        let stop_response = stop_program_internal(vec![arg.clone()], procs);
        if !stop_response.is_empty() {
            response.push_str(&stop_response);
        }
    }
    for arg in args {
        if let Some(program) = procs.config.programs.get(&arg) {
            let status = Arc::new(Mutex::new(Status::new(arg.clone(), String::from("starting"))));
            procs.status.retain(|s| s.lock().unwrap().name != arg);  // Remove any existing status with the same name
            procs.status.push(status.clone());
            if let Err(e) = start_process(arg.clone(), program.clone(), status.clone(), procs.processes.clone()) {
                response.push_str(&format!("Failed to restart program {}: {:?}\n", arg, e));
            } else {
                response.push_str(&format!("Program {} restarted\n", arg));
            }
        } else {
            response.push_str(&format!("Program {} not found in configuration\n", arg));
        }
    }

    if let Err(e) = channel.answer(response) {
        eprintln!("Failed to send restart response: {:?}", e);
    }
}


fn is_program_running(name: String, procs: &Procs) -> bool {
    let processes_guard = procs.processes.lock().unwrap();
    processes_guard
        .iter()
        .any(|(key, status)| {
            key.starts_with(&name) && status.lock().unwrap().state == "RUNNING"
        })
}


fn handle_status(args: Vec<String>, channel: BidirectionalMessage, procs: &Procs) {
    let mut status_message = String::new();
    let processes_guard = procs.processes.lock().unwrap();

    if processes_guard.is_empty() {
        status_message.push_str("Nothing to display");
    } else {
        for (name, status) in processes_guard.iter() {
            let status_guard = status.lock().unwrap();

            let pid_str = status_guard.child
                .as_ref()
                .map_or("N/A".to_string(), |child| child.lock().unwrap().id().to_string());

            let start_time_str = status_guard.start_time
                .map_or("N/A".to_string(), |t| t.format("%Y-%m-%d %H:%M:%S").to_string());


            status_message.push_str(&format!(
                "\n{}       {}       {}      {}\n",
                status_guard.name, pid_str, status_guard.state, start_time_str
            ));
        }
    }

    if let Err(e) = channel.answer(status_message) {
        eprintln!("Failed to send status response: {:?}", e);
    }
}

fn system_time(time: SystemTime) -> DateTime<Local> {
    let datetime: DateTime<Utc> = time.into();
    datetime.with_timezone(&Local)
}


fn handle_reload(args: Vec<String>, channel: BidirectionalMessage, procs: &mut Procs) {
    let new_config = get_config();
    let mut programs_to_start = Vec::new();

    // Arrêter et supprimer les programmes qui ne sont plus dans la nouvelle configuration
    let old_programs: Vec<String> = procs.config.programs.keys().cloned().collect();
    for name in old_programs {
        if !new_config.programs.contains_key(&name) {
            let _ = stop_program_internal(vec![name.clone()], procs);
            procs.status.retain(|s| s.lock().unwrap().name != name);
            procs.processes.lock().unwrap().remove(&name);
        }
    }

    // Démarrer les nouveaux programmes et ceux qui ont autostart changé à true
    for (name, program) in &new_config.programs {
        if !procs.config.programs.contains_key(name) || 
           (procs.config.programs[name].autostart == false && program.autostart) {
            if program.autostart {
                programs_to_start.push((name.clone(), program.clone()));
            }
        }
    }

    // Mettre à jour la configuration globale
    procs.config = new_config;

    // Démarrer les programmes identifiés
    for (name, program) in programs_to_start {
        let status = Arc::new(Mutex::new(Status::new(name.clone(), String::from("starting"))));
        procs.status.push(status.clone());
        let _ = start_process(name, program, status, procs.processes.clone());
    }

    channel.answer(String::from("Configuration reloaded")).unwrap();
}

fn start_process(
    name: String,
    program_config: ProgramConfig,
    status: Arc<Mutex<Status>>,
    processes: Arc<Mutex<HashMap<String, Arc<Mutex<Status>>>>>
) -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..program_config.numprocs {
        let instance_name = format!("{}_{}", name, i + 1);

        let status_clone = Arc::new(Mutex::new(Status::new(instance_name.clone(), String::from("starting"))));
        let program_config_clone = program_config.clone();
        let processes_clone = processes.clone();

        thread::spawn(move || {
            let start_time = SystemTime::now();

            let stdout_file = match File::create(&program_config_clone.stdout) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to create stdout file for program {}: {:?}", instance_name, e);
                    return;
                }
            };

            let child = match process::Command::new(&program_config_clone.cmd)
                .args(&program_config_clone.args)
                .current_dir(&program_config_clone.workingdir)
                .stdout(Stdio::from(stdout_file))
                .spawn()
            {
                Ok(child) => child,
                Err(e) => {
                    eprintln!("Failed to start process for program {}: {:?}", instance_name, e);
                    return;
                }
            };

            {
                let mut status_guard = status_clone.lock().unwrap();
                status_guard.state = String::from("RUNNING");
                status_guard.start_time = Some(system_time(start_time));
                status_guard.child = Some(Arc::new(Mutex::new(child)));
            }

            {
                let mut processes_guard = processes_clone.lock().unwrap();
                processes_guard.insert(instance_name.clone(), status_clone.clone());
            }

            loop {
                let status_guard = status_clone.lock().unwrap();
                if status_guard.state == "STOPPED" {
                    break;
                }
                drop(status_guard);
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
    Ok(())
}

fn load_config(procs: &mut Procs) {
    procs.config = get_config();
    for (name, program) in &procs.config.programs {
        if program.autostart {
            let status = Arc::new(Mutex::new(Status::new(name.clone(), String::from("starting"))));
            procs.status.push(status.clone());
            start_process(name.clone(), program.clone(), status, procs.processes.clone());
        }
    }
}


fn main_process() {
    "Daemon is Up".logs(LOGFILE, "Daemon");
    if std::fs::metadata(SOCK_PATH).is_ok() {
        println!("A socket is already present. Delete with \"rm -rf {}\" before starting", SOCK_PATH);
        std::process::exit(0);
    }
    let (talk_to_daemon, rec_in_daemon): (Sender<BidirectionalMessage>, Receiver<BidirectionalMessage>) = mpsc::channel();
    thread::spawn(move || server::launch_server(talk_to_daemon.clone()));
    let mut procs: Procs = Procs::new();
    load_config(&mut procs);
    for receive in rec_in_daemon {
        let command: Command = serde_yaml::from_str(&(receive.message.to_string())).expect("Error when parsing command");
        match command.cmd.as_str() {
            "start" => handle_start(command.args, receive, &mut procs),
            "stop" => handle_stop(command.args, receive, &mut procs),
            "restart" => handle_restart(command.args, receive, &mut procs),
            "status" => handle_status(command.args, receive, &procs),
            "reload" => handle_reload(command.args, receive, &mut procs),
            _ => panic!("Unknown command: Parsing error"),
        }
    }
}

pub fn taskmasterd() {
    match fork() {
        Ok(Fork::Parent { .. }) => process::exit(0),
        Ok(Fork::Child) => {
            match daemon(true, true) {
                Ok(Fork::Parent(child)) => {
                    println!("Daemon PID : {}\n", child);
                    process::exit(0);
                },
                Ok(Fork::Child) => main_process(),
                Err(_) => panic!("Daemonization failed"),
            }
        }
        Err(_) => panic!("Fork failed"),
    }
}
