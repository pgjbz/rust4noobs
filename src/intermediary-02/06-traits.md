# Traits

Na parte sobre [generics](./05-generics.md) deixamos o seguinte código por terminar.

```rust
fn maior<T>(lista: &[T]) -> T {
    let mut maior = lista[0];
    for &item in lista {
        if item > maior {
            maior = item;
        }
    } 
    maior
}

fn main() {
    let arr: [u8; 4] = [2,4,1,11];
    let maior = maior(&arr);
    println!("Maior elemento: {}", maior);
}
```

Afinal o que falta para esse código funciona? Falta determinamos que 'T' deve implementar algumas traits. O que são essas `traits`?? Elas são como contratos, um tipo deve implementar certas funções/métodos definidas por essa `trait`. E como declaramos uma? Seguimos o seguinte padrão `trait nome { assinaturas/métodos }`.

```rust
trait Pagavel  {
    fn total(&self) -> f64;
}
```

Assim definimos uma `trait`, agora precisamos implementar, vamos criar uma `struct Pedido` que ira implementar essa `trait`. Para dizer que algo implementa uma `trait` usamos o seguinte padrão `impl NomeTrait for NomeStruct { implementação }`

```rust
//--definição trait Pagável

struct Pedido {
    quantidade_items: u8,
    valor_items: f64
}

impl Pagavel for Pedido {
    fn total(&self) -> f64 {
        self.valor_items * self.quantidade_items as f64
    }
}

fn main() {
    let pedido = Pedido {
        quantidade_items: 10,
        valor_items: 10.5
    };
    let total = pedido.total();
    println!("Total do pedido: {}", total);
}
```

Agora podemos falar que nossa struct de Pedido implementa esta Trait. E agora se vamos utilizar um método genérico que tenha 'T' como parâmetro, porém queremos que 'T' seja pagável. Como faríamos isso? Temos dois modos, sendo eles:

```rust
fn pagar<T: Pagavel>(pagavel: T) {
    println!("Valor {} pago", pagavel.total());
}
```

Ou com a palavra 'where'

```rust
fn pagar<T>(pagavel: T)
where
    T: Pagavel,
{
    println!("Valor {} pago", pagavel.total());
}
```

Então podemos chamar o método genérico pagar passando o pedido como argumento.

```rust
//--Definição pedido, trait e implementação
fn main() {
    let pedido = Pedido {
        quantidade_items: 10,
        valor_items: 10.5,
    };
    pagar(pedido);
}
```

E o programa ira compilar. 

Podemos implementar mais de uma trait para algo.

```rust
//--declaração trait pagavel, struct e impl Pagavel
trait Cancelavel {
    fn cancelar(self);
}

impl Cancelavel for Pedido {
    fn cancelar(self) {
        println!("Pedido com {} itens cancelado", self.quantidade_items)
    }
}

fn main() {
    let pedido = Pedido {
        quantidade_items: 10,
        valor_items: 10.5,
    };
    pedido.cancelar();
}
```

Uma trait pode ter um método já implementado, que pode ou não ser sobrescrito.

```rust
//--declaração trait pagavel, struct e impl Pagavel e cancelavel

trait Tributavel {
    fn calcular_imposto(&self) -> f64 {
        0.01 * 200.0
    }
}

impl Tributavel for Pedido {} 

fn main() {
    let pedido = Pedido {
        quantidade_items: 10,
        valor_items: 10.5,
    };
    pedido.cancelar();
}
```

Caso queira sobrescrever a implementação de `Tributavel` seria feito como a implementação de qualquer outra trait.

```rust
impl Tributavel for Pedido {
    fn calcular_imposto(&self) -> f64 {
        self.valor_items * 0.01 
    }
} 
```

E se eu quiser que para implementar `Tributavel` e `Cancelavel` eu precise implementar a `trait` `Pagavel`? Seria usado uma estratégia parecida com a dos generics `trait NomeTrait: TraitQuePrecisaImplementar`

```rust
trait Pagavel {
    fn total(&self) -> f64;
}

trait Cancelavel: Pagavel {
    fn cancelar(self);
}

trait Tributavel: Pagavel {
    fn calcular_imposto(&self) -> f64 {
        0.01 * 200.0
    }
}
```
Do modo acima para implementar `Cancelavel` ou `Tributavel` precisamos implementar `Pagavel`, assim nos dando um novo poder nas implementações, PODER USAR OS MÉTODOS DEFINIDOS EM PAGAVEL.

