/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   logfile.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/31 16:35:57 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/06 02:34:41 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const LOGFILE:&'static str = "/Users/xtem/Desktop/Taskmaster/confs/logfile";

pub trait SaveLog {
    fn logs(&self, filename: &str, from: &str);
}

impl SaveLog for String {
    fn logs(&self, filename: &str, from: &str) {
        if std::fs::metadata(LOGFILE).is_err() {
            File::create(LOGFILE).expect("Failed to create the logfile");
        }
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(filename)
            .expect("Couldn't open logfile");
        let prompt = format!("\n{} From {} {}\n", "#".repeat(15),from,"#".repeat(15));
        file.write(prompt.as_bytes()).unwrap();
        file.write(self.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}

impl SaveLog for &str {
    fn logs(&self, filename: &str, from: &str) {
        if std::fs::metadata(LOGFILE).is_err() {
            File::create(LOGFILE).expect("Failed to create the logfile");
        }
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(filename)
            .expect("Couldn't open logfile");
        let prompt = format!("\n{} From {} {}\n", "#".repeat(15),from,"#".repeat(15));
        file.write(prompt.as_bytes()).unwrap();
        file.write(self.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}

impl SaveLog for str {
    fn logs(&self, filename: &str, from: &str) {
        if std::fs::metadata(LOGFILE).is_err() {
            File::create(LOGFILE).expect("Failed to create the logfile");
        }
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(filename)
            .expect("Couldn't open logfile");
        let prompt = format!("\n{} From {} {}\n", "#".repeat(15),from,"#".repeat(15));
        file.write(prompt.as_bytes()).unwrap();
        file.write(self.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}