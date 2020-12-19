use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::{exit, Command, ExitStatus};

fn main() {
    match dogr() {
        Ok(status) => exit(status.code().unwrap_or(-1)),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                eprintln!("\"dog\" command is not found.");
            } else {
                eprintln!("{}", err);
            }
        }
    }
}

fn dogr() -> io::Result<ExitStatus> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        if let Ok(ip) = args[1].parse::<IpAddr>() {
            let domain = match ip {
                IpAddr::V4(ipv4) => ipv4_to_domain(&ipv4),
                IpAddr::V6(ipv6) => ipv6_to_domain(&ipv6),
            };
            return run_dog(&[domain, "PTR".to_string()]);
        }
    }
    run_dog(&args[1..])
}

fn run_dog(args: &[String]) -> std::io::Result<ExitStatus> {
    Command::new("dog").args(args).spawn()?.wait()
}

fn ipv4_to_domain(ipv4: &Ipv4Addr) -> String {
    let octets = ipv4.octets();
    format!(
        "{}.in-addr.arpa",
        octets
            .iter()
            .rev()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(".")
    )
}

fn ipv6_to_domain(ipv6: &Ipv6Addr) -> String {
    let octets = ipv6.octets();
    format!(
        "{}.ip6.arpa",
        octets
            .iter()
            .rev()
            .map(|n| format!("{:x}.{:x}", n & 0xf, n >> 4))
            .collect::<Vec<String>>()
            .join(".")
    )
}
