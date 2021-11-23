# Tuplas

Aprendemos sobre [arrays](../basic/09-arrays.md) no módulo básico, agora iremos falar sobre as tuplas.

Tuplas é uma "coleção" de dados de tipos diferentes, em uma tupla podemos ter um `u8`, uma `String`, um `char`... Todos juntos, sua declaração é feita da seguinte maneira `let tuple: (tipo1, tipo2, tipo3...) = (valor1, valor2, valor3...)`.

```rust
fn main() {
    let tuple: (u8, i32, String, char) = (10, 25, String::from("Rust4noobs"), 'a');
    println!("{:?}", tuple);
}
```

E agora, ainda utilizando o exemplo acima, como eu faço para acessar o valor `i32` da tupla? Ou seja, o segundo item. Assim como no array começamos a contagem por `0` nas tuplas também fazemos isso, porém ao invés de utilizarmos os colchetes para acessar, utilizamos o `.0`

```rust
fn main() {
              //0,  1, , 2     , 3
    let tuple: (u8, i32, String, char) = (10, 25, String::from("Rust4noobs"), 'a');
    println!("Valor i32: {}", tuple.1);
}
```

Acessamos o valor i32 e printamos ele.

Na parte sobre [ownership](./03-ownership.md) utilizamos da estratégia de retornar o que foi passado por parâmetro para não perdemos a posse de memória da variável, podemos fazer isso com tuplas também.

```rust
fn main() {
    let texto1 = String::from("Rust");
    let texto2 = String::from("4Noob");
    let (mut devolve1, mut devolve2) = printa_duas_strings(texto1, texto2);
    devolve1.push_str(" qualquercoisasópraterexemplo");
    devolve2.push_str(" sérionaotiveideianenhuma");
    println!("{}", devolve1);
    println!("{}", devolve2);
}

fn printa_duas_strings(texto1: String, texto2: String) -> (String, String) {
    println!("{}", texto1);
    println!("{}", texto2);
    (texto1, texto2)
}
```

Conseguimos utilizar a mesma estratégia de retornar os parâmetros para não perder o ownership com uma tupla.

- [Próximo](./05-slices.md) - Slices