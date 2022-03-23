
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

fn _popcntu128(n: u128) -> u64{
	_popcntu64((n & 0x0000000000000000FFFFFFFFFFFFFFFF).try_into().unwrap())+_popcntu64((n >> 64).try_into().unwrap())
}



// counts the number prime numbers in the array minus the bits that are
fn count(_prime: &Vec<u128>, _size: usize, mut _extra: usize) -> u64{
	let mut temp = 0;
		for i in 0.._size {
				temp += 128-_popcntu128(_prime[i]);
		}
				let _extra: u64 = _extra.try_into().unwrap();
				return temp -(_extra-_popcntu128(_prime[_size-1]>>(128-_extra)));
}
	
// fn eratosthenes_sieve(_n: usize, _prime: Vec<u128>, _size: usize) -> (){
fn eratosthenes_sieve(_n: usize,_prime: &mut Vec<u128>) -> (){
	use num::integer::sqrt;
	for _i in (3..sqrt(_n)).step_by(2) {
		if (_prime[_i/256] & (1<<((_i>>1)%128))) == 0{
			for _j in (_i*_i.._n).step_by(2*_i) {
				_prime[_j/256] |= 1<< ((_j>>1)%128);
			}
		}
	}
}



fn main() {
	// The number you want to calculate to
	const TOTAL: usize = 10000000000;
	// calculates next multipule of 128 above Total
	const MULT: usize =256-(TOTAL%256)+TOTAL;
	// calculates the difference  between total nad Mult
	const EXTRA: usize  = (MULT-TOTAL)/2;
	// create and list of 64bit ints 128 time smaller then Mult
	const SIZE: usize  = MULT/256;
	let mut _prime: Vec<u128> = vec![0; SIZE];

	use std::time::Instant;
	let now = Instant::now();

	// eratosthenes_sieve(MULT,_prime,SIZE);
	eratosthenes_sieve(MULT,&mut _prime);

	println!("{}", count(&_prime, SIZE, EXTRA.try_into().unwrap()));

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);
}
