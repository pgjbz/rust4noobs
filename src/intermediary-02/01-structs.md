# Structs

Struct no modo mais simples de se falar é uma estrutura de valores em "uma única variável". Não consigo pensar em um modo mais fácil de explicar com palavras o que seria uma "struct", sua declaração é simples ela segue o seguinte padrão `struct Nome { campos }`.

```rust
struct Cliente {
    nome: String,
    ano_de_nascimento: u16,
    documento: String,
}
```

O modo de declaração dos campos/atributos de uma struct lembra bastante o de um [json](https://en.wikipedia.org/wiki/JSON), para criarmos uma variável de uma struct podemos fazer da seguinte maneira:

```rust
//--declaração da estrutura Cliente

fn main() {
    let cliente: Cliente = Cliente { 
        nome: String::from("Paulo"),
        ano_de_nascimento: 1999, 
        documento: String::from("Onde?") 
    };
}
```

Podemos acessar os campos da variável utilizando apenas um ".nome_do_campo".

```rust
//--declaração da estrutura Cliente

fn main() {
    let cliente = Cliente { 
        nome: String::from("Paulo"),
        ano_de_nascimento: 1999, 
        documento: String::from("Onde?") 
    };
    println!("Nome do cliente: {}", cliente.nome);
}
```

## Declarando comportamento para uma Struct

Em Rust uma struct pode ter funções associadas a ela, essas funções são chamadas de métodos, aqui temos uma das características de [Programação Orientada a Objetos]() no Rust, como a linguagem é de multiparadigmas temos alguns recursos desse modo de programação disponível.

Para implementarmos métodos para a struct `Cliente` utilizamos a palavra reservada `impl` seguida do nome da estrutura `impl Cliente { implementações }`, vamos começar com a implementação de um método estático para nos auxiliar na criação de variáveis do tipo `Cliente`.

```rust
struct Cliente {
    nome: String,
    ano_de_nascimento: u16,
    documento: String,
}

impl Cliente {
    fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {
        Self {
            nome: nome,
            ano_de_nascimento,
            /*
                Como o atributo tem o mesmo nome do parametro/variavel eu não preciso colocar o padrão chave:valor
            */
            documento
        }
    }
}

fn main() {
    let cliente = Cliente::new(String::from("Paulo"), 1999, String::from("Onde?"));
    println!("Nome do cliente: {}", cliente.nome);
}
```

A palavra `Self` com 'S' maiúsculo é um modo de falar que estamos se referindo a própria struct que esta sendo implementada.

Temos também métodos que dependem de uma [instância](https://en.wikipedia.org/wiki/Instance_(computer_science)) da struct, para este tipo de métodos utilizamos a própria struct utilizando a palavra `self` com 's' minúsculo como parâmetro do método, pode ser uma referência mutável, imutável ou pode não ser referência, porém, não sendo uma referência perdemos o ownership da struct e sua memória é liberada.

```rust
struct Cliente {
    nome: String,
    ano_de_nascimento: u16,
    documento: String,
}

impl Cliente {
    fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {
        Self {
            nome: nome,
            ano_de_nascimento,
            /*
                Como o atributo tem o mesmo nome do 
                parametro/variavel eu não preciso 
                colocar o padrão chave:valor
            */
            documento
        }
    }

    fn diz_oi(&self) {
        println!("{} disse oi", self.nome);
    }
    
    fn diz_tchau(self) {
        println!("{} disse tchau e foi embora", self.nome);
    } 

    fn mudar_nome(&mut self, novo_nome: String) {
        //para utilizar este método a instância de 
        //cliente deve ser mutável
        self.nome = novo_nome;
    }
}

fn main() {
    let mut cliente = Cliente::new(String::from("Paulo"), 1999, String::from("Onde?"));
    cliente.diz_oi();
    cliente.mudar_nome(String::from("Novo nome")); 
    cliente.diz_oi();
    cliente.diz_tchau(); //a partir daqui a memoria deste               //cliente foi liberada não conseguimos mais utilizar
}
```

Caso tentarmos utilizar a instância de cliente após a chamada do método `diz_tchau` teremos o seguinte erro

```bash
  --> main.rs:42:5
   |
37 |     let mut cliente = Cliente::new(String::from("Paulo"), 1999, String::from("Onde?"));
   |         ----------- move occurs because `cliente` has type `Cliente`, which does not implement the `Copy` trait
...
41 |     cliente.diz_tchau(); //a partir daqui a memoria deste      
   |             ----------- `cliente` moved due to this method call
42 |     cliente.diz_oi();         //cliente foi liberada não conseguimos mais utilizar
   |     ^^^^^^^ value borrowed here after move
   |
note: this function takes ownership of the receiver `self`, which moves `cliente`
  --> main.rs:25:18
   |
25 |     fn diz_tchau(self) {
   |                  ^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
```

- [Próximo](./02-enums.md) - Enum