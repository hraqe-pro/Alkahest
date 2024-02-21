use std::{error::{self, Error}, fs::File, io::{self, BufRead, BufReader, Read, Write}, path::Path};
use druid::piet::TextStorage;
use crate::server::remote_actions::remote_action::RemoteAction;
use ssh2::Session;
use thiserror::Error;
use crate::server::misc::class::Class;
use crate::server::server_management::session_manager::ActionCollection;

use super::remote_action::RemoteErrorTrait;

#[derive(Error, Debug)]
pub enum ExecuteCommandError {
    #[error("couldn't proceed with ssh2 request")]
    Ssh2Error(#[from] ssh2::Error),

    #[error("couldn't proceed with io request")]
    IoError(#[from] std::io::Error)
}

pub struct ExecuteCommand {
    pub command: String,
    pub sudo: bool 
}

impl RemoteAction for ExecuteCommand {
    fn execute(&self, session: &Session) {
        
        let try_execute = || -> Result<(), ExecuteCommandError> {
            let mut channel = session.channel_session()?;
            channel.exec(self.command.as_str())?;
            
            let mut result_string = "".to_string();
            channel.read_to_string(&mut result_string)?;

            channel.wait_close()?;

            println!("{}", result_string);

            Ok(())
        };

        //******************************* */
        match try_execute() {
            Ok(()) => {

            }
            Err(ExecuteCommandError::IoError(e)) => {
                
            }
            Err(ExecuteCommandError::Ssh2Error(e)) => {
                
            }
        };   
    }

    fn who_am_i(&self) {

    }

    /*
    fn get_owner(&self) -> &ActionCollection {
        
    }
     */

    /*
    
    fn new<'a>() -> &'a mut ExecuteCommand {
        return &ExecuteCommand{command: "".to_string(), sudo: false};
    }
     */
}

impl Class for ExecuteCommand {
    fn new() -> Self {
        ExecuteCommand {command: "".to_string(), sudo: false}
    }
}