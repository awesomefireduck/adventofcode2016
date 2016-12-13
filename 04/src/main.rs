#![feature(field_init_shorthand)]
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::vec::Vec;

fn main() {
    let input = File::open("input").expect("input does not exist");
    let rooms = parse_roomlist(input);
    let valid_rooms = filter_invalid_rooms(&rooms);
    let sector_id_sum : u64 = get_sector_ids(&valid_rooms).iter().sum();
    println!("{:?}", sector_id_sum);
    let room_list = decipher_room_names(&valid_rooms);
    println!("{:?}", room_list);
}

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: u64,
    checksum: String,
}

fn decipher_room_names(room_list: &Vec<&Room>) -> Vec<String> {

    let mut rooms = room_list.iter();
    let mut room_names : Vec<String> = vec![];
    while let Some(room) = rooms.next() {
        let offset : u8 = (room.sector_id % 26) as u8;
        room_names.push(decipher_room_name(&room.name, offset) + room.sector_id.to_string().as_str());
    }

    room_names
}


fn decipher_room_name(ciphertext: &String, offset: u8) -> String {
    ciphertext.chars().map(|c| {
        if c == '-' {
            return ' '
        }
        let charbyte : u8= c as u8;
        // a = 0, z=25
        let a_z_value = charbyte - ('a' as u8);
        let new_a_z_value = (a_z_value + offset) % 26;
        (new_a_z_value + 'a' as u8) as char
    }).collect()
}


impl std::str::FromStr for Room {
    type Err = &'static str;

    fn from_str(from_string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {

        fn split_sector_and_checksum(sector_id_and_checksum: String) -> (u64, String) {
            let sector_id: u64;
            let checksum: String;

            // "753[thaoeud]" -> "753[thaoeud" -> ["753", "thaoeud"]
            let mut parts = sector_id_and_checksum.trim_right_matches(']').split('[');

            // Some("753") -> "753" -> Ok(753) -> 753
            sector_id = parts.next().expect("sector_id_string").parse().expect("sector_id");

            // Some("thaoeud") -> "thaoeud" -> String("thaoeud")
            checksum = parts.next().expect("checksum").to_owned();
            (sector_id, checksum)
        }

        let name : String;
        let sector_id : u64;
        let checksum: String;

        let index = from_string.rfind("-").expect("index");
        let (name_str, sector_id_and_checksum) = from_string.split_at(index + 1);
        name = name_str.to_owned();
        let sector_id_and_checksum = split_sector_and_checksum(sector_id_and_checksum.to_owned());
        sector_id = sector_id_and_checksum.0;
        checksum  = sector_id_and_checksum.1;

        Ok(Room {
            name,
            sector_id,
            checksum,
        })
    }
}

fn parse_roomlist(input: File) -> Vec<Room> {
    let mut rooms : Vec<Room>= vec![];
    let mut lined_input = BufReader::new(input).lines();
    while let Some(line) = lined_input.next() {
        let room_line = line.expect("room on single line");
        rooms.push(room_line.parse::<Room>().expect("oaeuoea"));
    }
    rooms
}

fn filter_invalid_rooms(room_list: &Vec<Room>) -> Vec<&Room> {
    let rooms = room_list.iter();
    let valid_rooms = rooms.filter(verify_checksum);
    valid_rooms.collect()
}

fn verify_checksum(room: &&Room) -> bool {
    let ref name = room.name;
    let ref checksum = room.checksum;

    *checksum == calculate_checksum(&name)
}

fn calculate_checksum(name: &String) -> String {
    let mut name_parts = name.chars();
    let mut char_counts = vec![];

    while let Some(c) = name_parts.next() {
        if c != '-' {
            char_counts.push((c, name.matches(c).count()));
        }
    }
    char_counts.sort_by(|a, b| a.1.cmp(&(b.1)) );
    char_counts.dedup();
    let mut chars = char_counts.iter().rev().peekable();

    let mut checksum : String = "".to_owned();

    'a: while let Some(&(letter, count)) = chars.next() {
        if checksum.len() >= 6 {
            break;}

        let current_count = count;
        let mut tied_count : Vec<char> = vec![letter];
        'b: while let Some(&&(letter, count)) = chars.peek() {
            if count != current_count {
                break 'b;
            }
            // we throw these away since we already `peek`ed the values
            chars.next();

            tied_count.push(letter);
        }
        tied_count.sort();
        tied_count.dedup();
        let added_part = tied_count.iter().map(|a| *a).collect::<String>();
        checksum.push_str(added_part.as_str());
        checksum.push_str("");
    }
    // panic!("oauea");
    checksum[..5].to_owned()
}


fn get_sector_ids(rooms: &Vec<&Room>) -> Vec<u64> {
    rooms.iter().map(|room| room.sector_id ).collect()
}
