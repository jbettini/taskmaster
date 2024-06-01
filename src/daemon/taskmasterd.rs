/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterd.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:06:23 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/01 09:44:52 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod parsing;
pub mod server;
pub mod command;
// pub mod taskmasterctl;

const SOCK_PATH:&'static str = "/Users/xtem/Desktop/Taskmaster/confs/mysocket.sock";
const LOGFILE:&'static str = "/Users/xtem/Desktop/Taskmaster/confs/logfile";

use server::logfile::SaveLog;
use parsing::Config;
use command::Command;
use serde::{Deserialize, Serialize};
use server::bidirmsg::BidirectionalMessage;
use fork::{daemon, fork, Fork};
use std::process;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::io::stdin;

fn load_configs() {
    let configs: Config = Config::new("/Users/xtem/Desktop/Taskmaster/confs/taskmaster_confs.yaml");
    format!("Configuration File : /Users/xtem/Desktop/Taskmaster/confs/taskmaster_confs.yaml loaded").logs(LOGFILE, "Daemon");
}

fn handle_start(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from start fun"));
}

fn handle_stop(args: Vec<String>, channel :BidirectionalMessage) {
    if args.is_empty() {
        let exit_msg = "Daemon shutting down...";
        channel.answer(String::from("Quit"));
        exit_msg.logs(LOGFILE, "Daemon");
        println!("{}", exit_msg);
        std::fs::remove_file(SOCK_PATH);
        process::exit(0);
    } else {
        println!("Hello from stop fun because stop args is not empty");
    }
}

fn handle_restart(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from restart fun"));
}

fn handle_status(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from status fun"));
}

fn handle_reload(args: Vec<String>, channel :BidirectionalMessage) {
    channel.answer(String::from("Hello from reload fun"));
}

fn main_process() {
    // #set all the configs before listen client command and connexion
    // load_configs();
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
