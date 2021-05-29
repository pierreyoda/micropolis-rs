use std::cmp;

/// See https://rosettacode.org/wiki/Elementary_cellular_automaton/Random_Number_Generator
struct ElementaryCA {
    rule: u8,
    state: u64,
}
impl ElementaryCA {
    fn new(rule: u8) -> (u64, ElementaryCA) {
        let out = ElementaryCA { rule, state: 1 };
        (out.state, out)
    }
    fn next(&mut self) -> u64 {
        let mut next_state = 0u64;
        let state = self.state;
        for i in 0..64 {
            next_state |=
                (((self.rule as u64) >> (7 & (state.rotate_left(1).rotate_right(i as u32)))) & 1)
                    << i;
        }
        self.state = next_state;
        self.state
    }
}
fn rep_u64(val: u64) -> String {
    let mut out = String::new();
    for i in (0..64).rev() {
        if 1 << i & val != 0 {
            out += "\u{2588}";
        } else {
            out += "-";
        }
    }
    out
}

pub struct MicropolisRandom {
    seed: i32,
    next_random: u64,
    inner_prng: ElementaryCA,
}

impl MicropolisRandom {
    pub fn from_random_system_seed() -> Self {
        // simulate gettimeofday
        // TODO: check this, especially with WASM
        let now = chrono::Utc::now();
        let seed = (now.timestamp_nanos() / 1000) ^ now.timestamp();

        // build the corresponding seed
        let final_seed = seed as i32;
        Self::from_seed(final_seed)
    }

    fn from_seed(seed: i32) -> Self {
        Self {
            seed,
            next_random: seed as u64,
            inner_prng: ElementaryCA::new(30).1,
        }
    }

    pub fn seed(&mut self, seed: i32) {
        self.next_random = seed as u64;
    }

    /// Draw a random number in the given upper inclusive range.
    pub fn get_random(&mut self, range: i16) -> i16 {
        let local_range = range as u16 + 1;
        let max_multiple = (0xFFFF / local_range) * local_range; // TODO: ?

        let mut r_number: u16 = 0;
        loop {
            r_number = self.get_random_16() as u16;
            if r_number < max_multiple {
                break;
            }
        }

        (r_number % local_range) as i16
    }

    /// Draw a random number in the given upper inclusive range,
    /// with a preference towards smaller values.
    pub fn get_e_random(&mut self, range: i16) -> i16 {
        cmp::min(self.get_random(range), self.get_random(range))
    }

    /// Draw a random unsigned 16-bit number.
    pub fn get_random_16(&mut self) -> i16 {
        (self.sim_random() & 0x0000FFFF) as i16
    }

    /// Draw a random signed 16-bit number.
    pub fn get_random_16_signed(&mut self) -> u16 {
        let random = self.get_random_16();
        if random > 0x7FFF {
            (0x7FFF - random) as u16
        } else {
            random as u16
        }
    }

    /// Draw a random 32-bit number (internal function).
    fn sim_random(&mut self) -> i32 {
        self.next_random = self.inner_prng.next();
        ((self.next_random & 0xFFFF00) >> 8) as i32
    }
}
