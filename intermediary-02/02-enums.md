# Enum

Afinal para que serve um enum?

Um enum é uma forma de declarar constantes, de forma mais semantica, sua declaração é feita da seguinte maneira `enum Nome { valores }`.

```rust
enum Uf {
    Sp,
    Rj,
    Ce,
}
```

Os enums em Rust, podem também armazenar valores de forma parecida com uma [tupla](../intermediary-01/04-tuples.md), mas o modo de recuperar o valor sera mais explicado com mais detalhes na próxima parte.

```rust
enum Uf {
    Sp(String),
    Rj(String),
    Ce(String),
}
```

Por enquanto vamos fazer a seguinte alteração neste `enum` para conseguirmos escrever no console o valor do Enum

```rust
#[derive(Debug)]
enum Uf {
    Sp(String),
    Rj(String),
    Ce(String),
}
```

Esta alteração que fizemos também pode ser aplicada as [structs](./01-structs.md), para tirarmos proveito desta modificação iremos usar o macro `println!` da seguinte maneira.

```rust
//--declaração do enum
fn main() {
    let uf = Uf::Sp(String::from("São Paulo"));
    println!("{:?}", uf);
    println!("{:#?}", uf);
}
```

Este modo de uso ira escrever no console o modo em que a estrutura foi declarada, tanto para os enums quanto para as [structs](./01-structs.md).

## Métodos em enums.

Também conseguimos implementar métodos para nossos enums, do mesmo modo que fazemos com as `structs`.

```rust
//--Declaração do enum
impl Uf {
    fn retorna_sp() -> Self {
        Self::Sp(String::from("São Paulo"))
    }

    fn quem_sou_eu(&self) {
        todo!()
    }
}

fn main() {
    let uf = Uf::retorna_sp();
    println!("{:?}", uf);
    println!("{:#?}", uf);
}
```

Na implementação acima temos o método `retorna_sp` que ira retornar um enum já com o valor preenchido, e temos também o método `quem_sou_eu` que iremos implementar na próxima parte deste 4noobs.