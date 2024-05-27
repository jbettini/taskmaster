/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterd.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:06:23 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/27 14:08:08 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod parsing; 
mod server;
// use parsing;
use fork::{daemon, fork, Fork};
use std::process;
use std::{thread, time};

fn load_configs() {
    let configs :parsing::Config = parsing::Config::new("./confs/taskmaster_confs.yaml");
    println!("\n------------------\n\n{:#?}\n\n------------------\n", configs);
}

fn main_process() {
    // #set all the configs before listen client command and connexion
    load_configs();
    loop {
         let time_to_sleep = time::Duration::from_millis(5000);
         println!("Hello from daemon");
         thread::sleep(time_to_sleep);
    }
    //server::launch_server(),
}


pub fn taskmasterd() {
    match fork() {
        Ok(Fork::Parent { .. }) => process::exit(0),
        Ok(Fork::Child) => {
            match daemon(true, true) {
                Ok(Fork::Parent(child)) => {
                    println!("Starting daemon with this : {} PID", child);
                    process::exit(0);
                },
                Ok(Fork::Child) => main_process(),
                Err(_) => panic!("Daemonization failed"),
            }
        }
        Err(_) => panic!("Fork failed"),
    }
}
