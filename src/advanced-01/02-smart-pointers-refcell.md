# RefCell\<T>

Sinceramente, quando comecei a escrever este tópico sobre os `Smart Pointers` eu achei que seria mais fácil explicar sobre este carinha aqui. Porém, para estar mais enganado, somente se eu falar que Java é lento.

RefCell\<T> é como uma caixa de pandora, você precisa de muito cuidado e muita certeza do que esta fazendo quando o utiliza, este `Smart Pointer` tem checagem de "empréstimo"  em tempo de execução. Esta checagem obedece às mesmas regras de [ownership](../intermediary-01/03-ownership.md) que já conhecemos:

- referências imutáveis = "ilimitadas"
- referências mutáveis = exclusivas

O código abaixo, não tem problemas de compilação.


```rust
use std::cell::RefCell;

fn main() {
    let referência = RefCell::new(42);
    let referência_imutavel = referência.borrow();
    println!("{}", referência_imutavel);
    {
        let outra_imutavel = referência.borrow();
        println!("{}", outra_imutavel);
    }
    let mut referência_mutavel = referência.borrow_mut();
    *referência_mutavel = 69;
    println!("{}", referência_mutavel);
}
```

Podemos pensar "se compilou esta certo", porém temos um pequeno problema na hora de executar este programa.

```sh
42
thread 'main' panicked at 'already borrowed: BorrowMutError', ref-cell.rs:7:45
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

E é aqui que as coisas começam a complicar, vamos no famoso `baby step`, com calma, uma coisa e cada vez. Eu sei que pode ser confuso a primeira vista o programa compilar, sem problemas de `ownership` e na hora da execução esses problemas estourarem.

Os dados contidos dentro desse ponteiro são expostos de maneiras específicas, sendo possível expor de maneira imutável ou mutável. Tente pensar como se fosse uma caixa fechada com um conteúdo dentro dela. Esta caixa pode deixar várias pessoas verem o que tem dentro dela através de um furo, por exemplo, porém quando este conteúdo vai ser modificado este furo é tampado, não sendo mais possível que mais de uma pessoa visualize o que tem dentro desta caixa, até que esta mudança termine de ser feita.

## Mutabilidade interior

"Em Rust alguns tipos precisam ser alterados enquanto compartilhados". - https://doc.rust-lang.org/reference/interior-mutability.html 

Isso soa meio confuso, mas pense um tipo tem uma mutabilidade interior, quando o seu conteúdo pode ser modificado por meio de uma referência compartilhada por ele mesmo. Isto vai contra muitas coisas que aprendemos até agora. Foi o que aconteceu no nosso exemplo de `RefCell\<T>`, claro este não é o único tipo a implementar este tipo de comportamento, temos também `Cell`. Podemos criar o nosso próprio tipo usando `UnsafeCell`.

## Retomando

Respeitando as regras de `ownership` podemos resolver os problemas do código mostrado.

- Não devemos permitir mais de uma referência, quando temos uma referência mutável.
- referências mutáveis, são "infinitas", desde que não tenhamos referencias mutáveis.

```rust
use std::cell::RefCell;

fn main() {
    let referência = RefCell::new(42);
    
    let referência_imutavel = referência.borrow();
    
    println!("{}", referência_imutavel);
    {
        let outra_imutavel = referência.borrow();
        println!("{}", outra_imutavel);
    }
    drop(referência_imutavel); 
    let mut referência_mutavel = referência.borrow_mut();
    *referência_mutavel = 69;
    
    println!("{}", referência_mutavel);
}
```

Podemos corrigir o problema com algo bem interessante, no Rust temos um modo dizer "olha, pode devolver isso daqui pro sistema operacional", um método chamado `drop`, podemos utilizar ele para dizer que a referência imutável não sera mais utilizada, sendo assim podemos fazer um `borrow_mut`, naquele valor. 


## Múltiplas referências mutáveis.

Até agora aprendemos sobre o `Rc<T>` e o `RefCell<T>`, e claro podemos combinar os dois para criar múltiplas referências de algo que também podem ser mutáveis, vamos ao exemplo:

```rust
use std::{
    cell::RefCell,
    rc::Rc,
};

