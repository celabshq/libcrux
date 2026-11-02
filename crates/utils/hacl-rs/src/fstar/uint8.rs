pub fn eq_mask(a: u8, b: u8) -> u8 {
    let x = a ^ b;
    let minus_x = (!x).wrapping_add(1u8);
    let x_or_minus_x = x | minus_x;
    let xnx = x_or_minus_x.wrapping_shr(7);
    xnx.wrapping_sub(1u8)
}
