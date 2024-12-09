# Snake Game

Eu estava em dúvida em que projeto fazer neste ponto, foi difícil pensar em algo e, porque não  algo que eu nunca fiz?

Então vamos lá fazer o "jogo da cobrinha", vamos começar criando o projeto, usando o comando `cargo new snake-game`.

Teremos a estrutura padrão do projeto:

```bash
├── Cargo.toml
└── src
    └── main.rs
```

Vamos adicionar um arquivo chamado `lib.rs` na pasta `src`, este arquivo sera usado para declarar os nossos módulos. Em seguida criamos um arquivo chamado "ponto.rs" e nele iremos criar uma [struct](./01-structs.md) para as localizações no nosso jogo, vamos criar uma implementação a essa struct para facilitar a instânciação dessa `strutc`.

```rust
pub struct Ponto {
    pub x: usize,
    pub y: usize
}

impl Ponto {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
}
```

Para que este arquivo seja reconhecido no projeto, vamos adicionar no nosso arquivo `lib.rs` a seguinte linha `pub mod point;`. Note que tanto a `struct` quanto seus atributos e a implementação do método `new` estão com a palavra `pub`, que faz eles serem visíveis fora desse módulo.

Vamos printar o campo onde a cobrinha ira andar, e para testar vamos adicionar um ponto nesse tabuleiro.

```rust
fn main() {
    let ponto = Ponto::new(7, 7);
    let (x, y) = (15, 15);
    for x in 0..x {
        for y in 0..y {
            if ponto == (x, y) {
                print!("# ")
            }  else {
                print!("- ");   
            }
        }
        println!();
    }
}

```

Note que comparamos a nossa `struct` Ponto, com uma tupla de (x, y), para isso ser possível, precisamos implementar uma [trait](./06-traits.md) chamada [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) a implementação para isso é relativamente simples. A `trait` recebe um parâmetro [genérico](./05-generics.md) na implementação vamos falar que esse parâmetro genérico é uma `tupla (usize, usize)`. E a partir dai implementamos nossa comparação.

```rust
impl PartialEq<(usize, usize)> for Ponto {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}
```

Agora quando rodarmos o projeto com `cargo run`, teremos um tabuleiro no console com um ponto na posição (7, 7).

```bash
   Compiling snake-game v0.1.0 (/home/paulo.bezerra/workspace/ws-rust/rust4noobs/projects/snake-game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/snake-game`
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - # - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - -
```

Agora vamos criar a `struct` da nossa cobrinha, para isso vamos adicionar a cabeça - que é um ponto - e uma lista de pontos para o corpo. Criamos o arquivo "cobra.rs" e adicionamos o `pub mod cobra` no arquivo `lib.rs`, e no arquivo "cobra.rs" adicionamos a struct.

```rust
pub struct Cobra {
    pub cabeca: Ponto,
    pub corpo: Vec<Ponto>,
}

impl Default for Cobra {
    fn default() -> Self {
        Self { 
            cabeca: Ponto::new(7, 7), 
            corpo: vec![Ponto::new(6,7), Ponto::new(5,7)]
        }
    }
}
```

A implementação da `trait` [default](https://doc.rust-lang.org/std/default/trait.Default.html) serve para termos um valor padrão para a `struct`. Vamos separar a nossa função de desenhar o tabuleiro e vamos passar uma referência para a `struct` da `cobra`, então com base nos dados passados ali vamos desenhar a nossa cobra.

```rust
fn print_board(cobra: &Cobra) {
    let (x, y) = (15, 15);
    for y in 0..y {
        for x in 0..x {
            if cobra.cabeca == (x, y) {
                print!("0 ")
            } else if cobra.corpo.contains(&Ponto::new(x, y)) {
                print!("# ");   
            } else {
                print!("- ");   
            }
        }
        println!();
    }
}
```

Temos a função e agora é só chamar ela na nossa função `main`.

```rust
fn main() {
    print_board(&Cobra::default())
}
```

Após executar o comando `cargo run` temos o output:

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/snake-game`
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - # # 0 - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
- - - - - - - - - - - - - - - 
```

Agora temos a cabeça e o corpo, precismos começar a definir uma direção que a cobra irá seguir e movimentar o corpo da cobra.

Para isso criamos um enumarado de direções, seguimos o mesmo passo a passo, criamos um arquivo "direcao.rs" e adicionamos no arquivo `lib.rs` a declaração do módulo `pub mob direcao`.

Então adicionamos as 4 direções possíveis ao nosso enum.

```rust
#[derive(Clone, Copy)]
pub enum Direcao {
    Cima,
    Baixo,
    Direita,
    Esquerda,
}

