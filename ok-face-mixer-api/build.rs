use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../ok-face-mixer-web/");

    Command::new("trunk")
        .current_dir("../ok-face-mixer-web")
        .arg("build")
        .arg("--release")
        .spawn()
        .expect("trunk error")
        .wait()
        .expect("execution error");
}
