# Box\<T>

O `Smart Pointer` Box<T>, é bem versátil, ele aponta para a `heap` e também implementa as [traits](../intermediary-02/06-traits.md) `Deref` e `DerefMut`, sendo assim, é possível apontar para um endereço de moria na Heap e realizar modificações nesta região de memoria. Vamos a um exemplo mais simples.

```rust
fn main() {
    let number: Box<usize> = Box::new(42);
    println!("{}", number);
}
```

Neste exemplo acima, apontamos para uma região da `heap`, um numero inteiro sem sinal, o Box, obedece as mesmas regras de [ownership](../intermediary-01/03-ownership.md) que ponteiros comuns.

Como ele foi declarado como imutável, não podemos fazer o `DerefMut` e alterar o valor na heap, caso tentarmos fazer isso teremos o seguinte erro.

```rust
fn main() {
    let number: Box<usize> = Box::new(42);
    *number = 69;
    println!("{}", number);
}
```

```sh
error[E0594]: cannot assign to `*number`, as `number` is not declared as mutable
 --> boxx.rs:4:5
  |
3 |     let number = Box::new(1);
  |         ------ help: consider changing this to be mutable: `mut number`
4 |     *number = 69;
  |     ^^^^^^^^^^^^ cannot assign

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
```

Declarando a variável como mutável podemos realizar essa alteração.

```rust
fn main() {
    let mut number: Box<usize> = Box::new(42);
    *number = 69;
    println!("{}", number);
}
```

## Tipos recursivos

Tipos recursivos, são aqueles que tem algum atributo que é representa ele mesmo. Como por exemplo em uma arvore binaria, cada "galho" é um tipo recursivo.

```rust
struct Node {
    left: Option<Node>,
    right: Option<Node>,
    value: usize,
}

fn main() {
    todo!();
}
```

Caso tentarmos compilar o código acima teremos o seguinte output do compilador:

```sh
error[E0072]: recursive type `Node` has infinite size
 --> recursive.rs:2:1
  |
2 | struct Node {
  | ^^^^^^^^^^^ recursive type has infinite size
3 |     left: Option<Node>,
  |           ------------ recursive without indirection
4 |     right: Option<Node>,
  |            ------------ recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `Node` representable
  |
3 ~     left: Option<Box<Node>>,
4 ~     right: Option<Box<Node>>,
  |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0072`.

```

O próprio compilador já nos diz algo que podemos fazer para corrigir este problema, mas antes vamos entender o que seria essa mensagem aqui "recursive type has infinite size".

Teoricamente tipos recursivos podem crescer ao infinito e o compilador do Rust não sabe o quanto de memoria sera necessário para esse tipo, por isso essa mensagem. Porém ele sabe exatamente quanta memoria é necessária para um `Box`, sendo possível a criação deste tipo recursivo utilizando esse `Box`.

```rust
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: usize,
}

fn main() {
    todo!();
}
```

No livro [Rust Book](https://doc.rust-lang.org/book/ch15-01-box.html) é possivel ver um exemplo bem interessante utilizando uma [Cons List](https://en.wikipedia.org/wiki/Cons)
