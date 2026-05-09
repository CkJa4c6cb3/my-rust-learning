use std::{collections::HashMap, io};

fn main() {
    let numbers = vec![5, 2, 2, 3, 4, 1];
    median_and_mode(&numbers);
}

/*
整数のリストが与えられ、ベクタを使ってmedian(ソートされた時に真ん中に来る値)、
mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。
*/
fn median_and_mode(values: &[i32]) {
    let mut sorted = values.to_vec();
    sorted.sort();

    let median = sorted[sorted.len() / 2];

    println!("The median is {}", median);

    let mut counts = HashMap::new();

    for &num in values {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut max_value_key = 0;
    let mut max_value = 0;

    for (&key, &count) in &counts {
        if count > max_value {
            max_value = count;
            max_value_key = key;
        }
    }

    println!("最頻値は {}, {}回登場!", max_value_key, max_value);
}
