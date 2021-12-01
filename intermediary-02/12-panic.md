# Macro panic!

O macro `panic!` é utilizado quando queremos encerrar a execução de um programa com ou sem uma mensagem, quando utilizamos o `.unwrap` ou `.expect` em um [enum](./02-enums.md) `None` ou algum [Result](./11-result.md) com `Erro` basicamente o que acontece por trás dos panos ele executa a ação do `panic` encerrando a execução do programa, com ou sem uma mensagem, cabe ao programador decidir usar ou não esta feature.

## Utilizando o macro panic!

Para utilizar o macro seguimos os seguintes padrões `panic!()` ou `panic!(mensagem)`

```rust
fn main() {
    panic!();
}
```
Ao executar o código acima temos a saida

```bash
thread 'main' panicked at 'explicit panic', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrac
```

Ou executando o código abaixo

```rust
fn main() {
    panic!("Rust4Noobs");
}
```

Temos a saida com a mensagem abaixo:

```bash
thread 'main' panicked at 'Rust4Noobs', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

