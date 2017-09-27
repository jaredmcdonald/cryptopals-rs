use read_file::strings_from_filename;
use hex::parse_hex;
use edit_distance::get_edit_distance;
use aes::decrypt_aes_ecb;

fn is_aes_encrypted(bytes: &[u8]) -> bool {
    let keys = [
        vec![0; 16],
        vec![1; 16],
        vec![2; 16],
        vec![3; 16],
        vec![4; 16],
    ];

    // wattttt
    let decryption_attempts = keys.iter().map(|key| decrypt_aes_ecb(&bytes, key)).collect::<Vec<_>>();
    let distance_1 = get_edit_distance(&decryption_attempts[0], &decryption_attempts[1]) as i32;
    let distance_2 = get_edit_distance(&decryption_attempts[1], &decryption_attempts[2]) as i32;
    let distance_3 = get_edit_distance(&decryption_attempts[2], &decryption_attempts[3]) as i32;
    let distance_4 = get_edit_distance(&decryption_attempts[3], &decryption_attempts[4]) as i32;

    false
}

pub fn run_08() {
    let lines = strings_from_filename("08.txt");
    for line in lines {
        let bytes = parse_hex(&line);
        if is_aes_encrypted(&bytes) {
            println!("this string is AES ECB encrypted:\n{}", line);
        }
    }
}
