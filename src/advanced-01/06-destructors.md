## Destructors

Deveríamos ter falado sobre destructors antes, lá em [traits](../intermediary-02/06-traits.md), porém precisavamos de alguns conceitos antes como [lifetimes](./01-lifetimes.md), talvez [smart pointers](./02-smart-pointers.md), para começar, vamos utilizar aquele exemplo lá do início deste livro sobre [structs](../intermediary-02/01-structs.md). Mas antes vamos resumir os destrutores.
Diferente dos construtores, destrutores são executados quando algo é limpo da memória, a trait `Drop` é executada, estando na heap ou na stack. 


### Struct cliente.

Definir a struct `Cliente` é algo muito simples, vamos definir de um modo muito parecido com o que fizemos em [structs](../intermediary-02/01-structs.md), a struct, com atributos, um "construtor" e um método para dizer "oi". 

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
                parâmetro/variável eu não preciso
                colocar o padrão chave:valor
            */
            documento
        }
    }

    fn diz_oi(&self) {
        println!("{} disse oi", self.nome);
    }
}
```
Agora que vem o pulo do gato, vamos implementar a trait `Drop` para esta struct, igual o método `diz_tchau`, que definimos em `structs`.

```rust
# struct Cliente {
#     nome: String,
#     ano_de_nascimento: u16,
#     documento: String,
# }
# 
# impl Cliente {
#     fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {  
#         Self {
#             nome: nome,
#             ano_de_nascimento,
#             /*
#                 Como o atributo tem o mesmo nome do
#                 parâmetro/variável eu não preciso
#                 colocar o padrão chave:valor
#             */
#             documento
#         }
#     }
# 
#     fn diz_oi(&self) {
#         println!("{} disse oi", self.nome);
#     }
# }

impl Drop for Cliente {
    fn drop(&mut self) {
        println!("{} disse tchau e foi embora", self.nome);
    }
}
```

Como vimos, a trait apenas implementa um método, agora vemos ver o seu comportamento.

```rust
# struct Cliente {
#     nome: String,
#     ano_de_nascimento: u16,
#     documento: String,
# }
# 
# impl Cliente {
#     fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {  
#         Self {
#             nome: nome,
#             ano_de_nascimento,
#             /*
#                 Como o atributo tem o mesmo nome do
#                 parâmetro/variável eu não preciso
#                 colocar o padrão chave:valor
#             */
#             documento
#         }
#     }
# 
#     fn diz_oi(&self) {
#         println!("{} disse oi", self.nome);
#     }
# }

# impl Drop for Cliente {
#     fn drop(&mut self) {
#         println!("{} disse tchau e foi embora", self.nome);
#     }
# }

fn main() {
    let cliente = Cliente::new("Paulo".to_owned(), 1999, "00000000191".to_owned());
    cliente.diz_oi();
} // a partir deste ponto, a variavel cliente, foi limpa da memoria e a trait Drop foi chamada
```

Repare que, ao executar o código acima, aconteceu a execução da trait `Drop` sem precisarmos chamá-la. Claro, como recebemos uma referência mutável da struct, todas as regras de [ownership](../intermediary-01/03-ownership.md) ainda existem. Essa trait é mais útil do que parece, por exemplo, em bibliotecas de conexão com banco de dados, ao invés de precisarmos fechar a conexão manualmente, a partir do momento em que essa trait é chamada, podemos fechar a conexão, ou quando estamos usando `unsafe rust`, que ainda iremos retratar, podemos utilizar para limpar memória que alocamos manualmente, sim, isto é possível com `Rust`, porém não é o modo que normalmente programamos em `Rust`; por isso, é chamado de `unsafe Rust`.