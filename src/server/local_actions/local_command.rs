use std::fmt::format;
use std::process::Command;
use ssh2::Session;
use crate::server::remote_actions::remote_action::Action;

pub struct UploadFolder {
    pub ip: String,
    pub user: String,
    pub port: i32,
    pub password: String,
    pub source: String,
    pub destination: String
}

impl Action for UploadFolder {
    fn execute(&self, session: &Session) {
        match Command::new("pscp").arg("-pw").arg(format!("{}", self.password)).arg("-P").arg(&self.port.to_string()).arg("-r").arg(&self.source).arg(format!("{}{}{}{}{}", &self.user, "@", &self.ip, ":", &self.destination)).spawn() {
            Ok(mut answer) => {
                println!("{:?}", answer);
                answer.wait().unwrap();
            },
            Err(error) => {
                println!("{}", error)
            }
        }
    }
}