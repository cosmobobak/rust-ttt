

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IterBits {
    pub bitboard: u64,
}

impl Iterator for IterBits {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.bitboard == 0 {
            None
        } else {
            let bit = self.bitboard.trailing_zeros();
            // clear_lsb() is likely faster than clear(bit) as it's (subtract->and) rather than (shift->not->and)
            self.bitboard &= self.bitboard - 1;
            // if bit == 0 {
            //     unsafe { std::hint::unreachable_unchecked(); }
            // }
            Some(bit as usize)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.bitboard.count_ones() as usize;
        (count, Some(count))
    }
}