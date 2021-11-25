fn main() {
    for i in 0..100 {
        if i % 4 == 0 && i % 6 == 0 {
            continue;
        }
        println!("Numero atual {}", i);
    }
}