use std::process::Command;
fn main() {
    // let output = Command::new("ls")
    //     .output()
    //     .expect("failed to execute process");
    // let stdout = String::from_utf8(output.stdout).unwrap();
    // println!("{}", stdout);
    let out = compare("develop", "master");
    println!("{}", out);
}

fn compare(origin: &str, to: &str) -> String {
    let origin = origin;
    let to = to;
    let output = Command::new("git")
        .args(["diff", "--name-only", origin, to])
        .output()
        .expect("error");
    let stdout = String::from_utf8(output.stdout).unwrap();
    return stdout;
}
