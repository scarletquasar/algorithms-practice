use std::env;
use rand::seq::SliceRandom;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let numbers: Vec<i32> = args
        .into_iter()
        .filter(|x| is_string_numeric(x))
        .map(|x| x.parse().unwrap())
        .collect();

    let values: Vec<String> = quicksort(numbers).iter().map(|x| x.to_string()).collect();

    println!("{}", values.join(","));
}

fn is_string_numeric(str: &String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn quicksort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        return arr;
    }

    let pivot_index = &mut rand::thread_rng();
    let pivot = *arr.choose(pivot_index).unwrap();

    let mut left_elements: Vec<i32> = vec![];
    let mut right_elements: Vec<i32> = vec![];
    let mut middle_elements: Vec<i32> = vec![];

    for element in arr {
        if element < pivot {
            left_elements.push(element);
        }

        if element > pivot {
            right_elements.push(element);
        }

        if element == pivot {
            middle_elements.push(element);
        }
    }

    return [quicksort(left_elements), middle_elements, quicksort(right_elements)].concat();
    
}