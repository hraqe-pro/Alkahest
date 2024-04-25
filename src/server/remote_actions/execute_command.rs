use std::{error::{self, Error}, fs::File, io::{self, BufRead, BufReader, Read, Write}, path::Path};
use std::fmt::format;
use std::path::PathBuf;
use druid::piet::TextStorage;
use crate::server::remote_actions::remote_action::Action;
use ssh2::Session;
use thiserror::Error;
use crate::server::misc::class::Class;
use crate::server::server_management::action_collection;
use crate::server::server_management::session_manager::SessionManager;

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
    pub sudo: bool,
    pub execution_location: PathBuf
}

impl Action for ExecuteCommand {
    fn execute(&self, session: &SessionManager) {

        let try_execute = || -> Result<(), ExecuteCommandError> {
            let mut channel = session.session.channel_session()?;

            //echo password | sudo -S command

            //channel.exec(format!("{}{}",  if self.sudo { !format("") } else  { "" }, self.command.as_str()).as_str())?;

            if(self.sudo)
            {
                channel.exec(format!("cd {} && echo {} | sudo -S {}", self.execution_location.display(), session.credentials.password, self.command).as_str())?;

               println!("{}", format!("cd {} && echo {} | sudo -S {}", self.execution_location.display(), session.credentials.password, self.command));
            }
            else {
                channel.exec(format!("cd {} && {}", self.execution_location.display(), self.command).as_str())?;

                println!("{}", self.command);
            }

           // println!("{}", format!("{} {}",  if self.sudo { "sudo" } else  { "" }, self.command.as_str()));

            let mut result_string = "".to_string();
            channel.read_to_string(&mut result_string)?;

            channel.wait_close()?;

            println!("{}", result_string);

            Ok(())
        };

        //******************************* */
        match try_execute() {
            Ok(()) => {
                println!("Command executed")
            }
            Err(ExecuteCommandError::IoError(e)) => {
                println!("Execute command io error")
            }
            Err(ExecuteCommandError::Ssh2Error(e)) => {
                println!("Execute command ssh2 error")
            }
        };   
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
