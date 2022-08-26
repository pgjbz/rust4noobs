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