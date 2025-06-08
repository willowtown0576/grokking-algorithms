use chapter01::binary_search;

fn main() {
    let arr = [1, 3, 5, 7, 9];
    let target = 5;
    let result = binary_search(&arr, target);
    println!("Result: {:?}", result);
}
