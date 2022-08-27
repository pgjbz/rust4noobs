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
Simples quando vamos acessar o valor dentro de um `Mutex<T>`, ele realiza um bloqueio deste valor, então outra thread que tentar acessar este mesmo endereço de memoria fica bloqueado até o momento em que a thread que realizou o bloqueio o liberar.