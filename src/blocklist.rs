use std::{fs::File, io::{Read, Write}};

const BLOCKLIST_PATH: &str = "./blocklist.txt";

pub struct BlockList;

impl BlockList {
    fn write(websites: Vec<String>) {
        let mut file = File::create(BLOCKLIST_PATH).unwrap();
        for website in websites {
            file.write_all(format!("{}\n", website).as_bytes()).unwrap();
        }
    }
    pub fn read() -> Vec<String> {
        let mut file = File::open(BLOCKLIST_PATH).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
    pub fn add(website: String) {
        let mut websites = BlockList::read();
        websites.push(website);
        BlockList::write(websites);
    }
    pub fn remove(website: &str) {
        let mut websites = BlockList::read();
        websites.retain(|s| s != website);
        BlockList::write(websites);
    }
}

// tests for blocklist
#[cfg(test)]
mod tests {
    use std::{fs::File, io::{Write, Read}};

    use crate::blocklist::BlockList;

    use super::BLOCKLIST_PATH;

    // clears the blocklist before each test
    fn clear() {
        let mut file = File::create(BLOCKLIST_PATH).unwrap();
        file.write_all("".as_bytes()).unwrap();
    }

    #[test]
    fn test_write() {
        clear();
        let websites = vec![String::from("google.com"), String::from("youtube.com")];
        BlockList::write(websites.clone());
        // read the blocklist
        let mut file = File::open(BLOCKLIST_PATH).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let blocklist = contents
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(websites, blocklist);
    }

    // test read
    #[test]
    fn test_read() {
        clear();
        let websites = vec![String::from("google.com"), String::from("youtube.com")];
        BlockList::write(websites.clone());
        assert_eq!(websites, BlockList::read());
    }
}