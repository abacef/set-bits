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
        let unit_to_modify = index / HOLDING_UNIT_SIZE as u64;
        let units_to_allocate = unit_to_modify as isize - self.bit_vec.len() as isize + 1;
        let bit_in_unit_to_set = index % HOLDING_UNIT_SIZE as u64;
        let or_unit = 1 << bit_in_unit_to_set;
        println!("or unit {}", or_unit);

        if units_to_allocate > 0 {
            for _ in 0..units_to_allocate {
                self.bit_vec.push(0)
            }
        }

        self.bit_vec[unit_to_modify as usize] |= or_unit
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

    pub fn clear(index: u64) {

    }

    pub fn clear_many(from_index: u64, to_index: u64) {

    }

    pub fn flip(index: u64) {

    }

    pub fn flip_many(from_index: u64, to_index: u64) {

    }

    pub fn get(&self, index: u64) -> bool {
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
    fn basic_set_many() {
        let mut bs = SetBits::new();
        bs.set_many(4, 11);
        for i in 4..11 {
            assert!(bs.get(i));
        }
    }


}

