// use std::io::prelude::*;
use std::str::{FromStr};
use std::io::{self,Read, Write};
use std::net::TcpStream;
use std::net::Shutdown;

fn starter (ip: &str) -> io::Result<()> {
	let mut stream = TcpStream::connect(ip)?;

	let data = b"Hello";
	
	stream.write_all(data)?;

	// Recevies return code
	// I think this can be improvoved
	let mut rx_bytes = [0u8; 1];
	let _test=stream.read(&mut rx_bytes);
	let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
	let num = received.parse::<i32>().unwrap();

	// This returs nonsens 808464433
	// let num = u32::from_be_bytes(rx_bytes);
	println!("{}", num);

	if num == 1 {
		print!("yay");
		stream.shutdown(Shutdown::Both)		
	} else {
		print!("ERROR");
		stream.flush()
	}
}


fn main() {
	let _ips = vec!["127.0.0.1:8080","127.0.0.1:8080"];
	let ip = "127.0.0.1:8080";
	// let addr = SocketAddr::new(127, 0, 0, 1);
	// let server_details = "127.0.0.1:80";
	// let test: Ipv4Addr = ip.parse().expect("Unable to parse socket address");
	// let addr: addr =  server_detailsdr;
	starter (&ip);
}

