use std::cmp::Ordering;
use std::collections::HashMap;

pub fn avg(input: &Vec<i32>) -> Option<f64> {
    let sum = sum(&input);

    match sum {
        None => None,
        Some(sum_v) => Some((sum_v as f64) / (input.len() as f64)),
    }
}

pub fn sum(input: &Vec<i32>) -> Option<i32> {
    if input.len() == 0 {
        return None;
    }

    let mut sum: i32 = 0;

    for number in input {
        sum += number;
    }

    Some(sum)
}

pub fn sort(input: &mut Vec<i32>) {
    for c in (1..(input.len())).rev() {
        for index in 0..c {
            let left = match input.get(index) {
                Some(v) => v,
                None => break,
            };
            let right = match input.get(index + 1) {
                Some(v) => v,
                None => break,
            };

            // println!("{} <> {}", left, right);

            if let Ordering::Greater = left.cmp(&right) {
                let r = input.remove(index + 1);

                input.insert(index, r);
            }
        }

        // println!("============");
    }
}

pub fn median(input: &Vec<i32>) -> Option<f64> {
    let mut cl = input.clone();

    sort(&mut cl);

    match cl.len() % 2 {
        0 => {
            let tem_v = vec![
                match cl.get(cl.len() - 1) {
                    None => return None,
                    Some(&v) => v,
                },
                match cl.get(cl.len()) {
                    None => return None,
                    Some(&v) => v,
                },
            ];

            avg(&tem_v)
        }
        1 => match cl.get(cl.len() / 2) {
            None => None,
            Some(&v) => Some(v as f64),
        },
        _ => None,
    }
}

pub fn mode(input: &Vec<i32>) -> Option<i32> {
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for &num in input {
        let num_ref = count_map.entry(num).or_insert(0);

        *num_ref += 1
    }

    get_max_from_map(&count_map)
}

fn get_max_from_map(map: &HashMap<i32, i32>) -> Option<i32> {
    let mut latest_max: Option<i32> = None;

    for &value in map.values() {
        match latest_max {
            None => latest_max = Some(value),
            Some(latest_max_v) => {
                if let Ordering::Greater = value.cmp(&latest_max_v) {
                    latest_max = Some(value)
                }
            }
        }
    }

    latest_max
}