impl Default for Direcao {
    fn default() -> Self {
        Self::Direita
    }
}
```

Criamos o enum já implementando a `trait default` para nos auxiliar, como o padrão de início da cobra sempre vai ser para a direita, colocamos o retorno do método o valor `Self::Direita`. Já derivamos as `traits`, `Clone` e `Copy` para não precisar passar esse enum como referência todas às vezes.

Agora na nossa `struct` da cobra, vamos adicionar o atributo da direção.

```rust
pub struct Cobra {
    pub cabeca: Ponto,
    pub corpo: Vec<Ponto>,
    direcao: Direcao
}

impl Default for Cobra {
    fn default() -> Self {
        Self { 
            cabeca: Ponto::new(7, 7), 
            corpo: vec![Ponto::new(6,7), Ponto::new(5,7)],
            direcao: Default::default() 
        }
    }
}
```

Agora temos um modo de saber para qual direção a cobra está andando.

Na nossa `struct Ponto` vamos adicionar a função para alterar o valor do ponto.

```rust

impl Point {
   ...

    pub fn alterar(&mut self, direcao: Direcao) {
        match Direcao {
            Direcao::Right => self.x += 1,
            Direcao::Left => self.x -= 1,
            Direcao::Up => self.y -= 1,
            Direcao::Down => self.y += 1,
        }
    }
}
```

Vamos aproveitar e adicionar testes unitários para o método de alterar:

```rust
#[cfg(test)]
mod ponto_tests {
    use super::*;

    #[test]
    fn alterar_para_cima() {
        let mut ponto = Ponto::new(1, 1);
        ponto.alterar(Direcao::Cima);
        assert_eq!(Ponto::new(1, 0), ponto);
    }

    #[test]
    fn alterar_para_baixo() {
        let mut ponto = Ponto::new(1, 1);
        ponto.alterar(Direcao::Baixo);
        assert_eq!(Ponto::new(1, 2), ponto);
    }
    
    #[test]
    fn alterar_para_direita() {
        let mut ponto = Ponto::new(1, 1);
        ponto.alterar(Direcao::Direita);
        assert_eq!(Ponto::new(2, 1), ponto);
    }


    #[test]
    fn alterar_para_esquerda() {
        let mut ponto = Ponto::new(1, 1);
        ponto.alterar(Direcao::Esquerda);
        assert_eq!(Ponto::new(0, 1), ponto);
    }

}
```

Agora iremos adicionar a lógica para a cobra se mover, precisaremos de um método para mover a cabeça que é quem vai definir se o movimento é valido, se vai bater na parede, se vamos alterar a direção e já vamos adicionar os testes que consiste em, encerrar o jogo caso bata na parede, validar a posição dos pontos após algum movimento, etc.

```rust
impl Cobra {
    pub fn passo(&mut self, tabuleiro: (usize, usize)) -> Result<(), &'static str> {
        let posicao_anterior_cabeca = self.cabeca;
        self.mover_cabeca(&tabuleiro)?;
        self.mover_corpo(posicao_anterior_cabeca);
        Ok(())
    }

    pub fn alterar_direcao(&mut self, direcao: Direcao) {
        self.direcao = direcao;
    }

    fn mover_cabeca(&mut self, board: &(usize, usize)) -> Result<(), &'static str> {
        match self.direcao {
            Direcao::Cima if self.cabeca.y == 0 => Err("fim de jogo, esbarrou na parede de cima"),
            Direcao::Baixo if self.cabeca.y >= board.1 => Err("fim de jogo, esbarrou na parede de baixo"),
            Direcao::Esquerda if self.cabeca.x == 0 => Err("fim de jogo, esbarrou na parede da esquerda"),
            Direcao::Direita if self.cabeca.x >= board.0 => Err("fim de jogo, esbarrou na parede da direita"),
            _ => {
                self.cabeca.alterar(self.direcao);
                Ok(())
            }
        }
    }

    fn mover_corpo(&mut self, posicao_anterior_cabeca: Ponto) {
        let corpo = &mut self.corpo;
        let mut posicao_anterior = posicao_anterior_cabeca;
        for ponto in corpo.iter_mut() {
            std::mem::swap(&mut posicao_anterior, ponto);
        }
    }
}

