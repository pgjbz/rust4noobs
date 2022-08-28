<!-- Logo 4noobs -->

<p align="center">
  <a href="https://github.com/he4rt/4noobs" target="_blank">
    <img src="./src/assets/header_4noobs.svg">
  </a>
</p>

<!-- Title -->

<p align="center">
  <h2 align="center">Rust4Noobs</h2>

  <h1 align="center"><img src="./src/assets/rust.svg" alt="Imagem da linguagem" width="120"></h1>
  
  <p align="center">
    <br />
    <a href="#ROADMAP"><strong>Explore a documentação »</strong></a>
    <br />
    <br />
    <a href="https://github.com/pgjbz/rust4noobs/issues/new">Report Bug</a>
    ·
    <a href="https://github.com/pgjbz/rust4noobs/issues/new">Request Feature</a>
  </p>
</p>

## Sobre o Projeto
Projeto para introdução a linguagem de programação Rust, o objetivo deste repositório é inserir o leitor aos conceitos da linguagem Rust, como o seu modo de gerenciamento de memória e conceitos da linguagem.

## ROADMAP

- [Básico](./src/basic)
    - [Sobre a linguagem](./src/basic/01-about.md)
    - [Ambiente](./src/basic/02-environment.md)
    - [Hello World](./src/basic/03-hello-world.md)
    - [Tipos de dados](./src/basic/04-data-types.md)
    - [Operadores](./src/basic/05-operators.md)
    - [Condicionais](./src/basic/06-conditions.md)
    - [Loops](./src/basic/07-loops.md)
    - [Funções](./src/basic/08-functions.md)
    - [Arrays](./src/basic/09-arrays.md)
    - [Exercícios](./src/basic/10-exercises.md)
- [Intermediário 1](./src/intermediary-01)
    - [Strings](./src/intermediary-01/01-strings.md)
    - [Introdução a ponteiros](./src/intermediary-01/02-pointers-intro.md)
    - [Ownership](./src/intermediary-01/03-ownership.md)
    - [Tuplas](./src/intermediary-01/04-tuples.md)
    - [Slices](./src/intermediary-01/05-slices.md)
    - [Entrada de dados](./src/intermediary-01/06-user-input.md)
    - [Exercícios](./src/intermediary-01/07-exercises.md)
- [Intermediário 2](./src/intermediary-02)
    - [Struct](./src/intermediary-02/01-structs.md)
    - [Enum](./src/intermediary-02/02-enums.md)
    - [Match](./src/intermediary-02/03-match.md)
    - [Módulos](./src/intermediary-02/04-modules.md)
    - [Generics](./src/intermediary-02/05-generics.md)
    - [Traits](./src/intermediary-02/06-traits.md)
    - [Enum especial Option](./src/intermediary-02/07-option.md)
    - [Coleções: Vec](./src/intermediary-02/08-vec.md)
    - [Coleções: HashSet](./src/intermediary-02/09-hashset.md)
    - [Coleções: HashMap](./src/intermediary-02/10-hashmap.md)
    - [Tratamento de erros](./src/intermediary-02/11-result.md)
    - [Macro panic!](./src/intermediary-02/12-panic.md)
    - [Testes](./src/intermediary-02/13-tests.md)
    - [Mini projeto - Snake Game](./src/intermediary-02/14-snake.md)
- [Avançado 1](./src/advanced-01/)
  - [Lifetimes](./src/advanced-01/01-lifetimes.md)
  - [Smart Pointers](./src/advanced-01/02-smart-pointers.md)
    - [Box\<T>](./src/advanced-01/02-smart-pointers-box.md)
    - [Rc\<T>](./src/advanced-01/02-smart-pointers-rc.md)
    - [RefCell\<T>](./src/advanced-01/02-smart-pointers-refcell.md)
  - [Closures](./src/advanced-01/03-closures.md)
  - [Threads](./src/advanced-01/04-threads.md)
    - [Arc\<T> e Mutex\<T>](./src/advanced-01/04-threads-arc-mutex.md)
    
## Como Contribuir

Contribuições fazem com que a comunidade open source seja um lugar incrível para aprender, inspirar e criar. Todas contribuições
são **extremamente apreciadas**, para verificar como contribuir [clique aqui](CONTRIBUTING.md)



## [Mdbook](https://rust-lang.github.io/mdBook/)

Para realizar o build do projeto em livro você pode utilizar a ferramenta [mdbook](https://rust-lang.github.io/mdBook/). Assim fica mais fácil a leitura e você pode exportar até mesmo para o kindle. Para isso use um dos comandos abaixo:

```sh
$ mdbook build
```
Ou acesse [este link](https://rust4noobs.pgjbz.dev/) para ter acesso ao livro online

Para servir o livro localmente use:

```sh
$ mdbook serve
```

## Autores

- Paulo Gabriel Justino Bezerra - Desenvolvedor Java - [Linkedin](https://www.linkedin.com/in/paulogjbezerra/)

---

<p align="center">
  <a href="https://github.com/he4rt/4noobs" target="_blank">
    <img src="./src/assets/footer_4noobs.svg" width="380">
  </a>
</p>