#[derive(Debug)]
struct NomeGenerico {
    criatividade: Option<Rc<RefCell<NomeGenerico>>>,
    valor: usize
}

fn main() {
    let nome_generico = Rc::new(RefCell::new(NomeGenerico {
        criatividade: None,
        valor: 10
    }));
    let outra_referência = Rc::clone(&nome_generico);
    let mut referência_mutavel = nome_generico.borrow_mut();
    referência_mutavel.valor = 20;
    println!("{:#?}", nome_generico);
    println!("{:#?}", outra_referência);
}
```

Ao compilar e executar o código acima teremos o seguinte output:

```sh
RefCell {
    value: <borrowed>,
}
RefCell {
    value: <borrowed>,
}
```

O valor do RefCell, teve um "borrow", ou seja, esta ali, mas não está. Mas, esse não é o ponto que queremos chegar, reparem que eu tive duas referências imutáveis, realizei um `borrow_mut` e não tive erro em tempo de execução? Perceba que "burlamos" este ponto que foi uma pequena dor de cabeça anteriormente. Note que não precisamos dar um `drop` em referências anteriores, etc. Bem legal né?

Agora temos que encarar outro problema, referência cíclica e vazamento de memória.

```rust
use std::{
    cell::RefCell,
    rc::Rc,
};

#[derive(Debug)]
struct NomeGenerico {
    criatividade: Option<Rc<RefCell<NomeGenerico>>>,
    valor: usize
}

fn main() {
    let nome_generico = Rc::new(RefCell::new(NomeGenerico {
        criatividade: None,
        valor: 10
    }));
    let outra_referência = Rc::clone(&nome_generico);
    {
        let mut referência_mutavel = nome_generico.borrow_mut();
        referência_mutavel.valor = 20;
        referência_mutavel.criatividade = Some(Rc::clone(&outra_referência));
    }
    println!("{:#?}", nome_generico);
    println!("{:#?}", outra_referência);
}
```

Ao compilar o código acima e executar teremos uma chamada recursiva até um [stack overflow](https://www.techtarget.com/whatis/definition/stack-overflow) acontecer. Claro você não vai ver esse tipo de código a todo momento.

O que pode causar o vazamento de memória é o modo em que são ligados as referências, podendo chegar a um ponto onde um `Rc<T>`, tenha o contador zerado e não exista mais referências a serem "dropadas".

## referência Fraca

Um modo de contornarmos o problema acima é ao invés de utilizar um `Rc<T>` que é uma referência forte, podemos usar um `Weak<T>`, a grande diferença é que um `Rc<T>` tem um contador de referências fortes e fracas, as referências fracas não são tão importantes assim, ou seja, caso o contador de referências fortes chegue a 0 e ainda existirem referências fracas, a memória é liberada mesmo assim, em contrapartida, se tentarmos acessar a memória dessa referência fraca podemos ter um problema de acesso à memória indefinida, ou seja, memória que não pertence aquele processo.

```rust
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct NomeGenerico {
    criatividade: Option<Weak<RefCell<NomeGenerico>>>,
    valor: usize
}

fn main() {
    let nome_generico = Rc::new(RefCell::new(NomeGenerico {
        criatividade: None,
        valor: 10
    }));
    let outra_referência = Rc::clone(&nome_generico);
    {
        let mut referência_mutavel = nome_generico.borrow_mut();
        referência_mutavel.valor = 20;
        referência_mutavel.criatividade = Some(Rc::downgrade(&outra_referência));
    }
    println!("referências fracas: {}", Rc::weak_count(&nome_generico));
    println!("referências fortes: {}", Rc::strong_count(&nome_generico));
    println!("{:#?}", nome_generico);
    println!("{:#?}", outra_referência);
}
```

Ao compilar e executar o código acima teremos a seguinte saída:

```sh
referências fracas: 1
referências fortes: 2
RefCell {
    value: NomeGenerico {
        criatividade: Some(
            (Weak),
        ),
        valor: 20,
    },
}
RefCell {
    value: NomeGenerico {
        criatividade: Some(
            (Weak),
        ),
        valor: 20,
    },
}

```