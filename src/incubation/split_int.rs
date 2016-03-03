pub trait SplitInt<'a> {
    type Output;
    type MutOutput;
    fn split(self) -> Self::Output;
    fn split_mut(&'a mut self) -> Self::MutOutput;
}

impl <'a> SplitInt<'a> for u16 {
    type Output = (u8, u8);
    type MutOutput = (&'a mut u8, &'a mut u8);

    fn split(self) -> Self::Output {
        ((self >> 8) as u8, self as u8)
    }

    fn split_mut(&'a mut self) -> Self::MutOutput {
        split_mut_u16(self)
    }
}

#[cfg(target_endian="big")]
pub fn split_mut_u16(n: &mut u16) -> (&mut u8, &mut u8) {
    let high = n as *mut u16 as *mut u8;
    
    unsafe { 
        let low = high.offset(1);
        
        let high = &mut *high;
        let low = &mut *low;
        
        (high, low)
    }
}

#[cfg(target_endian="little")]
pub fn split_mut_u16(n: &mut u16) -> (&mut u8, &mut u8) {
    let low = n as *mut u16 as *mut u8;
    
    unsafe { 
        let high = low.offset(1);
        
        let low = &mut *low;
        let high = &mut *high;
        
        (high, low)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn u16_low() {
        let n: u16 = 0b11001100_10101010;
        let (_, low) = n.split();

        assert_eq!(low, 0b10101010);
    }

    #[test]
    fn u16_high() {
        let n: u16 = 0b11001100_10101010;
        let (high, _) = n.split();

        assert_eq!(high, 0b11001100);
    }

    #[test]
    fn mut_u16_get_low() {
        let mut n: u16 = 0b11001100_10101010;
        let (_, low) = n.split_mut();
        assert_eq!(*low , 0b10101010);
    }

    #[test]
    fn mut_u16_get_high() {
        let mut n: u16 = 0b11001100_10101010;
        let (high, _) = n.split_mut();
        assert_eq!(*high , 0b11001100);
    }

    #[test]
    fn mut_u16_set_low() {
        let mut n: u16 = 0b11001100_10101010;
        {
            let (_, low) = n.split_mut();
            *low  = 0b01010101;
        }
        assert_eq!(n, 0b11001100_01010101);
    }

    #[test]
    fn mut_u16_set_high() {
        let mut n = 0b11001100_10101010;
        {
            let (high, _) = split_mut_u16(&mut n);
            *high = 0b00110011;
        }
        assert_eq!(n, 0b00110011_10101010);
    }
}
