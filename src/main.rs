use std::process::Command;
fn main() {
    let out = compare("develop", "master");
    println!("{:?}", out);
}

fn compare<'a>(origin: &str, to: &str) -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--name-only", origin, to])
        .output()
        .expect("error");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let v: Vec<String> = stdout.split("\n").map(str::to_string).collect();
    return v;
}