...

#[cfg(test)]
mod cobra_tests {

    use super::*;

    #[test]
    fn mover_cabeca_cobra_para_direita_no_tabuleiro_deve_mover_com_sucesso() {
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![],
            direcao: Direcao::Right,
        };
        let expected_point = Ponto::new(8, 7);
        cobra.mover_cabeca(&(8, 8)).unwrap();
        assert_eq!(expected_point, cobra.cabeca);
    }

    #[test]
    fn mover_cabeca_cobra_para_esquerda_no_tabuleiro_deve_mover_com_sucesso() {
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![],
            direcao: Direcao::Left,
        };
        let expected_point = Ponto::new(6, 7);
        cobra.mover_cabeca(&(8, 8)).unwrap();
        assert_eq!(expected_point, cobra.cabeca);
    }

    #[test]
    fn mover_cabeca_cobra_para_cima_no_tabuleiro_deve_mover_com_sucesso() {
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![],
            direcao: Direcao::Up,
        };
        let expected_point = Ponto::new(7, 6);
        cobra.mover_cabeca(&(8, 8)).unwrap();
        assert_eq!(expected_point, cobra.cabeca);
    }

    #[test]
    fn mover_cabeca_cobra_para_baixo_no_tabuleiro_deve_mover_com_sucesso() {
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![],
            direcao: Direcao::Down,
        };
        let expected_point = Ponto::new(7, 8);
        cobra.mover_cabeca(&(8, 8)).unwrap();
        assert_eq!(expected_point, cobra.cabeca);
    }

    #[test]
    #[should_panic(expected = "fim de jogo, esbarrou na parede da direita")]
    fn mover_cabeca_cobra_para_direita_no_tabuleiro_deve_esbarrar_na_parede() {
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![],
            direcao: Direcao::Right,
        };
        cobra.mover_cabeca(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "fim de jogo, esbarrou na parede da esquerda")]
    fn mover_cabeca_cobra_para_esquerda_no_tabuleiro_deve_esbarrar_na_parede() {
        let mut cobra = Cobra {
            cabeca: Ponto::new(0, 7),
            corpo: vec![],
            direcao: Direcao::Left,
        };
        cobra.mover_cabeca(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "fim de jogo, esbarrou na parede de baixo")]
    fn mover_cabeca_cobra_para_baixo_no_tabuleiro_deve_esbarrar_na_parede() {
        let mut cobra = Cobra {
            cabeca: Ponto::new(0, 7),
            corpo: vec![],
            direcao: Direcao::Down,
        };
        cobra.mover_cabeca(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "fim de jogo, esbarrou na parede de cima")]
    fn mover_cabeca_cobra_para_cima_no_tabuleiro_deve_esbarrar_na_parede() {
        let mut cobra = Cobra {
            cabeca: Ponto::new(0, 0),
            corpo: vec![],
            direcao: Direcao::Up,
        };
        cobra.mover_cabeca(&(7, 7)).unwrap();
    }

    #[test]
    fn mover_cobra_inteira_para_a_direita_deve_mover() {
        let tabuleiro = (15, 15);
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![Ponto::new(6, 7)],
            direcao: Direcao::Right,
        };
        cobra.passo(board).unwrap();
        assert_eq!(Ponto::new(8, 7), cobra.cabeca);
        assert_eq!(Ponto::new(7, 7), *cobra.corpo.first().unwrap());
    }

    #[test]
    fn mover_cobra_inteira_para_a_esquerda_deve_mover() {
        let tabuleiro = (15, 15);
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![Ponto::new(6, 7)],
            direcao: Direcao::Left,
        };
        cobra.passo(board).unwrap();
        assert_eq!(Ponto::new(6, 7), cobra.cabeca);
        assert_eq!(Ponto::new(7, 7), *cobra.corpo.first().unwrap());
    }

    #[test]
    fn mover_cobra_inteira_para_cima_deve_mover() {
        let tabuleiro = (15, 15);
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![Ponto::new(6, 7)],
            direcao: Direcao::Up,
        };
        cobra.passo(board).unwrap();
        assert_eq!(Ponto::new(7, 6), cobra.cabeca);
        assert_eq!(Ponto::new(7, 7), *cobra.corpo.first().unwrap());
    }

    #[test]
    fn mover_cobra_inteira_para_baixo_deve_mover() {
        let tabuleiro = (15, 15);
        let mut cobra = Cobra {
            cabeca: Ponto::new(7, 7),
            corpo: vec![Ponto::new(6, 7)],
            direcao: Direcao::Down,
        };
        cobra.passo(board).unwrap();
        assert_eq!(Ponto::new(7, 8), cobra.cabeca);
        assert_eq!(Ponto::new(7, 7), *cobra.corpo.first().unwrap());
    }
}
```

Notem o método de mover a cabeça da cobra:

```rust
fn mover_cabeca(&mut self, board: &(usize, usize)) -> Result<(), &'static str> {
    match self.direcao {
        Direcao::Cima if self.cabeca.y == 0 => Err("fim de jogo, esbarrou na parede de cima"),
        Direcao::Baixo if self.cabeca.y >= board.1 => Err("fim de jogo, esbarrou na parede de baixo"),
        Direcao::Esquerda if self.cabeca.x == 0 => Err("fim de jogo, esbarrou na parede da esquerda"),
        Direcao::Direita if self.cabeca.x >= board.0 => Err("fim de jogo, esbarrou na parede da direita"),
        _ => {
            self.cabeca.alterar(self.direcao);
            Ok(())
        }
    }
}
```

Temos nessa implementação o uso de um `if` que segue o valor do [enum](./02-enums.md), afinal o que é isso?

Isso faz parte do [Pattern Match](./03-match.md), é algo que chamamos de [guards](https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html), do modo em que essa implementação é feita, temos duas validações para cair nesse ponto, o `enum` deve bater ali e a condição deve ser verdadeira, caso uma das duas condições falhe ele segue para o próximo `match`.


Na função de mover o corpo temos a lógica para mover o restante da cobra, guardamos a posição do ponto antes de ser alterada e fazemos o próximo item a ser iterado a obter essa posição. Para isso usamos o método da biblioteca padrão do Rust, `swap`, esse método troca o valor de duas referências que são passadas.

```rust
fn mover_corpo(&mut self, posicao_anterior_cabeca: Ponto) {
    let corpo = &mut self.corpo;
    let mut posicao_anterior = posicao_anterior_cabeca;
    for ponto in corpo.iter_mut() {
        std::mem::swap(&mut posicao_anterior, ponto);
    }
}
```

Agora temos a lógica de mover a cobra, mas temos um problema nela, no método de alterar a direção, não temos uma validação para saber se o jogador, selecionou a opção de direção contraria da que a cobra esta seguindo, vamos adicionar agora.

No nosso enum de direção, vamos adicionar um método para pegar a direção contraria.

```rust
impl Direcao {
    pub fn direcao_inversa(outro: Self) -> Self {
        match outro {
            Self::Cima => Self::Baixo,
            Self::Baixo => Self::Cima,
            Self::Direita => Self::Esquerda,
            Self::Esquerda => Self::Direita
        }
    }
}
```

Deixo o teste deste método por sua conta.

E agora no nosso método de alterar a direção, faremos a validação, também deixo por sua conta esta alteração, e os testes da mesma.

Agora que temos o tabuleiro do jogo sendo desenhado, e temos a movimentação da cobra programada, vamos adicionar o petisco que iremos ter que pegar no jogo. O petisco é um ponto, então não precisamos criar outra `struct` para ela, apenas vamos gerar um ponto aleatório e fazer o nosso render renderiza-lo.


```rust
fn gerar_petisco(cobra: &Cobra, tabuleiro: &(usize, usize)) -> Point {
    let mut petisco;
    loop {
        let x = rand::thread_rng().gen_range(0..=tabuleiro.0 - 1);
        let y = rand::thread_rng().gen_range(0..=tabuleiro.1 - 1);
        petisco = Point::new(x, y);
        if cobra.cabeca != petisco && !cobra.corpo.contains(&petisco) {
            break;
        }
    }
    petisco
}
```

Para esse [rand](https://crates.io/crates/rand) funcionar precisamos ir em nosso Cargo.toml e adicionar a seguinte dependência `rand = "0.8.5"` logo abaixo do `[dependencies]`, nesse método temos validações para não gerar um petisco em cima da cobra, ou seja, se o valor aleatório cair na cabeça ou em alguma parte do corpo da cobra, outro valor sera gerado. Quando o valor respeitar essa condição o `loop` para.

Agora precisamos aumentar o tamanho da cobra, para isso adicionamos um método que ira adicionar um ponto, no fim do corpo da cobra.


```rust
impl Cobra {
    ...
    pub fn aumentar_tamanho(&mut self) {
        let ultimo = self.body.last().unwrap().clone();
        self.body.push(ultimo);
    }

}

