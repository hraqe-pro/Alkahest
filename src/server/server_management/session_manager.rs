use ssh2::Session;
use crate::server::{credentials::Credentials, remote_actions::{execute_command::ExecuteCommand, remote_action::RemoteErrorTrait}};
use std::{error::Error, fs::File, vec, io::{self, BufRead, BufReader, Read, Write}, path::Path};
use crate::server::misc::class::Class;
use crate::server::remote_actions::remote_action::Action;
use crate::server::server_management::action_collection::ActionCollection;


pub struct SessionManager {
    credentials: Credentials,
    pub session: Session,
}

pub trait SessionManagerTrait {
    fn new(credentials: Credentials, session: Session) -> SessionManager;
    fn get_credentials(&self) -> &Credentials;

    fn execute_collection(&self, collection: &ActionCollection);
}

impl SessionManagerTrait for SessionManager {
    fn new(credentials: Credentials, session: Session) -> SessionManager {
        return SessionManager {credentials, session };
    }

    fn get_credentials(&self) -> &Credentials {
        return &self.credentials;
    }

    fn execute_collection(&self, collection: &ActionCollection) {
        for action in &collection.actions {
            action.execute(&self.session);
        }
    }
}

/*
macro_rules! generate_action {
    ($type: ty) => {
        RemoteActionEnum::<$type>
    };
}

pub trait ActionCollectionTrait {
    fn new_action<Type: Class + RemoteAction + 'static>(&mut self);
}
impl ActionCollectionTrait for ActionCollection {
    fn new_action<Type: Class + RemoteAction + 'static>(&mut self){

        self.actions.push(generate_action!(Type)(Type::new()));
    }
}
*/
