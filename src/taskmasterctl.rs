/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterctl.rs                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:19:09 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/19 05:29:01 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::{self ,Write ,stdin};

fn get_input() -> String {
    print!("$ ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error when reading on stdin");
    input
}

fn print_help() {
    println!(
        "Commands accepted:
         - help => print all the accepted commands
         - status => show programs status
         - reload => reload the main program
         - start \"program name\" => starting a program
         - restart \"program name\" => restarting a program
         - stop \"program name\" => stopping a program
         - stop \"daemon\" => stopping the main program"
    );
}


fn treat_args_commands(strs: Vec<&str>, command_name: &str) {
    if command_name == "start" || command_name == "restart" || command_name == "stop" {
        if strs.len() < 2 {
            println!("Error: {command_name} command needs at least one argument");
        } else {
            launch_command(strs[1..].to_vec(), command_name);
        }
    } else if command_name == "reload" || command_name == "status" {
        if strs.len() != 1 {
            println!("Error: {command_name} command should have no arguments");
        } else {
            launch_command(strs, command_name);
        }
    } else {
        panic!("Command not found: Parsing error");
    }
}

fn launch_command(args: Vec<&str>, command_name: &str) {
    match command_name {
        "start" => start_command(args),
        "restart" => restart_command(args),
        "stop" => stop_command(args),
        "reload" => reload_command(),
        "status" => status_command(),
        _ => panic!("Unknown command: {command_name}: Parsing error"),
    }
}


fn start_command(args: Vec<&str>) {
    println!("start command loaded with this args : {:?}", args);
}

fn restart_command(args: Vec<&str>) {
    println!("restart command loaded with this args : {:?}", args);
}

fn stop_command(args: Vec<&str>) {
    println!("stop command loaded with this args : {:?}", args);
}

fn reload_command() {
    println!("reload command loaded");
}

fn status_command() {
    println!("status command loaded");
}


pub fn launch() {
    loop {
        let input: String = get_input();
        let splited_input: Vec<&str> = input.split_whitespace().collect();
        if input.is_empty() {
            println!("Quit");
            break;
        }
        match splited_input.get(0) {
            Some(&"start") => treat_args_commands(splited_input, "start"),
            Some(&"restart") => treat_args_commands(splited_input, "restart"),
            Some(&"stop") => treat_args_commands(splited_input, "stop"),
            Some(&"status") => treat_args_commands(splited_input, "status"),
            Some(&"reload") => treat_args_commands(splited_input, "reload"),
            Some(&"help") => print_help(),
            None => println!("No command entered, use help!"),
            _ => println!("incorrect command :{} :Use help !", splited_input.get(0).unwrap()),
        }
    }
}