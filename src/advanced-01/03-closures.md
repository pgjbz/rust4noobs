# Closures

Closures em Rust são as funções anônimas, ou seja, funções que podem ou não ter algum nome. Vamos começar com exemplos mais simples e fáceis.

```rust
fn main() {
    let somar = |x, y| x + y;
    let valor = somar(1, 2);
    println!("{}", valor);
}
```

O código acima tem como saída:

```sh
3
```


Os tipos para uma closure podem ser diversos, caso quiséssemos declarar o tipo da `closure` faríamos da seguinte maneira:

```rust
fn main() {
    let somar = |x: i32, y: i32| x + y;
    let valor = somar(1, 2);
    println!("{}", valor);
}
```

Precisamos desse tipo de anotação, pois nem sempre o compilador consegue distinguir o tipo dos argumentos.

### Funções como argumento

É possível passar essas closures como o argumento de outra função. No exemplo abaixo, temos uma lista de números, transformamos em um iterador e chamamos o método `for_each` passando como argumento a função anônima que escreve no terminal o valor de 'x'.

```rust
fn main() {
    let numeros = vec![1, 2, 3, 4];
    numeros.iter().for_each(|x| println!("{}", x));
}
```

Podemos aproveitar do exemplo anterior e utilizar outra função, como o map para dar como saída outra coleção e fazer chamadas encadeadas. 

```rust
fn main() {
    let numeros = vec![1, 2, 3, 4];
    numeros
        .iter()
        .map(|x| {
            let mut valor = *x;
            if valor % 2 == 0 {
                valor += 10;
            } else {
                valor *= 10
            }
            valor
        })
        .for_each(|x| println!("{x}"))
}
```

As closures são bem poderosas e podemos utilizar elas de forma a ganhar flexibilidade em nossos códigos.

## Move?

Para este exemplo vamos implementar a sequência de Fibonacci, para isso vamos escrever uma função que retorna outra função. Espera... Uma função que retorna outra? Isso mesmo

```rust
fn fibonacci() -> impl FnMut() -> usize {
    let mut a = 0;
    let mut b = 1;
    return move || {
        a = b;
        b = a + b;
        return b - a;
    };
}

fn main() {
    let mut f = fibonacci();

    for i in 0..10 {
        println!("{}", f());
    }
}

```

O código acima ira nos devolver os 10 primeiros números da sequência de Fibonacci, aqui temos dois assuntos interessantes para tratar, o primeiro a palavra "impl" no retorno, o segundo a trait "FnMut" e uma segunda declaração do tipo de retorno? Como assim?

- impl: assim como a palavra reservada 'dyn', existe a palavra reservada 'impl' que pode ser usada para representar algum tipo dinâmico alocado na `stack` ao invés da `heap`.
- FnMut: `FnMut` assim como `Fn` e `FnOnce`, são traits que podem ser implementadas por de Closures, sendo elas, `FnMut` com valores mutáveis, `Fn` com valores imutáveis e `FnOnce` com valores imutáveis, sendo possível apenas uma única chamada.
- Segunda declaração de retorno: essa segunda declaração de retorno é nada mais nada menos que a declaração do retorno dessa `FnMut`, que também pode ser receber parâmetros como, por exemplo: `FnMut(i32) -> i32`, com argumentos na trait separados por virgula, `FnMut(Box<Foo>, i32) -> Bar`;

No corpo de nossa função temos uma palavrinha nova ali que á o `move`, esta palavra reservada indica que as variáveis 'a' e 'b', serão movidas para dentro do contexto da Closure.