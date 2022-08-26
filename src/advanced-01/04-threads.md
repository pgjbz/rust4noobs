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
    });
    let handle2 = thread::spawn(|| {
        println!("a = {}", a);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

Ai você pensa "hm... o compilador disse para eu mover o valor de um lugar para o outro, vou fazer isso".

```rust
fn main() {
    use std::thread;
    let mut a = 10;
    let handle1 = thread::spawn(move || {
        a = 20;
    });
    let handle2 = thread::spawn(move || {
        println!("a = {}", a);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
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
    });
    let handle2 = thread::spawn(move || {
        println!("a = {:?}", a);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
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


