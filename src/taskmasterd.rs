/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   taskmasterd.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/19 01:06:23 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/19 01:18:45 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod parsing;
use parsing::ProgramConfig;

pub fn launch() {
    let configs :Vec<ProgramConfig> = ProgramConfig::new("./confs/taskmaster_confs.yaml");
    for config in configs {
        println!("{:?}\n", config);
    }
}
