use std::{env, fs::File, io::Read, process::Command};

use blocker::Blocker;
use host_request::HostRequest;

use crate::blocklist::BlockList;

mod blocker;
mod blocklist;
mod host_request;

fn main() {
    // takes in seconds as an argument
    // launches a process that blocks the website for that amount of time

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        println!("No arguments provided");
        return;
    }

    match args[0].as_str() {
        "-a" => {
            if args.len() < 2 {
                println!("No website provided");
                return;
            }
            BlockList::add(args[1].clone());
            // write to bl
        }
        "-r" => {
            if args.len() < 2 {
                println!("No website provided");
                return;
            }
            BlockList::remove(&args[1]);
        }
        "-c" => {
            BlockList::clear();
        }
        "-b" => {
            let blocked_websites: Vec<String> = BlockList::read();
            let mut hosts_file = HostRequest::get_hosts();
            hosts_file = Blocker::add(hosts_file, blocked_websites);
            HostRequest::write_hosts(hosts_file);
        }
        "-f" => {
            let mut hosts_file = HostRequest::get_hosts();
            hosts_file = Blocker::clear(&hosts_file);
            HostRequest::write_hosts(hosts_file);
        }
        "-s" => {
            if args.len() < 2 {
                println!("No time provided");
                return;
            }
            let time = args[1].parse::<u64>().unwrap();
            // checks if child process is running
            // if not, then it launches a process that writes to hosts then
            // waits for the time and then clears the hosts file


            Command::new("osascript")
                .arg("-e")
                .arg(format!("do shell script \"./target/debug/siteblocker -w ${time}\" with administrator privileges"))
                .spawn()
                .expect("Failed to execute osascript command");
        }
        "-w" => {
            // child process that writes to hosts, waits for time, then clears hosts
            let time = args[1].parse::<u64>().unwrap();
            let blocked_websites: Vec<String> = BlockList::read();
            let mut hosts_file = HostRequest::get_hosts();
            hosts_file = Blocker::add(hosts_file, blocked_websites);

            // write to hosts
            HostRequest::write_hosts(hosts_file.clone());
            // wait for time
            std::thread::sleep(std::time::Duration::from_secs(time));
            // clear hosts
            hosts_file = Blocker::clear(&hosts_file);
            HostRequest::write_hosts(hosts_file);

            println!("Finished blocking websites");
        }
        "-p" => {
            let blocked_websites = BlockList::read();
            println!("Blocked websites: {:?}", blocked_websites);
        }
        _ => {
            println!("Invalid argument");
        }
    }
}
