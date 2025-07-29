pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty(){
        return 0.0
    }
    let sum: i32 = list.iter().sum();
    let count = list.len() as f64;
    sum as f64/ count
}

pub fn median(list: &[i32]) -> i32 {
    if list.is_empty(){
        return 0
    }
    let mut list = list.to_vec();
    list.sort();
    let mid = list.len() / 2;
    if list.len() % 2 == 0 {
        (list[mid - 1] + list[mid]) / 2
    } else {
        list[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    if list.is_empty(){
        return 0
    }
    let mut counts = std::collections::HashMap::new();
    for &item in list {
        *counts.entry(item).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut mode = 0;
    for (&item, &count) in &counts {
        if count > max_count {
            max_count = count;
            mode = item;
        }
    }
    mode
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//      #[inline]
//     fn approx_eq(a: f64, b: f64) -> bool {
//         (a - b).abs() < f64::EPSILON
//     }

//     #[test]
//     fn test_mean() {
//         let v = [4, 7, 5, 2, 5, 1, 3];
//         assert!(approx_eq(mean(&v), 3.857142857142857));
//     }

//     #[test]
//     fn test_median() {
//         assert_eq!(median(&[4, 7, 5, 2, 5, 1, 3]), 4);
//         assert_eq!(median(&[2, 1, 5, 2, 7, 4]), 3);
//         assert_eq!(median(&[1, 7, 5, 5, 6, 4]), 5);
//     }

//     #[test]
//     fn test_mode() {
//         let v = [4, 7, 5, 2, 5, 1, 3];
//         assert_eq!(mode(&v), 5);
//     }
// }
