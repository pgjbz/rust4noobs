# Generics

Afinal o que são Generics?

Generics é um modo de reaproveitar um código substituindo um tipo 'T' por um tipo concreto. O compilador de Rust, gera implementações com o tipo concreto para cada uso do generic, ele faz isso para não perdemos performance em tempo de execução.

## Declaração de um Generic

Para declarar um generic. Utilizamos o seguinte padrão `struct Nome<T> { implementacao }`

```rust
#[derive(Debug)]
struct Cliente<T> {
    nome: String,
    ano_nascimento: u16,
    documento: T
}

fn main() {
    let cliente: Cliente<String> = Cliente::<String> { 
        nome: String::from("Rust4Noobs"), 
        ano_nascimento: 2021,
        documento: String::from("https://github.com/pgjbz/rust4noobs") 
    }; 

    println!("{:#?}", cliente);
}
```

Do modo acima, o documento do `Cliente`, pode ser, uma String, um inteiro, um Enum, outra `struct`.

### Implementando um método genérico.

Continuando o exemplo acima, lembra da parte sobre as [structs](./01-structs.md) onde implementamos o método estático `new`, como que ficaria para este exemplo usando generic?

```rust
//--declaração da struct
impl<T> Cliente<T> {
    fn new(nome: String, ano_nascimento: u16, documento: T) -> Self {
        Self {
            nome,
            ano_nascimento,
            documento
        }
    }
}

fn main() {
    let cliente = Cliente::new(String::from("Rust4Noobs"), 
        2021, 
        String::from("https://github.com/pgjbz/rust4noobs"));
    println!("{:#?}", cliente);
    let cliente2 = Cliente::new(String::from("Rust4Noobs"), 
        2021, 
        123456789);
    println!("{:#?}", cliente2);
}
```

Eu não necessariamente tenho um limite de quantidade de Generics em uma struct. Por exemplo, e se nessa minha struct de Cliente eu quiser que o `ano_nascimento` possa ter outro tipo além de `u16`?

```rust
#[derive(Debug)]
struct Cliente<T, U> {
    nome: String,
    ano_nascimento: U,
    documento: T
}

impl<T, U> Cliente {
     fn new(nome: String, ano_nascimento: U, documento: T) -> Self {
        Self {
            nome,
            ano_nascimento,
            documento
        }
    }
}
```

### Funcões/Métodos com Generics

Claro generics não apenas para structs, métodos/funções também podem ter. Vamos para o seguinte exemplo.

```rust
fn maior<T>(lista: &[T]) -> T {
    let mut maior = lista[0];
    for &item in lista {
        if item > maior {
            maior = item;
        }
    } 
    maior
}

fn main() {
    let arr: [u8; 4] = [2,4,1,11];
    let maior = maior(&arr);
    println!("Maior elemento: {}", maior);
}
```

O código acima, esta quaaase funcionando, mas para funcionar precisamos explicar outro conceito, as 'traits'

- [Próximo](./06-traits.md) - Traits