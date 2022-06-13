use clap::Parser;
use notify::{
    watcher,
    DebouncedEvent::{self, *},
    RecursiveMode, Watcher,
};
use std::sync::mpsc::channel;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Input {
    #[clap(short, long)]
    pub directories: Vec<String>,
}

pub struct DirectoryMonitor {
    input: Input,
}

impl DirectoryMonitor {
    pub fn new(input: Input) -> Self {
        Self { input }
    }

    fn handle_event(&mut self, event: DebouncedEvent) {
        match event {
            NoticeWrite(path_buf) => {
                println!("Event: Notice Write to {:?}", path_buf)
            }
            NoticeRemove(path_buf) => {
                println!("Event: Notice Remove {:?}", path_buf)
            }
            Create(path_buf) => {
                println!("Event: Create {:?}", path_buf)
            }
            Write(path_buf) => {
                println!("Event: Write to {:?}", path_buf)
            }
            Chmod(path_buf) => {
                println!("Event: Chmod on {:?}", path_buf)
            }
            Remove(path_buf) => {
                println!("Event: Remove {:?}", path_buf)
            }
            Rename(path_buf_before, path_buf_after) => {
                println!(
                    "Event: Rename {:?} to {:?}",
                    path_buf_before, path_buf_after
                )
            }
            Rescan => {
                println!("Event: Rescan")
            }
            Error(error, path_buf) => match path_buf {
                Some(path) => println!("Error: {}, PATH = {:?}", error, path),
                None => println!("Error: {}", error),
            },
        }
    }

    pub fn monitor_directories(&mut self) {
        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_secs(0)).expect("Error initialising watcher");

        for directory in &self.input.directories {
            watcher
                .watch(directory.clone(), RecursiveMode::Recursive)
                .expect("Error adding directory to watcher");
        }

        println!("Monitoring directories: {:?}", self.input.directories);
        loop {
            match rx.recv() {
                Ok(event) => self.handle_event(event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}
