# Threads

Afinal o que são Threads? Podemos considerar threads como sub processos que compartilham da mesma memoria, diferente de estratégias como "fork" que é feita uma copia do processo com espeço de memoria isolado, as threads são sempre do mesmo processo e custam menos para serem criadas.

Estas threads são executadas de forma assíncrona, ou seja todas de uma vez, claro existem estratégias para a execução dessa thread que o seu sistema operacional provavelmente usa. 

## Threads em Rust

Quando um processo é criado temos a thread principal, que parte a partir da função `main`, a partir desta threads conseguimos criar threads novas.

```rust
use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello World!");
    });
}
```

Notem que utilizamos uma [closure](./03-closures.md) para dizer qual o comportamento que esta thread terá. Ao executarmos este programa temos o segu... espera, não temos output, como assim? A thread não executou? Sim, ela executou, porém não teve tempo o suficiente para escrever a mensagem na saída, já que a criação de uma thread não bloqueia a thread que a criou, ela continua sendo executada, o que podemos fazer para bloquear a thread que criou a thread é chamar o método 'join', do valor retornado por esse método `thread::spawn`,  ele nos retorna um `JoinHandle<T>`.

```rust
use std::thread::{self, JoinHandle};

fn main() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        println!("Hello World!");
    });
    handle.join().unwrap();
}
```

Agora executando o código acima, criamos uma segunda thread, e bloqueamos a thread principal, quando a thread criada termina sua execução a thread principal é liberada. Também é possível recuperar um valor de dentro da thread, basta a `closure` retornar um valor.

```rust
use std::thread::{self, JoinHandle};

fn main() {
    let handle: JoinHandle<usize> = thread::spawn(|| 42);
    let valor = handle.join().unwrap();
    println!("valor da retornado pela thread = {}", valor);
}
```

Vamos fazer um teste com mais threads agora, cada uma imprimindo um valor de 0 a 10 e bloqueando a thread principal até o termino dessa execução.

```rust
use std::thread;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 0..10 {
            println!("da thread 1: {i}");
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 0..10 {
            println!("da thread 2: {i}");
        }
    });
    let handle3 = thread::spawn(|| {
        for i in 0..10 {
            println!("da thread 3: {i}");
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}
```

Note que cada execução vai gerar uma saída diferente, pode ser que a thread 1 execute primeiro, e logo em sequencia a 3, pode ser que elas se misturem, isso acontece justamente por serem executadas de forma assíncrona.

## Concorrência e paralelismo

Concorrência e paralelismo são temas diferentes que andam lado a lado, paralelismo foi o que fizemos no exemplo anterior, executamos códigos de modo paralelo, ou seja ao mesmo tempo, já a concorrência aconteceria quando esses códigos paralelos tentassem acessar o mesmo recurso, Rust foi pensado para ser `thread safe`, ou seja, seguro para trabalhar com threads.

Se tentarmos usar o código abaixo não teremos sucesso em sua compilação.

```rust
fn main() {
    use std::thread;
    let mut a = 10;
    let handle1 = thread::spawn(|| {
        a = 20;
    }).join().unwrap();
    let handle2 = thread::spawn(|| {
        println!("a = {}", a);
    }).join().unwrap();
}
```

Ai você pensa "hm... o compilador disse para eu mover o valor de um lugar para o outro, vou fazer isso".

```rust
fn main() {
    use std::thread;
    let mut a = 10;
    let handle1 = thread::spawn(move || {
        a = 20;
    }).join().unwrap();
    let handle2 = thread::spawn(move || {
        println!("a = {}", a);
    }).join().unwrap();
}
```

Movemos o valor, e a bloqueamos a thread até o fim da execução e... 'a = 10'? Ué. Espera, aprendemos sobre o `Rc<T>` e sobre o `RefCell<T>`, vou criar uma referencia mutável e compartilhada.

```rust
use std::{cell::RefCell, rc::Rc, thread};

fn main() {
    let mut a = Rc::new(RefCell::new(10));
    let t2 = Rc::clone(&a);
    let handle1 = thread::spawn(move || {
        *t2.borrow_mut() = 42;
    }).join().unwrap();
    let handle2 = thread::spawn(move || {
        println!("a = {:?}", a);
    }).join().unwrap();
}

```

Outro erro:

```sh
error[E0277]: `Rc<RefCell<i32>>` cannot be sent between threads safely
   --> src/main.rs:6:19
    |
6   |       let handle1 = thread::spawn(move || {
    |  ___________________^^^^^^^^^^^^^_-
    | |                   |
    | |                   `Rc<RefCell<i32>>` cannot be sent between threads safely
7   | |         *t2.borrow_mut() = 42;
8   | |     });
    | |_____- within this `[closure@src/main.rs:6:33: 8:6]`
    |
    = help: within `[closure@src/main.rs:6:33: 8:6]`, the trait `Send` is not implemented for `Rc<RefCell<i32>>`
note: required because it's used within this closure
   --> src/main.rs:6:33
    |
6   |       let handle1 = thread::spawn(move || {
    |  _________________________________^
7   | |         *t2.borrow_mut() = 42;
8   | |     });
    | |_____^
note: required by a bound in `spawn`
   --> /home/pgjbz/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:653:8
    |
653 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `closures` due to previous error

```

Vamos parar e pensar um pouco... Por que nossa primeira tentativa não deu certo?

Quando utilizamos um tipo que implementa a `trait` Copy ao ser passada para outro contexto é feita uma cópia inteira de seu valor, ou seja, é feita uma passagem por valor e não por referencia, por isso apenas utilizar o `move` para mover a variável de contexto não nos da sucesso no que queremos fazer.

O segundo erro acontece porque os tipos `Rc<T>` e `RefCell<T>`, não são tipos seguros para serem mandados através das threads, ou seja, eles não tem segurança para threads. Por isso iremos ver sobre os tipos `Arc<T>`, `Mutex<T>`, e `RwLock<T>` que implementam `traits` como `Send` e `Sync`.

## Scope

A partir da versão ` 1.63.0` do Rust temos um novo modo de usar `threads` que é utilizando a função `scope`, essa função basicamente cria um escopo onde podemos criar `threads`, e manipular os dados, é feito um `join` automático em todas as threads criadas dentro deste escopo e os o compilador do Rust entende que esses dados podem ser usados "sem riscos".

Vamos utilizar o exemplo do [Rust Blog](https://blog.rust-lang.org/2022/08/11/Rust-1.63.0.html) e entender o que acontece nele.

```rust
use std::thread::scope;

fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    scope(|s| {
        s.spawn(|| {
            println!("ola a partir da primeira thread por escopo");
            dbg!(&a);
        });
        s.spawn(|| {
            println!("ola a partir da segunda thread por escopo");
            x += a[0] + a[2];
        });
        println!("ola da thread principal");
    });
    a.push(4);
    assert_eq!(x, a.len());
}
```

O que acontece é que ao criar o escopo, eu consigo fazer o empréstimo para o escopo, enquanto as threads deste escopo estiverem sendo executadas, eu consigo realizar operações com as variáveis externas sem a necessidade de utilizar o `move`, como só acessamos a variável `x` em uma das `threads` não temos problemas em modifica-la, ao fim do escopo temos acesso novamente as variáveis. Caso tentarmos modificar a variável "x" teremos um problema de [ownership](../intermediary-01/03-ownership.md), violando a regra de referencia exclusiva das referencias mutáveis.