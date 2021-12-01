use std::fs;

fn get_input(path: &str) -> Vec<u32> {
    let data = fs::read_to_string(path).unwrap();

    data.split('\n')
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

pub fn get_result_1() -> u32 {
    let input = get_input("./day_1/input-1.txt");

    calc_increased_number(input)
}

pub fn get_result_2() -> u32 {
    let input = get_input("./day_1/input-2.txt");

    calc_increased_number(sum_sliding_window(input, 3))
}

fn sum_sliding_window(vec: Vec<u32>, window_len: usize) -> Vec<u32> {
    let mut result = Vec::new();

    let mut i = 0;

    while i < vec.len() - window_len + 1 {
        let mut cur_sum = 0;
        let mut j = i;

        while j < i + window_len {
            cur_sum += vec[j];
            j += 1;
        }

        i += 1;
        result.push(cur_sum);
    }

    result
}

fn calc_increased_number(input: Vec<u32>) -> u32 {
    let mut res = 0u32;
    let mut prev = input[0];
    for item in input {
        if item > prev {
            res += 1;
        }

        prev = item
    }
    res
}
