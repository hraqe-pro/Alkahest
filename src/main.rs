mod server{
    pub mod credentials;
    pub mod server_management { pub mod session_manager; }
    pub mod remote_actions {
        pub mod execute_command;
        pub mod remote_action;
        pub mod manage_packages;
    }

    pub mod misc {
        pub mod class;
    }
}

use server::{credentials, remote_actions::execute_command::{ExecuteCommandError, ExecuteCommand}, server_management::session_manager::{ActionCollection, SessionManagerTrait}};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::{env, error::Error, io::{Read, Write}, path::Path};
use std::ops::DerefMut;
use druid::{Data, widget::{Label, Button}, Env, Widget, WindowDesc, AppLauncher};
use druid::widget::{Flex};

use crate::server::credentials::Credentials;
use crate::server::credentials::ServerFunctionality;
use crate::server::server_management::session_manager::{ActionCollectionTrait, SessionManager};
use crate::server::remote_actions::remote_action::RemoteAction;
//use crate::server::server_management::session_manager::execute_command;
//use crate::server::remote_actions::manage_packages::{AddPackage, RemovePackage};

extern crate dotenv;

use dotenv::dotenv;
use ssh2::ErrorCode::Session;


//widgets////////////////////////////////////////////////////////
#[derive(Clone, Data)]
struct TestWidget {
    num: u32
}
/////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let credentials = Credentials::new(
        std::env::var("IP").unwrap(),
        std::env::var("USER").unwrap(),
        std::env::var("PORT").unwrap().parse().unwrap(),
        std::env::var("PASSWORD").unwrap(),
    );

    

    let mut sessions = Vec::new();

    match credentials.connect().await {
        Ok(session) => { 
            sessions.push(SessionManager::new(credentials, session))
            
        },
        Err(_) => {}
    };

    
    let mut last_session = sessions.last_mut().unwrap();

    let test = ExecuteCommand {command: "unzip".to_string(), sudo: false};
    //let test2 = AddPackage {package_name: "unzip".to_string() };

    let last_action_collection = last_session.request_new_collection();
    last_action_collection.new_action::<ExecuteCommand>();
    unsafe {
        //let test =  last_action_collection.actions.last_mut().unwrap();
        last_action_collection.actions.last_mut().unwrap().execute(&(*last_action_collection.session_manager_owner).session);
    }
    //let new_action_collection = ActionCollection { actions: Vec::new(), session_manager_owner: last_session };
    //last_session.action_collections.push(new_action_collection);
    //last_session.action_collections.last_mut().unwrap().actions.push(Box::new(test));
    //last_session.action_collections.last_mut().unwrap().actions.push(Box::new(test2));
   // test_lambdas(last_session, &||last_session.test(&test_str.to_string()));
    
    Ok(())
}
