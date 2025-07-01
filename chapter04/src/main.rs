use chapter04::{sum, count};

fn main() {

    let arr = [1, 2, 3, 4, 5];

    let sum = sum(&arr);
    let count = count(&arr);

    println!("Sum of array elements: {}", sum);
    println!("Count of array elements: {}", count);

}
