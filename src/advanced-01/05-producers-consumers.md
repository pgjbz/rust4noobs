# Produtores e Consumidores

Agora que já falamos sobre [threads](./04-threads.md), conseguimos seguir com algumas estratégias para comunicação entre as threads. Em rust temos o modululo [std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/) que permite a comunicação entre as threads através da memoria, o que é bem interessante, porque ao invés de [competir](https://en.wikipedia.org/wiki/Concurrent_computing) pela memoria o processo usa da memoria para realizar a comunicação entre as threads.

Beleza, e como fazemos isso?

No moduloe `mpsc`, conseguimos criar um canal de comunicação com a função `channel`, esta função nos devolve duas coisas, um [Sender](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html) e um [Receiver](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html), onde o nosso sender pode ser clonado, tendo mais de um produtor, sendo assim temos, multiplos produtores e um consumidor, dai que vem o nome do modulo `mpsc - Multi-producer, single-consumer`. A função channel é uma função genérica, então depende de passarmos um tipo para esse parametro generico para ela.

```rust
use std::sync::mpsc::{channel, Sender, Receiver};

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel::<String>();
}
```

É comum encontrar em vários lugares os nomes `tx` e `rx` para os respectivos `Sender` e `Receiver`. Agora que criamos os canais, vamos clonar o `Sender` e criar uma thread usando ele.

```rust
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{sleep, spawn},
    time::Duration,
};

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel::<String>();

    let tx2 = tx.clone();

    spawn(move || {
        loop {
            let _ = tx2.send("hello from thread 1".to_string()); 
            sleep(Duration::from_millis(500));
        }
    });

}
```

O método `send` nos devolve um `Result<(), SendError>`, sendo que só é possivel acontecer o caso de erro caso o `Receiver` esteja fechado. Beleza, agora enviamos uma mensagem através do canal, e como consumimos ela? O `Receiver` tem um método chamado `recv` onde através dele, conseguimos ler todas as mensagens enviadas nesse canal, porém, só conseguimos ler uma mensagem por vez. 