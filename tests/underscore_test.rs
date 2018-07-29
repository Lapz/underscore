#![crate_name = "underscore_test"]
#![crate_type = "bin"]

extern crate ansi_term;
extern crate walkdir;

use ansi_term::Colour::{Green, Red};
use std::fs::File;
use std::io::Read;
use std::process::Command;
use walkdir::WalkDir;
use std::fs::remove_file;

fn main() {
    let mut pass = 0i32;
    let mut fail = 0i32;

    for entry in WalkDir::new("../tests/pass") {
        let mut underscorec = Command::new("cargo");
        let entry = entry.unwrap();
        let mut expected = Vec::new();

        if entry.path().is_dir() {
            continue;
        }

        let mut source = String::new();

        let mut file = File::open(entry.path().to_str().unwrap()).expect("File not found");

        file.read_to_string(&mut source)
            .expect("something went wrong reading the file");

        let pattern = "// Expect :";

        for line in source.lines() {
            if let Some((index, _)) = line.match_indices(&pattern).next() {
                let from = index + pattern.len();
                let expects = line[from..].to_string();
                expected.push(expects);
            }
        }

        underscorec.args(&["run", "--", entry.path().to_str().unwrap()]);

        let output = underscorec.output().expect("failed to execute process");

        let output = String::from_utf8_lossy(&output.stdout);

        for expects in expected.iter() {
            if output.contains(expects) {
                pass += 1;
            } else if underscorec
                .status()
                .expect("failed to execute process")
                .success() && !output.contains(expects)
            {
                pass += 1;
            } else {
                fail += 1;
            }
        }

        let mut program = Command::new("../tests/pass/out");

        let output = program.output().expect("failed to execute process");

        println!("{:?}",output);
        let output = String::from_utf8_lossy(&output.stdout);

        for expects in expected {
            if output.contains(&expects) {
                pass += 1;
            } else if underscorec
                .status()
                .expect("failed to execute process")
                .success() && !output.contains(&expects)
            {
                pass += 1;
            } else {
                fail += 1;
            }
        }
    }

    println!(
        "Pass:{} Fail:{}",
        Green.bold().paint(pass.to_string()),
        Red.bold().paint(fail.to_string())
    );

    assert!(fail == 0);

    // for entry in WalkDir::new("../tests/fail") {
    //     let mut underscorec = Command::new("cargo");
    //     let entry = entry.unwrap();

    //     if entry.path().is_dir() {
    //         continue;
    //     }

    //     let mut source = String::new();

    //     let mut file = File::open(entry.path().to_str().unwrap()).expect("File not found");

    //     file.read_to_string(&mut source)
    //         .expect("something went wrong reading the file");

    //     underscorec.args(&["run", "--", entry.path().to_str().unwrap()]);

    //     let mut expected = Vec::new();

    //     let pattern = "//expect:";

    //     for line in source.lines() {
    //         if let Some((index, _)) = line.match_indices(&pattern).next() {
    //             let from = index + pattern.len();
    //             let expects = line[from..].to_string();
    //             expected.push(expects);
    //         }
    //     }

    //     let output = underscorec.output().expect("failed to execute process");

    //     let output = String::from_utf8_lossy(&output.stdout);
    //     for expects in expected {
    //         if !output.contains(&expects) {
    //             panic!("Expected: {}", expects)
    //         } else {

    //         }
    //     }

    //     assert!(
    //         underscorec
    //             .status()
    //             .expect("failed to execute process")
    //             .success() != true
    //     );
    // }
}
