use crate::knot::Knot;
use itertools::Itertools;
pub fn get_colour_combinations(n: i32, p: &i32) -> Vec<Vec<i32>> {
    let combos: Vec<_> = (0..n).map(|_| (0..*p)).multi_cartesian_product().collect();
    combos
}

pub fn validate_colouring(colouring: &Vec<Vec<i32>>) -> bool {
    colouring[0] == colouring[colouring.len() - 1]
}

pub fn generate_colouring(starting_row: Vec<i32>, knot: &Knot) -> Vec<Vec<i32>> {
    let mut colouring: Vec<Vec<i32>> = Vec::new();
    let operations: Vec<(i32, i32)> = knot.braid_rep.clone();
    colouring.push(starting_row.clone());
    let mut current_row = starting_row;

    for operation in operations.iter() {
        let next_row = get_next_row(&current_row, *operation, knot.p);
        colouring.push(next_row.clone());
        current_row = next_row;
    }
    colouring
}
fn get_next_row(current_row: &Vec<i32>, operation: (i32, i32), p: i32) -> Vec<i32> {
    // operation = (top,bottom)
    let over_index = operation.0 as usize;
    let under_index = operation.1 as usize;
    let mut next_row = current_row.clone();
    next_row[under_index] = current_row[over_index];
    next_row[over_index] = modulo(2 * current_row[over_index] - current_row[under_index], p);

    next_row
}

fn modulo(mut num: i32, p: i32) -> i32 {
    if num >= 0 {
        return num % p;
    } else {
        while num < 0 {
            num += p;
        }
    }
    num
}
