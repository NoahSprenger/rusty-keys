/// IPC library for Rusty Keys
/// Resposible for commmuincation between the gui, cli, and the daemon. 

pub mod crypto_manager;
pub mod auth;


use std::io::Write;
use std::os::unix::net::UnixStream;
use std::time::SystemTime;
use std::collections::vec_deque::VecDeque; 
use aws_lc_rs::init; 
use aws_lc_rs::{rand, hmac};

// only allow a few select sockets for rusty-keys to run on. 
const ALLOWED_SOCKETS: [&str; 3] = ["/tmp/rusty-keys.sock", "/tmp/rusty-keys-cli.sock", "/tmp/rusty-keys-gui.sock"];

#[derive(Copy)]
pub enum Socket {
    Cli,
    Gui,
    Daemon,
}

pub struct IpcManager {
    socket: Socket,
    stream: UnixStream,
    buffer: VecDeque<u8>,
}

impl IpcManager {
    fn new(socket: Socket) -> Self {
        let stream = UnixStream::connect(ALLOWED_SOCKETS[socket as usize]).unwrap();
        Self {
            socket,
            stream,
            buffer: VecDeque::new(),
        }
    }


}