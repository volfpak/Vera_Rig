use serde_json;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
pub struct MemoryStore;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Default)]
pub struct ProcessedNotifications {
    tweet_ids: HashSet<String>,
}
impl MemoryStore {
    const FILE_PATH: &'static str = "./storage/memory.json";

    // Load memory from file
    pub fn load_memory() -> io::Result<Vec<String>> {
        if Path::new(Self::FILE_PATH).exists() {
            let data = fs::read_to_string(Self::FILE_PATH)?;
            let memory: Vec<String> = serde_json::from_str(&data)?;
            Ok(memory)
        } else {
            Ok(Vec::new()) // Return an empty vector if file doesn't exist
        }
    }

    // Add to memory
    pub fn add_to_memory(memory: &mut Vec<String>, item: &str) -> Result<(), String> {
        if !memory.contains(&item.to_string()) {
            memory.push(item.to_string());
            let _ = Self::save_memory(memory);
            Ok(())
        } else {
            Err("Memory Exists!".to_string())
        }
    }

    // Wipe memory
    pub fn wipe_memory(memory: &mut Vec<String>) -> io::Result<()> {
        memory.clear();
        Self::save_memory(memory)
    }

    // Count memories
    pub fn count_memories(memory: &Vec<String>) -> usize {
        memory.len()
    }

    // Save memory to file
    pub fn save_memory(memory: &Vec<String>) -> io::Result<()> {
        let data = serde_json::to_string(memory)?;
        let mut file = fs::File::create(Self::FILE_PATH)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // Get current memory
    pub fn get_memory() -> io::Result<Vec<String>> {
        Self::load_memory()
    }

    pub fn load_processed_tweets() -> Result<HashSet<String>, anyhow::Error> {
        match fs::read_to_string("storage/processed_tweets.json") {
            Ok(contents) => {
                let data: ProcessedNotifications = serde_json::from_str(&contents)?;
                Ok(data.tweet_ids)
            }
            Err(_) => Ok(HashSet::new())
        }
    }

    pub fn save_processed_tweets(processed_tweets: &HashSet<String>) -> Result<(), anyhow::Error> {
        let data = ProcessedNotifications {
            tweet_ids: processed_tweets.clone(),
        };
        let json = serde_json::to_string_pretty(&data)?;
        fs::create_dir_all("storage")?;
        fs::write("storage/processed_tweets.json", json)?;
        Ok(())
    }
}
