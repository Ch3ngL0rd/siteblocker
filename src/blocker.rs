const START_STRING: &str = "### Begin StudyBlock";
const END_STRING: &str = "### End StudyBlock";

pub struct Blocker;

// ### Begin StudyBlock
// ...
// ### End StudyBlock

impl Blocker {
    // Adds a single website to the hosts file
    pub fn add(hosts_text: String, website: String) -> String {
        /*
        1. Gets hosts_text
        2. Checks if website is already blocked
        3. If not, adds it to the hosts_text
        4. Returns the new hosts_text
        */

        if Self::is_blocked(&hosts_text, &website) {
            return hosts_text;
        }

        let mut blocked_websites = Self::get_blocked(&hosts_text);

        let mut new_hosts_text = String::new();

        let before_block = hosts_text
            .lines()
            .take_while(|s| !s.starts_with(START_STRING));

        let after_block = hosts_text
            .lines()
            .skip_while(|s| !s.starts_with(END_STRING))
            .skip(1);

        // filters out empty lines
        for line in before_block.chain(after_block).filter(|s| !s.is_empty()) {
            new_hosts_text.push_str(line);
            new_hosts_text.push_str("\n");
        }

        blocked_websites.push(website);

        new_hosts_text.push_str(&format!("{START_STRING}\n"));
        for blocked_website in blocked_websites {
            new_hosts_text.push_str(&format!("127.0.0.1\t{blocked_website}\n"));
            new_hosts_text.push_str(&format!("::\t{blocked_website}\n"));
        }
        new_hosts_text.push_str(&format!("{END_STRING}\n"));

        new_hosts_text
    }

    // function to check if a website is blocked
    pub fn is_blocked(hosts_text: &str, website: &str) -> bool {
        // gets studyblock from hosts_text
        let studyblock = hosts_text
            .lines()
            .take_while(|s| !s.starts_with(END_STRING))
            .skip_while(|s| !s.starts_with(START_STRING))
            .skip(1);

        // checks if website is in studyblock
        for line in studyblock {
            let line = line.split("\t").nth(1);
            if let Some(line) = line {
                if line == website {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn get_blocked(hosts_text: &str) -> Vec<String> {
        let mut blocked = Vec::new();

        let studyblock = hosts_text
            .lines()
            .take_while(|s| !s.starts_with(END_STRING))
            .skip_while(|s| !s.starts_with(START_STRING))
            .skip(1);

        for line in studyblock {
            let website = line.split("\t").nth(1);
            if let Some(website) = website {
                blocked.push(website.to_string());
            }
        }

        return blocked;
    }

    // function to remove a website from the hosts file
    pub fn remove(hosts_text: &str, website: &str) -> String {
        /*
        1. Gets hosts_text
        2. Checks if website is already blocked
        3. If so, removes it from hosts_text
        4. Returns the new hosts_text
        */
        if !Self::is_blocked(&hosts_text, &website) {
            return hosts_text.to_string();
        }
        let blocked_websites = Self::get_blocked(&hosts_text)
            .into_iter()
            .filter(|s| s != website)
            .collect::<Vec<String>>();

        let mut new_hosts_text = String::new();

        let before_block = hosts_text
            .lines()
            .take_while(|s| !s.starts_with(START_STRING));

        let after_block = hosts_text
            .lines()
            .skip_while(|s| !s.starts_with(END_STRING))
            .skip(1);

        for line in before_block.chain(after_block) {
            new_hosts_text.push_str(line);
            new_hosts_text.push_str("\n");
        }

        for blocked_website in blocked_websites {
            new_hosts_text.push_str(&format!("127.0.0.1 {blocked_website}"));
        }

        new_hosts_text
    }
}

// tests to check if the blocker is working
#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::blocker::Blocker;

    fn read_hosts() -> String {
        let mut hosts_text = String::new();
        let mut hosts_file = File::open("/etc/hosts").unwrap();
        hosts_file.read_to_string(&mut hosts_text).unwrap();
        hosts_text
    }

    #[test]
    fn test_add() {
        let hosts_text = read_hosts();
        let new_hosts = Blocker::add(hosts_text.clone(), String::from("google.com"));
        assert!(Blocker::is_blocked(&new_hosts, "google.com"));
    }

    #[test]

    fn test_remove() {
        let hosts_text = read_hosts();
        let new_hosts = Blocker::add(hosts_text.clone(), String::from("google.com"));
        let new_hosts = Blocker::remove(&new_hosts, "google.com");
        assert!(!Blocker::is_blocked(&new_hosts, "google.com"));
    }

    #[test]
    fn test_get_blocked() {
        let hosts_text = read_hosts();
        let new_hosts = Blocker::add(hosts_text.clone(), String::from("google.com"));
        let blocked = Blocker::get_blocked(&new_hosts);
        assert!(blocked.contains(&String::from("google.com")));
    }
}
