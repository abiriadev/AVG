use std::collections::hash_map::Entry;
use std::collections::HashMap;

mod avg;

fn main() {
    let empty_vector: Vec<i32> = vec![];

    let v1 = vec![1, 2, 3, 4];
    let fibonacci_vec = vec![1, 1, 2, 3, 5, 8];

    println!(
        "{}",
        match avg::avg(&v1) {
            Some(v) => v,
            None => 0f64,
        }
    );

    println!(
        "{}",
        match avg::avg(&empty_vector) {
            Some(v) => v,
            None => 0f64,
        }
    );

    println!(
        "{}",
        match avg::sum(&empty_vector) {
            Some(v) => v,
            None => 0,
        }
    );

    println!(
        "{}",
        match avg::sum(&empty_vector) {
            Some(v) => v,
            None => 0,
        }
    );

    println!(
        "{}",
        match avg::median(&empty_vector) {
            Some(v) => v,
            None => 0f64,
        }
    );

    println!(
        "{}",
        match avg::median(&empty_vector) {
            Some(v) => v,
            None => 0f64,
        }
    );
}
