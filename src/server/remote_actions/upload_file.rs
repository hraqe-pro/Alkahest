use std::{error::Error, fs, fs::File, io::{self, BufRead, BufReader, Read, Write}, path::Path, path::PathBuf};
use crate::server::remote_actions::remote_action::Action;
use ssh2::Session;
use thiserror::Error;
use crate::server::server_management::session_manager::SessionManager;

#[derive(Error, Debug)]
pub enum FileUploadError {
    #[error("couldn't find or open the file")]
    FileOpen(#[from] io::Error),

    #[error("couldn't send scp request")]
    SCPSend(#[from] ssh2::Error)
}

pub struct UploadFile {
    pub source: PathBuf,
    pub destination: PathBuf
}

impl Action for UploadFile {
    fn execute(&self, session: &SessionManager) {

        let try_execute = || -> Result<(), FileUploadError> {
            let file_bytes = fs::read(&self.source)?;

            let size = file_bytes.len() as u64;

            println!("file size2: {}", size);

            println!("file try: {} :: {}", &self.source.display(), self.destination.display());

            let mut remote_file = session.session.scp_send(&self.destination, 0o644, size, None)?;


            println!("file size3: {}", file_bytes.as_slice().len());

            remote_file.write_all(file_bytes.as_slice())?;

            remote_file.send_eof().unwrap();
            remote_file.wait_eof().unwrap();
            remote_file.close().unwrap();
            remote_file.wait_close().unwrap();

            Ok(())
        };

        match try_execute() {
            Ok(()) => {
                println!("File uploaded");
            }
            Err(FileUploadError::FileOpen(e)) => {
                println!("Couldn't open the file");
            }
            Err(FileUploadError::SCPSend(e)) => {
                println!("Couldn't send the file");
            }
        };   
    }
}