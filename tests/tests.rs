extern crate keccak_rust;
use keccak_rust::*;

const EMPTY_INPUT_BYTES: [Bit; 0] = [];

const SHORT_INPUT_BYTES: [Bit; 12] = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];

const LONG_INPUT_BYTES: [Bit; 445] = [76, 111, 114, 101, 109, 32, 105, 112, 115, 117, 109, 32, 100, 111, 108, 111, 114, 32, 115, 105, 116, 32, 97, 109, 101, 116, 44, 32, 99, 111, 110, 115, 101, 99, 116, 101, 116, 117, 114, 32, 97, 100, 105, 112, 105, 115, 99, 105, 110, 103, 32, 101, 108, 105, 116, 44, 32, 115, 101, 100, 32, 100, 111, 32, 101, 105, 117, 115, 109, 111, 100, 32, 116, 101, 109, 112, 111, 114, 32, 105, 110, 99, 105, 100, 105, 100, 117, 110, 116, 32, 117, 116, 32, 108, 97, 98, 111, 114, 101, 32, 101, 116, 32, 100, 111, 108, 111, 114, 101, 32, 109, 97, 103, 110, 97, 32, 97, 108, 105, 113, 117, 97, 46, 32, 85, 116, 32, 101, 110, 105, 109, 32, 97, 100, 32, 109, 105, 110, 105, 109, 32, 118, 101, 110, 105, 97, 109, 44, 32, 113, 117, 105, 115, 32, 110, 111, 115, 116, 114, 117, 100, 32, 101, 120, 101, 114, 99, 105, 116, 97, 116, 105, 111, 110, 32, 117, 108, 108, 97, 109, 99, 111, 32, 108, 97, 98, 111, 114, 105, 115, 32, 110, 105, 115, 105, 32, 117, 116, 32, 97, 108, 105, 113, 117, 105, 112, 32, 101, 120, 32, 101, 97, 32, 99, 111, 109, 109, 111, 100, 111, 32, 99, 111, 110, 115, 101, 113, 117, 97, 116, 46, 32, 68, 117, 105, 115, 32, 97, 117, 116, 101, 32, 105, 114, 117, 114, 101, 32, 100, 111, 108, 111, 114, 32, 105, 110, 32, 114, 101, 112, 114, 101, 104, 101, 110, 100, 101, 114, 105, 116, 32, 105, 110, 32, 118, 111, 108, 117, 112, 116, 97, 116, 101, 32, 118, 101, 108, 105, 116, 32, 101, 115, 115, 101, 32, 99, 105, 108, 108, 117, 109, 32, 100, 111, 108, 111, 114, 101, 32, 101, 117, 32, 102, 117, 103, 105, 97, 116, 32, 110, 117, 108, 108, 97, 32, 112, 97, 114, 105, 97, 116, 117, 114, 46, 32, 69, 120, 99, 101, 112, 116, 101, 117, 114, 32, 115, 105, 110, 116, 32, 111, 99, 99, 97, 101, 99, 97, 116, 32, 99, 117, 112, 105, 100, 97, 116, 97, 116, 32, 110, 111, 110, 32, 112, 114, 111, 105, 100, 101, 110, 116, 44, 32, 115, 117, 110, 116, 32, 105, 110, 32, 99, 117, 108, 112, 97, 32, 113, 117, 105, 32, 111, 102, 102, 105, 99, 105, 97, 32, 100, 101, 115, 101, 114, 117, 110, 116, 32, 109, 111, 108, 108, 105, 116, 32, 97, 110, 105, 109, 32, 105, 100, 32, 101, 115, 116, 32, 108, 97, 98, 111, 114, 117, 109, 46];

fn empty_output(num: u8) -> BitsVec {
    let res = match num {
        0 => vec![247, 24, 55, 80, 43, 168, 225, 8, 55, 189, 216, 211, 101, 173, 184, 85, 145, 137, 86, 2, 252, 85, 43, 72, 183, 57, 10, 189],
        1 => vec![197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112],
        2 => vec![44, 35, 20, 106, 99, 162, 154, 207, 153, 231, 59, 136, 248, 194, 78, 170, 125, 198, 10, 167, 113, 120, 12, 204, 0, 106, 251, 250, 143, 226, 71, 155, 45, 210, 178, 19, 98, 51, 116, 65, 172, 18, 181, 21, 145, 25, 87, 255],
        3 => vec![14, 171, 66, 222, 76, 60, 235, 146, 53, 252, 145, 172, 255, 231, 70, 178, 156, 41, 168, 195, 102, 183, 198, 14, 78, 103, 196, 102, 243, 106, 67, 4, 192, 15, 169, 202, 249, 216, 121, 118, 186, 70, 155, 203, 224, 103, 19, 180, 53, 240, 145, 239, 39, 105, 251, 22, 12, 218, 179, 61, 54, 112, 104, 14],
        _ => panic!("Wrong empty output test num")
    };
    res
}

