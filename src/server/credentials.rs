use tokio::net::TcpStream;
use ssh2::Session;

#[derive(Debug)]
pub enum ServerConnectionError {
    TcpStreamCreation,
    SessionObjectCreation,
    Handshake,
    Authentication
}

pub struct Credentials {
    ip: String,
    pub user: String,
    port: i32,
    pub password: String
}

pub trait ServerFunctionality {
    async fn connect(&self) -> Result<Session, ServerConnectionError>;
    fn new(ip: String, user: String, port: i32, password: String) -> Credentials;
}

impl ServerFunctionality for Credentials {
    fn new(ip: String, user: String, port: i32, password: String) -> Credentials{
        return Credentials { ip: ip, user: user, port: port, password: password }
    }

    async fn connect(&self) -> Result<Session, ServerConnectionError> {
        match TcpStream::connect(format!("{}:{}", self.ip, self.port)).await {
            Ok(tcp) => {
                match Session::new() {
                    Ok(mut session) => {
                        session.set_tcp_stream(tcp);
                        match session.handshake() {
                            Ok(()) => {
                                match session.userauth_password(self.user.as_str(), self.password.as_str()) {
                                    Ok(()) => {
                                        return Ok(session);
                                    }
                                    Err(ssh2::Error { .. }) => { return Err(ServerConnectionError::Authentication) }
                                }
                            }
                            Err(ssh2::Error { .. }) => { return Err(ServerConnectionError::Handshake) }
                        }
                    }
                    Err(Session) => { return Err(ServerConnectionError::SessionObjectCreation) }
                };
            }
            Err(tcp) => { return Err(ServerConnectionError::TcpStreamCreation) }
        };


    }
}
