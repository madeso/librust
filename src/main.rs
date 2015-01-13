extern crate librust;

fn main() {
	let mut rng = librust::rng::Rng::new(42);
	for _ in range(0,20) {
		for _ in range(0,15) {
			print!("{} ", rng.next01());
		}

		println!("");
	}

	for str in ["cat", "hat", "dog", "let"].iter() {
        let x = librust::crc32::Crc32::crc_str(*str);
        println!("{} -> {}", *str, x);
    }
}
