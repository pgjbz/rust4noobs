# Snake Game

Eu estava em dúvida em que projeto fazer neste ponto, foi dificil pensar em algo e porque algo que eu nunca fiz?

Então vamos lá fazer o "jogo da cobrinha", vamos começar criando o projeto, usando o comando `cargo new snake-game`.

Teremos a estrutura padrão do projeto

```bash
├── Cargo.toml
└── src
    └── main.rs
```

No arquivo Cargo.toml adicionaremos algumas dependencia para facilitar a nossa vida, serão elas [rand](https://rust-random.github.io/book/) e [crossterm](https://docs.rs/crossterm/0.23.0/crossterm/). Nosso Cargo.toml deve ficar parecido com isso:

```toml
[package]
name = "snake-game"
version = "0.1.0"
edition = "2021"

[dependencies]

crossterm = "0.23.0"
rand = "0.8.5"
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