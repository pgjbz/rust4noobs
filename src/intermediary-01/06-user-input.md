# Entrada de dados

Agora que conhecemos as [Strings](./01-strings.md), sabemos sobre os ponteiros, sistema de ownership podemos falar sobre a entrada de dados pelo terminal em nossa aplicação. Isso não foi passado antes, pois precisávamos entender alguns conceitos antes de utilizarmos o input de dados.

Para aceitarmos os dados a partir do console, usamos o seguinte código

```rust
fn main() {
    let mut string = String::new();
    println!("Entre com o seu texto: ");
    std::io::stdin().read_line(&mut string).unwrap();
    println!("Voce digitou {}", string);
}
```

Da biblioteca padrão do Rust utilizamos o `stdin`, ou seja, a entrada padrão de dados, precisamos de um [buffer](https://en.wikipedia.org/wiki/Data_buffer), neste caso utilizamos uma `String` mutável e vazia, para isso utilizamos o método `new` e passamos esse `buffer` como uma referencia mutável para o método `read_line`, o método `unwrap` ira fazer a nossa aplicação parar caso a leitura da entrada padrão falhe.

## Parse

Agora que sabemos como ler uma entrada do usuário, vamos aprender como transformar este texto em um tipo inteiro, por exemplo.

```rust
fn main() {
    let mut string = String::new();
    println!("Entre com um número: ");
    std::io::stdin().read_line(&mut string).unwrap();
    let numero = string.trim().parse::<i32>().unwrap();
    if numero >= 10 {
        println!("Seu número é maior ou igual a 10");
    } else {
        println!("Seu número é menor que 10");
    }
}
```

Lembra do método `trim` sem ele não iramos conseguir fazer esta conversão. O método `parse` é o responsável por converter de um texto para um tipo inteiro, ou um tipo com ponto flutuante. O tipo entre o sinal de menor e maior é o tipo que iremos fazer o parse.

Não iremos nos aprofundar tanto neste ponto, mostrando exemplos com todos os tipos de numéricos, mas iremos demonstrar essa conversão com uma inferência de tipo.

```rust
fn main() {
    let mut string = String::new();
    println!("Entre com um número: ");
    std::io::stdin().read_line(&mut string).unwrap();
    let numero = string.trim().parse().unwrap();
    numero_maior_igual_a_dez(numero);
}

fn numero_maior_igual_a_dez(numero: i32) {
    if numero >= 10 {
        println!("Seu número é maior ou igual a 10");
    } else {
        println!("Seu número é menor que 10");
    }
}
```

O compilador do Rust é inteligente o suficiente para saber que estamos fazendo um parse para o tipo `i32`, por estarmos chamando a função `numero_maior_igual_a_dez` que recebe um `i32` por parâmetro, e este parâmetro é a variável que ira receber o valor do parse.

### Casting

Se lembra da parte sobre [arrays](../basic/09-arrays.md) quando utilizamos o seguinte trecho de código

```rust
...
    let mut array: [u8; 7] = [0; 7];
    for i in 0..array.len() {
        array[i] = i as u8 + 10u8;
    }
...
```

Aquela palavra `as` foi a responsável em transformar um tipo `usize` em um tipo `u8`, chamamos este tipo de operação de [casting](https://en.wikipedia.org/wiki/Type_conversion), nem sempre este tipo de operação funciona, como nosso exemplo era um número bem pequeno conseguimos converter para um tipo `u8`, mas se o valor fosse maior que 255 esta conversão iria falhar, pois ocupara mais do que 1 byte. Então tome cuidado ao utilizar este tipo de operação, exemplo de código que ira falhar, nos dando um resultado não esperado.

```rust
fn main() {
    let a: usize = 10000;
    let b: u8 = a as u8;
    println!("Valor convertido: {}", b);
}
```
Lembrando este exemplo é apenas para tipos primitivos, futuramente iremos aprender outros tipos e iremos ver que nem sempre é possível realizar esta operação.

- [Próximo](./07-exercises.md) - Exercicios