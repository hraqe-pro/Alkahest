use ssh2::Session;
use crate::server::{credentials::Credentials, remote_actions::{execute_command::ExecuteCommand, remote_action::RemoteErrorTrait}};
use std::{error::Error, fs::File, vec, io::{self, BufRead, BufReader, Read, Write}, path::Path};
use crate::server::remote_actions::remote_action::RemoteAction;
use crate::server::misc::class::Class;

pub struct ActionCollection {
    pub actions: Vec<Box<dyn RemoteAction>>,
    pub session_manager_owner: *const SessionManager
}

pub struct SessionManager {
    credentials: Credentials,
    pub session: Session,
    pub action_collections: Vec<ActionCollection>
}

pub trait SessionManagerTrait {
    fn new(credentials: Credentials, session: Session) -> SessionManager;
    fn get_credentials(&self) -> &Credentials;
    fn request_new_collection(&mut self) -> &mut ActionCollection;
}

impl SessionManagerTrait for SessionManager {
    fn new(credentials: Credentials, session: Session) -> SessionManager {
        return SessionManager {credentials: credentials, session: session, action_collections: Vec::new()};
    }

    fn get_credentials(&self) -> &Credentials {
        return &self.credentials;
    }

    fn request_new_collection(&mut self) -> &mut ActionCollection {
        self.action_collections.push(ActionCollection { actions: Vec::new(), session_manager_owner: self });
        return self.action_collections.last_mut().unwrap();
    }
}


pub trait ActionCollectionTrait {
    fn new_action<Type: Class + RemoteAction + 'static>(&mut self);//-> &'a mut Type;
}

impl ActionCollectionTrait for ActionCollection {
    fn new_action<Type: Class + RemoteAction + 'static>(&mut self) {
        self.actions.push(Box::new(Type::new()));

       // return self.actions.last_mut().unwrap();
    }
}