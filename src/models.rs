use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Input {
    #[clap(short, long)]
    pub directories: Vec<String>,
}

pub struct DirectoryMonitor {

}