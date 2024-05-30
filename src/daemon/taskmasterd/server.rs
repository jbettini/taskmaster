/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/26 05:11:11 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/30 07:44:48 by jbettini         ###   ########.fr       */
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
    unix_stream.flush().expect("Flush Failed");
    let _n = match unix_stream.read(& mut buffer) {
        Ok(_n)  => {
            if _n == 0 {
                println!("client shutdown: \n---------\n{:?}\n---------\n", unix_stream);
            } else {
                // #send to daemon
                let string = String::from_utf8_lossy(&buffer).to_string();
                let ret = BidirectionalMessage::load_bidirectional_message(string.trim_matches('\0').to_string(), daemon.clone());
                // #answer to client 
                unix_stream
                    .write(ret.as_bytes())
                    .expect("Failed at writing onto the unix stream");
            }
        }
        Err (_err) => panic!("Error reading"),
    };
    println!("client shutdown: \n---------\n{:?}\n---------\n", unix_stream);
    Ok(true)
}

pub fn launch_server(talk_to_daemon: Sender<BidirectionalMessage>) {
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
        let talk_to_daemon = talk_to_daemon.clone(); 
        thread::spawn(move || handle_client_stream(unix_stream, talk_to_daemon).expect("Failed to handle stream"));
        // # archictetural choice if we want to have only one client at the same time 
        // .join()
        // .expect("Failed to create a thread for the client connexion");
    }
}