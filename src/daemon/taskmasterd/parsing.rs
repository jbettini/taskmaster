/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parsing.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/17 03:29:48 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/30 07:03:56 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// use std::collections::HashMap;
// use serde::{Serialize, Deserialize};
// use std::fs::File;
// use std::io::Read;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    programs: HashMap<String, ProgramConfig>,
}

impl Config {
    pub fn new(filename: &str) -> Self {
        let yaml = std::fs::read_to_string(filename).expect("Failed to read YAML file");
        let config: Config = serde_yaml::from_str(&yaml).expect("Failed to parse YAML : \n");
        config
    }          
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramConfig {
    pub cmd: String, 
    pub numprocs: usize,
    pub umask: Option<String>,
    pub workingdir: Option<String>,
    pub autostart: bool,
    pub autorestart: String,
    pub exitcodes: Vec<i32>,
    pub startretries: u32,
    pub starttime: u32,
    pub stopsignal: String,
    pub stoptime: u32,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub env: Option<HashMap<String, String>>,
}