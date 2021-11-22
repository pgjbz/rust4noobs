# Strings

Lembra dos [Arrays](../basic/09-arrays.md)? String são arrays, porém de um tipo específico `char`, existem varias formas de representar strings em Rust, nesta parte do 4Noobs, iremos utilizar um modo específico com o tipo [String](https://doc.rust-lang.org/stable/std/string/struct.String.html).

Para declararmos uma String utilizamos o seguinte padrão `let nome: String = String::from(texto entre aspas)`.

```rust
fn main() { 
    let texto = String::from("Hello World!"); 
} 
```

Acima temos a declaração da nossa String, nela temos alguns métodos, mas nesta parte iremos falar sobre os seguintes métodos len, push, push_str, trim.

Sendo os métodos `push` e `push_str` específicos para Strings mutáveis.

## String len

O método `len` nos retorna o tamanho da String, seu modo de uso é bem simples, apenas colocamos um `.len()`.

```rust
fn main() {
    let texto = String::from("Rust4Noobs");
    let tamanho_texto = texto.len();
    println!("O tamanho do texto é: {}", tamanho_texto);
}
```

## String push e push_str

Com o método `push`, conseguimos adicionar um caractere a nossa `String` e com o `push_str` conseguimos adicionar outra String.

```rust
fn main() {
    let mut texto = String::from("Rust");
    texto.push('4');
    texto.push_str("Noobs");
    println!("{}", texto);
}
```


## String trim

O método `trim` remove os espaços do início e no fim de uma String, porém não modifica o texto original, ele nos retorna outra String sem estes espaços. O método é bem útil para quando formos realizar algum tipo conversão, mas iremos falar sobre isso posteriormente, por enquanto iremos focar apenas neste método.

```rust
fn main() {
    let texto = String::from("   Rust4Noobs   ");
    println!("Sem trim: {}
Com o uso de trim: {}", texto, texto.trim());
    println!("A string original se mantém: {}", texto);
}
```

Não cobrimos alguns pontos importantes sobre este tipo de dado, por conta de alguns conceitos ainda não explicado, futuramente iremos retomar com as `Strings`. 

- [Próximo](./02-pointers-intro.md) - Introdução a ponteiros