/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   bidirmsg.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jbettini <jbettini@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/05/28 09:39:49 by jbettini          #+#    #+#             */
/*   Updated: 2024/05/30 05:32:05 by jbettini         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::sync::mpsc::{self, Sender, Receiver};

pub struct BidirectionalMessage {
    pub send_stream: Sender<String>,
    pub message: String,
}

impl BidirectionalMessage {
    fn new(message: String, send_stream: Sender<String>) -> BidirectionalMessage {
        BidirectionalMessage {
            send_stream,
            message,
        }
    }
    
    pub fn answer(&self, message: String) -> Result<(), mpsc::SendError< String >> {
        self.send_stream.send(message)
    }
    
    pub fn load_bidirectional_message(msg: String, target: Sender<BidirectionalMessage>) -> String{
        let (send_stream, rec_stream): (Sender<String>, Receiver<String>) = mpsc::channel();
        let msg_stream = Self::new(msg, send_stream);
        target.send(msg_stream).expect("Failed to send in load bidirectional fun");
        let ret = rec_stream.recv().expect("Failed to receive");
        ret
    }
}