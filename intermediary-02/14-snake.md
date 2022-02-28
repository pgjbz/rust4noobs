# Snake Game

Eu estava em dúvida em que projeto fazer neste ponto, foi dificil pensar em algo e porque algo que eu nunca fiz?

Então vamos lá fazer o "jogo da cobrinha", vamos começar criando o projeto, usando o comando `cargo new snake-game`.

Teremos a estrutura padrão do projeto

```bash
├── Cargo.toml
└── src
    └── main.rs
```

Vamos adicionar um arquivo chamado `lib.rs` dentro da pasta `src`, este arquivo sera usado para declarar os nosso módulos. Em seguida vamos criar um arquivo chamado "ponto.rs" e nele iremos criar uma [struct](./01-structs.md) para as localizações no nosso jogo, vamos criar uma implementação a essa struct para facilitar a instanciação dessa `strutc`.

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

Para que este arquivo seja reconhecido no projeto, vamos adicionar no nosso arquivo `lib.rs` a seguinte linha `pub mod point;`. Note que tanto a `struct` quanto seus atributos e a implementação do método `new` estão com a palavra `pub`, que faz eles serem visiveis fora desse módulo.

Vamos printar o campo onde a cobrinha ira andar, e para testar vamos adicionar um ponto nesse tabuleiro.

```rust
fn main() {
    let point = Point::new(7, 7);
    let (x, y) = (15, 15);
    for x in 0..x {
        for y in 0..y {
            if point == (x, y) {
                print!("# ")
            }  else {
                print!("- ");   
            }
        }
        println!();
    }
}

```

Node que comparamos a nossa `struct` Ponto, com uma tupla de (x, y), para isso ser possivel, precisamos implementar uma [trait](./06-traits.md) chamada [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) a implementação para isso é relativamente simples. A `trait` recebe um parametro [generico](./05-generics.md) na implementação vamos falar que esse parametro genérico é uma `tupla (usize, usize)`. E a partir dai implementamos nossa comparação.

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

Agora vamos criar a `struct` da nossa cobrinha, para isso vamos adicionar a cabeça - que é um ponto - e uma lista de pontos para o corpo. Criamos o arquivo "cobra.rs" e adicionamos o `pub mod cobra` no arquivo `lib.rs`, e no arquivo "cobra.rs" adicionamos a struct

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

