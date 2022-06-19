fn main() {
    let arr: [i32; 7] = [2, 4, 5, 6, 9, 0, 10];
    let sum = sum_array(arr);
    println!("Sum: {}", sum);
}

fn sum_array(arr: [i32; 7]) -> i32 {
    let mut sum = 0;
    for item in arr.iter() {
        sum += item;
    }
    sum
}