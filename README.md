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

- [Básico](https://rust4noobs.pgjbz.dev/basic/index.html)
    - [Sobre a linguagem](https://rust4noobs.pgjbz.dev/basic/01-about.html)
    - [Ambiente](https://rust4noobs.pgjbz.dev/basic/02-environment.html)
    - [Hello World](https://rust4noobs.pgjbz.dev/basic/03-hello-world.html)
    - [Tipos de dados](https://rust4noobs.pgjbz.dev/basic/04-data-types.html)
    - [Operadores](https://rust4noobs.pgjbz.dev/basic/05-operators.html)
    - [Condicionais](https://rust4noobs.pgjbz.dev/basic/06-conditions.html)
    - [Loops](https://rust4noobs.pgjbz.dev/basic/07-loops.html)
    - [Funções](https://rust4noobs.pgjbz.dev/basic/08-functions.html)
    - [Arrays](https://rust4noobs.pgjbz.dev/basic/09-arrays.html)
    - [Exercícios](https://rust4noobs.pgjbz.dev/basic/10-exercises.html)
- [Intermediário 1](https://rust4noobs.pgjbz.dev/intermediary-01/index.html)
    - [Strings](https://rust4noobs.pgjbz.dev/intermediary-01/01-strings.html)
    - [Introdução a Ponteiros](https://rust4noobs.pgjbz.dev/intermediary-01/02-pointers-intro.html)
    - [Ownership](https://rust4noobs.pgjbz.dev/intermediary-01/03-ownership.html)
    - [Tuplas](https://rust4noobs.pgjbz.dev/intermediary-01/04-tuples.html)
    - [Slices](https://rust4noobs.pgjbz.dev/intermediary-01/05-slices.html)
    - [Entrada de dados](https://rust4noobs.pgjbz.dev/intermediary-01/06-user-input.html)
    - [Exercícios](https://rust4noobs.pgjbz.dev/intermediary-01/07-exercises.html)
- [Intermediário 2](https://rust4noobs.pgjbz.dev/intermediary-02/index.html)
    - [Struct](https://rust4noobs.pgjbz.dev/intermediary-02/01-structs.html)
    - [Enum](https://rust4noobs.pgjbz.dev/intermediary-02/02-enums.html)
    - [Match](https://rust4noobs.pgjbz.dev/intermediary-02/03-match.html)
    - [Módulos](https://rust4noobs.pgjbz.dev/intermediary-02/04-modules.html)
    - [Generics](https://rust4noobs.pgjbz.dev/intermediary-02/05-generics.html)
    - [Traits](https://rust4noobs.pgjbz.dev/intermediary-02/06-traits.html)
    - [Enum especial Option](https://rust4noobs.pgjbz.dev/intermediary-02/07-option.html)
    - [Coleções: Vec](https://rust4noobs.pgjbz.dev/intermediary-02/08-vec.html)
    - [Coleções: HashSet](https://rust4noobs.pgjbz.dev/intermediary-02/09-hashset.html)
    - [Coleções: HashMap](https://rust4noobs.pgjbz.dev/intermediary-02/10-hashmap.html)
    - [Tratamento de erros](https://rust4noobs.pgjbz.dev/intermediary-02/11-result.html)
    - [Macro panic!](https://rust4noobs.pgjbz.dev/intermediary-02/12-panic.html)
    - [Testes](https://rust4noobs.pgjbz.dev/intermediary-02/13-tests.html)
    - [Mini projeto - Snake Game](https://rust4noobs.pgjbz.dev/intermediary-02/14-snake.html)
- [Avançado 1](https://rust4noobs.pgjbz.dev/advanced-01/index.html)
  - [Lifetimes](https://rust4noobs.pgjbz.dev/advanced-01/01-lifetimes.html)
  - [Smart Pointers](https://rust4noobs.pgjbz.dev/advanced-01/02-smart-pointers.html)
    - [Box\<T>](https://rust4noobs.pgjbz.dev/advanced-01/02-smart-pointers-box.html)
    - [Rc\<T>](https://rust4noobs.pgjbz.dev/advanced-01/02-smart-pointers-rc.html)
    - [RefCell\<T>](https://rust4noobs.pgjbz.dev/advanced-01/02-smart-pointers-refcell.html)
  - [Closures](https://rust4noobs.pgjbz.dev/advanced-01/03-closures.html)
  - [Threads](https://rust4noobs.pgjbz.dev/advanced-01/04-threads.html)
    - [Arc\<T>, Mutex\<T> e RwLock\<T>](https://rust4noobs.pgjbz.dev/advanced-01/04-threads-arc-mutex-rwlock.html)]
  - [Produtores e Consumidores](https://rust4noobs.pgjbz.dev/advanced-01/05-producers-consumers.html)
- [Extras](https://rust4noobs.pgjbz.dev/extras/index.html)
- [Referências](https://rust4noobs.pgjbz.dev/SOURCES.html)
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

## Pessoas que melhoraram este projeto

<!-- readme: collaborators,contributors -start -->
<table>
<tr>
    <td align="center">
        <a href="https://github.com/pgjbz">
            <img src="https://avatars.githubusercontent.com/u/22059237?v=4" width="100;" alt="pgjbz"/>
            <br />
            <sub><b>Paulo Gabriel Justino Bezerra</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/silasms">
            <img src="https://avatars.githubusercontent.com/u/84996376?v=4" width="100;" alt="silasms"/>
            <br />
            <sub><b>Silas Medeiros</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/cherryramatisdev">
            <img src="https://avatars.githubusercontent.com/u/86631177?v=4" width="100;" alt="cherryramatisdev"/>
            <br />
            <sub><b>Cherry Ramatis</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/danielcti">
            <img src="https://avatars.githubusercontent.com/u/31549323?v=4" width="100;" alt="danielcti"/>
            <br />
            <sub><b>Daniel Cavalcanti</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/herniqeu">
            <img src="https://avatars.githubusercontent.com/u/95002561?v=4" width="100;" alt="herniqeu"/>
            <br />
            <sub><b>Gzsr</b></sub>
        </a>
    </td></tr>
</table>
<!-- readme: collaborators,contributors -end -->

<p align="center">
  <a href="https://github.com/he4rt/4noobs" target="_blank">
    <img src="./src/assets/footer_4noobs.svg" width="380">
  </a>
</p>