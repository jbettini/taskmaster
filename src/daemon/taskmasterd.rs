/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterd.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:06:23 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/28 13:40:34 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod parsing;
pub mod server;

use server::bidirmsg::BidirectionalMessage;
use fork::{daemon, fork, Fork};
use std::process;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

fn load_configs() {
    let configs :parsing::Config = parsing::Config::new("./confs/taskmaster_confs.yaml");
    println!("\n------------------\n\n{:#?}\n\n------------------\n", configs);
}

fn main_process() {
    // #set all the configs before listen client command and connexion
    
    // load_configs();

    let (talk_to_main, rec_in_main): (Sender<BidirectionalMessage>, Receiver<BidirectionalMessage>) = mpsc::channel();
    thread::spawn(move || server::launch_server(talk_to_main.clone()));
    for msg in rec_in_main {
        // msg = command by client 
        println!("\n\n-------------------\n\nreceive in daemon : \n {}\n\n-------------------\n\n", msg.message);
        // return for the client 
        msg.answer("Ferme ta geule".to_string()).unwrap();
    }

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
