fn main() {
    let mut buffer = String::new();
    let mut texto = String::from("Rust4Noobs");
    println!("Entre com uma frase: ");
    std::io::stdin().read_line(&mut buffer).unwrap();
    adiciona_texto(&mut texto, buffer);
    println!("Texto final: {}", texto);
}

fn adiciona_texto(texto_in: &mut String, texto_add: String) {
    texto_in.push_str(", ");
    texto_in.push_str(&texto_add.trim());
}
