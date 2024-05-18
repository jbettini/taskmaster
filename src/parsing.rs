/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parsing.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/17 03:29:48 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/18 21:59:59 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod parsing {
    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};
    use std::fs::File;
    use std::io::Read;
 
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ProgramConfig {
        pub command: String, 
        pub numofprocs: usize,
        pub autostart: bool,
        pub autorestart: String,
        pub exitcodes: Vec<i32>,
        pub starttime: u32,
        pub startretry: u32,
        pub stopsignal: String,
        pub timetostop: u32,
        pub stdout: Option<String>,
        pub stderr: Option<String>,
        pub env: Option<HashMap<String, String>>,
        pub workdir: Option<String>,
        pub umask: Option<String>,
    }   

    impl ProgramConfig {
        pub fn new(filename: &str) -> Vec<Self> {
            let mut file = File::open(filename) 
                .expect("Failed to open the confs file");
       
            let mut buf: String = String::new();
            file.read_to_string(&mut buf)
                .expect("Failed to Read to read the confs file");

            let confs: Vec<ProgramConfig> = serde_yaml::from_str(&buf)
                .expect("Failed to Serialise the confs file");
            confs
        }          
}   
}   
   