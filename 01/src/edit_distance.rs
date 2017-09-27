// https://prismoskills.appspot.com/lessons/Bitwise_Operators/Count_ones_in_an_integer.jsp
// todo: there's almost certainly a better way to write this
fn count_ones(n: u8) -> u32 {
    let mut count = 0;
    let mut m = n;
    while m != 0 {
        m = m & (m - 1);
        count = count + 1;
    }
    count
}

pub fn get_edit_distance(a: &[u8], b: &[u8]) -> u32 {
    let mut counter = 0u32;
    for i in 0..a.len() {
        counter = counter + count_ones(a[i] ^ b[i]);
    }
    counter
}
