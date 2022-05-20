fn main() {
    let mut buffer = String::new();
    println!("Entre com um número: ");
    std::io::stdin().read_line(&mut buffer).unwrap();
    let numero: u8 = buffer.trim().parse().unwrap();
    if numero >= 0 && numero <= 50 {
        println!("O número é de primeiro grau");
    } else if numero > 50 && numero <= 120 {
        println!("O número é de segundo grau");
    } else if numero > 120 && numero <= 200 {
        println!("O número é de terceiro grau");
    } else {
        println!("O número é de quarto grau")
    }
}
