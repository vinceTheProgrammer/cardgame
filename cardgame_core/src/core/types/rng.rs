#[derive(Debug, Clone, Copy)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    pub(crate) fn new(seed: u64) -> Self {
        // avoid zero seed (xorshift degenerates)
        let seed = if seed == 0 { 0xdead_beef_cafe_babe } else { seed };
        Self { state: seed }
    }

    pub(crate) fn next_u64(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(2685821657736338717)
    }

    pub(crate) fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    pub(crate) fn range_u32(&mut self, max_exclusive: u32) -> u32 {
        // NOTE: modulo bias exists, but usually acceptable.
        // If you want bias-free, you can implement rejection sampling.
        if max_exclusive == 0 {
            return 0;
        }
        self.next_u32() % max_exclusive
    }

    pub(crate) fn coin_flip(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }

    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        let len = v.len();
        if len <= 1 {
            return;
        }

        for i in (1..len).rev() {
            let j = (self.next_u32() as usize) % (i + 1);
            v.swap(i, j);
        }
    }
}
