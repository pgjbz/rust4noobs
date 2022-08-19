# Ownership

Ownership é o método que Rust usa para o seu gerenciamento de memória, basicamente enquanto algum [escopo](https://en.wikipedia.org/wiki/Scope_(computer_science)) do código tem aquele pedaço de memória ele é da nossa aplicação, quando esse pedaço de memória sai deste escopo ela é devolvida para o [Sistema Operacional](https://simple.wikipedia.org/wiki/Operating_system).

Vamos entender um pouco melhor como funciona o escopo de uma variável.

```rust
fn main() {

    let a = 10;
    if a == 10 {
        let b = 1;
        /*
            Neste ponto do código as variáveis 'a' e 'b' existem
        */
    }
    /*
        a partir deste ponto não é possível acessar a variável 'b',
        pois ela esta fora de escopo
    */
    println!("Valor de b {}", b);
}
```

Ao tentar compilar o código acima temos o seguinte erro:

```bash
error[E0425]: cannot find value `b` in this scope
  --> main.rs:14:31
   |
14 |     println!("Valor de b {}", b);
   |                               ^ help: a local variable with a similar name exists: `a`

error: aborting due to previous error
```

Neste caso 'b' pertence a um escopo menor do que 'a', a partir do momento que sai do bloco `if` a variável 'b' desaparece.

E como isso funciona no sistema de ownership? Para explicar isso iremos utilizar o tipo que aprendemos no começo deste módulo, as [Strings](./01-strings.md).

```rust
fn main() {
    let mut meu_texto = String::from("Rust4Noobs");
    printa_string(meu_texto);
    /*
        Vamos tentar fazer outra coisa com essa String
    */
    meu_texto.push_str(", é legal!");
}

fn printa_string(string: String) {
    println!("{}", string);
}
```
Espera... Não compila, temos o seguinte erro:

```bash
error[E0382]: borrow of moved value: `meu_texto`
 --> main.rs:7:5
  |
2 |     let mut meu_texto = String::from("Rust4Noobs");
  |         ------------- move occurs because `meu_texto` has type `String`, which does not implement the `Copy` trait
3 |     printa_string(meu_texto);
  |                   --------- value moved here
...
7 |     meu_texto.push_str(", é legal!");
  |     ^^^^^^^^^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
```

Vamos entender o que aconteceu aqui.

O tipo `String` é um "tipo especial", ele sempre é passado por referência, nunca é feito uma cópia de seu valor, quando chamamos a função `printa_string` e em seu parâmetro passamos nossa String, a posse de sua memória é transferida para a função, quando a função termina a memória é devolvida para o Sistema Operacional.

```rust
fn main() {
    let mut meu_texto = String::from("Rust4Noobs");
    printa_string(meu_texto);
    /*
        Vamos tentar fazer outra coisa com essa String
    */
    meu_texto.push_str(", é legal!");
}

fn printa_string(string: String) {
    println!("{}", string);
} //a partir daqui a memoria foi devolvida para o sistema
```

Mas se ainda quisermos utilizar a variável `meu_texto`? Podemos fazer a função `printa_string` retornar a String passada por argumento e pegar o ownership de volta.

```rust
fn main() {
    let mut meu_texto = String::from("Rust4Noobs");
    meu_texto = printa_string(meu_texto);
    meu_texto.push_str(", é legal!");
    println!("{}", meu_texto);
}

fn printa_string(string: String) -> String {
    println!("{}", string);
    string
}
```

Agora o código ira compilar, todavia esta não é a única maneira de fazer isso, lembra dos [ponteiros](./02-pointers-intro.md)? Vamos utilizar aqui.

```rust
fn main() {
    let mut meu_texto = String::from("Rust4Noobs");
    printa_string(&meu_texto);
    meu_texto.push_str(", é legal!");
    println!("{}", meu_texto);
}

fn printa_string(string: &String) {
    println!("{}", string);
}
```

E também funcionou, por enquanto vamos tentar pensar da seguinte maneira, como eu utilizei o ponteiro, eu emprestei a esta função a variável, ela fez o que tinha que fazer e me devolveu. Não perdemos o ownership da variável.

Utilizando esta estratégia de [passagem por referência](https://en.wikipedia.org/wiki/Reference_(computer_science)) conseguimos utilizar algumas outras coisas como as referências mutáveis, modificar o valor sem perder o ownership

```rust
fn main() {
    let mut meu_texto = String::from("Rust4Noobs");
    printa_string(&meu_texto);
    adiciona_texto(&mut meu_texto);
    println!("{}", meu_texto);
}

fn printa_string(string: &String) {
    println!("{}", string);
}

fn adiciona_texto(string: &mut String) {
    string.push_str(", é legal!");
}
```

E temos sucesso novamente. 