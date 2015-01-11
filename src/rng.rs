extern crate std;
// extern crate time;

pub struct Rng {
	index : usize,
	state : [u32; 16],
	// Rngs shouldn't be copyable, but this introduce problems in the constructor, hrm...
	// _marker: std::marker::NoCopy
}

impl Rng {
	pub fn new(seed:u32) -> Rng {
		let mut r = Rng {
			index: 0,
			// doesn't seem to be supported...
			// http://stackoverflow.com/questions/26757355/how-do-i-collect-into-an-array
			// state: range(0, 16).map(|i| i*seed).collect::<Vec<u32>>().slice(0,16)
			state: [0;16]
		};
		for i in range(0,16) {
			r.state[i] = seed * i as u32;
		}
		r
	}

	// seems to be that the the time is hard to import on windows, lets keep it hidden for now...
	// fn new_ns() -> Rng {
	// 	let ct = time::time::get_time();
	// 	return Rng::new(ct.nsep as u32);
	// }

	pub fn next(&mut self) -> u32 {
		let mut a = self.state[self.index];
		let mut c = self.state[(self.index + 13) & 15];
		let b = a ^ c ^ (a << 16) ^ (c << 15);
		c = self.state[(self.index + 9) & 15];
		c ^= c >> 11;
		self.state[self.index] = b ^ c;
		a = self.state[self.index];
		let d = a ^ ((a << 5) & 0xDA442D24);
		self.index = (self.index + 15) & 15;
		a = self.state[self.index];
		self.state[self.index] = a ^ b ^ d ^ (a << 2) ^ (b << 18) ^ (c << 28);
		self.state[self.index]
	}

	pub fn next01(&mut self) -> f32 {
		self.next() as f32 / std::u32::MAX as f32
	}
}
