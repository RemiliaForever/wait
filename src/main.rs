use std::env;
use std::net::TcpStream;
use std::os::unix::process::CommandExt;
use std::process::exit;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), i32> {
    let mut wait_interval: u64 = 3;
    let mut wait_counts: u64 = 20;
    env::vars().for_each(|(k, v)| match k.as_ref() {
        "WAIT_INTERVAL" => wait_interval = v.parse().expect("bad WAIT_INTERVAL value!"),
        "WAIT_COUNTS" => wait_counts = v.parse().expect("bad WAIT_COUNTS value!"),
        _ => (),
    });

    let mut hosts = Vec::new();
    let mut command = String::new();
    let mut flag = false;
    let args = env::args();
    for arg in args {
        if flag {
            command += " ";
            command += &arg;
            continue;
        } else {
            if arg == "--" {
                flag = true;
            } else if arg.contains(":") {
                hosts.push(arg);
            }
        }
    }
    if command.is_empty() {
        println!("Usage: WAIT_INTERVAL=3 WAIT_COUNTS=20 wait [host:port] [host:port] -- command");
        exit(-1);
    } else {
        for host in hosts {
            let mut count = 0;
            println!("[WAIT] check {}", host);
            loop {
                match TcpStream::connect(&host) {
                    Err(_) => println!("test failed!"),
                    Ok(_) => break,
                }
                count += 1;
                if count > wait_counts {
                    println!("[WAIT] {} timeout!", host);
                    exit(-1);
                }
                sleep(Duration::from_secs(wait_interval));
            }
        }
        println!("[WAIT] exec {}", command);
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!("exec {}", &command))
            .exec();
        Ok(())
    }
}
