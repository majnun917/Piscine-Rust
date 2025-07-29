pub fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    while n > 0 {
        let mut need_swap = false;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                need_swap = true;
            }
        }
        n -= 1;
        if !need_swap {
            break;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//      #[test]
//     fn test_ordering() {
//         let mut v = [3, 2, 4, 5, 1, 7, 9, 8];
//         let mut v_clone = v;

//         v_clone.sort_unstable();
//         bubble_sort(&mut v);

//         assert_eq!(v, v_clone);
//     }
// }
