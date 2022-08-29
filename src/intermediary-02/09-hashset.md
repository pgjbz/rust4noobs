# HashSet

O `HashSet` é uma das coleções da biblioteca padrão do Rust, esta biblioteca é parecida com o [Vec](./08-vec.md), porém ela não permite repetições. Ela utiliza algum algorítimo de [Hash](./https://en.wikipedia.org/wiki/Hash_function) para garantir que os elementos inseridos serão únicos.

## Usando um HashSet

Um `HashSet` é uma coleção genérica, e assim como o ela armazena dados de vários tipo, porém para tais dados serem armazenados eles precisam implementar certas [traits](./06-traits.md), para começar vamos utilizar os tipos primitivos.

### Criando um HashSet

Temos alguns modos para criar um `HashSet`. 

Temos um modo declarando o tipo e instânciando que segue o padrão `let nome: HashSet<tipo> = HashSet::new()`, o modo por inferência, `let nome = HashSet::<tipo>::new()`, claro podemos fazer `let nome: HashSet<tipo> = HashSet::<tipo>::new()` ou `let nome = HashSet::new()` e o tipo é definido pelo primeiro uso.

```rust
fn main() {
    let mut hashSet: HashSet<i32> = HashSet::new();
    let mut hashSet2 = HashSet::<i32>::new();
    let mut hashSet3 = HashSet::new();
    hashSet3.insert(10);
}
```

<small>No exemplo acima só utilizamos `mut` para poder manipular o `HashSet`</small><br><br>
Temos alguns métodos para trabalhar com um `HashSet`, como, por exemplo, `insert`, `contains`, `remove`, `get`

### Método insert

O método `insert` funciona para inserirmos um elemento ao `HashSet`. Já o utilizamos no exemplo de criação de um `HashSet`, este método nos retorna um booleano com `true` caso consiga inserir com sucesso e false caso não consiga.

```rust
fn main() {
    let mut set = HashSet::new();
    if set.insert(10) {
        println!("Eitcha lele");
    }
    if !set.insert(10) {
        println!("Que coisa nao");
    }
}
```

### Método contains

O método contains, serve para verificarmos se um elemento existe no `HashSet` nos retornando um `true` caso exista e `false` caso não.


```rust
fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    if set.contains(&10) {
        println!("EXISTEEEEEEEEEEEEEEEEEEEEE");
    }
}
```

### Método remove

O método remove como o seu nome diz, remove um elemento da coleção. Nos retornando `true` caso consiga remover e `false` caso não.

```rust
fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    if set.remove(&10) {
        println!("Removido");
    }
}
```

### Método get

O método get recupera um valor da coleção, nos retornando um [Option](./07-option.md).

```rust
fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    let num = set.get(&10);
}
```

Temos alguns métodos com funcionamentos iguais aos de um [Vec](./08-vec.md) como `is_empty`, `clear`, `len`.

# Usando um HashSet com um "tipo nosso"

Nem sempre é tão simples utilizar um `HashSet`, para tipos que criamos, como [enums](./02-enums.md) ou [structs](./01-structs.md), como no exemplo abaixo:

```rust
use std::collections::HashSet;

struct Cliente {
    id: i32,
    nome: String
}

fn main() {
    let mut set = HashSet::<Cliente>::new();
    set.insert(Cliente { id: 10, nome: "Rust4Noobs".to_string() });
}
```

Ao tentarmos utilizar o método `insert` teremos um problema.

```bash
error[E0599]: the method `insert` exists for struct `HashSet<Cliente>`, but its trait bounds were not satisfied
  --> src/main.rs:10:9
   |
3  | struct Cliente {
   | --------------
   | |
   | doesn't satisfy `Cliente: Eq`
   | doesn't satisfy `Cliente: Hash`
...
10 |     set.insert(Cliente { id: 10, nome: "Rust4Noobs".to_string() });
   |         ^^^^^^ method cannot be called on `HashSet<Cliente>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `Cliente: Eq`
           `Cliente: Hash`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `traits` due to previous error
```

Para utilizarmos um `HashSet` nossa struct precisa implementar duas traits, sendo elas `Eq` e `Hash`, porém para implementar `Eq` é necessário implementar `PartialEq`, essas traits podem ser geradas por macros como no exemplo abaixo.

```rust
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Cliente {
    id: i32,
    nome: String
}

fn main() {
    let mut set = HashSet::<Cliente>::new();
    set.insert(Cliente { id: 10, nome: "Rust4Noobs".to_string() });
}
```

Essas traits servem para comparação e geração do Hash, ou podemos também implementar por nós mesmos.

```rust
use std::hash::Hash;

struct Cliente {
    id: i32,
    nome: String
}

impl PartialEq for Cliente {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id //decidimos comparar apenas o id, ignorando o nome
    }
}

impl Hash for Cliente {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state); //geramos o hash apenas com o id
    }
}

impl Eq for Cliente {}

fn main() {
    let mut set = HashSet::<Cliente>::new();
    set.insert(Cliente { id: 10, nome: "Rust4Noob".to_string() });
}
```

Agora podemos utilizar a nossa struct em um `HashSet`