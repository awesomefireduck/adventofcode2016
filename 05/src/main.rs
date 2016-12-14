extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let start_string = "reyedfim";
    let door_code = calculate_door_code(start_string);
    println!("{}", door_code);
}


fn get_next_char(hash : &str) -> Option<&str> {
    let next_char = match hash.starts_with("00000") {
        // sixth character
        true => Some(&hash[5..6]),
        false => None,
    };
    next_char
}


fn get_hash(prefix: &str, index: u64) -> String {
    let mut md5device = Md5::new();
    let indexstring = index.to_string();

    md5device.input_str(prefix);
    md5device.input_str(indexstring.as_str());

    md5device.result_str()
}

fn calculate_door_code(prefix: &str) -> String {
    let mut code : String = String::from("");
    let mut index : u64 = 0;

    while index <= std::u64::MAX && code.len() <= 8  {
        let hash = get_hash(prefix, index);
        if let Some(next_char) = get_next_char(hash.as_str()) {
            code += next_char;
        }
        index = index + 1;
    }
    code
}
