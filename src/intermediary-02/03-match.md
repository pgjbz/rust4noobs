# Match

Em Rust não temos a estrutura de decisão [switch](https://en.wikipedia.org/wiki/Switch_statement), em seu lugar temos a expressão `match`, o seu comportamento é parecido, porém, com alguns recursos a mais. Vamos retomar aquele código dos [enums](./02-enums.md).

```rust
#[derive(Debug)]
enum Uf {
    Sp(String),
    Rj(String),
    Ce(String),
}

impl Uf {
    fn retorna_sp() -> Self {
        Self::Sp(String::from("São Paulo"))
    }

    fn quem_sou_eu(&self) {
        todo!()
    }
}

fn main() {
    let uf = Uf::retorna_sp();
    println!("{:?}", uf);
    println!("{:#?}", uf);
}
```

Ficamos de implementar o método `quem_sou_eu` de propósito, nele iremos utilizar a estrutura de decisão `match`. 

```rust
//--declaração do enum
impl Uf {
    //--outros métodos
    fn quem_sou_eu(&self) {
        match self {
            Sp(_) => println!("Eu sou São Paulo"),
        }
    }
}

fn main() {
    let uf = Uf::retorna_sp();
    uf.quem_sou_eu();
}
```

Caso tentarmos compilar o código, teremos o seguinte erro:

```bash
error[E0004]: non-exhaustive patterns: `&Rj(_)` and `&Ce(_)` not covered
  --> main.rs:14:15
   |
2  | / enum Uf {
3  | |     Sp(String),
4  | |     Rj(String),
   | |     -- not covered
5  | |     Ce(String),
   | |     -- not covered
6  | | }
   | |_- `Uf` defined here
...
14 |           match self {
   |                 ^^^^ patterns `&Rj(_)` and `&Ce(_)` not covered
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `&Uf`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0004`.
```

O porquê deste erro?

Rust nos obriga a cobrir todos os casos que podem acontecer. Para isso vamos fazer a seguinte alteração.

```rust
//--declaração do enum
impl Uf {
    //--outros métodos
    fn quem_sou_eu(&self) {
        match self {
            Uf::Sp(_) => println!("Eu sou São Paulo"),
            Uf::Rj(_) => println!("Eu sou Rio de Janeiro"),
            Uf::Ce(_) => println!("Eu sou Ceará"),
        }
    }
}

fn main() {
    let uf = Uf::retorna_sp();
    uf.quem_sou_eu();
}
```

Agora nosso código ira rodar, mas reparem que eu posso criar uma instância de `enum` com um valor e o código se repete nos 3 casos. Eu posso melhorar isso um pouco, realizando a troca do '_' por um nome, e também posso deixar todos no mesmo 'match' utilizando um pipe '|'.


```rust
//--declaração do enum
impl Uf {
    //--outros métodos
    fn quem_sou_eu(&self) {
        match self {
            Uf::Sp(nome) | Uf::Rj(nome) | Uf::Ce(nome) => println!("Eu sou {}", nome),
        }
    }
}

fn main() {
    let uf = Uf::retorna_sp();
    uf.quem_sou_eu();
}
```

E temos o mesmo resultado. Porém se tivermos várias e várias opções e eu não precisar capturar os valores e terem a mesma tratativa. Porém, caso quisermos utilizar o mesmo modo acima e tivermos enum com quantidade de valores diferentes ou incompatíveis precisamos ignorar todos os valores não correspondentes para isso usamos o '_'.

```rust
enum Repositorio {
    Este(String, u16),
    Outros(String, u16, u8),
}

fn main() {
    let url_rust4noobs = String::from("https://github.com/pgjbz/rust4noobs");
    let qtd_stars_atuais_rust4noobs: u16 = 22;
    let url_4noobs = String::from("https://github.com/he4rt/4noobs");
    let qtd_stars_atuais_4noobs: u16 = 1964;
    let qualquer_numero_so_para_diferenciar: u8 = 20;

    let rust4noobs = Repositorio::Este(url_rust4noobs, qtd_stars_atuais_rust4noobs);
    let _4noobs = Repositorio::Outros(url_4noobs, qtd_stars_atuais_4noobs, qualquer_numero_so_para_diferenciar);

    match rust4noobs {
        Repositorio::Este(url, stars) | Repositorio::Outros(url, stars, _) => println!("Repositorio {}, estrelas {}", url, stars),
    }
}
```

Claro podemos quebrar em mais 'match' para não perder essas informações.

```rust
enum Repositorio {
    Este(String, u16),
    Outros(String, u16, u8),
}

fn main() {
    let url_rust4noobs = String::from("https://github.com/pgjbz/rust4noobs");
    let qtd_stars_atuais_rust4noobs: u16 = 22;
    let url_4noobs = String::from("https://github.com/he4rt/4noobs");
    let qtd_stars_atuais_4noobs: u16 = 1964;
    let qualquer_numero_so_para_diferenciar: u8 = 20;

    let rust4noobs = Repositorio::Este(url_rust4noobs, qtd_stars_atuais_rust4noobs);
    let _4noobs = Repositorio::Outros(url_4noobs, qtd_stars_atuais_4noobs, qualquer_numero_so_para_diferenciar);

    match rust4noobs {
        Repositorio::Este(url, stars) => println!("Repositorio {}, estrelas {}", url, stars),
        Repositorio::Outros(url, stars, n) => println!("Repositorio {}, estrelas {}, numero aleatório para diferenciar {}", url, stars, n),
    }
}
```

E se eu tenho, por exemplo  um `u8` eu preciso realizar um 'match' para cada possibilidade? Não eu posso fazer um 'match' com o '_' novamente, seria como o 'default' do switch, mas lembre-se quando usamos o '_' ignoramos o valor.

```rust
fn main() {
    let numero: u8 = 10;
    match numero {
        1 => println!("Um"),
        2 => println!("Dois"),
        3 => println!("Três"),
        _ => println!("Qualquer outro numero")
    }
}
```

Com o código acima cobrimos todas as possibilidades possíveis e qualquer valor que não seja 1, 2 ou 3 teremos a mesma tratativa. 

A expressão match também pode ser utilizada comparar por uma faixa de valor. Utilizando a o 'match' da seguinte maneira 'inicio..=fim'

```rust
fn main() {
    let numero = 20;
    match numero {
        0..=9 => println!("Menor que 10"),
        _ => println!("Igual ou maior que 10")
    }
}
```

Ou podemos também utilizar o match para retornar algum valor de qualquer tipo. Mas todos os pontos de retornos tem que ser do mesmo tipo, ou algum tipo de expressão como `continue`, `return`, vamos usar o mesmo exemplo anterior, contudo retornando um boolean e armazenando em uma variável.

```rust
fn main() {
    let numero = 20;
    let menor_que_10 = match numero {
        0..=9 => true,
        _ => false
    };
    if menor_que_10 {
        println!("Menor que 10");
    } else {
        println!("Igual ou maior que 10");
    }
}
```

Match é uma estrutura bem poderosa, e seu uso é relativamente simples ~~por enquanto~~