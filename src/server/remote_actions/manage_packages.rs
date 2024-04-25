use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Read, Write}, path::Path};
use crate::server::{remote_actions::remote_action::Action, server_management::session_manager::{SessionManager, SessionManagerTrait}};
use ssh2::Session;
use thiserror::Error;
use crate::server::server_management::action_collection;

use super::remote_action::RemoteErrorTrait;

#[derive(Error, Debug)]
pub enum AddPackageError {
    #[error("couldn't proceed with ssh2 request")]
    Ssh2Error(#[from] ssh2::Error),

    #[error("couldn't proceed with io request")]
    IoError(#[from] std::io::Error)
}

impl RemoteErrorTrait for AddPackageError {

}

#[derive(Error, Debug)]
pub enum RemovePackageError {
    #[error("couldn't proceed with ssh2 request")]
    Ssh2Error(#[from] ssh2::Error),

    #[error("couldn't proceed with io request")]
    IoError(#[from] std::io::Error)
}

impl RemoteErrorTrait for RemovePackageError {
    
}

pub struct AddPackage {
    pub package_name: String,
}

pub struct RemovePackage {
    pub package_name: String,
}

impl<'a> Action for AddPackage {
    fn execute(&self, session: &SessionManager) {

        /*
        let try_execute = || -> Result<(), AddPackageError> {
            let user = &self.session_manager.get_credentials().user;
            let password = &self.session_manager.get_credentials().password;

            let mut channel = session.channel_session()?;
            channel.exec(format!("echo \"{}\" | sudo -S apt install -y {}", password, user).as_str())?;
            let mut s = String::new();
            channel.read_to_string(&mut s)?;
            println!("{}", s);
            channel.wait_close();
            println!("{}", channel.exit_status()?);

            Ok(())
        };

        match try_execute() {
            Ok(()) => {

            }
            Err(AddPackageError::IoError(e)) => {
                
            }
            Err(AddPackageError::Ssh2Error(e)) => {
                
            }
        };   
         */
    }
}

impl<'a> Action for RemovePackage {
    fn execute(&self, session: &SessionManager) {
    
        /*
        let try_execute = || -> Result<(), AddPackageError> {
            let user = &self.session_manager.get_credentials().user;
            let password = &self.session_manager.get_credentials().password;

            let mut channel = session.channel_session()?;
            channel.exec(format!("echo \"{}\" | sudo -S apt purge -y {}", password, user).as_str())?;
            let mut s = String::new();
            channel.read_to_string(&mut s)?;
            println!("{}", s);
            channel.wait_close();
            println!("{}", channel.exit_status()?);
            
            Ok(())
        };

        match try_execute() {
            Ok(()) => {

            }
            Err(AddPackageError::IoError(e)) => {
                
            }
            Err(AddPackageError::Ssh2Error(e)) => {
                
            }
        };  
         */ 
    }
}