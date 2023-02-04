use std::{process::Command};

use blocker::Blocker;
use host_request::HostRequest;

use crate::{blocklist::BlockList, host_request, blocker};

pub fn sion(args: Vec<String>) -> Option<String> {
    // takes in seconds as an argument
    // launches a process that blocks the website for that amount of time

    if args.len() == 0 {
        println!("No arguments provided");
        return Some(String::from("No arguments provided"));
    }

    match args[0].as_str() {
        "-a" => {
            if args.len() < 2 {
                return Some(String::from("No website provided"));
            }
            BlockList::add(args[1].clone());
            return Some(String::from("Added website to blocklist"));
        }
        "-r" => {
            if args.len() < 2 {
                return Some(String::from("No website provided"));
            }
            BlockList::remove(&args[1]);
            return Some(String::from("Removed website from blocklist"));
        }
        "-c" => {
            BlockList::clear();
            return Some(String::from("Cleared blocklist"));
        }
        "-b" => {
            let blocked_websites: Vec<String> = BlockList::read();
            let mut hosts_file = HostRequest::get_hosts();
            hosts_file = Blocker::add(hosts_file, blocked_websites);
            HostRequest::write_hosts(hosts_file);
            return Some(String::from("Blocked websites"));
        }
        "-f" => {
            let mut hosts_file = HostRequest::get_hosts();
            hosts_file = Blocker::clear(&hosts_file);
            HostRequest::write_hosts(hosts_file);
            return Some(String::from("Cleared hosts file"));
        }
        "-s" => {
            if args.len() < 2 {
                return Some(String::from("No time provided"));
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
            return Some(String::from("Started blocking websites"));
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
            return Some(String::from("Finished blocking websites"));
        }
        "-p" => {
            let blocked_websites = BlockList::read();
            return Some(String::from(blocked_websites.join(" ")));
        }
        _ => {

        }
    }
    return Some(String::from("Invalid argument"));
}
