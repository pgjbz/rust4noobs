# Arrays

# Arrays

Chegou o momento daquele assunto que é um dos terrores para quem esta começando a programar, os [arrays](https://en.wikipedia.org/wiki/Array_data_structure).

Array é uma estrutura de dados sequencial que armazena o mesmo tipo de dados, seriam como uma sequência de células de memória indexadas. Geralmente iniciamos sua contagem a partir do número 0.

 <h1 align="center"><img src="../assets/arrays.svg" alt="arrays" width="100%"></h1>

Em Rust declaramos arrays da seguinte maneira `let nome: [tipo; tamanho] = [valor; tamanho]`, Rust nos obriga a inicializar o array, então faremos da seguinte maneira.

```rust
fn main() {
    let array: [u8; 7] = [0; 7];
}
```

Deste modo temos um array de 7 posições preenchido com o valor 0, para acessarmos valores específicos dentro do array utilizamos os colchetes `[posicao]`, para modificarmos qualquer valor dentro do array também precisamos do uso da palavra `mut`.

Em um array temos o método `len` onde conseguimos saber o tamanho do array, este método é muito util para realizar um [loop for](./07-loops.md) para percorrer o array.

Iremos realizar uma simples operação com um array, teremos um array de 7 posições e iremos percorrer esse array e daremos o valor para cada posição com a seguinte regra: posição + 10, e logo depois iremos imprimir no console este array.

```rust
fn main() {
    let mut array: [u8; 7] = [0; 7];
    for i in 0..array.len() {
        array[i] = i as u8 + 10u8;
    }

    for i in 0..array.len() {
        println!("Pos: {}, val: {}", i, array[i]);
    }
}
```

O uso da palavra `as` será discutido depois, após a execução do código acima temos o resultado:

```bash
Pos: 0, val: 10
Pos: 1, val: 11
Pos: 2, val: 12
Pos: 3, val: 13
Pos: 4, val: 14
Pos: 5, val: 15
Pos: 6, val: 16
```

Também temos outro modo de executar este loop para realizar o print dos valores:

```rust
fn main() {
    let mut array: [u8; 7] = [0; 7];
    for i in 0..array.len() {
        array[i] = i as u8 + 10u8;
    }

    for val in array {
        println!("Val: {}", val);
    }
}
```

Do modo em que fizemos perdemos a informação do índice que estamos percorrendo, mas temos o seguinte resultado:

```bash
Val: 10
Val: 11
Val: 12
Val: 13
Val: 14
Val: 15
Val: 16
```

Para termos o índice de volta podemos fazer da seguinte maneira:

```rust
fn main() {
    let mut array: [u8; 7] = [0; 7];
    for i in 0..array.len() {
        array[i] = i as u8 + 10u8;
    }

    for (i, val) in array.iter().enumerate() {
        println!("pos: {}, val: {}", i, val);
    }
}
```

Assim temos a saída:

```bash
pos: 0, val: 10
pos: 1, val: 11
pos: 2, val: 12
pos: 3, val: 13
pos: 4, val: 14
pos: 5, val: 15
pos: 6, val: 16
```

- [Próximo](./10-exercises.md) - Exercícios