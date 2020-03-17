use tiger_hash::{tiger_str, TigerState};

#[test]
fn two_blocks2() {
    assert_eq!(
        [0x3D9AEB03D1BD1A63, 0x57B2774DFD6D5B24, 0xDD68151D503974FC],
        tiger_str("Tiger - A Fast New Hash Function, by Ross Anderson and Eli Biham, proceedings of Fast Software Encryption 3, Cambridge, 1996.")
    );
}

#[test]
fn two_blocks() {
    assert_eq!(
        [0xEBF591D5AFA655CE, 0x7F22894FF87F54AC, 0x89C811B6B0DA3193],
        tiger_str("Tiger - A Fast New Hash Function, by Ross Anderson and Eli Biham, proceedings of Fast Software Encryption 3, Cambridge.")
    );
}

#[test]
fn str_1024() {
    assert_eq!(
        [0xB83EB4E53440C5, 0x76AC6AAEE0A74858, 0x25FD15E70A59FFE4],
        tiger_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-")
    );
}

#[test]
fn abc() {
    assert_eq!(
        [0xF258C1E88414AB2A, 0x527AB541FFC5B8BF, 0x935F7B951C132951],
        tiger_str("abc")
    );
}

#[test]
fn empty() {
    assert_eq!(
        [0x24F0130C63AC9332, 0x16166E76B1BB925F, 0xF373DE2D49584E7A],
        tiger_str("")
    );
}

#[test]
fn str_tiger() {
    assert_eq!(
        [0x9F00F599072300DD, 0x276ABB38C8EB6DEC, 0x37790C116F9D2BDF],
        tiger_str("Tiger")
    );
}

#[test]
fn one_block() {
    assert_eq!(
        [0x87FB2A9083851CF7, 0x470D2CF810E6DF9E, 0xB586445034A5A386],
        tiger_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-")
    );
}

#[test]
fn one_block2() {
    assert_eq!(
        [0x467DB80863EBCE48, 0x8DF1CD1261655DE9, 0x57896565975F9197],
        tiger_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ=abcdefghijklmnopqrstuvwxyz+0123456789")
    );
}

#[test]
fn one_block_tiger() {
    assert_eq!(
        [0xC410A042968868A, 0x1671DA5A3FD29A72, 0x5EC1E457D3CDB303],
        tiger_str("Tiger - A Fast New Hash Function, by Ross Anderson and Eli Biham")
    );
}

#[test]
fn str_64k() {
    let mut data: Vec<u8> = Vec::new();
    for i in 0..65536 {
        data.push((i & 0xFF) as u8)
    }
    let mut state = TigerState::new();
    state.update(data.as_ref());
    let res = state.finalize();
    assert_eq!(
        [0x8EF43951B3F5F4FD, 0x1D41AFE51B420E71, 0x0462F233C3AAA8E1],
        res
    );
}
