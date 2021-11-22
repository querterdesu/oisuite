use std::fs;
use std::process::Command;
use crate::*;
use termion::color;
use termion::style;
use std::time::Instant;

pub fn generate(args: &Vec<String>) {
    if args.len() <= 4 {
        throw_lerror("Not enough arguments provided!");
    }

    let packname = &args[2];
    let timelimit: u128 = args[3].parse::<u128>().unwrap();
    let limit: i32 = args[4].parse().unwrap();

    match fs::remove_dir_all(format!("tests/{}", packname)) {
        Ok(_) => {},
        Err(_) => throw_error("Failed to remove tests!"),
    };
    match fs::create_dir(format!("tests/{}", packname)) {
        Ok(_) => {},
        Err(_) => throw_error("Failed to create test directory!"),
    };

    println!("Compiling {}{}test generator{}{}...", color::Fg(color::Yellow), style::Bold, color::Fg(color::Reset), style::Reset);

    Command::new("g++")
        .arg("generate_tests.cpp")
        .arg("-o")
        .arg("gent")
        .status()
        .expect("Failed to generate testcase :(");

    println!("Compiling {}{}testing algorithm{}{}...", color::Fg(color::Yellow), style::Bold, color::Fg(color::Reset), style::Reset);
    Command::new("g++")
        .arg("brute.cpp")
        .arg("-o")
        .arg("brute")
        .status()
        .expect("Failed to generate testcase :(");

    let mut working: i32 = 0;
    match fs::write(String::from(format!("tests/{}/testinfo", packname)), format!("0\n{}\n{}", timelimit, limit)) {
        Ok(_) => {},
        Err(_) => throw_error("Failed to write testinfo!"),
    };

    for i in 1..limit+1 {

        let mut outp_i = Command::new("./gent");

        let temp_i = outp_i.output().expect("uwu");
        let stdout_i = String::from_utf8(temp_i.stdout).unwrap();

        match fs::write(format!("tests/{}/{}.in", packname, i), stdout_i) {
            Ok(_) => {},
            Err(_) => throw_error("Failed to write testcase!"),
        };

        let mut outp_o = Command::new("./brute");

        outp_o.stdin(fs::File::open(format!("tests/{}/{}.in", packname, i)).unwrap());

        let temp_o = outp_o.output().expect("uwu2");
        let stdout_o = String::from_utf8(temp_o.stdout).unwrap();

        match fs::write(format!("tests/{}/{}.out", packname, i), stdout_o) {
            Ok(_) => {},
            Err(_) => throw_error("Failed to write testcase!"),
        };

        println!("{}🗸 Generated testcase {} successfully!{}", color::Fg(color::Green), i, color::Fg(color::Reset));
        working += 1;
    }
    match fs::remove_file("gent") {
        Ok(_) => {},
        Err(_) => throw_error("Failed to remove generator!"),
    };
    match fs::remove_file("brute") {
        Ok(_) => {},
        Err(_) => throw_error("Failed to remove brute force algorithm!"),
    };
    println!("Successfully generated {}{}/{}{} testcases.", color::Fg(color::Green), working, limit, color::Fg(color::Reset));
}

