// use std::io::prelude::*;
use std::{
	collections::hash_map::{DefaultHasher},
	fs,
	hash::{Hash, Hasher},
	io::{Read, Write},
	net::{TcpListener, TcpStream, Shutdown},
	str::{FromStr, from_utf8},
	string::ToString,
	process::exit,
	fs::{File},
};

const FILE_PATH: &str = "/home/r2r0m0c0/Projects/Rust/Networking/server/list.txt";

// write errors to log file
fn error(string: String) {
	println!("{}",string);
	const LOG_FILE: &str = "/home/r2r0m0c0/Projects/Rust/Networking/LOG.txt";
	if !std::path::Path::new(LOG_FILE).exists() {
		File::create(LOG_FILE);
	}
	let mut file = File::options().append(true).open(LOG_FILE).unwrap();
	file.write_all(string.as_bytes());
}

/*
Taken for https://doc.rust-lang.org/std/hash/index.html#
Just hashes a string
*/
fn calculate_hash(string: &String) -> u64 {
	let mut temp = DefaultHasher::new();
	string.hash(&mut temp);
	temp.finish()
}

/*
This Function was made with the help of David Krauthamer.
It parses a a fill of string to a vet of Strings
*/
fn import_list() -> Vec<String> {
	let contents = fs::read_to_string(FILE_PATH);
	let list: Vec<String> = contents.expect("REASON").lines()
			.filter_map(|val| match val.strip_prefix("/ZFS/Storage/Plex/") {
					Some(v) => Some(v),
					None => {
							error("A fucky wucky happened parsing string from\n".to_owned()+val+"\n");
							None
					}
			})
			.map(String::from_str).map(Result::unwrap).collect();
	list
}

fn spawner (ip: &str, item: &String,mut pos: usize, listlen: usize) -> usize {
	let mut stream = TcpStream::connect(ip);
	let mut hash = calculate_hash(&item).to_string();
	// if hash returns less then 19 this will fail loop mabby
	if hash.len() == 19 {
		hash="0".to_owned()+&hash;
	}
	let buf = pos.to_string()+&hash+&item;
	if buf.len() < 150 {
		stream.as_mut().expect("REASON").write_all(buf.as_bytes());
	} else {
		error("ERROR spawner buffer to long\n".to_owned()+&buf+"\n");
	}

	let mut rx_bytes = [0u8; 1];
		stream.as_mut().expect("REASON").read(&mut rx_bytes);
	let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
	let num = received.parse::<i32>().unwrap();
	if num == 1 {
		stream.expect("REASON").shutdown(Shutdown::Both);
	} else {
		error("Problem sending data to".to_owned()+ip+"\n");
		stream.expect("REASON").shutdown(Shutdown::Both);
	}
	if pos >= listlen {
		exit(0)
	}
	pos+=1;
	return pos
}

fn main() {
	let ips = vec!["127.0.0.1:8080"];
	let mut pos = 0;

	let list = import_list();
	// minus 1 to account for starting of list being 0
	let listlen = list.len()-1;

	for ip in &ips {
		pos = spawner (ip,&list[pos],pos,listlen);
	}
	let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
	for stream in listener.incoming() {
		match stream {
				Ok(mut stream) => {
					let mut rx_bytes = [0u8; 1];
					stream.read(&mut rx_bytes);
					let received = from_utf8(&rx_bytes).expect("valid utf8");
					stream.shutdown(Shutdown::Both);
					if received.parse::<i64>().unwrap() == 1 {
					pos = spawner (ips[0],&list[pos],pos,listlen);
					}
				}
				Err(_e) => { /* connection failed */ }
		}
	}
}

