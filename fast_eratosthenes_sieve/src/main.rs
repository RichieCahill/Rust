
// _mm_popcnt_u64 For rust
#[inline]
fn _popcntu64(mut x: u64) -> u64{
	use std::arch::asm;
	unsafe {
	    asm!(
	        "popcnt {x}, {x}",
					x = inout(reg) x,
	    );
			return x;
	}
}

// counts the number prime numbers in the array minus the bits that are
fn count(prime: &Vec<u64>, size: usize, extra: usize) -> u64{
	let mut temp = 0;
		for i in 0..size {
				temp += 64-_popcntu64(prime[i]);
		}
				let extra: u64 = extra.try_into().unwrap();
				return temp -(extra-_popcntu64(prime[size-1]>>(64-extra)));
}
	
// fn eratosthenes_sieve(n: usize, prime: Vec<u64>, size: usize) -> (){
fn eratosthenes_sieve(n: usize,prime: &mut Vec<u64>) -> (){
	use num::integer::sqrt;
	for i in (3..sqrt(n)).step_by(2) {
		if (prime[i>>7] & (1<<((i>>1)%64))) == 0{
			for j in (i*i..n).step_by(2*i) {
				prime[j>>7] |= 1<< ((j>>1)%64);
			}
		}
	}
}



fn main() {
	// The number you want to calculate to
	const TOTAL: usize = 1000000000;
	// calculates next multipule of 128 above Total
	const MULT: usize =128-(TOTAL%128)+TOTAL;
	// calculates the difference  between total nad Mult
	const EXTRA: usize  = (MULT-TOTAL)/2;
	// create and list of 64bit ints 128 time smaller then Mult
	const SIZE: usize  = MULT/128;
	let mut prime: Vec<u64> = vec![0; SIZE];

	use std::time::Instant;
	let now = Instant::now();

	// eratosthenes_sieve(MULT,prime,SIZE);
	eratosthenes_sieve(MULT,&mut prime);

	println!("{}", count(&prime, SIZE, EXTRA.try_into().unwrap()));

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

}
