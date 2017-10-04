use pkcs_7::unpad;

pub fn run_15() {
    println!("{:?}", unpad("ICE ICE BABY\x04\x04\x04\x04".as_bytes(), 16));
    println!("{:?}", unpad("ICE ICE BABY\x01\x02\x03\x04".as_bytes(), 16));
}
