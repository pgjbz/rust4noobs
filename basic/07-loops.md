# Loops

Caso queiramos repetir alguma instrução, várias e várias vezes, o que fazemos? Escrevemos o mesmo código inúmeras vezes? Claro que não, para fazermos isso utilizamos loops.

Em Rust temos 3 tipos de loops `for`, `while` e `loop`, todos de fácil utilização, todos com suas peculiaridades, mas no fim utilizamos para a mesma coisa, repetir coisas. 

## Loop FOR

O loop `for`, provavelmente é o mais utilizado, não necessariamente do modo que iremos aprender agora, mas isso vem depois, agora iremos focar no básico de sua utilização.

Sua declaração é feita da seguinte maneira `for variavel in de..até`

```rust
fn main() {
    for i in 0..10 {
        println!("{}", i);
    }
}
```

Agora temos um loop for que vai de 0 até 9, pode não ser muito intuitivo logo de início, lendo o código parece que iria ir de 0 até 10, todavia sempre sera `valor até - 1`

## Loop WHILE

O loop `while` é uma estrutura de repetição que se repete por tempo indeterminado, diferente do loop `for` ela ira se repetir infinitamente enquanto a condição for verdadeira.

Sua declaração é `while condicao`

```rust
fn main() {
    let mut i = 0;
    while i <= 10 {
        println!("{}", i);
        i += 1;
    }
}
```

Agora temos um loop `while`que ira repetir enquanto i for menor ou igual a 10, a partir do momento que esta condição não for satisfeita, o loop ira ser encerrado. 

Em alguns momentos queremos loops infinitos, o que você faria? Utilizaria um `while true`? Não é necessário no Rust temos...

## Loop "loop"

Quando queremos ter um loop infinito podemos utilizar a palavra reservada loop, esta palavra cria um bloco que se repete infinitamente

```rust
fn main() {
    loop {
        println!("Rust4Noobs");
    }
}
```

Agora temos um bloco que irá escrever "Rust4Noobs" infinitamente.

## Palavra break

Nem sempre queremos que um loop execute completamente antes de encerrar, para isso temos a palavra `break` ela tem a função de parar uma estrutura de repetição. Serve tanto para blocos for, while e loop

```rust
fn main() {
    for i in 1..100 {
        println!("{}", i);
        if i % 3 == 0 && i % 9 == 0 {
            println!("Parou!");
            break;
        }
    }

    let mut i = 0;
    while i <= 100 {
        println!("{}", i);
        if i == 10 {
            println!("Parou!");
            break;
        }
        i+= 1;

    }

    i = 0;

    loop {
        println!("{}", i);
        if i == 10 {
            println!("Parou!");
            break;
        }
        i+= 1;
    }
}
```

