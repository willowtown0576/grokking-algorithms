
pub fn sum(arr: &[usize]) -> usize {
    if arr.len() == 1 {
        return arr[0];
    } else {
        return arr[0] + sum(&arr[1..]);
    }
}

pub fn count(arr: &[usize]) -> usize {
    if arr.is_empty() {
        0
    } else {
        1 + count(&arr[1..])
    }
}

pub fn max(arr: &[usize]) -> usize {
    if arr.len() == 1 {
        arr[0]
    } else {
        let max_rest = max(&arr[1..]);
        if arr[0] > max_rest {
            arr[0]
        } else {
            max_rest
        }
    }
}

pub fn binary_search(arr: &[usize], target: usize) -> Option<usize> {
    if arr.is_empty() {
        None
    } else {
        let mid = arr.len() / 2;
        if arr[mid] == target {
            Some(mid)
        } else if arr[mid] < target {
            binary_search(&arr[mid + 1..], target).map(|i| i + mid + 1)
        } else {
            binary_search(&arr[..mid], target)
        }
    }
}

pub fn quick_sort(arr: &[usize]) -> Vec<usize> {
    if arr.len() < 2 {
        arr.to_vec()
    } else {
        let pivot = arr[0];
        let (less, greater): (Vec<usize>, Vec<usize>) = arr[1..]
            .iter()
            .partition(|&&x| x < pivot);
        let mut sorted = quick_sort(&less);
        sorted.push(pivot);
        sorted.extend(quick_sort(&greater));
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let arr = [1, 2, 3, 4, 5];
        let sum = sum(&arr);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_count() {
        let arr = [1, 2, 3, 4, 5];
        let count = count(&arr);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_max() {
        let arr = [1, 2, 3, 4, 5];
        let max = max(&arr);
        assert_eq!(max, 5);
    }

    #[test]
    fn test_binary_search() {
        let arr = [1, 2, 3, 4, 5];
        let index = binary_search(&arr, 3);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn test_quick_sort() {
        let arr = [5, 3, 8, 4, 2];
        let sorted = quick_sort(&arr);
        assert_eq!(sorted, vec![2, 3, 4, 5, 8]);
    }
}
