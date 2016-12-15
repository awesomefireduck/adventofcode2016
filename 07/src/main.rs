use std::vec::Vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
fn main() {
    let input = File::open("input").expect("oeuaeouoeauoeuaeo");
    let mut lined_input = BufReader::new(input).lines().peekable();
    let mut ipv7_addresses_with_tls_support : Vec<String>= vec![];

    while let Some(line) = lined_input.next() {
        let ipv7_address = line.expect("IPv7 line");
        if ipv7_has_tls_support(&ipv7_address) {
            ipv7_addresses_with_tls_support.push(ipv7_address);;;
        }
    }

    let count = ipv7_addresses_with_tls_support.len();
    println!("tls support: {}", count);

}


fn ipv7_has_tls_support(ipv7_address :&String) -> bool {

    // every other part is hypernet_part, first is regular
    let mut address_parts = ipv7_address.split(|c| c == '[' || c == ']');
    let mut tls_support = false;

    while let Some(part) = address_parts.next() {
        if part_has_abba(&part) {
            tls_support = true;
        }

        if let Some(hypernet_part) = address_parts.next() {
            if part_has_abba(&hypernet_part) {
                tls_support = false;
                break;
            }
        }
    }
    tls_support
}

fn part_has_abba(address_part: &str) -> bool {
    let mut has_abba = false;
    if address_part.len() < 4 {
        return false
    }
    let last_slice_start : usize = address_part.len() - 3;
    for i in 0..last_slice_start {
        let j = i + 4;
        println!("{}", &address_part[i..j]);
        if slice_has_abba(&address_part[i..j]) {
            has_abba = true;
        }
    }
    has_abba
}


fn slice_has_abba(part_slice: &str) -> bool {
    assert!(part_slice.len() == 4);

    // [a, b, b, a]
    let chars : Vec<char> = part_slice.chars().collect();

    // a__a
    chars[0] == chars[3] &&
    // _bb_
    chars[1] == chars[2] &&
    // a != b
    chars[0] != chars[1]
}

#[cfg(test)]
mod tests {
    use super::{slice_has_abba, part_has_abba, ipv7_has_tls_support};

    #[test]
    fn slice_has_abba_test() {
        let slices = [
            ("abba",    true),
            ("u  u",    true),
            (" oo ",    true),
            ("aaaa",    false),
            ("    ",    false),
        ];

        for slice in slices.iter() {
            println!("test: '{}' => {}", slice.0, slice.1);
            assert_eq!(slice_has_abba(slice.0), slice.1);
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed: part_slice.len() == 4")]
    fn slice_has_abba_too_short_test() {
        let slice = "abb";
        println!("test: '{}' => panic", slice);
        slice_has_abba(slice);
    }

    #[test]
    #[should_panic(expected = "assertion failed: part_slice.len() == 4")]
    fn slice_has_abba_too_long_test() {
        let slice = "abbaa";
        println!("test: '{}' => panic", slice);
        slice_has_abba(slice);
    }

    #[test]
    fn part_has_abba_test() {
        let parts = [
            ("abba",    true),
            ("aabbaa",  true),
            ("orrassa", true),
            ("u  u",    true),
            (" oo ",    true),
            ("aaaa",    false),
            ("a",       false),
            ("aaa",     false),
            ("    ",    false),
        ];
        for part in parts.iter() {
            println!("test: '{}' => {}", part.0, part.1);
            assert_eq!(part_has_abba(part.0), part.1);
        }
    }
    #[test]
    fn ipv7_has_tls_support_test() {
        let parts = [
            ("aaaa[aaaa]aabba", true),
            ("", false),
            ("abba[mnop]qrst", true),
            ("abcd[bddb]xyyx", false),
            ("aaaa[qwer]tyui", false),
            ("ioxxoj[asdfgh]zxcvbn", true),
        ];
        for part in parts.iter() {
            assert_eq!(ipv7_has_tls_support(&part.0.to_string()), part.1);
        }
    }
}

