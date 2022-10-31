// use std::io::prelude::*;
use std::{
	collections::hash_map::{self, DefaultHasher},
	fs,
	hash::{Hash,Hasher},
	io::{self, Read, Write},
	net::{TcpListener,TcpStream,Shutdown},
	path::{PathBuf},
	str::{FromStr,from_utf8},
	string::ToString, process::exit,
};

fn spawner (ip: &str, list: Vec<String>,mut pos: usize) -> io::Result<()> {
	let mut stream = TcpStream::connect(ip)?;
	let mut hash = calculate_hash(list[pos].clone()).to_string();
	if hash.len() == 19 {
		hash="0".to_owned()+&hash;
	}
	let buf = pos.to_string()+&hash+&list[pos];
	println!("{}", pos);
	println!("{}", hash.len());
	println!("{}", hash);
	println!("{}", list[pos]);
	if buf.len() < 150 {
		stream.write_all(buf.as_bytes())?;
	} else {
		println!("error buf To long");
	}

	let mut rx_bytes = [0u8; 1];
	let _test=stream.read(&mut rx_bytes);
	let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
	let num = received.parse::<i32>().unwrap();
	if pos == list.len()-1 {
			exit(0)
	}
	if num == 1 {
		print!("yay");
		pos+=1;
		stream.shutdown(Shutdown::Both)
	} else {
		print!("ERROR");
		stream.shutdown(Shutdown::Both)
	}
}

/*
I hate this function with a burning passion!!! However it should work.
The problems are as its 2 loop isn't the end of the world because the current dataset is relatively small.
I think it should be possible to make it a single loop to read and clean the data.
Also its copying the vec I was just fed up fighting with the borrow checker.
// https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html
*/
fn import_list(file_path: PathBuf) -> Vec<String> {
	let contents = fs::read_to_string(file_path);
	let list: Vec<String> = contents.expect("REASON").lines()
			.filter_map(|val| match val.strip_prefix("/ZFS/Storage/Plex/") {
					Some(v) => Some(v),
					None => {
							println!("A fucky wucky happened parsing !");
							None
					}
			})
			.map(String::from_str).map(Result::unwrap).collect();
	list
}

/*
Taken for https://doc.rust-lang.org/std/hash/index.html#
Just hashes a string
*/
fn calculate_hash(string: String) -> u64 {
	let mut temp = DefaultHasher::new();
	string.hash(&mut temp);
	temp.finish()
}

fn main() {
	let mut list: Vec<String> = vec![];
	let ips = vec!["127.0.0.1:8080"];
	const FILE_PATH: &str = "/home/r2r0m0c0/Projects/Rust/Networking/server/list.txt";
	let mut pos = 0;

	list = import_list((&FILE_PATH).into());
	for ip in &ips {
		spawner (ips[0],list.clone(),pos);
		pos+=1;
	}

	let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
	for stream in listener.incoming() {
		match stream {
				Ok(mut stream) => {
					println!("in loop");
					let mut rx_bytes = [0u8; 1];
					let _test=stream.read(&mut rx_bytes);
					let received = from_utf8(&rx_bytes).expect("valid utf8");
					println!("{}", received);
					println!("{}", pos);
					stream.shutdown(Shutdown::Both);
					if received.parse::<i64>().unwrap() == 1 {
						spawner (ips[0],list.clone(),pos);
						pos+=1;
					}
								
										
				}
				Err(_e) => { /* connection failed */ }
		}
	}
}

// clippy
// rist fmt