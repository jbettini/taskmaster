/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   command.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/26 05:06:43 by jbettini          #+#    #+#             */
/*   Updated: 2024/06/04 19:06:36 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new () -> Self {
        Command {
            cmd: String::new(),
            args: Vec::new(),
        }
    }
}

