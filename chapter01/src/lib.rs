
pub fn binary_search(arr: &[usize], target: usize) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = arr[mid];

        if guess == target {
            return Some(mid);
        } else if guess > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 3, 5, 7, 9];
        let target_1 = 5;
        let target_2 = 10;
        let result_1 = binary_search(&arr, target_1);
        let result_2 = binary_search(&arr, target_2);

        assert_eq!(result_1, Some(2));
        assert_eq!(result_2, None);
    }
}
