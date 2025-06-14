/// 選択ソートアルゴリズムを使用して配列をインプレースでソートします。
///
/// # アルゴリズム
///
/// 選択ソートは入力配列を2つの部分に分割します：
/// - 左から右へ構築されるソート済みの部分配列
/// - 配列の残りを占める未ソートの部分配列
///
/// 各反復では、アルゴリズムは未ソート部分配列から最小要素を見つけ、
/// 未ソート部分の最も左の要素と交換します。これにより、2つの部分配列の境界が
/// 右に1つ移動します。
///
/// # 引数
///
/// * `arr` - ソートされるusize要素の可変スライス
///
/// # 時間計算量
///
/// - 最悪の場合: O(n²)
/// - 平均的な場合: O(n²)
/// - 最良の場合: O(n²)
///
/// # 空間計算量
///
/// O(1) - このアルゴリズムはインプレースでソートし、一定量の追加空間しか使用しません。
///
/// # 使用例
///
/// ```
/// let mut arr = [5, 3, 8, 4, 2];
/// chapter02::selection_sort(&mut arr);
/// assert_eq!(arr, [2, 3, 4, 5, 8]);
/// ```
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
