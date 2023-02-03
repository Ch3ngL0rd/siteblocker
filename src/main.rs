use std::{fs::File, io::Read};

use blocker::Blocker;
use host_request::HostRequest;

use crate::blocklist::BlockList;

mod blocker;
mod host_request;
mod blocklist;

fn main() {
    // takes in seconds to block if -s is passed
    // checks if -s is passed in
    let mut args = std::env::args();
    println!("{:?}", args);
    args.next();
    args.next();
    let seconds = args.next().unwrap_or(String::from("0"));
    println!("{:?}", seconds);
    let seconds = seconds.parse::<u64>().unwrap_or(0);

    const website : &str = "google.com";

    println!("Blocking {} for {} seconds", website, seconds);

    // remove website from hosts file
    let hosts = HostRequest::get_hosts();
    let new_hosts = Blocker::remove(&hosts, website);
    HostRequest::write_hosts(new_hosts);

    let hosts = HostRequest::get_hosts();
    let new_hosts = Blocker::add(hosts, String::from(website));
    HostRequest::write_hosts(new_hosts);

    if seconds > 0 {
        std::thread::sleep(std::time::Duration::from_secs(seconds));
        let hosts = HostRequest::get_hosts();
        let new_hosts = Blocker::remove(&hosts, website);
        HostRequest::write_hosts(new_hosts);
        println!("{} has been unblocked", website);
        return;
    }
}

fn read_hosts() -> String {
    let mut hosts_text = String::new();
    let mut hosts_file = File::open("/etc/hosts").unwrap();
    hosts_file.read_to_string(&mut hosts_text).unwrap();
    hosts_text
}
