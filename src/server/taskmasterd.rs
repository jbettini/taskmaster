/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterd.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:06:23 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/26 06:10:15 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod parsing; 
mod server;
use parsing::ProgramConfig;

pub fn taskmasterd() {
    server::launch_server();
    // #need to set it in the daemon
    // let configs :Vec<ProgramConfig> = ProgramConfig::new("./confs/taskmaster_confs.yaml");
    // for config in configs {
    //     println!("{:?}\n", config);
    // } 
}
