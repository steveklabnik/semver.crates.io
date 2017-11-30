use std::process::Command;
use std::io;
use std::io::prelude::*;

fn main() {
    print!("Building semver-wasm...");
    io::stdout().flush().unwrap();

    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path=semver-wasm/Cargo.toml")
        .arg("--target=wasm32-unknown-unknown")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("done");
    } else {
        println!();
        panic!("failed! output: {:?}", output);
    }

    print!("Do you have wasm-opt installed...");
    io::stdout().flush().unwrap();

    let output = Command::new("wasm-opt")
        .output();

    if output.is_ok() && output.unwrap().status.success() {
        println!("yes");
        print!("running wasm-opt...");
        io::stdout().flush().unwrap();

        let output = Command::new("wasm-opt")
            .arg("-Oz")
            .arg("semver-wasm/target/wasm32-unknown-unknown/release/semver_wasm.wasm")
            .args(&["-o", "semver-wasm/target/wasm32-unknown-unknown/release/semver_wasm.wasm"])
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            println!("done");
        } else {
            println!();
            println!("Failed! Output:");
            println!("{}", std::str::from_utf8(&output.stdout).unwrap());
            std::process::exit(1);
        }
    } else {
        println!("no. continuting without wasm-opt optimization.")
    }

    print!("Do you have wasm-gc installed...");
    io::stdout().flush().unwrap();

    let output = Command::new("wasm-gc")
        .output();

    if output.is_ok() && output.unwrap().status.success() {
        println!("yes");
        print!("running wasm-gc...");
        io::stdout().flush().unwrap();

        let output = Command::new("wasm-gc")
            .arg("semver-wasm/target/wasm32-unknown-unknown/release/semver_wasm.wasm")
            .args(&["-o", "semver-wasm/target/wasm32-unknown-unknown/release/semver_wasm.wasm"])
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            println!("done");
        } else {
            println!();
            panic!("failed! output: {:?}", output);
        }
    } else {
        println!("no. continuting without wasm-gc optimization.")
    }

    print!("Building the server...");
    io::stdout().flush().unwrap();

    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--manifest-path=server/Cargo.toml")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("done");
    } else {
        println!("failed! Output:");
        println!("{}", std::str::from_utf8(&output.stderr).unwrap());
        std::process::exit(1);
    }

    println!("Running the server!");
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--manifest-path=server/Cargo.toml")
        .spawn()
        .expect("failed to execute process");

    child.wait().expect("failed to wait on child");
}