fn short_output(num: u8) -> BitsVec {
    let res = match num {
        0 => vec![37, 40, 220, 123, 26, 44, 44, 69, 211, 27, 162, 229, 178, 29, 70, 247, 133, 88, 22, 14, 240, 70, 160, 77, 242, 14, 34, 51],
        1 => vec![236, 208, 225, 8, 169, 142, 25, 42, 241, 210, 194, 80, 85, 244, 227, 190, 215, 132, 181, 200, 119, 32, 78, 115, 33, 154, 82, 3, 37, 31, 234, 171],
        2 => vec![163, 194, 239, 46, 227, 4, 33, 123, 21, 176, 156, 55, 2, 25, 174, 93, 192, 162, 42, 144, 250, 241, 153, 239, 183, 57, 235, 212, 118, 190, 142, 249, 59, 59, 171, 47, 5, 107, 105, 26, 251, 79, 191, 251, 8, 233, 49, 229],
        3 => vec![64, 16, 231, 146, 172, 179, 60, 118, 127, 27, 208, 164, 150, 25, 118, 155, 240, 68, 238, 242, 232, 172, 163, 39, 23, 20, 157, 188, 149, 226, 178, 125, 220, 36, 170, 133, 40, 198, 167, 242, 161, 139, 146, 108, 69, 208, 175, 242, 144, 193, 143, 30, 223, 160, 79, 206, 60, 233, 27, 124, 105, 243, 229, 245],
        _ => panic!("Wrong short output test num")
    };
    res
}

fn long_output(num: u8) -> BitsVec {
    let res = match num {
        0 => vec![51, 206, 47, 198, 90, 198, 98, 250, 225, 118, 67, 208, 142, 72, 87, 44, 114, 183, 153, 34, 170, 234, 93, 54, 211, 148, 60, 140],
        1 => vec![171, 147, 168, 150, 22, 217, 121, 245, 138, 221, 165, 42, 160, 197, 111, 57, 237, 233, 136, 18, 142, 244, 122, 184, 164, 210, 195, 196, 212, 92, 80, 73],
        2 => vec![17, 120, 53, 118, 58, 72, 106, 26, 224, 189, 35, 200, 174, 19, 22, 150, 200, 20, 4, 61, 158, 74, 242, 36, 3, 77, 170, 238, 188, 240, 11, 84, 161, 30, 101, 96, 153, 113, 178, 203, 208, 86, 14, 157, 220, 15, 137, 158],
        3 => vec![55, 189, 231, 33, 202, 124, 199, 148, 50, 193, 195, 245, 26, 56, 76, 228, 53, 87, 195, 26, 6, 187, 159, 209, 143, 205, 139, 255, 138, 180, 124, 71, 99, 71, 139, 138, 31, 105, 250, 182, 103, 93, 141, 17, 190, 201, 147, 175, 230, 89, 239, 132, 214, 183, 155, 233, 240, 118, 17, 81, 63, 100, 233, 97],
        _ => panic!("Wrong long output test num")
    };
    res
}

#[test]
fn keccak224_empty_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA224, StateBitsWidth::F1600);
    keccak.append(&mut EMPTY_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        empty_output(0)
    );
}

#[test]
fn keccak224_short_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA224, StateBitsWidth::F1600);
    keccak.append(&mut SHORT_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        short_output(0)
    );
}

#[test]
fn keccak224_long_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA224, StateBitsWidth::F1600);
    keccak.append(&mut LONG_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        long_output(0)
    );
}

#[test]
fn keccak256_empty_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA256, StateBitsWidth::F1600);
    keccak.append(&mut EMPTY_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        empty_output(1)
    );
}

#[test]
fn keccak256_short_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA256, StateBitsWidth::F1600);
    keccak.append(&mut SHORT_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        short_output(1)
    );
}

#[test]
fn keccak256_long_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA256, StateBitsWidth::F1600);
    keccak.append(&mut LONG_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        long_output(1)
    );
}

#[test]
fn keccak384_empty_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA384, StateBitsWidth::F1600);
    keccak.append(&mut EMPTY_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        empty_output(2)
    );
}

#[test]
fn keccak384_short_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA384, StateBitsWidth::F1600);
    keccak.append(&mut SHORT_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        short_output(2)
    );
}

#[test]
fn keccak384_long_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA384, StateBitsWidth::F1600);
    keccak.append(&mut LONG_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        long_output(2)
    );
}

#[test]
fn keccak512_empty_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA512, StateBitsWidth::F1600);
    keccak.append(&mut EMPTY_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        empty_output(3)
    );
}

#[test]
fn keccak512_short_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA512, StateBitsWidth::F1600);
    keccak.append(&mut SHORT_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        short_output(3)
    );
}

#[test]
fn keccak512_long_input_bytes() {
    let mut keccak = Keccak::new(SecurityLevel::SHA512, StateBitsWidth::F1600);
    keccak.append(&mut LONG_INPUT_BYTES);
    assert_eq!(
        keccak.hash(),
        long_output(3)
    );
}
