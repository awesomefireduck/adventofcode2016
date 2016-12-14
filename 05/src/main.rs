extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::HashMap;

fn main() {
    let start_string = "reyedfim";
    // let door_code = calculate_door_code(start_string);
    let door_code = calculate_second_door_code(start_string, 8);
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

fn calculate_second_door_code(prefix: &str, codelength: usize) -> String {
    let mut code_parts : HashMap<usize, char> = HashMap::new();
    let mut index = 0;

    while index <= std::u64::MAX   {
        if code_parts.len() >= codelength {
            break;
        }
        let hash = get_hash(prefix, index);
        get_next_char_by_position(hash.as_str(), &mut code_parts, codelength);
        index = index + 1;
    }

    let mut code = String::new();
    for i in 0..codelength {
        code.push(*code_parts.get(&i).expect("code char"));
    }
    code
}

fn get_next_char_by_position(hash : &str, mut code_parts: &mut HashMap<usize, char>, codelength: usize) {
    if hash.starts_with("00000") {
       get_char_by_position(&hash[5..7], &mut code_parts, codelength);
    }
}

fn get_char_by_position(hash_part :&str, mut code_parts: &mut HashMap<usize, char>, codelength: usize) {
    let (position_str, character_str) = hash_part.split_at(1);

    if let Ok(position) = position_str.parse::<usize>() {
        if position >= codelength {
            return
        }
        println!("{}, {}", position_str, character_str);
        if let None = code_parts.get(&position) {

            let character = character_str.chars().next().expect("char");
            code_parts.insert(position, character);
            println!("{}", code_parts.len());

        }
    }
}
