/// IPC library for Rusty Keys
/// Resposible for commmuincation between the gui, cli, and the daemon. 


pub mod rusty_keys {
    pub mod ipc {
        include!(concat!(env!("OUT_DIR"), "/rusty_keys.ipc.rs"));
    }
}

pub mod crypto_manager;
pub mod auth;

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::collections::vec_deque::VecDeque;

use aws_lc_rs::kem::{Ciphertext, DecapsulationKey, ML_KEM_512};
use prost::Message;
use rusty_keys::ipc::PublicKey; 
// only allow a few select sockets for rusty-keys to run on. 
const ALLOWED_SOCKETS: [&str; 3] = ["/tmp/rusty-keys.sock", "/tmp/rusty-keys-cli.sock", "/tmp/rusty-keys-gui.sock"];

#[derive(Copy, Clone)]
pub enum Socket {
    Cli,
    Gui,
    Daemon,
}

pub struct IpcManager {
    socket: Socket,
    stream: UnixStream,
    buffer: VecDeque<u8>,
    sessions: Vec<(u32, PublicKey)>, // list of active sessions
}

impl IpcManager {
    fn new(socket: Socket) -> Self {
        let stream = UnixStream::connect(ALLOWED_SOCKETS[socket as usize]).unwrap();
        
        // restrict the socket using POSIX permissions, read/write only.
        let socket_file = std::fs::OpenOptions::new().write(true).open(ALLOWED_SOCKETS[socket as usize]).unwrap();
        let mut perms = socket_file.metadata().unwrap().permissions();
        perms.set_mode(0o600);
        socket_file.set_permissions(perms).unwrap();
        
        Self {
            socket,
            stream,
            buffer: VecDeque::new(),
            sessions: Vec::new(),
        }
    }

    /// Only the daemon should be allowed to initiate a session. The clients can request a session, but the daemon must be in charge of creating it.
    #[cfg(feature = "daemon")]
    fn setup_session_secret(&mut self, session_id: u32, public_key: PublicKey) {
        // we need to make it so that Alice and Bob arrive on the same secret. 

        let decapsulation_key = DecapsulationKey::generate(&ML_KEM_512).unwrap();
        let encapsulation_key = decapsulation_key.encapsulation_key().unwrap();
        let encapsulation_keys_bytes = encapsulation_key.key_bytes().unwrap();
        
        // prepare our message for the client. 
        self.stream.write();
    }

    fn send_message(&mut self, message: rusty_keys::ipc::Transaction) {
        let mut buf = Vec::new();
        message.encode_length_delimited(&mut buf).unwrap();
        
        // encrypt the buffer now  with the sessions public key
        let session = self.sessions.iter().find(|(id, _)| *id == message.authenticated_session_id).unwrap();


        self.stream.write(&buf).unwrap();
    }
}