# Coleção Vec

Em Rust já temos coleções implementadas, uma delas é a `Vec<T>`, é uma coleção que armazena uma quantidade de elementos. A vantagem de usar essa coleção é que diferente de um [array](../basic/09-arrays.md) é que seu tamanho é flexível. O que nos da certa vantagem. Podemos criar um `Vec` com os métodos estáticos `new` ou `with_capacity`, ou com o macro `vec!`, sendo a opção com o macro um `Vec` já inicializado com valores. Podemos declarar o `Vec`

```rust
fn main() {
    let mut vec = Vec::<i32>::new();
    let mut vec2: Vec<i32> = Vec::new();
    let mut vec3 = Vec::<i32>::with_capacity(10); //inicia o Vec já com uma capacidade
    let mut vec4: Vec<i32> = Vec::with_capacity(10); //inicia o Vec já com uma capacidade
    let mut vec5 = vec![1,2,3];
}
```

Iniciar um `Vec` já com uma capacidade o que nos da vantagem de deixar mais rápido a inserção de novos elementos nesse `Vec`, mas não vamos confundir a capacidade dele, com o tamanho dele, a capacidade é "o quanto cabe" e o tamanho é o "o quanto tem".

Note que declaramos todos os `Vec` como mutáveis, não é obrigatório serem declarados como mutáveis, nós só utilizamos isso caso queiramos realizar qualquer modificação no `Vec`, como adicionar, ou remover valores do mesmo.

O compilador do Rust é inteligente o suficiente para saber o tipo de um `Vec` pelo primeiro elemento adicionado. 

## Métodos de Vec

Em Vec temos diversos métodos, mas agora iremos falar sobre os seguintes métodos: push, pop, len, clear, is_empty, contains, get, insert

### Método push

O método `push` é responsável por adicionar um elemento ao `Vec`, o método ira falhar caso a capacidade do `Vec` ultrapasse o valor máximo de um `isize`
```rust
fn main() {
    let mut lista = Vec::new();
    lista.push(10);
}
```

### Método insert

Com o método insert conseguimos adicionar um valor na posição escolhida

```rust
fn main() {
    let mut lista = vec![1, 2, 3];
    lista.insert(0, 4);
}
```

### Método pop

O método `pop` remove o último elemento do `Vec` e nos retornar um `Option<T>`, sendo o `Some` caso tenha algum elemento e `None` caso o `Vec` esteja vazio.

```rust
fn main() {
    let mut lista = vec![1, 2, 3];
    let ultimo: Option<i32> = lista.pop();
}
```
### Método len

O método `len` retorna o tamanho do `Vec`

```rust
fn main() {
    let lista = vec![1,2,3];
    let tamanho = lista.len();
}
```

### Método get

O método `get` retorna um `Option<&T>` caso a posição solicitada exista, caso não exista é retornado um `None`, a vantagem de utilizar este método ao invés de um colchete e a posição `[pos]` é que se tentarmos acessar uma posição inexistente não paramos a execução da aplicação.


```rust
fn main() {
    let lista = vec![1,2,3];
    let primeiro_elemento: Option<i32> = lista.get(0);
    // let invalido = lista[1000]; //esta linha ira parar a execução do programa
}
```

### Método clear

O método `clear` limpa o `Vec` o deixando vazio

```rust
fn main() {
    let mut lista = vec![1,2,3];
    lista.clear();
    println!("Tamanho da lista {}", lista.len());
}
```

### Método is_empty
O método `is_empty` retorna um boleano, sendo true caso esteja vazio e false caso o tenha algum elemento

```rust
fn main() {
    let lista = Vec::<i32>::new();
    if lista.is_empty() {
        println!("A lista esta vazia");
    }
}
```

### Método contains
O método `contains`retorna um boleano, sendo true caso o valor exista e false caso não exista.

```rust
fn main() {
    let lista = vec![1, 2, 3];
    if lista.contains(&3) {
        println!("O numero 3 existe na lista");
    }
}
```