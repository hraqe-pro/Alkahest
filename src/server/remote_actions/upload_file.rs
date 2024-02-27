use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Read, Write}, path::Path, path::PathBuf};
use crate::server::remote_actions::remote_action::RemoteAction;
use ssh2::Session;
use thiserror::Error;

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

impl RemoteAction for UploadFile {
    fn execute(&self, session: &Session) {

        let try_execute = || -> Result<(), FileUploadError> {
            let file = File::open(&self.source)?;
            let mut buf_reader = BufReader::new(file);

            let data = buf_reader.fill_buf()?;
            let size = data.len() as u64;

            let mut remote_file = session.scp_send(&self.destination, 0o644, size, None)?;
            remote_file.write(data)?;

            Ok(())
        };

        match try_execute() {
            Ok(()) => {

            }
            Err(FileUploadError::FileOpen(e)) => {
                
            }
            Err(FileUploadError::SCPSend(e)) => {
                
            }
        };   
    }
}