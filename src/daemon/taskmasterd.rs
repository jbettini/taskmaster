/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterd.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:06:23 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/30 07:49:08 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod parsing;
pub mod server;
pub mod command;
// pub mod taskmasterctl;

use parsing::Config;
use command::Command;
use serde::{Deserialize, Serialize};
use server::bidirmsg::BidirectionalMessage;
use fork::{daemon, fork, Fork};
use std::process;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

// fn load_configs() {
//     let configs: Config = Config::new("/Users/xtem/Desktop/Taskmaster/confs/taskmaster_confs.yaml");
     // if configs. {
//         println!("\n------------------\n\n{:#?}\n\n------------------\n", configs2);
        
//     // }
// }

fn handle_start(args: Vec<String>) {
    println!("Hello from start fun");
}

fn handle_stop(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from stop"));
    println!("Hello from stop fun");
}

fn handle_restart(args: Vec<String>) {
    println!("Hello from restart fun");
}

fn handle_status(args: Vec<String>, ) {
    println!("Hello from status fun");
}

fn handle_reload(args: Vec<String>) {
    println!("Hello from reload fun");
}

fn main_process() {
    // #set all the configs before listen client command and connexion
    
    // load_configs();

    let (talk_to_daemon, rec_in_daemon): (Sender<BidirectionalMessage>, Receiver<BidirectionalMessage>) = mpsc::channel();
    thread::spawn(move || server::launch_server(talk_to_daemon.clone()));
    
    loop {
        let receive = rec_in_daemon.recv().expect("Error when recv in daemon");
        // println!("\n\n------\n{:?}\n------\n\n", &receive.message);
        // println!("Before the crash");
        let command: Command = serde_yaml::from_str(&(receive.message.to_string())).expect("Error when parsing command");
        match command.cmd.as_str() {
            "start" => handle_start(command.args),
            "stop" =>  handle_stop(command.args, receive),
            "restart" =>  handle_restart(command.args),
            "status" =>  handle_status(command.args),
            "reload" =>  handle_reload(command.args),
            _ => panic!("Unknown command: Parsing error"),
        }
        // // receive.answer("OKKK".to_string());
    }
    // for channel in rec_in_main {
    //     println!("channel : {}", channel.message);
    //     // let command: Command = serde_yaml::from_str(&channel.message).expect("Failed to parse YAML CMD\n");
    //     // println!("Weshhhhh {:#?}", command);
    //     // match command.cmd.as_str() {
    //     //     "start" => handle_start(command.args),
    //     //     "stop" =>  handle_stop(command.args, channel),
    //     //     "restart" =>  handle_restart(command.args),
    //     //     "status" =>  handle_status(command.args),
    //     //     "reload" =>  handle_reload(command.args),
    //     //     _ => panic!("Unknown command: Parsing error"),
    //     // }
    //     // return for the client 
    // }

}


pub fn taskmasterd() {
    match fork() {
        Ok(Fork::Parent { .. }) => process::exit(0),
        Ok(Fork::Child) => {
            match daemon(false, true) {
                Ok(Fork::Parent(child)) => {
                    println!("Daemon PID : {}", child);
                    process::exit(0);
                },
                Ok(Fork::Child) => main_process(),
                Err(_) => panic!("Daemonization failed"),
            }
        }
        Err(_) => panic!("Fork failed"),
    }
}
