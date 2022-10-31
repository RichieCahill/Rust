//  BufReader, BufRead
// use std::io::{Read, Write};
use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
	io::{self, Read, Write},
	net::{TcpListener,TcpStream,Shutdown},
	process::Command,
	// str::{from_utf8, FromStr},
};

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

fn ls (in_file: String) -> i32{
	let test = "/ZFS/Storage/Plex/".to_owned()+&in_file;
	let status = Command::new("ls")
	.arg("-l")
	.arg("-a")
	.arg(test)
	.status()
	.expect("ffmpeg command failed to start");
	if status.success() {
		return 0
	} else {
		return 1
	}
}

fn request_handler(mut stream: &TcpStream) -> (i64, String) {
		// Array with a fixed size
	let mut rx_bytes = [0u8; 150];
	let _test=stream.read(&mut rx_bytes);
	let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
	let trimmed = received.trim_matches(char::from(0));

	let (pos, temp) = trimmed.split_at(1);
	let (hast, data) = temp.split_at(20);
	println!("{}", pos);
	println!("{}", hast);
	println!("{}", data);

	
	if hast.parse::<u64>().unwrap() == calculate_hash(data) {
		stream.write_all(b"1");
		(pos.parse::<i64>().unwrap(), data.to_string())
	} else {
		stream.write_all(b"0");
		(-1, "error".to_string())
	}
}

fn response (ip: &str, code: usize, pos: i64) -> io::Result<()> {
	let mut stream = TcpStream::connect(ip)?;

	if code == 1 {
		stream.write_all(code.to_string().as_bytes())?;
	}
	stream.shutdown(Shutdown::Both)


}

fn calculate_hash(string: &str) -> u64 {
	let mut temp = DefaultHasher::new();
	string.hash(&mut temp);
	temp.finish()
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				let (pos, file) = request_handler(&stream);
				if pos == -1 {
					continue;
				}
				let test = ls(file);
				response("127.0.0.1:8081",1,pos);

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
