use ssh2::Session;
use std::fs;
use std::path::PathBuf;
use crate::server::remote_actions::execute_command::ExecuteCommand;
use crate::server::remote_actions::remote_action::Action;
use crate::server::remote_actions::upload_file::UploadFile;

pub struct UploadFolder {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub delete_if_already_exists: bool
}

trait UploadFolderTrait {
    fn check_directory_tree(&self, session: &Session, start_path: &PathBuf, last_child: &PathBuf) -> Result<(), std::io::Error> {

        let paths = fs::read_dir(start_path)?;

        if(!start_path.exists())
        {
            println!("Folder doesn't exist");

            return Ok(())
        }

        for sub_path in paths {
            if(sub_path.as_ref().unwrap().path().is_dir())
            {
                let last_dir = PathBuf::from(sub_path.as_ref().unwrap().path().file_name().unwrap());

                let mut last_child_copy = PathBuf::from(&last_child);

                last_child_copy.push(last_dir);

                let make_directory = ExecuteCommand {
                    command: format!("mkdir \"{}\"", last_child_copy.display().to_string().replace("\\", "/")),
                    sudo: true
                };

                make_directory.execute(&session);


                self.check_directory_tree(session, &sub_path.as_ref().unwrap().path(), &last_child_copy).unwrap()
            }
            else {


                let mut file_destination = PathBuf::from(&last_child);

                file_destination.push(sub_path.as_ref().unwrap().file_name());

                let upload_file = UploadFile {
                    source: sub_path.as_ref().unwrap().path(),
                    destination: file_destination
                };

                println!("{} :: {}", &sub_path.as_ref().unwrap().path().display(), last_child.to_path_buf().display());

                upload_file.execute(session);
            }
        }
        Ok(())
    }
}

impl Action for UploadFolder {
    fn execute(&self, session: &Session) {



        let last_child = PathBuf::from(format!("{}/", self.source.file_name().unwrap().to_str().unwrap()));

        let mut destination_copy = PathBuf::from(&self.destination);

        destination_copy.push(last_child);


        if(self.delete_if_already_exists)
        {
            let delete_old_dir = ExecuteCommand {
                command: format!("rm -r {}", destination_copy.display().to_string().replace("\\", "/")),
                sudo: true
            };

            delete_old_dir.execute(&session);
        }

        let make_directory = ExecuteCommand {
            command: format!("mkdir {}", destination_copy.display().to_string().replace("\\", "/")),
            sudo: true
        };

        make_directory.execute(session);

        let mut source_copy = PathBuf::from(&self.source);

        println!("{}, {}", source_copy.display(), destination_copy.display());

        self.check_directory_tree(&session, &source_copy, &destination_copy).unwrap();
    }
}

impl UploadFolderTrait for UploadFolder {

}