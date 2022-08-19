# Módulos

Os módulos são a forma em que o Rust tem para organizar o código, eles podem ser feitos no mesmo arquivo, em arquivos diferentes, ou em até subdiretórios do projeto.

Vamos pegar aquele primeiro exemplo utilizado nas [structs](./01-structs.md) e colocar ele dentro de um módulo, para declarar um módulo fazemos da seguinte maneira `mod NomeDoModulo { conteúdo }`.

```rust
mod nota_fiscal {
    struct Cliente {
        nome: String,
        ano_de_nascimento: u16,
        documento: String,
    }

    impl Cliente {
        fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {
            Self {
                nome,
                ano_de_nascimento,
                documento
            }
        }
    }
}

fn main() {
    let cliente = Cliente::new(String::from("Paulo"), 1999, String::from("Onde?"));
}
```

Separamos a struct `Cliente` em outro módulo, agora vamos tentar compilar este código... Espera temos um erro.

```bash
error[E0433]: failed to resolve: use of undeclared type `Cliente`
  --> src/main.rs:20:23
   |
20 |     let mut cliente = Cliente::new(String::from("Paulo"), 1999, String::from("Onde?"));
   |                       ^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
1  | use crate::nota_fiscal::Cliente;
   |

For more information about this error, try `rustc --explain E0433`.
error: could not compile `modulos` due to previous error
```

A struct não foi encontrada no escopo. O compilador esta dizendo para importamos o cliente, vamos importar utilizando o que o compilador diz para importar `use crate::nota_fiscal::Cliente;`

```rust
//--declaração do modulo

use crate::nota_fiscal::Cliente;

use crate::nota_fiscal::Cliente;

fn main() {
    let cliente = Cliente::new(String::from("Paulo"), 1999, String::from("Onde?"));
}
```

E compilamos... Outro erro.

```bash
error[E0603]: struct `Cliente` is private
  --> src/main.rs:19:24
   |
19 | use crate::nota_fiscal::Cliente;
   |                        ^^^^^^^ private struct
   |
note: the struct `Cliente` is defined here
  --> src/main.rs:2:5
   |
2  |     struct Cliente {
   |     ^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `modulos` due to previous error
```

Por padrão todas as declarações dentro de um módulo são privadas, para resolvermos este problema utilizamos a palavra `pub`, vamos adicionar isso tanto para a struct quanto para a implementação do método estático `new`, porque ele também é privado.

```rust
mod nota_fiscal {
    pub struct Cliente {
        nome: String,
        ano_de_nascimento: u16,
        documento: String,
    }

    impl Cliente {
        pub fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {
            Self {
                nome,
                ano_de_nascimento,
                documento
            }
        }
    }
}

//--método main
```

Os atributos também são privados, caso tentarmos acessar qualquer um deles teremos outro erro de compilação, podemos acessar através de outros métodos públicos, ou deixando os atributos públicos.

```rust
mod nota_fiscal {
    pub struct Cliente {
        pub nome: String,
        pub ano_de_nascimento: u16,
        pub documento: String,
    }
//--métodos
}

```

## Separando módulos em outros arquivos

Não adianta muito criamos módulos para organizar o nosso código e mantermos tudo no mesmo arquivo. Vamos começar criando um arquivo `nota_fiscal.rs` e jogando o código do Cliente para este arquivo.


A estrutura do nosso projeto fica da seguinte maneira.

```bash
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    └── nota_fiscal.rs
```

main.rs
```rust
mod nota_fiscal;

use crate::nota_fiscal::Cliente;

fn main() {
    let cliente = Cliente::new(String::from("Paulo"), 1999, String::from("Onde?"));
    println!(
        "Nome: {}\nAno de nascimento: {}, Documento: {}",
        cliente.nome, cliente.ano_de_nascimento, cliente.documento
    );
}
```

nota_fiscal.rs
```rust
pub struct Cliente {
    pub nome: String,
    pub ano_de_nascimento: u16,
    pub documento: String,
}

impl Cliente {
    pub fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {
        Self {
            nome,
            ano_de_nascimento,
            documento,
        }
    }
}
```

