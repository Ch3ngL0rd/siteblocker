use std::{env, fs::File, io::Read, process::Command};

use blocker::Blocker;
use host_request::HostRequest;

use crate::blocklist::BlockList;

mod blocker;
mod blocklist;
mod host_request;
mod sion;

fn main() {
    // takes in seconds as an argument
    // launches a process that blocks the website for that amount of time

    let args: Vec<String> = env::args().skip(1).collect();

    sion::sion(args);
}
