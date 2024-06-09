/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   initconfig.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/06/04 01:03:01 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/06 02:40:19 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod parsing;
pub mod checker;

// use umask::Umask;
// use umask::ToUmask;

use checker::{Schecker, Uchecker};
use parsing::ProgramConfig;
use std::{collections::HashMap, sync::{Arc, Mutex}, time::SystemTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub programs: HashMap<String, ProgramConfig>,
}


pub struct Procs {
    pub config: Config,
    pub status: Vec<Arc<Mutex<Status>>>,
}

impl Procs {
    pub fn new() -> Self {
        Procs {
            config: get_config(),
            status: Vec::new(),
        }
    }
}

pub struct Status {
    pub name: String,
    pub state: String,
    pub start_time: Option<SystemTime>,
}

impl Status {
    pub fn new (name: String, state: String) -> Self {
        Status {
            name,
            state,
            start_time: None,
        }
    }
}


fn check_config(config: & mut Config) {
    for prog in config.programs.values_mut() {
        prog.cmd.trim_assign();
        prog.workingdir.trim_assign();
        prog.umask.check_umask();
        prog.autorestart.check_autorestart();
        prog.stopsignal.check_stopsignal();
        prog.numprocs.u32_field_checker();
        prog.startretries.u32_field_checker();
        prog.stoptime.u32_field_checker();
    }
}

pub fn get_config() -> Config {
    let yaml_path =  "/home/ramzi/Desktop/Taskmaster/confs/taskmaster_confs.yaml";
    let yaml = std::fs::read_to_string(yaml_path).expect("Failed to read YAML file");
    let mut config = serde_yaml::from_str(&yaml).expect("Failed to parse YAML : \n");
    check_config(& mut config);
    config
}