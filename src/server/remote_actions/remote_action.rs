use ssh2::Session;
use crate::server::remote_actions::execute_command::ExecuteCommand;
use crate::server::remote_actions::manage_packages::{AddPackage, RemovePackage};
use crate::server::remote_actions::upload_file::UploadFile;

use crate::server::server_management::session_manager::ActionCollection;

pub trait RemoteErrorTrait {
    
}
#[enum_delegate::register]
pub trait RemoteAction {
    fn execute(&self, session: &Session);
}


#[enum_delegate::implement(RemoteAction)]
pub enum RemoteActionEnum {
    UploadFile(UploadFile),
    ExecuteCommand(ExecuteCommand),
    AddPackage(AddPackage),
    RemovePackage(RemovePackage),
}
