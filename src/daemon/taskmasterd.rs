/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterd.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:06:23 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/06 15:35:17 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod server;
pub mod command;
pub mod initconfig;
// pub mod taskmasterctl;

const SOCK_PATH: &'static str = "/home/ramzi/Desktop/Taskmaster/confs/mysocket.sock";
const LOGFILE: &'static str = "/home/ramzi/Desktop/Taskmaster/confs/logfile";

use initconfig::parsing::ProgramConfig;
use initconfig::{get_config, Procs, Status};
use server::logfile::SaveLog;
use command::Command;
use server::bidirmsg::BidirectionalMessage;
use fork::{daemon, fork, Fork};
use std::time::{Duration, SystemTime};
use std::{thread, process, time};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


fn handle_stop(args: Vec<String>, channel: BidirectionalMessage) {
    if args.is_empty() {
        let exit_msg = "Daemon shutting down...";
        channel.answer(String::from("Quit")).expect("Error when channel.answer is used");
        exit_msg.logs(LOGFILE, "Daemon");
        println!("{}", exit_msg);
        if std::fs::metadata(SOCK_PATH).is_ok() {
            std::fs::remove_file(SOCK_PATH).unwrap();
        }
        thread::sleep(Duration::from_secs(2));
        println!("Daemon Exit");
        process::exit(0);
    } else {
        println!("Hello from stop fun because stop args is not empty");
    }
}

fn handle_start(args: Vec<String>, channel: BidirectionalMessage, procs: &mut Procs) {
    let mut response = String::new();
    for arg in args {
        if let Some(program) = procs.config.programs.get(&arg) {
            let status = Arc::new(Mutex::new(Status::new(arg.clone(), String::from("starting"))));
            procs.status.push(status.clone());
            start_process(arg.clone(), program.clone(), status);
            response.push_str(&format!("Program {} started\n", arg));
        } else {
            response.push_str(&format!("Program {} not found in configuration\n", arg));
        }
    }
    channel.answer(response).unwrap();
}

fn handle_restart(args: Vec<String>, channel: BidirectionalMessage, procs: &mut Procs) {
    handle_stop(args.clone(), channel.clone());
    handle_start(args.clone(), channel.clone(), procs);
}

fn handle_status(args: Vec<String>, channel: BidirectionalMessage, procs: &Procs) {
    let mut status_message = String::new();
    for status in &procs.status {
        let status_guard = status.lock().unwrap();
        let start_time_str = status_guard.start_time
            .map_or("N/A".to_string(), |t| {
                match t.elapsed() {
                    Ok(duration) => format_duration(duration),
                    Err(_) => "unknown".to_string(),
                }
            });
        status_message.push_str(&format!(
            "{}     {}  since ->    {}\n",
            status_guard.name, status_guard.state, start_time_str
        ));
    }
    channel.answer(status_message).unwrap();
}

fn handle_reload(args: Vec<String>, channel: BidirectionalMessage) {
    channel.answer(String::from("Hello from reload fun")).unwrap();
}

fn start_process(name: String, program_config: ProgramConfig, status: Arc<Mutex<Status>>) {
    thread::spawn(move || {
        loop {
            let start_time = SystemTime::now();

            let mut child = process::Command::new(&program_config.cmd)
                //.args(&program_config.args)
                .current_dir(&program_config.workingdir)
                .spawn()
                .expect("Failed to start process");

            {
                let mut status_guard = status.lock().unwrap();
                status_guard.state = String::from("running");
                status_guard.start_time = Some(start_time);
            }

            let status_result = child.wait().expect("Failed to wait on child");

            let uptime = match start_time.elapsed() {
                Ok(duration) => format_duration(duration),
                Err(_) => String::from("unknown"),
            };

            {
                let mut status_guard = status.lock().unwrap();
                if !program_config.exitcodes.contains(&status_result.code().unwrap_or(-1)) {
                    status_guard.state = String::from("stopped unexpectedly");
                    if program_config.autorestart == "always" || program_config.autorestart == "unexpected" {
                        continue;
                    }
                } else {
                    status_guard.state = String::from("stopped");
                }
                status_guard.start_time = Some(SystemTime::now() - Duration::new(uptime.parse::<u64>().unwrap_or(0), 0));
            }
            break;
        }
    });
}

fn load_config(procs: &mut Procs) {
    procs.config = get_config();
    for (name, program) in &procs.config.programs {
        if program.autostart {
            let status = Arc::new(Mutex::new(Status::new(name.clone(), String::from("starting"))));
            procs.status.push(status.clone());
            start_process(name.clone(), program.clone(), status);
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    format!("{}d, {}h, {}m, {}s", days, hours % 24, minutes % 60, secs % 60)
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
            "stop" => handle_stop(command.args, receive),
            "restart" => handle_restart(command.args, receive, &mut procs),
            "status" => handle_status(command.args, receive, &procs),
            "reload" => handle_reload(command.args, receive),
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
