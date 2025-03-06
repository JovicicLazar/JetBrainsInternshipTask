use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Config {
    sections: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            sections: HashMap::new(),
        }
    }

    pub fn load_file(&mut self, path: &str) -> bool {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                println!("Couldn’t open INI file: {}", path);
                return false;
            }
        };

        let mut reader = BufReader::new(file);

        let mut contents = String::new();

        match reader.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read INI file: {}", path);
                return false;
            }
        };
        self.parse(&contents);
        true
    }

    fn parse(&mut self, contents: &str) {
        let mut current_section = String::new();

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                self.sections.entry(current_section.clone()).or_insert_with(HashMap::new);
            } else if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();

                let value: String = value.trim().to_string();

                let section = self.sections.entry(current_section.clone()).or_insert_with(HashMap::new);

                section.insert(key.clone(), value.clone());
            }
        }
    }

    pub fn get_as_string(&self, section: &str, key: &str) -> Option<String> {
        match self.sections.get(section).and_then(|section| section.get(key)) {
            Some(value) => Some(value.clone()),
            None => {
                println!("Value not found in hashmap for section '{}', key '{}'", section, key);
                None
            }
        }
    }

    pub fn get_as_int(&self, section: &str, key: &str) -> Option<i32> {
        match self.sections.get(section).and_then(|section| section.get(key)) {
            Some(value) => match value.parse::<i32>() {
                Ok(int_value) => Some(int_value),
                Err(_) => {
                    println!("Value '{}' for section '{}', key '{}' is not an integer", value, section, key);
                    None
                }
            },
            None => {
                println!("Value not found in hashmap for section '{}', key '{}'", section, key);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_get_success() {
        let mut config = Config::new();
        let ini = "[request]\nhost=127.0.0.1\nport=8080\n[downloader]\nchunk_size=50000";
        config.parse(ini);

        assert_eq!(
            config.get_as_string("request", "host"),
            Some("127.0.0.1".to_string())
        );
        assert_eq!(config.get_as_int("request", "port"), Some(8080));
        assert_eq!(config.get_as_int("downloader", "chunk_size"), Some(50000));
    }

    #[test]
    fn test_get_string_not_found() {
        let mut config = Config::new();
        let ini = "[request]\nhost=127.0.0.1";
        config.parse(ini);

        assert_eq!(config.get_as_string("request", "port"), None); // Triggers "not found"
        assert_eq!(config.get_as_string("missing", "host"), None); // Triggers "not found" for section
    }

    #[test]
    fn test_get_int_errors() {
        let mut config = Config::new();
        let ini = "[request]\nhost=127.0.0.1\nport=abc\n[downloader]\nchunk_size=50000";
        config.parse(ini);

        assert_eq!(config.get_as_int("request", "timeout"), None); // Triggers "not found"

        assert_eq!(config.get_as_int("request", "port"), None); // Triggers "not an integer"
    }

    #[test]
    fn test_empty_config() {
        let config = Config::new();
        assert_eq!(config.get_as_string("request", "host"), None); // Triggers "not found"
        assert_eq!(config.get_as_int("request", "port"), None); // Triggers "not found"
    }

    #[test]
    fn test_comments_and_empty_lines() {
        let mut config = Config::new();
        let ini = "# Comment\n[request]\n\nhost=127.0.0.1\nport=8080";
        config.parse(ini);

        assert_eq!(config.get_as_string("request", "host"), Some("127.0.0.1".to_string()));
        assert_eq!(config.get_as_int("request", "port"), Some(8080));
    }
}