use ssh2::Session;

use crate::server::server_management::session_manager::ActionCollection;

pub trait RemoteErrorTrait {
    
}
pub trait RemoteAction {
    fn execute(&self, session: &Session);
    fn who_am_i(&self);
    //fn get_owner(&self) -> &ActionCollection;
}