```rust
impl Cancelavel for Pedido {
    fn cancelar(self) {
        println!("Pedido custando {} cancelado", self.total())
    }
}

impl Tributavel for Pedido {
    fn calcular_imposto(&self) -> f64 {
        0.01 * self.total()   
    }
}
```

E se eu implementar duas traits com métodos iguais? Não podemos chamar diretamente o método implementado.

```rust
struct Cachorro {}

trait Animal {
    fn comer(&self);
}

trait Fome {
    fn comer(&self);
}

impl Animal for Cachorro {
    fn comer(&self) {
        println!("Cachorro comendo... animal");
    }
}

impl Fome for Cachorro {
    fn comer(&self) {
        println!("Cachorro comendo por estar com fome");
    }
}

fn main() {
    let cachorro = Cachorro {};
    cachorro.comer();
}
```

Perdão pelo exemplo bobo, mas o código acima daria o seguinte erro.

```bash
error[E0034]: multiple applicable items in scope
  --> src/main.rs:25:14
   |
25 |     cachorro.comer();
   |              ^^^^^ multiple `comer` found
   |
```

Este erro acontece por termos múltiplas implementações de método com a mesma assinatura. Para chamar o método `comer` podemos fazer da seguinte maneira `trait::metodo(&instancia)`.

```rust
//--declarações e implementações
fn main() {
    let cachorro = Cachorro {};
    Animal::comer(&cachorro);
    Fome::comer(&cachorro);
}
```

Podemos também implementar traits para tipos já existentes.

```rust
impl Fome for i32 {
    fn comer(&self) {
        println!("Um numero esta comendo por estar com fome? Isso faz sentido?")
    }
}

fn main() {
    a.comer();
}
```

Claro essa implementação de uma trait teria que fazer sentido, não é mesmo?

### Traits já existentes

Em Rust já temos uma boa quantidade de traits já existentes, como, por exemplo, a trait `Iterator`, com essa `trait` podemos criar nossas próprias implementações de algo iterável e utilizar os recursos da linguagem, como um loop for, por exemplo.

```rust
struct Contador {
    contagem: u64
}

impl Iterator for Contador {
    type Item = u64; /*futuramente iremos explicar com mais detalhes o que é isso, 
    mas considere que é um modo de usar Generics de uma forma que impedimos múltiplas implementações da mesma trait pra mesma coisa */

    fn next(&mut self) -> Option<Self::Item> {
        if self.contagem >= 100 {
            None
        } else {
            self.contagem += 1;
            Some(self.contagem)
        }
    }
}

fn main() {
    let contador = Contador { contagem: 0 };

    for i in contador {
        println!("Numero atual: {}", i);
    }
}
```

Ao executar o código acima teremos a saída.

```bash
...
Numero atual: 89
Numero atual: 90
Numero atual: 91
Numero atual: 92
Numero atual: 93
Numero atual: 94
Numero atual: 95
Numero atual: 96
Numero atual: 97
Numero atual: 98
Numero atual: 99
Numero atual: 100
```

### Derive

O comando `#[derive(AlgumaCoisaAqui)]`, é um macro para implementação de algumas `traits`, quando usamos o `#[Derive(Debug)]` estamos informando ao compilador que queremos que aquela `struct/enum` ira implementar a trait `Debug`, porém isso é gerado de forma automática pelo compilador.

### Voltando ao problema inicial.

Agora que entendemos como as `traits` funcionam vamos retomar o problema que deixamos no fim da [parte anterior](./05-generics.md)

Precisamos limitar 'T' para duas `traits` especificas, essas `traits` já são existentes na linguagem, sendo elas `PartialOrd` e `Copy`, para falar que o argumento precisa implementar mais de uma `trait` utilizamos o '+', com o seguinte padrão 'T: Trait1 + Trait2 + Trait3....`

```rust
fn maior<T>(lista: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut maior = lista[0];
    for &item in lista {
        if item > maior {
            maior = item;
        }
    }
    maior
}

fn main() {
    let arr: [u8; 4] = [2, 4, 1, 11];
    let maior = maior(&arr);
    println!("Maior elemento: {}", maior);
}
```

Ao executar o nosso código finalmente terá sucesso e a seguinte saída no console.

```bash
Maior elemento: 11
```

Este capitulo sobre `traits` ficou maior que do eu esperava, mas espero que tenha ficado claro o uso delas e a importância dessa funcionalidade.