use clap::Parser;
use rocsys::models::Input;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent::{*, self}};
use std::{sync::mpsc::channel};
use std::time::Duration;

fn handle_event(event: DebouncedEvent) {
    match event {
        NoticeWrite(path_buf) => { println!("Event: Notice Write to {:?}", path_buf) },
        NoticeRemove(path_buf) => { println!("Event: Notice Remove {:?}", path_buf) },
        Create(path_buf) => { println!("Event: Create {:?}", path_buf) },
        Write(path_buf) => { println!("Event: Write to {:?}", path_buf) },
        Chmod(path_buf) => { println!("Event: Chmod on {:?}", path_buf) },
        Remove(path_buf) => { println!("Event: Remove {:?}", path_buf) },
        Rename(path_buf_before, path_buf_after) => { println!("Event: Rename {:?} to {:?}", path_buf_before, path_buf_after) },
        Rescan => { println!("Event: Rescan") },
        Error(error, path_buf) => {
            match path_buf {
                Some(path) => println!("Error: {}, PATH = {:?}", error, path),
                None => println!("Error: {}", error)
            }
        },
    }
}

fn main() {
    let app = Input::parse();
    println!("{:?}", app.directories);

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(0)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(app.directories[0].clone(), RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
           Ok(event) => handle_event(event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}
