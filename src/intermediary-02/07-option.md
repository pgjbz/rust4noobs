# Enum especial `Option<T>`

Em Rust não temos <strong>nulo</strong>, isso mesmo a linguagem não aplica o [conceito de ponteiros nulos](https://en.wikipedia.org/wiki/Null_pointer), para dizer se algo existe ou não temos o enum `Option<T>`, este `enum`, os valores possíveis para este `enum` são `Some(T)` e `None`. Temos alguns métodos neste enum, como `is_none`, `is_some`, `unwrap`, `expected`, `or_else`, `or`.

## Extraindo o valor de dentro de um `Option<T>`

Podemos extrair o valor de um `Option`, pelos métodos, `unwrap`, `expect`, por um `match`, ou por um `if let`. Cada modo de extrair tem sua peculiaridade, com o `unwrap` ou com o `expect` caso o valor seja `None` temos uma falha na aplicação e sua execução é abortada.

```rust
struct Cliente {
    nome: String,
    idade: Option<u8>,
}

fn main() {
    let cliente = Cliente {
        nome: "Rust4Noobs".to_string(),
        idade: None,
    };
    cliente.idade.unwrap();
}
```

Ao executar o código acima teremos a execução do programa abortada e a mensagem de erro:

```bash
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:9:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

A diferença entre o `unwrap`e o `expect`, é que com o `expect` podemos definir uma mensagem para este erro

```rust
# struct Cliente {
#     nome: String,
#     idade: Option<u8>,
# }
//--declaração da struct
fn main() {
    let cliente = Cliente {
        nome: "Rust4Noobs".to_string(),
        idade: None,
    };
    cliente.idade.expect("idade não informada");
}
```

O código acima causa um erro a mensagem informada.

```bash
thread 'main' panicked at 'idade não informada', src/main.rs:8:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Mas se quisermos evitar este erro, como fazemos isso? Podemos utilizar os métodos `is_none` ou `is_some` para verificar isso.

```rust
# struct Cliente {
#     nome: String,
#     idade: Option<u8>,
# }
//--declaração da struct
fn main() {
    let cliente = Cliente {
        nome: "Rust4Noobs".to_string(),
        idade: Some(21),
    };
    if cliente.idade.is_some() {
        let idade = cliente.idade.unwrap();
        println!("O cliente {} tem {} anos", cliente.nome, idade);
    }
}
```

Agora temos uma checagem se o valor existe, podemos usar o `is_none` para adicionar um tratamento para caso a idade não exista.

```rust
# struct Cliente {
#     nome: String,
#     idade: Option<u8>,
# }
//--declaração da struct
fn main() {
    let mut cliente = Cliente {
        nome: "Rust4Noobs".to_string(),
        idade: None,
    };
    if cliente.idade.is_none() {
        println!("Idade do cliente não informada, favor informar:");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        cliente.idade = Some(buffer.trim().parse().unwrap());
    }
}
```

Mas esse talvez não seja o melhor modo de fazer isso.

## Extraindo o valor com um match

O operador [match](./03-match.md) pode ser utilizado para [enums](./02-enums.md), lembra dos `enums` com valores associados? O `Option` é um `enum` com valores associados. Então podemos utilizar o `match` para chegar se o valor existe.

```rust
# struct Cliente {
#     nome: String,
#     idade: Option<u8>,
# }
//--declaração da struct
fn main() {
    let cliente = Cliente {
        nome: "Rust4Noobs".to_string(),
        idade: None,
    };
    match cliente.idade {
        Some(idade) => println!("A idade do cliente {} é {}", cliente.nome, idade),
        None => println!("Idade do cliente {} não foi informada", cliente.nome)
    }
}
```

Claro podemos utilizar de todos os aspectos do `match` nessa abordagem

## Operador if let

O operador `if let` é geralmente usado para tratativas pequenas. Onde realizamos uma validação e já atribuímos o valor a uma variável. Podendo ser feito da seguinte maneira `if let Some(nome ou ignora o valor) = expressao teste { codigo } else { se nao }`.

```rust
# struct Cliente {
#     nome: String,
#     idade: Option<u8>,
# }
//--declaração da struct
fn main() {
    let cliente = Cliente {
        nome: "Rust4Noobs".to_string(),
        idade: None,
    };
    if let Some(idade) = cliente.idade {
        println!("A idade do cliente {} é {}", cliente.nome, idade)
    } else {
        println!("Idade do cliente {} não foi informada", cliente.nome)
    }
}
```

O código acima tem o mesmo resultado do código com o `match`, claro o `if let` também pode retornar algo, assim como o `match`

```rust
# struct Cliente {
#     nome: String,
#     idade: Option<u8>,
# }
//--declaração da struct
fn main() {
    let cliente = Cliente {
        nome: "Rust4Noobs".to_string(),
        idade: None,
    };
    let idade = if let Some(idade) =  cliente.idade {
        idade
    } else {
        34 + 35
    };
    println!("A idade do cliente {} é {}", cliente.nome, idade);
}
```

E temos sucesso, caso o valor exista é retornado o valor dentro de `Some(idade)` caso não exista é retornado o resultado da soma de `34 + 35`.

## Operador while let

O operador `if let` tem um irmão o `while let`, seu comportamento é parecido, lembram da implementação de da [trait](./06-traits.md) `Iterator` que realizamos? Caso não lembre aqui esta ela.

```rust
struct Contador {
    contagem: u64
}

impl Iterator for Contador {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.contagem >= 100 {
            None
        } else {
            self.contagem += 1;
            Some(self.contagem)
        }
    }
}
```

Ao implementar essa trait, temos o método `next` que nos retorna um `Option`, podemos utilizar esse retorno para ir iterando o nosso contador.

```rust
# struct Contador {
#     contagem: u64
# }
# impl Iterator for Contador {
#     type Item = u64;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         if self.contagem >= 100 {
#             None
#         } else {
#             self.contagem += 1;
#             Some(self.contagem)
#         }
#     }
# }
//--declaração do contador
fn main() {
    let mut contador = Contador { contagem: 0 };

    while let Some(n) = contador.next() {
        println!("Contador atual {}", n)
    }
}
```

O enum `Option`, nos da um controle muito grande, e nos possibilita vários modos de tratar valores inexistentes, vale a pena se aprofundar mais nele.
