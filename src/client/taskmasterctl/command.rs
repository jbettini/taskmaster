/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   command.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/26 05:06:43 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/01 09:23:07 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use serde::{Serialize, Deserialize};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process;


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
        println!("Before : {}", buf);
        unix_stream
            .write(buf.as_bytes())
            .expect("Failed at writing onto the unix stream");
        unix_stream.flush()
            .expect("Flush Failed");
        let mut buffer = [0; 4096];
        unix_stream
            .read(& mut buffer)
            .expect("Failed at reading the unix stream");
        let response = String::from_utf8_lossy(&buffer).to_string().trim_matches('\0').to_string();
        match response.as_str() {
            "Quit" => {
                println!("{}", response);
                process::exit(0);
            },
            _ => println!("{}", response),
        }
        println!("We received this response: {:?}", response);
    }

    pub fn handle_cmd(&mut self, mut unix_stream: & UnixStream) {
        // # handle Args
        if ["start", "restart"].contains(&self.cmd.as_str()) && self.args.len() <= 0 {
            println!("{} command need to take at least one argument, please retry", self.cmd);
        } else if self.cmd == "reload" && self.args.len() != 0 {
            println!("reload command does not take arguments, please retry");
        } else {
            // #handle cmd
            let cmd_yaml: String = serde_yaml::to_string(&self)
                .expect("Program crash when cmd is formatted in yaml");
            Command::exchange_with_server(unix_stream, cmd_yaml);
        }
    }
}

