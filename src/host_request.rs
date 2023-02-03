// Gets permission from user to open the hosts file permenantly

use std::process::Command;

pub struct HostRequest;

impl HostRequest {
    pub fn is_permitted() -> bool {
        let output = Command::new("sudo")
            .arg("ls")
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            true
        } else {
            false
        }
    }

    pub fn get_hosts() -> String {
        let output = Command::new("sudo")
            .arg("cat")
            .arg("/etc/hosts")
            .output()
            .expect("failed to execute process");

        String::from_utf8_lossy(&output.stdout).to_string()
    }

    // writes hosts to /etc/hosts
    pub fn write_hosts(hosts: String) {
        let output = Command::new("sudo")
            .arg("sh")
            .arg("-c")
            .arg(format!("echo \"{}\" > /etc/hosts", hosts))
            .output()
            .expect("failed to execute process");
    }
}
