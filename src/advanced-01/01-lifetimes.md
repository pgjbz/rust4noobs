# Lifetimes

Lifetimes, a os lifetimes, eles são parte essencial do código Rust, com os lifetimes o [borrow checker](https://rustc-dev-guide.rust-lang.org/borrow_check.html) do Rust consegue saber quando liberar a memoria de algo, por quanto tempo algo fica "vivo".

É comum associarmos o lifetime com o escopo, e não esta errado, mas também não esta completamente correto.

Por exemplo no código abaixo o lifetime respeita completamente o escopo.

```rust
fn main() {
    {
        let lifetime = "lifetime";
        println!("{}", lifetime);
    } 
    /*
        Terminou o escopo do bloco acima, a memoria é liberada,
        fim do lifetime.
    */
}
```

Outro ponto importante que devemos levar em consideração é que o dono do recurso é o responsável por devolver a memoria ao SO. No código abaixo vemos isso:

```rust

fn escreva(texto: String) {
    /*
        Esta função é a responsável por devolver ao sistema operacional
        Todo a memoria solicitada.
        Quando ela adquiri o ownership da String ela tem a responsabilidade de
        liberar a memoria, vimos isso mais no inicio deste livro,
        até aqui nenhuma novidade.
    */
    println!("{}", texto);
}

fn main() {
    escreva("Rust4noobs".to_string());
}
```

## Lifetimes genéricos

Como assim lifetimes genéricos?

Lembra do capitulo sobre [generics](../intermediary-02/05-generics.md), funciona quase da mesma maneira, temos a seguinte `struct`

```rust
struct Life {
    string_slice: &str
}
```

Notem que temos uma referencia dentro da struct, isso é um problema, o compilador do Rust não sabe até quando essa referencia vai viver, e é neste ponto que o lifetime começa a deixar de ser apenas relacionado ao escopo.

Ao tentar compilar o código acima, temos o seguinte erro.

```sh
error[E0106]: missing lifetime specifier
 --> test.rs:3:23
  |
3 |         string_slice: &str
  |                       ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
2 ~     struct Life<'a> {
3 ~         string_slice: &'a str
  |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.
```

Nós precisamos informar o lifetime deste `string slice`, como sugere o compilador, note que a sintaxe é muito parecida com a dos `generics`.

```rust
struct Life<'a> {
    string_slice: &'a str
}
```

E agora vemos que nosso código compila. Claro não perdemos os poderes dos `generics` fazendo isso e não nos limitamos apenas as `structs`.

```rust
struct Life<'a, T> {
    data: &'a T
}

impl<'a, T> Life<'a, T> {
    fn new(data: &'a T) -> Self {
        Self {
            data
        }
    }
}
```

No exemplo acima adicionei os `generics` e o lifetime, juntos e também em uma implementação. A lógica para fazer isto em um método é a mesma.

### Menor lifetime

Uma coisa que devemos levar em consideração é que, quando tivermos duas referencias o compilador do rust sempre vai considerar o menor lifetime, por exemplo.

```rust
fn main() {
    let a = "abc";
    {
        let b = "a";
        let maior = maior(a, b);
        println!("{}", maior);
    }
}

fn maior<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

Neste exemplo o compilador do Rust assume o lifetime da variável `b` para o lifetime da função. Outro ponto que devemos levar em consideração é que podem existir mais de um lifetime, se pegarmos este mesmo código acima, adicionarmos um segundo lifetime e usarmos o mesmo para o parâmetro `b` para outro nome de lifetime, estaremos informando que esta função pode ter dois lifetimes possíveis.

```rust
fn main() {
    let a = "abc";
    {
        let b = "a";
        let maior = maior(a, b);
        println!("{}", maior);
    }
}

fn maior<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

Ao tentarmos compilar o código acima teremos o seguinte erro

```sh
error[E0623]: lifetime mismatch
  --> test.rs:14:9
   |
10 | fn maior<'a, 'b>(a: &'a str, b: &'b str) -> &'a str {
   |                                 -------     -------
   |                                 |
   |                                 this parameter and the return type are declared with different lifetimes...
...
14 |         b
   |         ^ ...but data from `b` is returned here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.

```

Vamos entender um pouco melhor o que acontece ali. Temos dois lifetimes, então o compilador entende que podemos ter duas referencias que vivem tempos diferentes, porém não é possível retornar a variável `b`, pois não temos garantia que o parâmetro `b`ira viver mais ou menos que o parâmetro `a`, então não podemos ter ele como retorno, a função do modo que ficou agora somente pode retornar o parâmetro `a`.

Mas qual seria a utilidade disso? Basicamente, você consegue utilizar essa variação de lifetimes para processar dados que vão viver tempos diferentes, como por exemplo um valor que vai viver apenas dentro daquela função como já foi explicado no capítulo de [slices](../intermediary-01/05-slices.md), caso um resultado de uma função dependa de dois lifetimes em uma struct por exemplo, podemos informar ao compilador utilizando mais de um parâmetro de lifetime. No livro [Rust for Rustaceans](https://nostarch.com/rust-rustaceans) podemos encontrar um exemplo bem interessante sobre este assunto.

