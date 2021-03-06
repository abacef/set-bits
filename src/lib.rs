const HOLDING_UNIT_SIZE: u32 = 32;


pub struct SetBits {
    bit_vec: Vec<u32>,
}

impl SetBits {
    pub fn new() -> SetBits {
        SetBits {
            bit_vec: vec![]
        }
    }

    fn shrink_to_fit() {

    }

    pub fn num_true() {

    }

    pub fn highest_set_bit() {

    }

    pub fn set(&mut self, index: u64) {
        let unit_index_to_modify = index as usize / HOLDING_UNIT_SIZE as usize;
        let bit_in_unit_to_set = index % HOLDING_UNIT_SIZE as u64;
        let or_unit = 1 << bit_in_unit_to_set;

        if unit_index_to_modify + 1 > self.bit_vec.len() {
            for _ in self.bit_vec.len()..(unit_index_to_modify + 1) {
                self.bit_vec.push(0)
            }
        }

        self.bit_vec[unit_index_to_modify] |= or_unit
    }

    /*
     * Can probably be optimized for speed if necessary in the future, for example using the set
     * function implementation but not calculating the initial variables over and over again
     */
    /**
     * Sets all the bits from from_index (inclusive) to to_index (exclusive)
     */
    pub fn set_many(&mut self, from_index: u64, to_index: u64) {
        for i in from_index..to_index {
            self.set(i);
        }
    }

    /**
     * if bit is already clear, then do nothing
     */
    pub fn clear(&mut self, index: u64) {
        let unit_index_to_modify = index as usize / HOLDING_UNIT_SIZE as usize;
        if unit_index_to_modify < self.bit_vec.len() {
            let bit_in_unit_to_clear = index % HOLDING_UNIT_SIZE as u64;
            let mut clear_unit = 1 << bit_in_unit_to_clear;
            clear_unit &= u32::MAX; // flip all the bits
            self.bit_vec[unit_index_to_modify] &= clear_unit
        }
    }

    pub fn clear_many(from_index: u64, to_index: u64) {

    }

    pub fn flip(index: u64) {

    }

    pub fn flip_many(from_index: u64, to_index: u64) {

    }

    pub fn get(&self, index: u64) -> bool {
        if self.bit_vec.len() == 0 {
            false
        } else {
            let unit_index_to_check = index / HOLDING_UNIT_SIZE as u64;
            if unit_index_to_check > self.bit_vec.len() as u64 {
                false
            } else {
                let bit_in_unit_to_check = index % HOLDING_UNIT_SIZE as u64;
                let mask = 1 << bit_in_unit_to_check;
                self.bit_vec[unit_index_to_check as usize] & mask > 0
            }
        }
    }

    pub fn any_set(&self, from_index: u64, to_index: u64) -> bool {
        for i in from_index .. to_index {
            if self.get(i) {
                return true
            }
        }
        false
    }
}






#[cfg(test)]
mod tests {
    use crate::SetBits;

    #[test]
    fn basic_set_and_get() {
        let mut bs = SetBits::new();
        bs.set(5);
        assert!(!bs.get(4));
        assert!(bs.get(5));
        assert!(!bs.get(6));
        assert!(!bs.get(1034))
    }

    #[test]
    fn no_value_set__all_bits_false() {
        let bs = SetBits::new();
        assert!(!bs.get(0));
        assert!(!bs.get(5));
        assert!(!bs.get(100));
    }

    #[test]
    fn basic_set_many() {
        let mut bs = SetBits::new();
        bs.set_many(4, 11);
        for i in 4..11 {
            assert!(bs.get(i));
        }
    }


}

