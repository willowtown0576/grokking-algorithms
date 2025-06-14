
pub fn selection_sort(arr: &mut [usize]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 3, 8, 4, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 8]);
    }
}