A implementação da `trait` [default](https://doc.rust-lang.org/std/default/trait.Default.html) serve para termos um valor padrão para a `struct`. Vamos separar a nossa função de desenhar o tabuleiro e vamos passar uma referencia para a `struct` da `cobra`, então com base nos dados passados ali vamos desenhar a nossa cobra.

```rust
fn print_board(cobra: &Cobra) {
    let (x, y) = (15, 15);
    for y in 0..y {
        for x in 0..x {
            if cobra.cabeca == (x, y) {
                print!("0 ")
            } else if cobra.body.contains(&Ponto::new(x, y)) {
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
    print_board(&Snake::default())
}
```

Após executar o comando `cargo run` temos o outpu:

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

Então adicionamos as 4 direções possiveis ao nosso enum.

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

Criamos o enum já implementando a `trait default` para nos auxiliar, como o padrão de inicio da cobra sempre vai ser para a direita, colocamos o retorno do método o valor `Self::Direita`. Já derivamos as `traits`, `Clone` e `Copy` para não precisar passar esse enum como referencia todas as vezes.

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

Agora temos um modo de saber para qual direção a cobra esta andando.

Na nossa `struct Ponto` vamos adicionar a função para alterar o valor do ponto.

```rust

impl Point {
   ...

    pub fn alterar(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }
}
```

Vamos aproveitar e adicionar testes unitarios para o método de alterar:

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

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn mover_cabeca(&mut self, board: &(usize, usize)) -> Result<(), &'static str> {
        match self.direction {
            Direcao::Cima if self.cabeca.y == 0 => Err("game over, dump in top wall"),
            Direcao::Baixo if self.cabeca.y >= board.1 => Err("game over, dump in down wall"),
            Direcao::Esquerda if self.cabeca.x == 0 => Err("game over, dump in left wall"),
            Direcao::Direita if self.cabeca.x >= board.0 => Err("game over, dump in right wall"),
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
        let mut cobra = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Right,
        };
        let expected_point = Point::new(8, 7);
        snake.move_head(&(8, 8)).unwrap();
        assert_eq!(expected_point, snake.head);
    }

    #[test]
    fn mover_cabeca_cobra_para_esquerda_no_tabuleiro_deve_mover_com_sucesso() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Left,
        };
        let expected_point = Point::new(6, 7);
        snake.move_head(&(8, 8)).unwrap();
        assert_eq!(expected_point, snake.head);
    }

    #[test]
    fn mover_cabeca_cobra_para_cima_no_tabuleiro_deve_mover_com_sucesso() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Up,
        };
        let expected_point = Point::new(7, 6);
        snake.move_head(&(8, 8)).unwrap();
        assert_eq!(expected_point, snake.head);
    }

    #[test]
    fn mover_cabeca_cobra_para_baixo_no_tabuleiro_deve_mover_com_sucesso() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Down,
        };
        let expected_point = Point::new(7, 8);
        snake.move_head(&(8, 8)).unwrap();
        assert_eq!(expected_point, snake.head);
    }

    #[test]
    #[should_panic(expected = "game over")]
    fn mover_cabeca_cobra_para_direita_no_tabuleiro_deve_esbarrar_na_parede() {
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![],
            direction: Direction::Right,
        };
        snake.move_head(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "game over")]
    fn move_snake_head_to_left_in_board_should_bump_into_the_wall() {
        let mut snake = Snake {
            head: Point::new(0, 7),
            body: vec![],
            direction: Direction::Left,
        };
        snake.move_head(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "game over")]
    fn move_snake_head_to_down_in_board_should_bump_into_the_wall() {
        let mut snake = Snake {
            head: Point::new(0, 7),
            body: vec![],
            direction: Direction::Down,
        };
        snake.move_head(&(7, 7)).unwrap();
    }

    #[test]
    #[should_panic(expected = "game over")]
    fn move_snake_head_to_up_in_board_should_bump_into_the_wall() {
        let mut snake = Snake {
            head: Point::new(0, 0),
            body: vec![],
            direction: Direction::Up,
        };
        snake.move_head(&(7, 7)).unwrap();
    }

    #[test]
    fn move_entire_snake_to_right_should_be_move() {
        let board = (15, 15);
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Direction::Right,
        };
        snake.step(board).unwrap();
        assert_eq!(Point::new(8, 7), snake.head);
        assert_eq!(Point::new(7, 7), *snake.body.first().unwrap());
    }

    #[test]
    fn move_entire_snake_to_left_should_be_move() {
        let board = (15, 15);
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Direction::Left,
        };
        snake.step(board).unwrap();
        assert_eq!(Point::new(6, 7), snake.head);
        assert_eq!(Point::new(7, 7), *snake.body.first().unwrap());
    }

    #[test]
    fn move_entire_snake_to_up_should_be_move() {
        let board = (15, 15);
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Direction::Up,
        };
        snake.step(board).unwrap();
        assert_eq!(Point::new(7, 6), snake.head);
        assert_eq!(Point::new(7, 7), *snake.body.first().unwrap());
    }

    #[test]
    fn move_entire_snake_to_down_should_be_move() {
        let board = (15, 15);
        let mut snake = Snake {
            head: Point::new(7, 7),
            body: vec![Point::new(6, 7)],
            direction: Direction::Down,
        };
        snake.step(board).unwrap();
        assert_eq!(Point::new(7, 8), snake.head);
        assert_eq!(Point::new(7, 7), *snake.body.first().unwrap());
    }
}
```