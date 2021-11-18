# Funções

Em Rust já vimos a função `main` a função responsável por ser o ponto de partida da nossa aplicação, porém não é muito legal realizarmos todas as operações dentro desta única função, porque além de causar a repetição de código ainda temos um grande problema, ela ficará <strong>GIGANTESCA</strong>, para resolver este problema podemos criar funções menores, que fazem pequenas coisas. 

## Funções sem argumento

Temos funções com e sem argumentos, com e sem retorno, agora iremos falar das sem argumentos e com ou sem retorno.

A declaração de uma função em Rust é simples, utilizamos o seguinte padrão `fn nome()` ou `fn nome() -> tipo retorno`

```rust
fn quanto_e_um_mais_dois() -> u8 {
    3
}

fn escreve_hello_world_dez_vezes() {
    for i in 0..10 {
        println!("Hello World!");
    }
}
```

## Retorno de funções

Temos dois modos de realizar o retorno de uma função em Rust, um deles é o retorno sendo a última linha do bloco da função sem a palavra `return` e sem o `;` o segundo modo é utilizarmos a palavra `return` propriamente dita.

```rust
fn retorno_implicito() -> bool {
    true
}

fn retorno_explicito() -> u8 {
    if 10 > 1 {
        return 200; //a palavra return encerra a função e retorna o valor
    }
    1 //retorno implicito na mesma função
}
```

## Funções com parâmetros

Temos também podemos passar argumentos nas funções, para isso utilizamos o seguinte padrão de assinatura `fn nome_funcao(parametros com seus tipos)` ou `fn nome_funcao(parametros com seus tipos) -> tipo retorno`.

Por exemplo, precisamos de um programa que faça o cálculo de impostos para uma nota fiscal e no fim deste cálculo nos exiba no console o valor destes impostos.

O modo mais ingenuo de fazer isso com o que aprendemos até agora seria da seguinte maneira:

```rust
fn main() {
    let valor = 100.0;
    let icms = valor * 0.01;
    let iss = valor * 0.10;
    println!("Icms: {}", icms);
    println!("Iss: {}", iss);
}
```

Porém, temos um problema aí, e se eu quiser calcular vários valores diferentes, eu iria repetir este o bloco do cálculo? Não, eu posso extrair este calculo para uma função, a mesma coisa para exibir no console. Ficando da seguinte maneira:

```rust
fn main() {
    let valor = 100.0;
    let icms = calcula_icms(valor);
    let iss = calcula_iss(valor);
    escreve_icms(icms);
    escreve_iss(iss);
}

fn calcula_icms(valor: f32) -> f32 {
    valor * 0.01
}

fn calcula_iss(valor: f32) -> f32 {
    valor * 0.10
}

fn escreve_icms(icms: f32) {
    println!("Icms: {}", icms);
}

fn escreve_iss(iss: f32) { 
    println!("Iss: {}", iss);
}
```

A primeira instância parece apenas que escrevemos mais, porém com os nomes expressivos conseguimos saber exatamente o que esta acontecendo e podemos chamar estes blocos de códigos de diversos lugares.

As funções são algo muito útil, mas em Rust temos que ter um certo cuidado com elas, mas este cuidado iremos ver na parte intermediaria deste 4Noobs.