Agora o projeto dividido. Porém, esta não é a única maneira, podemos utilizar uma pasta com o mesmo nome `nota_fiscal` e dentro da pasta um arquivo `mod.rs`, para termos o mesmo efeito.

E teríamos a seguinte estrutura de projeto.

```bash
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    └── nota_fiscal
        └── mod.rs
```

O código de `nota_fiscal.rs` é transferido para `mod.rs` na pasta `nota_fiscal` e nada mais é mudado. Temos o mesmo comportamento e o código dividido em módulos.

### Importando e re-exportando módulos

Vamos realizar a criação de um módulo chamado `pedido` dentro de nosso modulo `nota_fiscal` e dentro do módulo `pedido` vamos criar um módulo `produto`.

A estrutura do nosso projeto ficara da seguinte maneira:

```bash
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    └── nota_fiscal
        ├── mod.rs
        └── pedido
            └── mod.rs
```

nota_fiscal/mod.rs
```rust
pub mod pedido;

#[derive(Debug)]
pub struct Cliente {
    pub nome: String,
    pub ano_de_nascimento: u16,
    pub documento: String,
}

impl Cliente {
    pub fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {
        Self {
            nome,
            ano_de_nascimento,
            documento
        }
    }
}
```
nota_fiscal/pedido/mod.rs
```rust
use self::produto::Produto;

use super::Cliente;

#[derive(Debug)]
pub struct Pedido<'a> {
    pub cliente: Cliente,
    pub produtos: &'a [Produto]
}

impl<'a> Pedido<'a> {
    pub fn new(cliente: Cliente, produtos: &'a [Produto]) -> Self {
        Self {
            cliente,
            produtos
        }
    }
}

pub mod produto {
    #[derive(Debug)]
    pub struct Produto {
        pub nome: String,
        pub preco: f64
    }

    impl Produto {
        pub fn new(nome: String, preco: f64) -> Self {
            Self {
                nome,
                preco
            }
        }
    }
}
```

main.rs
```rust
mod nota_fiscal;

use nota_fiscal::pedido::{Pedido, produto::Produto};

use crate::nota_fiscal::Cliente;

fn main() {
    let cliente = Cliente::new(String::from("Rust4Noobs"),
1999, String::from("Q?"));
    let produto = Produto::new(String::from("4Noobs"), 0f64);
    let produtos = &[produto];
    let pedido = Pedido::new(cliente, produtos);
    println!("{:#?}", pedido)
}

```
Nesse exemplo temos vários modos de imports, temos um impor com a palavra `crate` que é a raiz do nosso projeto. Seria o modo de import do mesmo projeto com o path absoluto, temos também o `super` que é um import a partir do modulo anterior, ou seja, o modulo que declara aquele módulo como tal. Meio confuso, mas conforme vamos praticando fica mais fácil de entender. E temos o import a partir de `nota_fiscal' sendo um modulo do nosso projeto, podemos importar tudo a partir dele, é um modulo que foi declarado em nosso main. 

Futuramente iremos utilizar outro modo de projeto que ira utilizar o arquivo `lib.rs`, onde também podemos declarar os módulos e remover isso do `main.rs`, com esse arquivo podemos importar conforme o nome do projeto no `Cargo.toml`

lib.rs
```rust
pub mod nota_fiscal;
```

main.rs
```rust
use modulos::nota_fiscal::pedido::{Pedido, produto::Produto};

use modulos::nota_fiscal::Cliente;

fn main() {
    let cliente = Cliente::new(String::from("Rust4Noobs"),
1999, String::from("Q?"));
    let produto = Produto::new(String::from("4Noobs"), 0f64);
    let produtos = &[produto];
    let pedido = Pedido::new(cliente, produtos);
    println!("{:#?}", pedido)
}
```

Nota:

Quando declaramos um módulo interno e utilizamos o "use" como "pub" o modulo interno é exportado como se fosse parte do módulo que o declarou.
