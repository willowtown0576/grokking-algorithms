use chapter02::selection_sort;

fn main() {
    let mut arr = [5, 3, 8, 4, 2];
    selection_sort(&mut arr);
    println!("{:?}", arr)
}
