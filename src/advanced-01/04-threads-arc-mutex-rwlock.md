# Arc\<T>

O `Smart Pointer` Arc\<T> é um ponteiro com referencia contada assim como o [Rc\<T>](./02-smart-pointers-rc.md) como grande diferencial de ser seguro para ser usado com `threads`, mas o que faz ele ser seguro para essa finalidade e o Rc\<T> não? O que mais influencia neste ponto é o modo em que o seu contador é implementado, para o Rc\<T> o contador é feito de uma maneira mais simplificada, sem nenhum tipo de preocupação com sobre escrita decorrente a leitura e escritas em momentos simultâneos, já o Arc\<T> se preocupa com este tipo de operação e para isso utiliza [operações atômicas](https://pt.wikipedia.org/wiki/Transa%C3%A7%C3%A3o_at%C3%B4mica).

O modo de uso do Arc\<T> e do Rc\<T> não se distanciam muito. Temos `new`, `clone`, `downgrade`, etc. Um ponto que devemos ficar atentos é que o `Weak\<T>` retornado pelo `downgrade` de um `Arc\<T>` é de outro módulo.

```rust
use std::{
    sync::{Arc, Weak},
    thread,
};

fn main() {
    let a = Arc::new(10);
    let arc_para_t1 = Arc::clone(&a);
    let handle1 = thread::spawn(move || {
        println!("lendo arc da t1: {}", arc_para_t1);
        println!(
            "Estou na t1 e Arc tem {} referencias fortes",
            Arc::strong_count(&arc_para_t1)
        );
    });
    let arc_para_t2 = Arc::clone(&a);
    let handle2 = thread::spawn(move || {
        println!("lendo arc da t2: {}", arc_para_t2);
        println!(
            "Estou na t2 e Arc tem {} referencias fortes",
            Arc::strong_count(&arc_para_t2)
        );
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!(
        "Estou na thread main e Arc tem {} referencias fortes",
        Arc::strong_count(&a)
    );
}
```

Ao executar o código acima, teremos saídas diferentes a cada momento da execução. Sendo possível até mesmo a impressão da seguinte maneira:

```sh
lendo arc da t2: 10
lendo arc da t1: 10
Estou na t1 e Arc tem 3 referencias fortes
Estou na t2 e Arc tem 3 referencias fortes
Estou na thread main e Arc tem 1 referencias fortes
```

Note que não tivemos problemas na diminuição de referencias fortes mesmo após encerrar as duas `threads`.

# Mutex\<T>

O `Mutex<T>` assim como a `RefCell\<T>` contém mutabilidade interior, é possível realizar o empréstimo mutável a partir de referencias imutáveis, porém a sua peculiaridade é que para isso, o valor fica BLOQUEADO, mas como assim?
Simples quando vamos acessar o valor dentro de um `Mutex<T>`, ele realiza um bloqueio deste valor, então outra thread que tentar acessar este mesmo endereço de memoria fica bloqueado até o momento em que a thread que realizou o bloqueio o liberar. Quando realizamos o bloqueio nos é retornado um `Result<MutexGuard<T>, PoisonError<...>>`.
Este `MutexGuard<T>` que é o responsável por liberar o bloqueio dos dados.
```rust
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(10);
    let mutex_guard = mutex.lock().unwrap();
    println!("mutex bloqueado: {:#?}", mutex);
    drop(mutex_guard);
    println!("mutex desbloqueado: {:#?}", mutex);
}
```

Repare no exemplo acima, quando imprimimos as informações de debug do `Mutex<T>`, quando esta bloqueado, o valor do campo `data` aparece como `<locked>`, ou seja, o valor esta bloqueado até o `mutex_guard` ser liberado. No exemplo acima utilizamos do artificio do `drop`.

Assim como `RefCell<T>` podemos realizar o bloqueio de maneira mutável.

```rust
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(10);
    let mutex_guard = mutex.lock();
    println!("mutex bloqueado: {:#?}", mutex);
    drop(mutex_guard);
    {
        let mut valor = mutex.lock().unwrap();
        *valor = 69;
    }
    println!("mutex desbloqueado: {:#?}", mutex);
}
```

## Retomando

Lembra do exemplo inicial, que não conseguimos fazer a compilação dele, nem fazer com que o comportamento fosse o desejado? Agora que conhecemos `Arc<T>` e `Mutex<T>`, conseguimos alterar aquele código para obter sucesso.

```rust
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let a = Arc::new(Mutex::new(10));
    let t2 = Arc::clone(&a);
    thread::spawn(move || {
        *t2.lock().unwrap() = 42;
    })
    .join()
    .unwrap();
    thread::spawn(move || {
        println!("a = {}", a.lock().unwrap());
    })
    .join()
    .unwrap();
}
```

Agora o nosso projeto compila e roda da maneira correta, conseguimos compartilhar a mesma região de memoria em diversas threads diferentes, único ponto é que necessitamos de um bloqueio temporário na região de memoria. 
Em aplicações reais, não teremos casos tão simples assim, como uma thread esperando outra para iniciar, varias threads podem estar rodando ao mesmo tempo e acessando a mesma região de memória. Felizmente Rust é uma linguagem segura para uso em multi-thread e já nos prove muitos recursos para nos auxiliar nessa jornadas de códigos assíncronos.

## RwLock\<T>

O `RwLock<T>` é um ponteiro com o comportamento parecido com o `Mutex<T>`, usamos ele quando precisamos ler um valor a partir de várias threads e apenas uma delas pode realizar operação de escrita por vez.

```rust
use std::{
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

fn main() {
    let valor = Arc::new(RwLock::new(1));
    let trocas = Arc::clone(&valor);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        println!("thread de trocas mudando o valor para valor + 1 e esperando mais 5 segundos");
        let mut write_lock = trocas.write().unwrap();
        *write_lock += 1;
        thread::sleep(Duration::from_secs(5));
    });
    let mut handles = vec![];

    for i in 1..10 {
        let v = Arc::clone(&valor);
        let handle = thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));
            let read = v.read().unwrap();
            println!("thread {} lendo o valor = {}", i + 1, read);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
```

No exemplo acima, temos uma `thread` que a cada 5 segundos realiza um bloqueio de escrita, espera mais 5 segundos e libera o bloqueio, e também temos outras 10 `threads` que a cada 1 segundo realizam a leitura do valor, repare que o único momento em que as 10 threads pausam é o momento em que a thread de escrita realiza o bloqueio e espera por 5 segundos para liberar este bloqueio. Assim que o bloqueio é liberado as operações de leitura acontecem normalmente.