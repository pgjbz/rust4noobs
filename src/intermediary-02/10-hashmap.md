# HashMap

A última coleção que iremos falar é o `HashMap`, esta coleção funciona como um [HashSet](./09-hashset.md), porém com o conceito de [chave-valor](./https://hazelcast.com/glossary/key-value-store/ ), onde temos uma chave associada a um valor, esta chave <strong>NÃO</strong> se repete, assim como um valor em um `HashSet`, porém para inserirmos e recuperarmos valores, utilizamos a chave. Assim como no `HashSet`, essa chave deve implementar `Eq` e `Hash` e para implementar `Eq` é necessário implementar `PartialEq`.

## Criando um HashMap

Para criar um `HashMap`, precisamos informar dois tipos genéricos.

```rust
fn main() {
    let mut map1 = HashMap::<String, i32>::new();
    let mut map2: HashMap<String, i32> = HashMap::new();
    let mut map3 = HashMap::new();
    map3.insert("Rust4Noobs".to_string(), i32::MAX);
}
```

Temos vários métodos para criar um `HashMap`, acima temos alguns exemplos. Os métodos que iremos falar serão, `insert`, `remove`, `get`, `get_mut`, `len`, `remove`, `clear`.

### Método insert

Para inserirmos uma chave e valor em um `HashMap` precisamos utilizar o método `insert` passando como argumentos uma `chave`e um `valor` este método nos retorna um [Option](./07-option.md).

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("Rust4Noobs", i32::MAX);
}
```

O método `insert` tem uma peculiaridade, ao inserir um valor e este valor ainda não existe, o `Option` retornado sera um `None` e o valor será inserido no `HashMap`, caso o valor já exista o `Option` retornado sera um `Some` com o valor antigo e este valor é substituído na coleção.

```rust
fn main() {
    let mut map = HashMap::new();
    let inserido = map.insert("Rust4Noobs", i32::MAX);
    println!("{:?}", inserido);
    let inserido = map.insert("Rust4Noobs", i32::MIN);
    println!("{:?}", inserido);
}
```

### Método get

O método `get` funciona de maneira parecida com o `Vec` e o `Set`, porém passamos uma chave e nos é devolvido um `Option`. Caso o valor exista temos um `Some` com o valor caso não temos um `None`.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("Rust4Noobs", i32::MAX);
    let rust4_noobs = map.get("Rust4Noobs");
    let rust4_experts = map.get("Rust4Experts");

    println!("{:?}", rust4_noobs);
    println!("{:?}", rust4_experts);
}
```

### Método get_mut

O método `get_mut` do mesmo modo que o método `get`, com a diferença de nos retornar uma referência mutável do valor e somente pode ser usado caso o `HashMap` seja mutável.

```rust
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Cliente {
    id: i32,
    nome: String
}

fn main() {
    let mut map = HashMap::new();
    map.insert("Rust4Noobs", Cliente { id: 0, nome: "https://github.com/pgjbz/rust4noobs".to_string()});
    let rust4_noobs = map.get_mut("Rust4Noobs");
    println!("{:?}", rust4_noobs);
    if let Some(rust) = rust4_noobs {
        rust.nome = "Nome Brabo".to_string();
    }
    let rust4_noobs = map.get_mut("Rust4Noobs");
    println!("{:?}", rust4_noobs);
}
```
<small>Sim, podemos redeclarar uma variável com um nome já existente e eu só mostrei isso agora, eu errei, eu sei</small>

### Método len 

O método `len` nos retorna o tamanho do `HashMap`.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("Rust4Noobs", i32::MAX);
    let len = map.len();
    println!("len = {}", len);
}
```

### Método remove

Utilizamos o método `remove` quando queremos remover alguma chave do nosso `HashMap`, nos retornando um `Option`, sendo `Some` contendo o valor removido, caso o valor exista e tenha sido removido e `None` caso o valor removido não exista.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("Rust4Noobs", i32::MAX);
    let rust4_noobs = map.get("Rust4Noobs");
    println!("Rust4Noobs {:?}", rust4_noobs);
    let valor_removido = map.remove("Rust4Noobs");
    println!("Valor removido {:?}", valor_removido);
    let rust4_noobs = map.get("Rust4Noobs");
    println!("Rust4Noobs {:?}", rust4_noobs);
}
```
### Método clear

O método `clear` é o mais simples dentre todos, este método apenas, limpa o nosso `HashSet`.

```rust
fn main() {
    let mut map = HashMap::new();
    map.insert("Rust4Noobs", i32::MAX);
    map.clear();
    println!("Tamanho do HashSet = {}", map.len());
}
```