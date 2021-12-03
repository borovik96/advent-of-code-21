use std::{collections::HashMap, fs};

fn get_input(path: &str) -> Vec<String> {
    let data = fs::read_to_string(path).unwrap();

    data.split('\n').map(|n| n.to_string()).collect()
}

fn calc_most_common_bit_at_position(data: &Vec<String>, pos: usize, equal_case: char) -> char {
    let mut counter = 0;
    for bits in data {
        let bit = bits.chars().nth(pos).unwrap();
        if bit == '1' {
            counter += 1;
        }
    }

    if data.len() - counter == counter {
        return equal_case;
    }

    if counter > data.len() / 2 {
        '1'
    } else {
        '0'
    }
}

fn invert_bit(ch: char) -> char {
    if ch == '1' {
        '0'
    } else {
        '1'
    }
}

fn get_gama_rate(data: Vec<String>) -> String {
    let mut map = HashMap::new();

    for bits in &data {
        for (index, bit) in bits.chars().enumerate() {
            if bit == '1' {
                let counter = map.entry(index).or_insert(0);

                *counter += 1;
            }
        }
    }

    let mut res = String::new();

    let mut i = 0;

    while i < map.len() {
        let bit = if *map.get(&i).unwrap() >= data.len() / 2 {
            '1'
        } else {
            '0'
        };

        res.push(bit);

        i += 1;
    }

    res
}

fn invert_str(str: &String) -> String {
    let mut res = String::new();

    for chr in str.chars() {
        res.push(invert_bit(chr));
    }

    res
}

fn str_to_decimal(str: String) -> i32 {
    let mut res = 0;

    for (i, bit) in str.chars().rev().enumerate() {
        res += 2i32.pow(i.try_into().unwrap()) * if bit == '1' { 1 } else { 0 }
    }

    res
}

pub fn get_result_1() -> i32 {
    let input = get_input("day_3/input-1.txt");
    let gamma = get_gama_rate(input);
    let epsilon = invert_str(&gamma);

    str_to_decimal(gamma) * str_to_decimal(epsilon)
}

fn get_oxygen_gen_rate(data: &Vec<String>) -> String {
    let mut data = data.clone();

    let mut i = 0;

    while data.len() > 1 {
        let good_bit = calc_most_common_bit_at_position(&data, i, '1');
        data = Vec::from_iter(
            data.iter()
                .filter_map(|s| {
                    if s.chars().nth(i).unwrap() == good_bit {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .clone(),
        );

        i += 1;
    }

    data.pop().unwrap()
}

fn get_co2_rate(data: &Vec<String>) -> String {
    let mut data = data.clone();

    let mut i = 0;

    while data.len() > 1 {
        let good_bit = invert_bit(calc_most_common_bit_at_position(&data, i, '1'));

        data = Vec::from_iter(
            data.iter()
                .filter_map(|s| {
                    if s.chars().nth(i).unwrap() == good_bit {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .clone(),
        );

        i += 1;
    }

    data.pop().unwrap()
}

pub fn get_result_2() -> i32 {
    let input = get_input("day_3/input-2.txt");
    let oxygen = get_oxygen_gen_rate(&input);
    let co2 = get_co2_rate(&input);

    str_to_decimal(oxygen) * str_to_decimal(co2)
}
