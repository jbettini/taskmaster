/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/26 05:11:11 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/04 23:12:22 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod bidirmsg;
pub mod logfile;

use bidirmsg::BidirectionalMessage;
use std::sync::mpsc::Sender;
use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::fs::File;
use std::thread;
use std::io;
use logfile::SaveLog;
use std::os::unix::io::AsRawFd;
use std::sync::Mutex;
use std::time::Duration;

const LOGFILE:&'static str = "/Users/xtem/Desktop/Taskmaster/confs/logfile";
const SOCK_PATH: &'static str = "/Users/xtem/Desktop/Taskmaster/confs/mysocket.sock";

// fn handle_client_stream(mut unix_stream: UnixStream, daemon: Sender<BidirectionalMessage>) -> Result<bool, io::Error> {
fn handle_client_stream(mut unix_stream: UnixStream, daemon: Mutex<Sender<BidirectionalMessage>>) -> Result<bool, io::Error> {
    // TODO : init only one time buffer for a better memory complexity 
    let client_id = format!("Client {}",&unix_stream.as_raw_fd());
    loop {
        let mut buffer = [0; 4096];
        unix_stream.flush().expect("Flush Failed");
        let _n = match unix_stream.read(& mut buffer) {
            Ok(_n)  => {
                if _n == 0 {
                    format!("{} Disconnected", client_id).logs(LOGFILE, &client_id);
                    break ;
                } else {
                    // #send to daemon
                    let string = String::from_utf8_lossy(&buffer).to_string();
                    string.logs(LOGFILE, &client_id);
                    // TODO : do a mutex here
                    let daemon  = daemon.lock().unwrap();
                    let ret = BidirectionalMessage::load_bidirectional_message(string.trim_matches('\0').to_string(), daemon.clone());
                    // let ret = BidirectionalMessage::load_bidirectional_message(string.trim_matches('\0').to_string(), daemon.clone());
                    ret.logs(LOGFILE, "Daemon");
                    // #answer to client 
                    unix_stream
                        .write(ret.as_bytes())
                        .expect("Failed at writing onto the unix stream");
                    // buffer.fill();
                    thread::sleep(Duration::from_secs(10));
                }
            }
            Err (_err) => panic!("Error reading"),
        };
    }
    
    println!("client number : {} shutting down", unix_stream.as_raw_fd());
    Ok(true)
}

pub fn launch_server(talk_to_daemon: Sender<BidirectionalMessage>) {
    if std::fs::metadata(LOGFILE).is_err() {
        File::create(LOGFILE).expect("Failed to create the logfile");
    }
    let unix_listener = UnixListener::bind(SOCK_PATH)
            .expect("Could not create the unix socket");
    "Server is up and ready to accept connections".logs(LOGFILE, "Daemon");
    loop {
        println!("Waiting for new connection...");
        let (unix_stream, _socket_address) = unix_listener
            .accept()
            .expect("Failed at accepting a connection on the unix listener");
        format!("New client connected with the fd {}", &unix_stream.as_raw_fd()).logs(LOGFILE, "Daemon");
        // let talk_to_daemon_clone = talk_to_daemon.clone(); 
        // thread::spawn(move || handle_client_stream(unix_stream, talk_to_daemon_clone).expect("Failed to handle stream"));
        let mutex_talk_to_daemon = Mutex::new(talk_to_daemon.clone());
        thread::spawn(move || handle_client_stream(unix_stream, mutex_talk_to_daemon).expect("Failed to handle stream"));

        // # archictetural choice : If we want only 1 client just uncomment and get the code out of the loop
        // .join()
        // .expect("Failed to create a thread for the client connexion");
    }
}