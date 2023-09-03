use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::Ipv4Addr;
use std::path::Path;
use std::str::FromStr;

use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path to file>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let re_cidr = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}\/[0-9]{1,2}$").unwrap();
    let re_ip = Regex::new(r"^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    for line in reader.lines() {
        match line {
            Ok(content) => {
                if re_cidr.is_match(&content) {
                    match cidr_to_ips(&content) {
                        Ok(ips) => {
                            for ip in ips.iter() {
                                println!("{}", ip);
                            }
                        }
                        Err(_) => eprintln!("Error parsing {}", content),
                    };
                }else if re_ip.is_match(&content){
                    println!("{}", content);
                }
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    Ok(())
}

fn cidr_to_ips(cidr: &String) -> Result<Vec<Ipv4Addr>, Box<dyn Error>> {
    let parts: Vec<&str> = cidr.split('/').collect();
    let ip = Ipv4Addr::from_str(parts[0])?;
    let prefix_len: u32 = parts[1].parse()?;
    let octets = ip.octets();
    let start = ((octets[0] as u32) << 24)
        + ((octets[1] as u32) << 16)
        + ((octets[2] as u32) << 8)
        + (octets[3] as u32);

    let num_ips = 1u32 << (32 - prefix_len);

    Ok((start..(start + num_ips)).map(Ipv4Addr::from).collect())
}
