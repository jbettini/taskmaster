/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterd.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:06:23 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/06 01:35:20 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod server;
pub mod command;
pub mod initconfig;
// pub mod taskmasterctl;

const SOCK_PATH:&'static str = "/Users/xtem/Desktop/Taskmaster/confs/mysocket.sock";
const LOGFILE:&'static str = "/Users/xtem/Desktop/Taskmaster/confs/logfile";

use server::logfile::SaveLog;
use command::Command;
use server::bidirmsg::BidirectionalMessage;
use fork::{daemon, fork, Fork};
use std::{thread, process, time};
use std::sync::mpsc::{self, Sender, Receiver};


fn handle_stop(args: Vec<String>, channel :BidirectionalMessage) {
    if args.is_empty() {
        let exit_msg = "Daemon shutting down...";
        channel.answer(String::from("Quit")).expect("Error when channel.answer is used");
        exit_msg.logs(LOGFILE, "Daemon");
        println!("{}", exit_msg);
        if std::fs::metadata(SOCK_PATH).is_ok() {
            std::fs::remove_file(SOCK_PATH).unwrap();
        }
        thread::sleep(time::Duration::from_secs(2));
        println!("Daemon Exit");
        process::exit(0);
    } else {
        println!("Hello from stop fun because stop args is not empty");
    }
}

fn handle_start(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from start fun")).unwrap();
}

fn handle_restart(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from restart fun")).unwrap();
}

fn handle_status(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from status fun")).unwrap();
}

fn handle_reload(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from reload fun")).unwrap();
}

fn main_process() {
    // #set all the configs before listen client command and connexion
    initconfig::load_configs();
    "Daemon is Up".logs(LOGFILE, "Daemon");
    if std::fs::metadata(SOCK_PATH).is_ok() {
        println!("A socket is already present. Delete with \"rm -rf {}\" before starting", SOCK_PATH);
        std::process::exit(0);
    }
    let (talk_to_daemon, rec_in_daemon): (Sender<BidirectionalMessage>, Receiver<BidirectionalMessage>) = mpsc::channel();
    thread::spawn(move || server::launch_server(talk_to_daemon.clone()));
    for receive in rec_in_daemon{
        // println!("{:?}", receive.message);
        let command: Command = serde_yaml::from_str(&(receive.message.to_string())).expect("Error when parsing command");
        match command.cmd.as_str() {
            "start" => handle_start(command.args, receive),
            "stop" =>  handle_stop(command.args, receive),
            "restart" =>  handle_restart(command.args, receive),
            "status" =>  handle_status(command.args, receive),
            "reload" =>  handle_reload(command.args, receive),
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
