use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Read, Write}, path::Path, path::PathBuf};
use crate::server::remote_actions::remote_action::Action;
use ssh2::Session;
use thiserror::Error;
use crate::server::server_management::session_manager::SessionManager;

#[derive(Error, Debug)]
pub enum FileDownloadError {
    #[error("couldn't find or open the file")]
    FileOpen(#[from] io::Error),

    #[error("couldn't send scp request")]
    SCPReceive(#[from] ssh2::Error)
}

pub struct DownloadFile {
    pub source: PathBuf,
    pub destination: PathBuf
}

impl Action for DownloadFile {
    fn execute(&self, session: &SessionManager) {

        let try_execute = || -> Result<(), FileDownloadError> {
            let (mut remote_file, stat) = session.session.scp_recv(Path::new(&self.source))?;

            let mut contents = Vec::new();

            remote_file.read_to_end(&mut contents)?;

            let mut downloaded_file = File::create(&self.destination)?;

            downloaded_file.write_all(&contents)?;

            remote_file.send_eof().unwrap();
            remote_file.wait_eof().unwrap();
            remote_file.close().unwrap();
            remote_file.wait_close().unwrap();

            Ok(())
        };

        match try_execute() {
            Ok(()) => {
                println!("File downloaded");
            }
            Err(FileDownloadError::FileOpen(e)) => {
                println!("Couldn't open the file");
            }
            Err(FileDownloadError::SCPReceive(e)) => {
                println!("Couldn't download the file");
            }
        };   
    }
}