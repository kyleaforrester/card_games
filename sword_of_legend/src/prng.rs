use std::time::{SystemTime, UNIX_EPOCH};
use std::num::Wrapping;

#[derive(Debug)]
pub struct Prng {
    state: u64,
}

impl Prng {
    pub fn new() -> Prng {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let seed = ((duration.subsec_nanos() as u64) << 32) | duration.as_secs();

        Prng {
            state: seed,
        }
    }

    //PRNG Algorithm
    //Credit:
    //https://nullprogram.com/blog/2017/09/21/
    fn gen_u32(&mut self) -> u32 {
        let state = Wrapping(self.state);
        let m = Wrapping(0x9b60933458e17d7d);
        let a = Wrapping(0xd737232eeccdf7ed);
        let new_state = state * m + a;
        self.state = new_state.0;

        let shift = 29 - (new_state.0 >> 61);
        (new_state.0 >> shift) as u32
    }

    pub fn random(&mut self) -> f64 {
        // Return the next random floating-point number in the range 0.0 <= X < 1.0
        self.gen_u32() as f64 / 4294967296u64 as f64
    }

    pub fn choose<'a, T: ?Sized>(&mut self, array: &Vec::<&'a T>) -> &'a T {
        let num: u32 = self.gen_u32();

        array[(num as usize) % array.len()]
    }

    pub fn shuffle<T>(&mut self, vec: &mut Vec<T>) {
        for i in (1..vec.len()).into_iter().rev() {
            let swap_idx = (self.gen_u32() as usize) % (i + 1);
            vec.swap(swap_idx, i);
        }
    }

    pub fn randint(&mut self, inclusive_start: u32, inclusive_end: u32) -> u32 {
        (self.gen_u32() % (inclusive_end - inclusive_start + 1)) + inclusive_start
    }
}
