# Box\<T>

O `Smart Pointer` Box<T>, é bem versátil, ele aponta para a `heap` e também implementa as [traits](../intermediary-02/06-traits.md) `Deref` e `DerefMut`, sendo assim, é possível apontar para um endereço de moria na Heap e realizar modificações nesta região de memória. Vamos a um exemplo mais simples.

```rust
fn main() {
    let numero: Box<usize> = Box::new(42);
    println!("{}", numero);
}
```

Neste exemplo acima, apontamos para uma região da `heap`, um numero inteiro sem sinal, o Box, obedece as mesmas regras de [ownership](../intermediary-01/03-ownership.md) que ponteiros comuns.

Como ele foi declarado como imutável, não podemos fazer o `DerefMut` e alterar o valor na heap, caso tentarmos fazer isso teremos o seguinte erro.

```rust
fn main() {
    let numero: Box<usize> = Box::new(42);
    *numero = 69;
    println!("{}", numero);
}
```

```sh
error[E0594]: cannot assign to `*numero`, as `numero` is not declared as mutable
 --> boxx.rs:4:5
  |
3 |     let numero = Box::new(1);
  |         ------ help: consider changing this to be mutable: `mut numero`
4 |     *numero = 69;
  |     ^^^^^^^^^^^^ cannot assign

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
```

Declarando a variável como mutável podemos realizar essa alteração.

```rust
fn main() {
    let mut numero: Box<usize> = Box::new(42);
    *numero = 69;
    println!("{}", numero);
}
```

## Tipos recursivos

Tipos recursivos, são aqueles que tem algum atributo que é representa ele mesmo. Como por exemplo em uma arvore binaria, cada "galho" é um tipo recursivo.

```rust
struct Galho {
    esquerda: Option<Galho>,
    direita: Option<Galho>,
    valor: usize,
}

fn main() {
    todo!();
}
```

Caso tentarmos compilar o código acima teremos o seguinte output do compilador:

```sh
error[E0072]: recursive type `Galho` has infinite size
 --> recursive.rs:2:1
  |
2 | struct Node {
  | ^^^^^^^^^^^ recursive type has infinite size
3 |     esquerda: Option<Galho>,
  |           ------------ recursive without indirection
4 |     direita: Option<Galho>,
  |            ------------ recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `Node` representable
  |
3 ~     esquerda: Option<Box<Galho>>,
4 ~     direita: Option<Box<Galho>>,
  |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0072`.

```

O próprio compilador já nos diz algo que podemos fazer para corrigir este problema, mas antes vamos entender o que seria essa mensagem aqui "recursive type has infinite size".

Teoricamente tipos recursivos podem crescer ao infinito e o compilador do Rust não sabe o quanto de memória sera necessário para esse tipo, por isso essa mensagem. Porém ele sabe exatamente quanta memória é necessária para um `Box`, sendo possível a criação deste tipo recursivo utilizando esse `Box`.

```rust
struct Galho {
    esquerda: Option<Box<Galho>>,
    direita: Option<Box<Galho>>,
    valor: usize,
}

fn main() {
    todo!();
}
```

No livro [Rust Book](https://doc.rust-lang.org/book/ch15-01-box.html) é possivel ver um exemplo bem interessante utilizando uma [Cons List](https://en.wikipedia.org/wiki/Cons).


## Valores Dinâmicos

Recuperando o exemplo das traits de animais do da capítulo sobre [traits](../intermediary-02/06-traits.md)


Vamos supor que queremos ter uma lista de animais, sendo eles do tipo Cachorro, Gato e Papagaio. Podemos utilizar a palavra reservada `dyn` dentro de um `Box<T>`, para representar valores dinâmicos.

```rust
trait Animal {
    fn comer(&self);
}

struct Cachorro {}

impl Animal for Cachorro {
    fn comer(&self) {
        println!("Cachorro comendo...");
    }
}

struct Gato {}

impl Animal for Gato {
    fn comer(&self) {
        println!("gato comendo...");
    }
}

struct Papagaio {}

impl Animal for Papagaio {
    fn comer(&self) {
        println!("papagaio comendo...");
    }
}
fn main() {
    let animais: Vec<Box<dyn Animal>> = vec![
        Box::new(Gato {}),
        Box::new(Cachorro {}),
        Box::new(Papagaio {}),
    ];
    for animal in animais {
        animal.comer();
    }
}
```

Utilizando valores dinâmicos podemos ter tipos diferentes que implementam a mesma trait. Só devemos saber que, um ponteiro dinâmico ocupa o dobro do espaço que um ponteiro comum, isso por que temos um ponteiro para o tipo em questão e outro ponteiro para a implementação desta `trait`.