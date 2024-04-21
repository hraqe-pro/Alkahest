use ssh2::Session;
use crate::server::remote_actions::execute_command::ExecuteCommand;
use crate::server::remote_actions::manage_packages::{AddPackage, RemovePackage};
use crate::server::remote_actions::upload_file::UploadFile;
use crate::server::remote_actions::upload_folder::UploadFolder;

use crate::server::server_management::action_collection::ActionCollection;

pub trait RemoteErrorTrait {
    
}
#[enum_delegate::register]
pub trait Action {
    fn execute(&self, session: &Session);
}


#[enum_delegate::implement(Action)]
pub enum ActionEnum {
    UploadFile(UploadFile),
    ExecuteCommand(ExecuteCommand),
    AddPackage(AddPackage),
    RemovePackage(RemovePackage),
    UploadFolder(UploadFolder),

}