pub fn test(args: &Vec<String>) {
    if args.len() <= 2 {
        throw_lerror("Not enough arguments provided!");
    }

    let packname = &args[2];
    println!("Compiling {}{}algorithm{}{}...", color::Fg(color::Yellow), style::Bold, color::Fg(color::Reset), style::Reset);

    Command::new("g++")
        .arg("main.cpp")
        .arg("-o")
        .arg("main")
        .status()
        .expect("Failed to compile :(");

    let testinfo = fs::read(format!("tests/{}/testinfo", packname)).unwrap();
    let detailedinfo = String::from_utf8(testinfo).unwrap();

    let split = detailedinfo.lines();
    let split_v: Vec<&str> = split.collect();

    let advanced_check = &split_v[0];
    let timelimit: u128 = split_v[1].parse::<u128>().unwrap();
    let amount: i32 = split_v[2].parse().unwrap();
    let ccc = "1";
    let mut passed = 0;

    if advanced_check == &ccc {
        let mut done = vec![false; amount as usize];

        let mut log = String::from("");
        for k in 2..amount+2 {
            let i: usize = k as usize;
            let req = split_v[i+1];
            let mut can_exec = true;
            if req != "n" {
                let check: usize = req.parse::<i32>().unwrap() as usize;
                if !done[check - 1] {
                    can_exec = false;
                }
            }
            if can_exec {
                let mut rmain = Command::new("./main");
                rmain.stdin(fs::File::open(format!("tests/{}/{}.in", packname, i-1)).unwrap());
                let before = Instant::now();
                let output = rmain.output().expect("Failed running algorithm!");
                let after = Instant::now();
                let stdout_output = String::from_utf8(output.stdout).unwrap();
                let tdiff = after.duration_since(before).as_millis();

                let expectedf = fs::read(format!("tests/{}/{}.out", packname, i-1)).unwrap();
                let expected = String::from_utf8(expectedf).unwrap();


                if stdout_output.trim() == expected.trim() {
                    if tdiff <= timelimit {
                        done[i-2] = true;
                        passed += 1;
                        println!("Testcase {} succeeded ({}ms)!", i-1, tdiff);
                    } else {
                        println!("Time for testcase {} exceeded! ({}/{}ms)", i-1, tdiff, timelimit);
                        log += format!("\n\nTestcase {}: Time limit exceeded ({}/{}ms)", i-1, tdiff, timelimit).as_str();
                    }
                } else {
                    println!("Testcase {} failed ({}ms)!", i-1, tdiff);
                    log += format!("\n\nTestcase {}: Got \"{}\", expected \"{}\"", i-1, stdout_output, expected).as_str();
                }
            } else {
                println!("Testcase {} skipped because testcase {} failed", i-1, req)
            }
        }
        match fs::write(format!("tests/{}/log", packname), log) {
            Ok(_) => {},
            Err(_) => throw_error("Failed to write log!"),
        };
    } else {
        let mut log = String::from("");
        for k in 1..amount+1 {
            let i: usize = k as usize;
            let mut rmain = Command::new("./main");
            rmain.stdin(fs::File::open(format!("tests/{}/{}.in", packname, i)).unwrap());
            let before = Instant::now();
            let output = rmain.output().expect("Failed running algorithm!");
            let after = Instant::now();
            let stdout_output = String::from_utf8(output.stdout).unwrap();
            let tdiff = after.duration_since(before).as_millis();

            let expectedf = fs::read(format!("tests/{}/{}.out", packname, i)).unwrap();
            let expected = String::from_utf8(expectedf).unwrap();


            if stdout_output.trim() == expected.trim() {
                if tdiff <= timelimit {
                    passed += 1;
                    println!("Testcase {} succeeded ({}ms)!", i, tdiff);
                } else {
                    println!("Time for testcase {} exceeded! ({}/{}ms)", i, tdiff, timelimit);
                    log += format!("\n\nTestcase {}: Time limit exceeded ({}/{}ms)", i, tdiff, timelimit).as_str();
                }
            } else {
                println!("Testcase {} failed ({}ms)!", i, tdiff);
                log += format!("\n\nTestcase {}: Got \"{}\", expected \"{}\"", i, stdout_output, expected).as_str();
            }
        }
        match fs::write(format!("tests/{}/log", packname), log) {
            Ok(_) => {},
            Err(_) => throw_error("Failed to write log!"),
        };
    }
    match fs::remove_file("main") {
        Ok(_) => {},
        Err(_) => throw_error("Failed to remove algorithm!"),
    };
    println!("Testing {} ended successfully. {}/{} testcases succeeded", packname, passed, amount);
}