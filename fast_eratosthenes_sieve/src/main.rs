use num::integer::sqrt;

// use std::arch::x86_64::*;

// counts the number prime numbers in the array minus the bits that are
fn count(_prime: &Vec<u64>, _size: usize, _extra: u32) -> u32{
	let mut temp = 0;
		for i in 0.._size {
				temp += _prime[i].count_zeros();
		}
		// return temp - (extra-_mm_popcnt_u64(prime[size-1]>>(64-extra)));

		return temp	- (_extra-(_prime[_size-1]>>(64-_extra)).count_ones())
		;
}
	
// fn eratosthenes_sieve(_n: usize, _prime: Vec<u64>, _size: usize) -> (){
fn eratosthenes_sieve(_n: usize,_prime: &mut Vec<u64>) -> (){
	for _i in (3..sqrt(_n)).step_by(2) {
		if (_prime[_i>>7] & (1<<((_i>>1)%64))) == 0{
			for _j in (_i*_i.._n).step_by(2*_i) {
				_prime[_j>>7] |= 1<< ((_j>>1)%64);
			}
		}
	}

	
	for x in _prime {
    println!("{:#02x}", x);
	}
}



fn main() {
	// The number you want to calculate to
	const TOTAL: usize = 1024;
	// calculates next multipule of 128 above Total
	const MULT: usize =128-(TOTAL%128)+TOTAL;
	// calculates the difference  between total nad Mult
	const EXTRA: usize  = (MULT-TOTAL)/2;
	// create and list of 64bit ints 128 time smaller then Mult
	const SIZE: usize  = MULT/128;
	let mut _prime: Vec<u64> = vec![0; SIZE];
	// eratosthenes_sieve(MULT,_prime,SIZE);
	eratosthenes_sieve(MULT,&mut _prime);



	let test = count(&_prime, SIZE, EXTRA.try_into().unwrap());

	println!("{}", test);



}