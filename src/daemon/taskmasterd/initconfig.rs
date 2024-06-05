/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   initconfig.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/06/04 01:03:01 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/06 01:30:52 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod parsing;
pub mod checker;

// use umask::Umask;
// use umask::ToUmask;

use checker::{Schecker, Uchecker};
use parsing::ProgramConfig;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    programs: HashMap<String, ProgramConfig>,
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

fn deserialize_config(filename: &str) -> Config {
    let yaml = std::fs::read_to_string(filename).expect("Failed to read YAML file");
    serde_yaml::from_str(&yaml).expect("Failed to parse YAML : \n")
}


pub fn load_configs() {
    let mut config = deserialize_config("./confs/taskmaster_confs.yaml");
    check_config(& mut config);
    println!("{:#?}", config);
    
    // for prog in config.programs.values_mut() {
        
    // }
    // format!("Configuration File : ./confs/taskmaster_confs.yaml loaded").logs(LOGFILE, "Daemon");
}