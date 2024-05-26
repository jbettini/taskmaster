/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   server.rs                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/26 05:11:11 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/26 05:26:37 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write};
use std::{thread, time::Duration};
use ctrlc;
use std::io;

fn handle_stream(mut unix_stream: UnixStream) -> Result<bool, io::Error> {
    let mut buffer = [0; 256];
    loop {
        unix_stream.flush().expect("Flush Failed");
        let _n = match unix_stream.read(& mut buffer) {
            Ok(_n)  => {
                if _n == 0 {
                    break ;
                } else {
                    print!("{}", String::from_utf8_lossy(&buffer));
                }
            }
            Err (_err) => panic!("Error reading"),
        };
    
    }
    Ok(true)
}

pub fn launch_server() {
    let socket_path = "/Users/xtem/test/unixServer/sock/mysocket.sock";
    ctrlc::set_handler(move || {
        println!("received Ctrl+C\nDeleting files...");
        if std::fs::metadata(socket_path).is_ok() {
            std::fs::remove_file(socket_path).expect("Failed to remove socket");
        }
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    if std::fs::metadata(socket_path).is_ok() {
        println!("A socket is already present. Delete {} before starting", socket_path);
        std::process::exit(0);
    }

    let unix_listener = UnixListener::bind(socket_path)
            .expect("Could not create the unix socket");
    loop {
        println!("Waiting for new connection...");
        let (mut unix_stream, _socket_address) = unix_listener
            .accept()
            .expect("Failed at accepting a connection on the unix listener");
        println!("New connection accepted");
        // need to bee handled in a thread or a process
        handle_stream(unix_stream).expect("Failed to handle stream");
    }
}
