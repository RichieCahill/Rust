//  BufReader, BufRead
// use std::io::{Read, Write};

// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};

use std::slice::RChunksExact;
use std::str::{from_utf8, FromStr};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::mpsc::Receiver;

// Wrapper for ffmpeg that output 0 on success and 1 on failure
fn ffmpeg (in_file: &str, out_file: &str) -> i32{
	let status = Command::new("ffmpeg")
	.arg("-hwaccel")
	.arg("cuda")
	.arg("-hwaccel_output_format")
	.arg("cuda")
	.arg("-loglevel")
	.arg("quiet")
	.arg("-stats")
	.arg("-i")
	.arg(in_file)
	.arg("-tune")
	.arg("hq")
	.arg("-c:a")
	.arg("copy")
	.arg("-c:v")
	.arg("hevc_nvenc")
	.arg(out_file)
	.status()
	.expect("ffmpeg command failed to start");
	if status.success() {
		return 0
	} else {
		return 1
	}
}

fn ls (in_file: &str, out_file: &str) -> i32{
	let status = Command::new("ls")
	.arg("-l")
	.arg("-a")
	.status()
	.expect("ffmpeg command failed to start");
	if status.success() {
		return 0
	} else {
		return 1
	}
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

// fn calculate_hash<T: Hash>(t: &T) -> u64 {
// 	let mut s = DefaultHasher::new();
// 	t.hash(&mut s);
// 	s.finish()
// }


fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	for stream in listener.incoming() {
		match stream {
				Ok(stream) => {
					request_handler(&stream);
				}
				Err(_e) => { /* connection failed */ }
		}
	}
		// const DIR: &str  = "/home/r2r0m0c0/Projects/Rust/Networking/client/";
		// let in_file = DIR.to_owned()+"original.mp4";
		// let out_file = DIR.to_owned()+"new.mkv";
		// let temp = ffmpeg(&in_file,&out_file);
		// println!("{}", temp);
	}
