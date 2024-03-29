# Rc\<T>

Rc\<T> é um tipo de `Smart Pointer` com contagem de referências, o seu nome vem de `Reference Counting`, parecido com o [Box\<T>](02-smart-pointers-box.md), porém com contagem de referências e sem a implementação da `trait DerefMut`, sendo assim é um tipo de `Smart Pointer` imutável. Sua declaração é parecida com a de um Box\<T>, mas o que vai diferenciá-lo?

Vamos supor que temos o seguinte código:

```rust
fn main() {
    let numero = Box::new(69);
    escreva(numero);
    faca_qualquer_coisa(numero);
}

fn faca_qualquer_coisa<T>(valor: Box<T>) {
    todo!()
}

fn escreva<T: std::fmt::Display>(valor: Box<T>) {
    println!("valor = {}", valor)
}
```

O código acima ira nos retornar o seguinte erro:

```sh
error[E0382]: use of moved value: `numero`
 --> ref-count.rs:4:25
  |
2 |     let numero = Box::new(69);
  |         ------ move occurs because `numero` has type `Box<i32>`, which does not implement the `Copy` trait
3 |     escreva(numero);
  |             ------ value moved here
4 |     faca_qualquer_coisa(numero);
  |                         ^^^^^^ value used here after move

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
```

Isso acontece  justamente pelas regras de [ownership](../intermediary-01/03-ownership.md) do Rust. Um modo para contornarmos esse problema seria utilizando o Rc\<T>, e aqui que ele tem uma vantagem. Por ser uma referência compartilhada podemos clonar esta referência, claro apenas as informações necessárias, como o endereço de memória e... Só.

```rust
use std::rc::Rc;

fn main() {
    let numero = Rc::new(69);
    escreva(Rc::clone(&numero));
    faca_qualquer_coisa(Rc::clone(&numero));
}

fn faca_qualquer_coisa<T>(valor: Rc<T>) {
    todo!()
}

fn escreva<T: std::fmt::Display>(valor: Rc<T>) {
    println!("valor = {}", valor)
}
```

Realizamos o clone pelo chamando a implementação da `struct` Rc\<T>, mas nada nos impede de chamar o método `.clone()` da instância, porém não é o padrão utilizado por ai.

Este tipo de ponteiro, somente irá liberar a memória alocada quando o seu contador de referências chegar a 0. Ou seja, ninguém mais, apontar para aquela região de memória. No exemplo abaixo, podemos ver como saber o atual valor deste contador.

```rust
use std::rc::Rc;

fn main() {
    let referência = Rc::new(42);
    println!("contador atual esta em: {}", Rc::strong_count(&referência));
    let segunda_referência = Rc::clone(&referência);
    println!("contador atual esta em: {}", Rc::strong_count(&referência));
    {
        let terceira_referência = Rc::clone(&segunda_referência);
        println!("contador atual esta em: {}", Rc::strong_count(&referência));
    }
    println!("contador atual esta em: {}", Rc::strong_count(&referência));
} 
/*
    após o fim do escopo da função main, o contador chega a '0'
    a memória é liberada.
*/
```

A saída do programa acima é:

```sh
contador atual esta em: 1
contador atual esta em: 2
contador atual esta em: 3
contador atual esta em: 2
```

Repare que o valor do contador muda, independente de realizar o `clone` a partir da primeira referência, ou da segunda referência, isso acontece, porque os dois apontam para o mesmo local da memória.