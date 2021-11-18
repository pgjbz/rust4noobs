# Tipos de dados

Rust é uma linguagem fortemente e estaticamente tipada, nela temos alguns tipos

### Tipos

| Tipo | Tamanho | Valor Máximo | Valor Mínimo |  |
| ---  | ---     | ---          | ---          | ---      |
|  i8  | 1 byte  | 127          | -128         | Numérico |
|  u8  | 1 byte  | 255          | 0            | Numérico |
|  i16 | 2 bytes | 32767        | -32768       | Numérico |
|  u16 | 2 bytes | 65535        | 0            | Numérico |
|  i32 | 4 bytes | 2147483647   | -2147483648  | Numérico |
|  u32 | 4 bytes | 4294967295   | 0            | Numérico |
|  i64 | 8 bytes | 9223372036854775807   | -9223372036854775808            |Numérico |
|  u64 | 8 bytes | 18446744073709551615   | 0            | Numérico |
|  f32 | 4 bytes | 340282350... |-340282350... | Numérico |
|  f64 | 8 bytes | 1797693134862315700... | -1797693134862315700... | Numérico |
| bool | 1 byte  | true | false | booleano
| char | depende  | depende  | depende | character

Temos também o tipo `usize` que vai dependender da arquitetura do sistema operacional.

### Variáveis

Para declararmos "variáveis" em Rust utilizamos a palavra reservada `let`, mas o uso de apenas essa palavra para a declaração das "variáveis" armazena tipos imutáveis, para termos variáveis que podem ser alteradas temos que utilizar outra palavra reservada `mut`.

Temos dois modos de declarar variáveis, uma em que falamos o tipo para o compilador e outra que o compilador "decide o tipo" para nós. Para a declaração que informamos o tipo temos a seguinte sintaxe `let nome: tipo = valor` e para a que o compilador decide temos a sintaxe `let nome = valor`;

```rust
fn main() {
    let idade_atual: u8 = 22;
    let ano_nascimento = 1999; //inferencia de tipo

    println!("Idade atual {}, ano de nascimento: {}", idade_atual, ano_nascimento);
}
```

No exemplo acima utilizamos a declaração do tipo e inferência do mesmo, mas também podemos dar uma dica ao compilador ao tipo que iremos utilizar, o tipo padrão para números inteiros é `i32`, mas é um disperdicio de alguns bytes, podemos fazer da seguinte maneira.

```rust
fn main() {
    let idade_atual: u8 = 22;
    let ano_nascimento = 1999_u16;

    println!("Idade atual {}, ano de nascimento: {}", idade_atual, ano_nascimento);
}
```

Deste modo temos outro jeito de informar ao compilador que tipos queremos utilizar, porém, para este exemplo não faz muito sentido.

```rust
fn main() {
    let crab_power = 100_f32;

    println!("Poder do carangueijo {}%", crab_power);
}
```

Mas para casos como este, esse tipo de abordagem faz mais sentido.

- [Próximo](./05-operators.md) - Condicionais