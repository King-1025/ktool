use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use des::TdesEde3;
use base64;

use std::time::SystemTime;

pub fn sign(vstr : &str) -> String {
        let mut ss = String::from("");
        let schar = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        for c in vstr.chars() {
           let b = c as u8;
           let i = ((b >> 4) & 15) as usize;
           ss += &(schar[i].to_string());
           let i = (b & 15) as usize;
           ss += &(schar[i].to_string());
        }
        return ss;
}

pub fn stime() -> (String, String) {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
       Ok(n) => {
                    let mir = n.as_millis();
                    return (mir.to_string(), (mir+100000).to_string());
                },
       Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn x3des(key: &str, iv: &str, plaintext: &str) -> String {
    type ALG = Cbc<TdesEde3, Pkcs7>;
    let cipher = ALG::new_var(key.as_bytes(), iv.as_bytes()).unwrap();
    let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());
    return base64::encode_config(&ciphertext, base64::STANDARD);
}
