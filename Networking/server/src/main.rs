// use std::io::prelude::*;
// use std::str::{FromStr};
use std::io::{self, Read, Write};
use std::net::{TcpListener,TcpStream,Shutdown};
use std::hash::{Hash,Hasher};
use std::collections::hash_map::{self, DefaultHasher};
use std::fs;

fn spawner (ip: &str, list: &Vec<&str>,mut pos: usize) -> io::Result<()> {
	let mut stream = TcpStream::connect(ip)?;
	let hash = calculate_hash(list[pos]).to_string();
	let spos = pos.to_string();
	let buf = spos+&hash+list[pos];
	stream.write(buf.as_bytes())?;

	let mut rx_bytes = String::new();
	let _test=stream.read_to_string(&mut rx_bytes);
	let num = rx_bytes.parse::<i32>().unwrap();
	// let mut rx_bytes = [0u8; 1];
	// let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
	// let num = received.parse::<i32>().unwrap();

	println!("{}", num);

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
*/
fn import_list (file_path: &str,mut out: &Vec<&str>){
	let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
	let mut list = contents.lines().collect::<Vec<_>>();
	for temp in list.iter_mut() {
		*temp = temp.strip_prefix("/ZFS/Storage/Plex/").unwrap();
	}
	let mut out = list.to_owned();
	// out = &list.to_owned();
}

/*
Taken for https://doc.rust-lang.org/std/hash/index.html#
Just hashes a string
*/
fn calculate_hash(string: &str) -> u64 {
	let mut temp = DefaultHasher::new();
	string.hash(&mut temp);
	temp.finish()
}



fn request_handler(mut stream: &TcpStream) -> io::Result<()> {
	// Array with a fixed size
let mut rx_bytes = [0u8; 150];

let _test=stream.read(&mut rx_bytes);
let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
println!("{}", received);

// let mut rx_bytes2 = String::new();
// let _test=stream.read_to_string(&mut rx_bytes);
// println!("{}", rx_bytes);

let data = b"1";
stream.write_all(data);
stream.flush()
// ffmpeg();
}


fn main() {
	// let mut list: Vec<&str> = vec![];
	let ips = vec!["127.0.0.1:8080"];
	const FILE_PATH: &str = "/home/r2r0m0c0/Projects/Rust/Networking/server/list.txt";
	let mut pos = 0;

	// import_list(&FILE_PATH,&list);
	let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
	let mut list = contents.lines().collect::<Vec<_>>();
	for temp in list.iter_mut() {
		*temp = temp.strip_prefix("/ZFS/Storage/Plex/").unwrap();
	}
	// for ip in ips {
		spawner (ips[0],&list,pos);
	// }
	let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
	for stream in listener.incoming() {
		match stream {
				Ok(stream) => {
					request_handler(&stream);
				}
				Err(_e) => { /* connection failed */ }
		}
	}
}

