# Condições

Em Rust como em todas as linguagens de programação, ou pelo menos a maioria, temos estruturas de decisões, são bem parecidas, com base em uma condição tomamos uma decisão.

Para utilizar estruturas condicionais em Rust devemos utilizar a palavra reservada `if`.

```rust
fn main() {
    let a = 20;
    let b = 10;
    
    if a > b {
        println!("'a' é maior que 'b'");
    }
}
```

E se quisermos executar algo caso a condição não seja verdadeira?? Utilizamos a palavra `else`

```rust
fn main() {
    let a = 10;
    let b = 20;
    
    if a > b {
        println!("'a' é maior que 'b'");
    } else {
        println!("'b' é maior que 'a'");
    }
}
```

Simples né? E se quisermos realizar outra checagem caso a primeira condição não de verdadeira? Simples combinamos o `else` e o `if`

```rust
fn main() {
    let a = 10;
    let b = 20;
    let c = 15;
    
    if a > b {
        println!("'a' é maior que 'b'");
    } else if b > c {
        println!("'b' é maior que 'a'");
    } else { //caso não aconteça nenhum dos casos cai aqui
        println!("'c' é maior que 'b'");
    }
}
```