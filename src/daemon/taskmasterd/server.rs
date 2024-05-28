/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/26 05:11:11 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/28 14:04:48 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod bidirmsg;

use bidirmsg::BidirectionalMessage;
use std::sync::mpsc::Sender;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::thread;
use std::io;

fn handle_client_stream(mut unix_stream: UnixStream, daemon: Sender<BidirectionalMessage>) -> Result<bool, io::Error> {
    let mut buffer = [0; 256];
    loop {
        unix_stream.flush().expect("Flush Failed");
        let _n = match unix_stream.read(& mut buffer) {
            Ok(_n)  => {
                if _n == 0 {
                    break ;
                } else {
                    // handle input
                    let string = String::from_utf8_lossy(&buffer).to_string();
                    let ret = BidirectionalMessage::load_bidirectional_message(string, daemon.clone());
                    print!("\n\n-------------------\n\nreceive in handle client stream {}\n\n----------\n\n", ret);
                }
            }
            Err (_err) => panic!("Error reading"),
        };
    
    }
    println!("client shutdown: \n---------\n{:?}\n---------\n", unix_stream);
    Ok(true)
}

pub fn launch_server(target: Sender<BidirectionalMessage>) {
    let socket_path = "/Users/xtem/Desktop/Taskmaster/confs/mysocket.sock";
    if std::fs::metadata(socket_path).is_ok() {
        println!("A socket is already present. Delete with \"rm -rf {}\" before starting", socket_path);
        std::process::exit(0);
    }
    let unix_listener = UnixListener::bind(socket_path)
            .expect("Could not create the unix socket");
    loop {
        println!("Waiting for new connection...");
        let (unix_stream, _socket_address) = unix_listener
            .accept()
            .expect("Failed at accepting a connection on the unix listener");
        println!("New connection accepted");
        let targetclone = target.clone(); 
        thread::spawn(move || handle_client_stream(unix_stream, targetclone).expect("Failed to handle stream"));
        // # archictetural choice if we want to have only one client at the same time 
        // .join()
        // .expect("Failed to create a thread for the client connexion");
    }
}


    // # handler for ctrl
    // use ctrlc
    // ctrlc::set_handler(move || {
    //     println!("received Ctrl+C: Deleting files...");
    //     if std::fs::metadata(socket_path).is_ok() {
    //         std::fs::remove_file(socket_path).expect("Failed to remove socket");
    //         println!("Server Shutdowned");
    //     }
    //     std::process::exit(0);
    // }).expect("Error setting Ctrl-C handler");