use std::fmt::format;
use ssh2::Session;
use crate::server::remote_actions::execute_command::ExecuteCommand;
use crate::server::remote_actions::remote_action::Action;
use crate::server::server_management::session_manager::SessionManager;
use std::path::PathBuf;

pub enum PackageType {
    AptGet,
    Npm
}

pub struct InstallPackage {
    pub package_name: String,
    pub package_type: PackageType
}

impl Action for InstallPackage{
    fn execute(&self, session: &SessionManager) {
        let installing_command;

        match self.package_type {
            PackageType::AptGet => {
                 installing_command = ExecuteCommand {
                    command: format!("apt-get install {} -y", self.package_name),
                    sudo: true,
                     execution_location: PathBuf::from("~")
                };
            }
            PackageType::Npm => {
                installing_command = ExecuteCommand {
                    command: format!("npm -g install {}", self.package_name),
                    sudo: true,
                    execution_location: PathBuf::from("~")
                };
            }
        }


        installing_command.execute(session);
    }
}