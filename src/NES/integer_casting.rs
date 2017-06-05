pub trait CastWithNegation {
    fn cast_with_neg(&self) -> u16;
}

impl CastWithNegation for u8 {
    fn cast_with_neg(&self) -> u16 {
        if (*self)&0x80 == 0x80 {
            let inverse = !(*self) + 1;
            !(inverse as u16) + 1
        } else {
            (*self) as u16
        }
    }

}