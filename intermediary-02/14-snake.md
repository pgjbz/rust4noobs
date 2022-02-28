# Snake Game

Eu estava em dúvida em que projeto fazer neste ponto, foi dificil pensar em algo e porque algo que eu nunca fiz?

Então vamos lá fazer o "jogo da cobrinha", vamos começar criando o projeto, usando o comando `cargo new snake-game`.

Teremos a estrutura padrão do projeto

```bash
├── Cargo.toml
└── src
    └── main.rs
```

Vamos adicionar um arquivo chamado `lib.rs` dentro da pasta `src`, este arquivo sera usado para declarar os nosso módulos. Em seguida vamos criar um arquivo chamado "ponto.rs" e nele iremos criar uma [struct](./01-structs.md) para as localizações no nosso jogo, vamos criar uma implementação a essa struct para facilitar a instanciação dessa `strutc`.

```rust
pub struct Ponto {
    pub x: usize,
    pub y: usize
}

impl Ponto {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
}
```

Para que este arquivo seja reconhecido no projeto, vamos adicionar no nosso arquivo `lib.rs` a seguinte linha `pub mod point;`. Note que tanto a `struct` quanto seus atributos e a implementação do método `new` estão com a palavra `pub`, que faz eles serem visiveis fora desse módulo.

Vamos printar o campo onde a cobrinha ira andar, e para testar vamos adicionar um ponto nesse tabuleiro.

```rust
fn main() {
    let point = Point::new(7, 7);
    let (x, y) = (15, 15);
    for x in 0..x {
        for y in 0..y {
            if point == (x, y) {
                print!("# ")
            }  else {
                print!("- ");   
            }
        }
        println!();
    }
}

```

Node que comparamos a nossa `struct` Ponto, com uma tupla de (x, y), para isso ser possivel, precisamos implementar uma [trait](./06-traits.md) chamada [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) a implementação para isso é relativamente simples. A `trait` recebe um parametro [generico](./05-generics.md) na implementação vamos falar que esse parametro genérico é uma `tupla (usize, usize)`. E a partir dai implementamos nossa comparação.

```rust
impl PartialEq<(usize, usize)> for Ponto {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}
```

Agora quando rodarmos o projeto com `cargo run`, teremos um tabuleiro no console com um ponto na posição (7, 7).

```bash
   Compiling snake-game v0.1.0 (/home/paulo.bezerra/workspace/ws-rust/rust4noobs/projects/snake-game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/snake-game`
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - # - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - -
```