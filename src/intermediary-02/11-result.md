# Tratamento de erros

Em Rust não temos exceções, não temos `nulos`, temos [Option](./07-option.md) e para tratar erros temos `Result<T, E>`, o tipo `Result` assim como o tipo `Option` é um [Enum](./02-enums.md), contendo dois valores `Ok(T)` e `Err(E)`, sendo `Ok` o e `Err` quando tivermos uma falha.

Assim como `Option` o `Result` também tem alguns métodos. Iremos fazer algo um pouco diferente neste capítulo, ele sera mais prático, porém nada muito complexo.

Tendo o código abaixo.

```rust
use std::fs::File;

fn main() {
    let file = File::open("rust4noobs.txt");
}
```

A variável file, esta recebendo um `Result<File, std::io::Error>`, podemos realizar apenas um `.unwrap()` para caso o arquivo não exista, ou não termos permissão para acesso, ou qualquer outro erro aconteça encerarmos a execução do programa, ou caso de sucesso prosseguirmos com a solução do problema. Mas não queremos fazer isso. Caso o tivermos algum erro, mais específico, o arquivo não existir, vamos criar este arquivo e escrever nele "Rust4Noobs", caso qualquer um destes processos do tratamento falhe, iremos simplesmente encerrar o programa.

```rust
fn main() {
    let file = abre_arquivo("rust4noobs.txt");
}

fn abre_arquivo(caminho: &str) -> File {
    match File::open(caminho) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                let mut file = File::create("rust4noobs.txt").unwrap();
                file.write_all(b"Rust4Noobs").unwrap();
                file
            },
            e => {
                eprintln!("Tivemos um probleminha aqui {:?}", e);
                process::exit(1)
            }
        }
    }
}
```

No código acima, tentamos abrir o arquivo, caso tenhamos sucesso, retornamos o arquivo, caso aconteça algum erro executamos o procedimento de validar o tipo do erro, caso o erro seja do tipo "NotFound", então criamos o arquivo e escrevemos nele, já dando `.unwrap` caso de algum erro, após isso já retornamos o arquivo, para ser utilizado. Caso o erro não seja do tipo `NotFound` apenas escrevemos o erro e finalizamos o processo.

Agora vamos eliminar esses `unwrap` com o operador `?`, para isso precisamos realizar algumas modificações. 

```rust
fn main() -> Result<(), io::Error> {
    let file = abre_arquivo("rust4noobs.txt")?;
    Ok(())
}

fn abre_arquivo(caminho: &str) -> Result<File, io::Error> {
    match File::open(caminho) {
        Ok(file) => Ok(file),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                let mut file = File::create("rust4noobs.txt")?;
                file.write_all(b"Rust4Noobs")?;
                Ok(file)
            },
            e => {
                eprintln!("Tivemos um probleminha aqui {:?}", e);
                process::exit(1)
            }
        }
    }
}
```

No nosso `main` adicionamos ```-> Result<(), io::Error>``` e no fim retornamos  `Ok(())` no método `abre_arquivo` agora ao invés de retornarmos o arquivo diretamente, retornamos um `Result<File, io::Error>` e substituímos os `.unwrap()` por um `?`, o operador `?` realiza o processo de `unwrap` e caso não tenha sucesso ele propaga o erro para quem o chamou, por isso precisamos retornar um `Result`, quando chegamos no método `main`, não queremos tratar então só propagamos o erro.

## Criando nossos próprios erros.

Enquanto eu aprendo mais sobre Rust e escrevo este 4Noobs, eu estou aprendendo mais sobre interpretadores. Em alguns pontos código do meu interpretador eu preciso de `Result` ao invés de `Option`, isso quando é algo que pode falhar e ter tratativas diferentes dependendo do  erro. Por exemplo, no meu interpretador eu considero o fim do arquivo como um erro. Porém, não é um erro que para a execução do interpretador, é um erro que significa que não tenho mais tokens, então posso seguir para a próxima parte. Agora caso tenhamos um erro de sintaxe, salvamos isso em um erro para informar a quem estiver usando a linguagem.

Temos abaixo um exemplo simples desse caso.

```rust
enum ParseError {
    Eof,
    Inaceitavel(String)
}

fn faz_o_parse_ai(mock: u8) -> Result<(), ParseError> {
    
    if mock >= 100 && mock <= 199 {
        return Err(ParseError::Inaceitavel("como assim vc me deu esse valor?".to_string()));
    } else if mock > 200 {
        return Err(ParseError::Eof)
    }
    Ok(())
}

fn main() {
    let mut erros = Vec::new();
    match faz_o_parse_ai(35 + 34) {
        Ok(()) => {},
        Err(e) => match e {
            ParseError::Eof => {/* só continua para a próxima parte */},
            ParseError::Inaceitavel(msg) => erros.push(msg),
        }
    }
}
```

O uso do meu próprio `Enum` e é algo que facilita a minha leitura, por saber que se trata especificamente de um erro de `Parse`, assim como as `exceções`em `Java` ou qualquer outra linguagem, use algo que faça sentido, não retorne um erro de parse o erro for sobre não conseguir vender maçãs.

Claro como `Result<T, E>` é um tipo genérico, não é obrigatório que o meu erro seja um `Enum` ou qualquer coisa, posso retornar um `i32`, `String`, qualquer coisa.

- [Próximo](./12-panic.md) - Macro Panic