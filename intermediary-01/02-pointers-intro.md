# Introdução a Ponteiros

Afinal, o que são [ponteiros](https://en.wikipedia.org/wiki/Pointer_(computer_programming))? Ponteiros seriam como váriaveis, porém elas armazenam outro tipo de informação, o endereço de memória de outro dado.

Fazendo uma analogia com o mundo real, a sua casa seria uma variável do tipo Casa, você consegue realizar ações e modifica-la como você deseja ~~desde que não, afete a integridade de sua moradia~~, mas você tem um conhecido que também pode modificar sua casa, porém ele não mora nela e tem o seu endereço, ele sabe como chegar a sua casa para fazer esta modificação, este seria um ponteiro.

Ta não foi um bom exemplo, mas creio que fique mais fácil de entender com um desenho.

 <h1 align="center"><img src="../assets/pointer.svg" alt="arrays" width="100%"></h1>

Resumidamente os ponteiros simplismente dizem ~~"olha eu sei onde você mora, fica esperto!"~~ "Eu conheço o endereço daquele cara ali".

## Ponteiros em Rust

Lembra que acabamos de ver as [strings](./01-strings.md), o tipo String é de certa forma um ponteiro, ele aponta para um endereço de memória localizado no [heap](https://blog.pantuza.com/artigos/heap-vs-stack), é um "tipo especial" de ponteiro, mas agora iremos utilizar exemplos mais fáceis de manipular.

### Referencia em Rust

Para representarmos ponteiros em Rust utilizamos o caractere `&`.


```rust
fn main() {
    let a: u8 = 10;
    let b: &u8 = &a;
    println!("Valor de a: {}\nvalor de a a partir de b:{}", a, b)
}
```

No exemplo acima a variável 'b' faz referência a variável 'a' do tipo `u8`. 

Podemos também ter referencias mutáveis, porém existem duas regras, a primeira é a variável referenciada também deve ser mutável, assim podemos alterar o valor de uma variável a partir de outra que a referência, mas para isso realizamos uma "desreferenciação" com o caractere `*`

```rust
fn main() {
    let mut a: u8 = 10;
    let b: &mut u8 = &mut a;
    *b = 20u8;
    println!("Valor de a: {}", a);
}
```

A outra regra é, só podemos ter uma única referência mutável para a variável:

```rust
fn main() {
    let mut a: u8 = 10;
    let b: &mut u8 = &mut a;
    let _c: &mut u8 = &mut a;
    *b = 20u8;
    println!("Valor de a: {}", a);
}
```

Ao tentar compilar o código acima temos o seguinte erro:

```bash
error[E0499]: cannot borrow `a` as mutable more than once at a time
 --> main.rs:4:23
  |
3 |     let b: &mut u8 = &mut a;
  |                      ------ first mutable borrow occurs here
4 |     let _c: &mut u8 = &mut a;
  |                       ^^^^^^ second mutable borrow occurs here
5 |     *b = 20u8;
  |     --------- first borrow later used here

error: aborting due to previous error
```

Com isso entramos no sistema de ownership do Rust, que daremos continuidade na próxima parte desde 4noobs.

- [Próximo](./03-ownership.md)