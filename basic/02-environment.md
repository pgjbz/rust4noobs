# Ambiente

## Instalação do Rust

### Linux

As distribuições Linux testadas para a instalação de Rust foram:

- Arch Linux
- Fedora
- Ubuntu

Para realizar a instalação da linguagem é necessário ter a ferramenta [curl](https://curl.se/) instalada no sistema, para a instalação do `curl` nas distribuições testadas foram utilizados os seguintes métodos:

Arch Linux:

```
sudo pacman -S curl
```

Fedora:

```
sudo dnf install curl
```

Ubuntu:

```
sudo apt install curl
```

Com a ferramenta `curl` instalada foi utilizado o método de instalação [recomendado](https://www.rust-lang.org/pt-BR/learn/get-started) no site da linguagem:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Este comando ira baixar o instalador do Rust e executa-lo no shell, para seguir escolha a opção 1 aperte enter. A instalação seria realizada.

### Windows

OBS. Método de instalação utilizando o gerenciador de pacotes do Windows oficial da Microsoft [winget](https://winget.run/).

Para instalar o Rust no Windows execute o seguinte comando em um PowerShell como administrador:

```
winget install -e --id Rustlang.rust-gnu-x64
```

### Validando instalação

Para validarmos a instalação utilizamos o comando:

```
rustup --version
```

Ele deve nos mostrar algo parecido com isso:

<p>
rustup 1.24.3 (ce5817a94 2021-05-31)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.56.1 (59eed8a2a 2021-11-01)`
</p>


# IDEs

Temos algumas [IDE](https://pt.wikipedia.org/wiki/Ambiente_de_desenvolvimento_integrado)'s que podem facilitar a nossa vida no desenvolvimento utilizando Rust
- [IntelliJ](https://www.jetbrains.com/pt-br/idea/download/) com o Plugin para Rust
- [VSCode](https://code.visualstudio.com/) com os seguintes plugins:
    - [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) para debug
    - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) linguage server é util para lermos a documentação e termos o intellisense enquanto codamos
    - [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) para nos ajudar no gerenciamento de dependência do projeto