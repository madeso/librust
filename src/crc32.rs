//! CRC32 implementation based on https://github.com/ledbettj/crc32/tree/master/rust unknown license.
//! Port http://csbruce.com/software/crc32.c instead as it's public domain

pub struct Crc32 {
    table: [u32; 256],
    value: u32,
}

static CRC32_INITIAL: u32 = 0xedb88320;

impl Crc32 {
    pub fn new() -> Crc32 {
        let mut c = Crc32{table: [0;256], value: 0xffffffff,};
        for i in range(0, 256) {
            let mut v = i as u32;
            for _ in range(0, 8) {
                v = if v & 1 != 0 { CRC32_INITIAL ^ (v >> 1) } else { v >> 1 }
            }
            c.table[i] = v;
        }
        c
    }
    
    pub fn start(&mut self) { self.value = 0xffffffff; }
    
    pub fn update(&mut self, buf: &[u8]) {
        for &i in buf.iter() {
            self.value =
                self.table[((self.value ^ (i as u32)) & 0xFF) as usize] ^
                    (self.value >> 8);
        }
    }

    pub fn finalize(&mut self) -> u32 { self.value ^ 0xffffffffu32 }

    pub fn crc(buf: &[u8]) -> u32 {
    	let mut c = Crc32::new();
        c.start();
        c.update(buf);
        c.finalize()
    }

    pub fn crc_str(x:&str) -> u32 {
	    Crc32::crc(x.to_string().into_bytes().as_slice())
	}
}
