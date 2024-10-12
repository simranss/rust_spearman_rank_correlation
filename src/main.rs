use std::io::{self, Write};

fn main() {
    let list_one_string = take_input("Enter the first list of numbers (separated by ','): ");
    let list_one: Vec<f32> = convert_input(list_one_string);
    let list_two_string = take_input("Enter the second list of numbers (separated by ','): ");
    let list_two: Vec<f32> = convert_input(list_two_string);

    println!("first list: {:?}", list_one);
    println!("second list: {:?}", list_two);

    if list_one.len() == list_two.len() {

        let n = list_one.len();
        let choice = take_input("What type of data is this?\n1: Directly Rank\n2: Need to convert to Rank\n");

        if choice == "1" {
            let cf: Result<f32, _> = take_input("Enter the correction factor (cf): ").parse();

            if let Ok(value) = cf {

                let rank_diff = get_rank_diff(list_one, list_two);
                let summation_d_squared = get_summation_d_squared(rank_diff);

                let coefficient = get_coefficient(n, summation_d_squared, value);
                println!("the coefficient of correlation is {}", coefficient);
            } else {
                println!("Invalid cf");
            }
        } else if choice == "2" {
            let (ranks_one, cf_one) = convert_to_rank(list_one);
            let (ranks_two, cf_two) = convert_to_rank(list_two);
            let rank_diff = get_rank_diff(ranks_one, ranks_two);
            let summation_d_squared = get_summation_d_squared(rank_diff);

            let coefficient = get_coefficient(n, summation_d_squared, cf_one + cf_two);
            println!("the coefficient of correlation is {}", coefficient);
        } else {
            println!("invalid choice");
        }
    } else {
        println!("The two lists should have equal amount of data");
    }
}

fn get_coefficient(n: usize, summation_d_squared: f32, cf: f32) -> f32 {
    let right_side = (6.0 * (summation_d_squared + cf)) / (n * (n*n - 1)) as f32;

    1.0 - right_side
}

fn get_rank_diff(list_one: Vec<f32>, list_two: Vec<f32>) -> Vec<f32> {
    let result = list_one
        .iter()
        .zip(list_two.iter())
        .map(|(a, b)| a - b)
        .collect();
    result
}

fn convert_to_rank(list: Vec<f32>) -> (Vec<f32>, f32){
    let mut indexed_list: Vec<(usize, f32)> = list
        .iter()
        .cloned()
        .enumerate()
        .collect();
    indexed_list
        .sort_by(|a, b| b.1
            .partial_cmp(&a.1)
            .unwrap()
        );
    let mut ranks = vec![0.0; list.len()];
    let mut cf = 0.0;

    let mut i = 0;
    while i < indexed_list.len() {
        let mut j = i;
        while j + 1 < indexed_list.len() && indexed_list[j].1 == indexed_list[j + 1].1 {
            j += 1;
        }

        let m = (j - i + 1) as f32;
        if m > 1.0 {
            cf += (m * (m * m - 1.0)) / 12.0;
        }
        let avg_rank = ((i + 1 + j + 1) as f32) / 2.0;
        for k in i..=j {
            ranks[indexed_list[k].0] = avg_rank;
        }
        i = j + 1;
    }
    
    (ranks, cf)
}

fn get_summation_d_squared(rank_diff: Vec<f32>) -> f32 {
    rank_diff.iter().map(|val| val * val).sum()
}

fn convert_input(input: String) -> Vec<f32> {
    let numbers: Vec<f32> = input
        .split(',')
        .map(|number| number.trim().parse::<f32>())
        .filter_map(|result| result.ok())
        .collect();
    
    numbers
}

fn take_input(hint: &str) -> String {
    print!("{hint}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) 
        .expect("Failed to read line");

    let input = input.trim();

    input.to_string()
}
