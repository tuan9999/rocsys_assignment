# Rocsys challenge
## Description
This CLI monitors a directory or set of directories for any changes within that directory or any subdirectories.

## How to run
On the command line in the root of the repo execute:
- `cargo build`
- `./target/debug/rocsys -d <path_to_directory>`

for multiple directories:
- `./target/debug/rocsys -d <path_to_directory_1> -d <path_to_directory_2>`