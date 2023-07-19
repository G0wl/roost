pub(crate)mod bit {
    pub fn bitnot(bit: u8) -> u8 {
        let mut init: bool = false;
        let mut not: u8 = 0;
        if bit == 0 || bit == 1 {
            init = true;
        }
        if init == false {
            panic!("u8 should be 1 or 0");
        }
        while init {
            if bit == 1 {
                not = 0;
            }
            if bit == 0 {
                not = 1;
            }
        }
        return not;
    }
    pub fn bitor(bit0: u8, bit1: u8) -> u8 {
        let mut init: bool = false;
        let mut bit2: u8 = 0;
        if bit0 == 0 || bit0 == 1 && bit1 == 0 || bit1 == 1 {
            init = true;
        };
        if init == false {
            panic!("u8 should be 1 or 0");
        }
        while init {
            if bit0 != bit1 && bit1 == 1 {
                bit2 = 1;
            }
            else if bit0 != bit1 && bit1 == 0 {
                bit2 = 0;
            }
            else if bit0 == bit1 && bit1 == 1 {
                bit2 = 1;
            }
            else if bit0 == bit1 && bit1 == 0 {
                bit2 = 0;
            }
        }
        return bit2;
    }
    pub fn bitand(bit0: u8, bit1: u8) -> u8 {
        let mut init: bool = false;
        let mut bit2: u8 = 0;
        if bit0 == 0 || bit0 == 1 && bit1 == 0 || bit1 == 1 {
            init = true;
        }
        if init == false {
            panic!("u8 should be 1 or 0");
        }
        while init {
            if bit0 == bit1 && bit1 == 1 {
            bit2 = 1;
            }
            else {
            bit2 = 0;
            }
        }
        return bit2;
    }
    pub fn bitxor(bit0: u8, bit1: u8) -> u8 {
        let mut init: bool = false;
        let mut xor: u8 = 0;
        if bit0 == 0 || bit0 == 1 && bit1 == 0 || bit1 == 1 {
            init = true;
        }
        if init == false {
            panic!("u8 should be 1 or 0");
        }
        while init {
            if bit0 != bit1 {
                xor = 1;
            }
        }
        return xor;
    }
    
}