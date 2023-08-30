use std::fs::File;
use std::io::{Error, Write};
use std::time;

fn create_file() -> Result<File, Error> {
    let mut data_file = File::create("example.txt").expect("creation failed");
    data_file
        .write("Hello, World!".as_bytes())
        .expect("write failed");

    Ok(data_file)
}

// reads the file from disk multiple times.
fn non_cache() -> u128 {
    create_file().expect("creating the file failed");

    let now = time::Instant::now();
    // we read the file 1m times.
    for _ in 0..1000000 {
        let content = std::fs::read_to_string("example.txt").expect("opening the file failed");

        // do whatever.
        if content.contains("space") {
            println!("space found");
        }
    }

    now.elapsed().as_millis()
}

struct CacheManager {
    content: String,
}

impl CacheManager {
    fn new(content: String) -> Self {
        CacheManager { content }
    }

    fn read_content(&self) -> &String {
        &self.content
    }
}

fn cache() -> u128 {
    create_file().expect("creating the file failed");

    let now = time::Instant::now();

    // we load the content once.
    let content = std::fs::read_to_string("example.txt").expect("opening the file failed");

    // create the cache manager.
    let cache_manager = CacheManager::new(content);

    for _ in 0..1000000 {
        let content = cache_manager.read_content();

        if content.contains("space") {
            println!("space found");
        }
    }

    now.elapsed().as_millis()
}

fn main() {
    let non_cache_time = non_cache();
    let cache_time = cache();

    println!(
        "cache is {} times faster than non-cache",
        non_cache_time / cache_time
    );
}
