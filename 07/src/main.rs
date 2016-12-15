use std::vec::Vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let input = File::open("input").expect("oeuaeouoeauoeuaeo");
    let mut lined_input = BufReader::new(input).lines().peekable();
    let mut ipv7_addresses_with_tls_support : Vec<String>= vec![];
    let mut ipv7_addresses_with_ssl_support : Vec<String>= vec![];

    while let Some(line) = lined_input.next() {
        let ipv7_address = line.expect("IPv7 line");
        if ipv7_has_tls_support(&ipv7_address) {
            ipv7_addresses_with_tls_support.push(ipv7_address.clone());
        }
        if ipv7_has_ssl_support(&ipv7_address) {
            ipv7_addresses_with_ssl_support.push(ipv7_address);
        }
    }

    let count = ipv7_addresses_with_tls_support.len();
    println!("tls support: {}", count);

    let count = ipv7_addresses_with_ssl_support.len();
    println!("ssl support: {}", count);
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


fn ipv7_has_ssl_support(ipv7_address: &String) -> bool {
    // every other part is hypernet_part, first is supernet
    let mut address_parts = ipv7_address.split(|c| c == '[' || c == ']');

    let mut supernet_strings : Vec<&str> = vec![];
    let mut hypernet_strings : Vec<&str> = vec![];

    while let Some(part) = address_parts.next() {
        supernet_strings.push(part);

        if let Some(hypernet_part) = address_parts.next() {
            hypernet_strings.push(hypernet_part);
        }
    }

    // HashSet<a, b>, skip the second a
    let mut aba_parts : HashSet<(char, char)> = HashSet::new();
    // HashSet<a, b> instead of <b, a> for easy comparison later
    let mut bab_parts : HashSet<(char, char)> = HashSet::new();

    for supernet_str in supernet_strings.iter() {
        aba_parts.extend(get_aba_parts(&supernet_str));
    }

    for hypernet_str in hypernet_strings.iter() {
        bab_parts.extend(get_bab_parts(&hypernet_str));
    }

    aba_parts.intersection(&bab_parts).count() > 0
}


fn get_aba_parts(supernet_part: &str) -> HashSet<(char, char)>{
    let mut aba_parts : HashSet<(char, char)> = HashSet::new();
    if supernet_part.len() < 3 {
        return aba_parts
    }
    let last_slice_start = supernet_part.len() - 2;
    for i in 0..last_slice_start {
        let j = i + 3;
        if slice_is_aba(&supernet_part[i..j]) {
            let chars : Vec<char> = supernet_part[i..j].chars().collect();
            aba_parts.insert((chars[0], chars[1]));
        }
    }
    aba_parts
}


fn get_bab_parts(hypernet_part: &str) -> HashSet<(char, char)> {
    get_aba_parts(hypernet_part).iter().map(|&(b, a)| (a, b) ).collect()
}

fn slice_is_aba(slice: &str) -> bool {
    let chars : Vec<char> = slice.chars().collect();

    chars[0] == chars[2] && chars[1] != chars[0]
}



#[cfg(test)]
mod tests {

    #[test]
    fn slice_has_abba() {
        let slices = [
            ("abba",    true),
            ("u  u",    true),
            (" oo ",    true),
            ("aaaa",    false),
            ("    ",    false),
        ];

        for slice in slices.iter() {
            println!("test: '{}' => {}", slice.0, slice.1);
            assert_eq!(super::slice_has_abba(slice.0), slice.1);
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed: part_slice.len() == 4")]
    fn slice_has_abba_too_short() {
        let slice = "abb";
        println!("test: '{}' => panic", slice);
        super::slice_has_abba(slice);
    }

    #[test]
    #[should_panic(expected = "assertion failed: part_slice.len() == 4")]
    fn slice_has_abba_too_long() {
        let slice = "abbaa";
        println!("test: '{}' => panic", slice);
        super::slice_has_abba(slice);
    }

    #[test]
    fn part_has_abba() {
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
            assert_eq!(super::part_has_abba(part.0), part.1);
        }
    }
    #[test]
    fn ipv7_has_tls_support() {
        let parts = [
            ("aaaa[aaaa]aabba", true),
            ("", false),
            ("abba[mnop]qrst", true),
            ("abcd[bddb]xyyx", false),
            ("aaaa[qwer]tyui", false),
            ("ioxxoj[asdfgh]zxcvbn", true),
        ];
        for part in parts.iter() {
            assert_eq!(super::ipv7_has_tls_support(&part.0.to_string()), part.1);
        }
    }
    #[test]
    fn ipv7_has_ssl_support() {
        let parts = [
            ("aba[bab]xyz", true),
            ("xyx[xyx]xyx", false),
            ("aaa[kek]eke", true),
            ("zazbz[bzb]cdb", true),
        ];
        for part in parts.iter() {
            println!("expected: {} => {}", part.0, part.1);
            assert_eq!(super::ipv7_has_ssl_support(&part.0.to_string()), part.1);
        }
    }

    #[test]
    fn get_aba_parts() {
        use std::collections::HashSet;

        let parts = [
            ("ioixoj[asdfgh]zxcvbn", ('i', 'o'))
        ];

        let mut aba = HashSet::new();
        for part in parts.iter() {
            aba.insert(part.1);
            assert_eq!(super::get_aba_parts(&part.0), aba);
            aba.clear();
        }
    }

}

