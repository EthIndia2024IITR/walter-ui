use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{thread, time};

fn main() {
    // Path to the YAML config file
    let config_path = "/path/to/your/config.yaml";

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(config_path, RecursiveMode::NonRecursive).unwrap();

    // Spawn a thread to echo "Hello, world!" every 20 seconds
    thread::spawn(|| {
        loop {
            println!("Hello, world!");
            thread::sleep(Duration::from_secs(20));
        }
    });

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Write(path) => {
                    println!("Config file changed: {:?}", path);
                    // Read and process the YAML config file
                    let mut file = File::open(config_path).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    println!("Config contents: {}", contents);
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}