```
Para testar esse método é interessante, validarmos o tamanho do corpo e se a posição do ponto adicionado, é igual à posição do último ponto anterior após a cobra se mover.

Agora temos que fazer o jogo funcionar, estamos quase lá.

Vamos criar um arquivo `jogo` onde teremos a `struct Jogo`. Aquele mesmo processo de sempre, cria o arquivo, adiciona na `lib.rs`.

A struct é a mais simples possível, ela é apenas.


```rust
struct Jogo;
```
Então vamos alterar o método que desenha o tabuleiro para gerar uma [String](../intermediary-01/01-strings.md), vamos usa-la para desenhar o tabuleiro inteiro de uma vez e também vamos movê-la para a `struct Jogo`.

```rust
impl Jogo {
    fn gerar_tabuleuro(cobra: &Cobra, petisco: &Ponto, tabuleiro: &(usize, usize)) -> String {
        let mut buffer = String::new();
        for y in 0..tabuleiro.1 {
            for x in 0..tabuleiro.0 {
                if cobra.cabeca == (x, y) {
                    buffer.push_str("0 ")
                } else if cobra.corpo.contains(&Ponto::new(x, y)) {
                    buffer.push_str("# ");
                } else if *petisco == (1x, y) {
                    buffer.push_str("+ ");
                } else {
                    buffer.push_str("- ");
                }
            }
            buffer.push('\n');
        }
        buffer
    }
}
```

Agora que fizemos essa alteração, vamos jogar o gerador do petisco para essa mesma struct.

```rust
impl Jogo {
    ...
    fn gerar_petisco(cobra: &Cobra, tabuleiro: &(usize, usize)) -> Point {
        let mut petisco;
        loop {
            let x = rand::thread_rng().gen_range(0..=tabuleiro.0 - 1);
            let y = rand::thread_rng().gen_range(0..=tabuleiro.1 - 1);
            petisco = Point::new(x, y);
            if cobra.cabeca != petisco && !cobra.corpo.contains(&petisco) {
                break;
            }
        }
        petisco
    }
}
```

E estamos quase lá, falta apenas um loop infinito, onde, movemos a cobra, limpamos a tela anterior, redesenhamos a tela e capturamos a tecla acionada.
Para facilitar o processo vamos adicionar mais uma dependência no arquivo `Cargo.toml`, sera dependência [termion](https://crates.io/crates/termion).

Então nosso arquivo ficará parecido com isso:

```toml
[package]
name = "snake-game"
version = "0.1.0"
edition = "2021"

