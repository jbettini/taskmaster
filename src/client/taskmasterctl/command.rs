/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   command.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/26 05:06:43 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/30 07:49:39 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use serde::{Serialize, Deserialize};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};


#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
}

impl Command {
    fn new () -> Self {
        Command {
            cmd: String::new(),
            args: Vec::new(),
        }
    }
    
    
    fn to_vec_of_strings(v: Vec<&str>) -> Vec<String> {
        let mut ret:Vec<String> = Vec::new();
        for strs in v {
            ret.push(strs.to_string());
        }
        ret
    }
    
    pub fn parse(line: String) -> Option<Command> {
        let splited_input: Vec<&str> = line.split_whitespace().collect();
        let cmd = match splited_input.get(0) {
            Some(cmd) => {
                if ["start", "stop", "restart", "reload", "status", "exit", "help"].contains(cmd) {
                    let mut result = Self::new();
                    result.cmd = cmd.to_string();
                    result.args = Self::to_vec_of_strings(splited_input[1..].to_vec());
                    Some(result)
                } else {
                    None
                }
            }
            _ => None,
        };
        cmd
    }

    pub fn exchange_with_server(mut unix_stream: &UnixStream, buf: String) {
        unix_stream
            .write(buf.as_bytes())
            .expect("Failed at writing onto the unix stream");
        unix_stream
            .shutdown(std::net::Shutdown::Write)
            .expect("Could not shutdown writing on the stream");
        let mut response = String::new();
        unix_stream
            .read_to_string(&mut response)
            .expect("Failed at reading the unix stream");
        drop(unix_stream);
        // edit answer
        println!("We received this response: {}", response);
    }

    pub fn handle_cmd(&mut self, mut unix_stream: & UnixStream) {
        // # handle Args
        if ["start", "restart"].contains(&self.cmd.as_str()) && self.args.len() <= 0 {
            println!("{} command need to take at least one argument, please retry", self.cmd);
        } else if self.cmd == "reload" && self.args.len() != 0 {
            println!("reload command does not take arguments, please retry");
        } else {
            // #handle cmd
            let cmd_yaml: String = serde_yaml::to_string(&self).expect("Program crash when cmd is formatted in yaml");
            Command::exchange_with_server(unix_stream, cmd_yaml);
        }
    }
}

