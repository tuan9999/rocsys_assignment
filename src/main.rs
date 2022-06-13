use clap::Parser;
use rocsys::models::{DirectoryMonitor, Input};

fn main() {
    let input = Input::parse();
    let mut directory_monitor = DirectoryMonitor::new(input);
    directory_monitor.monitor_directories();
}