[dependencies]

rand = "0.8.5"
termion = "1.5.6"
```

Vamos adicionar o método estático na `struct Jogo` que ira fazer a "mágica" acontecer.

```rust
impl Jogo {
    pub fn run() -> Result<(), &'static str> {
        let mut cobra: Cobra = Default::default();
        let tabuleiro = (15, 15);
        let mut petisco = Self::gerar_petisco(&snake, &tabuleiro);
        let mut stdin = termion::async_stdin().keys();
        loop {
            if cobra.cabeca == snack {
                cobra.aumentar_tamanho();
                petisco = Self::gerar_petisco(&cobra, &tabuleiro);
            } else if cobra.corpo.contains(&cobra.cabeca) {
                return Err("fim de jogo, a cobra bateu nela mesma");
            }
            let tabuleiro_jogo = Self::gerar_tabuleuro(&cobra, &petisco, &tabuleiro);
            print!(
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide
            );
            println!("{}", tabuleiro_jogo);
            let stdout = io::stdout().into_raw_mode().unwrap();
            let input = stdin.next();
            if let Some(Ok(key)) = input {
                match key {
                    Key::Char('a') | Key::Left => cobra.alterar_direcao(Direcao::Esquerda),
                    Key::Char('w') | Key::Up => cobra.alterar_direcao(Direcao::Cima),
                    Key::Char('s') | Key::Down => cobra.alterar_direcao(Direcao::Baixo),
                    Key::Char('d') | Key::Right => cobra.alterar_direcao(Direcao::Direita),
                    _ => {},
                }
            }
            stdout.lock().flush().unwrap();
            thread::sleep(Duration::from_millis(500));
            cobra.passo(tabuleiro)?;
        }
    }
}
```

Esse método agrupa tudo o que nós precisamos, criamos a cobra, criamos o primeiro petisco, definimos o tamanho do tabuleiro e começamos a trabalhar.

Na linha ```let mut stdin = termion::async_stdin().keys();``` criamos um modo [assincrono](https://en.wikipedia.org/wiki/Async/await) de capturar as teclas digitadas pelo usuário, utilizando a dependencia do `termion`, assim que entramos no loop, fazemos as primeiras verificações, que são:
 - Validar se a cabeça da cobra está na mesma posição de um petisco
    - Se sim => seu tamanho aumenta e outro petisco é gerado.
    - Se não => valida se a cabeça está na mesma posição de seu corpo
        - Se sim => encerra o jogo com a mensagem de fim de jogo
        - Se não => continua a execução

Então geramos o tabuleiro e armazenamos em uma variável. Com um método do `termion`, limpamos o terminal, posicionamos o mouse na primeira posição e escondemos o cursor. Logo em sequência desenhamos tabuleiro do jogo. Transformamos a saída em [raw mode](https://en.wikipedia.org/wiki/Terminal_mode), lemos uma tecla e caso alguma tecla tenha sido pressionada validamos qual foi, em um `match` assim alteramos a direção que a cobra esta andando caso necessário. Limpamos a saída, esperamos meio segundo com o método `thread::sleep(Duration::from_milis(500))` e então fazemos a cobra dar mais um passo. O processo todo se repete.


Adicionamos a o nosso `main` a chamada a esse método:

```rust
fn main(){
    if let Err(msg) = Game::run() {
        eprintln!("{}", msg)
    }
}
```

E pronto, temos nosso jogo da cobrinha feito e funcionando, deixo para você os testes finais e adiciono alguns desafios:
- Faça o jogo pausar
- Adicione um placar ao jogo
- Quando a cobra alcançar o tamanho máximo (x * y) mostre uma mensagem de vitória e encerre o jogo