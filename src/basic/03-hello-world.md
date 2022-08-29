# Nosso primeiro "Hello World"

Após a instalação de um compilador nada melhor que iniciarmos com o famoso "Hello World!", a instalação da linguagem nos fornece a ferramenta `cargo` com esta ferramenta conseguimos criar  projetos em Rust, gerenciar dependências, rodar testes, rodar a aplicação e dar build na aplicação.

Para criarmos nosso primeiro projeto, iremos utilizar o seguinte comando:

```bash
cargo new hello-rust
```

Este comando ira criar um projeto com a seguinte estrutura:

```bash
├── Cargo.toml
└── src
    └── main.rs
```

O arquivo `Cargo.toml` é o arquivo onde temos as informações sobre o projeto, como nome, versão, autor(es), dependências, opções de build, edição, etc.

Na pasta `src` temos o código-fonte do nosso projeto, neste caso o como o nosso projeto é de um executável temos o arquivo `main.rs`,  nele esta o início de nossa jornada em códigos Rust.

```rust
fn main() {
    println!("Hello, world!");
}
```

Temos a palavra reservada `fn` que é palavra que define uma função, temos o [macro](https://doc.rust-lang.org/book/ch19-06-macros.html) `println!` que é o responsável por escrever no nosso console.

Executando o comando

```bash
cargo run
```

Teremos nosso primeiro código em Rust sendo executado e a mensagem `Hello, world!` sendo mostrada no nosso console. 