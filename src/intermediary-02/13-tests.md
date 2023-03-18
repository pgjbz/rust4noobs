# Testes

Em Rust temos macros para testarmos nossas features, como os macros para "[assertions](https://en.wikipedia.org/wiki/Assertion)" sendo eles `assert!`, `assert_eq!` e `assert_ne!`. Podemos utiliza-los em nosso código para testar as nossas funcionalidades, utilizando a ideia de [módulos](./04-modules.md) podemos criar um módulo especifico para os testes. E para isso utilizamos a opção de compilação `#[cfg(test)]`, esta configuração irá definir que é um módulo para testes e nesse módulo nós adicionamos nossos códigos de testes, e para informar que são testes utilizamos outro macro `#[test]`.
Este módulo de testes NÃO é compilado em nosso executável final.

Abaixo temos um exemplo da declaração de um módulo de testes já contendo dois testes, utilizando os macros `assert!`, `assert_eq!` e `assert_ne!`:

```rust
#[cfg(test)]
mod tests {

    #[test]
    fn deve_ser_verdadeiro() {
        let maior = 1 > 0;
        assert!(maior)
    }

    #[test]
    fn devem_ser_iguais() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn devem_ser_diferentes() {
        assert_ne!(1 + 1, 10)
    }

}
```

Com apenas esses 3 macros de `assert`, conseguimos testar muitas coisas, vamos a um exemplo mais complexo.

Temos um cliente que deseja fazer a integração de pagamentos com outro serviço e antes de enviar a requisição para realizar a integração, deve ser feito algumas validações, sendo elas:

- O valor a ser pago deve ser maior que 0.0
- O valor a ser pago deve ser menor ou igual a 1000.0

Vamos começar a escrever os nossos testes, antes de escrevemos nosso código.

```rust
mod integracao_de_pagamento {

    pub fn valor_eh_valido(valor: f64) -> bool {
        todo!()
    }

}

#[cfg(test)]
mod tests {

    use super::integracao_de_pagamento;

    #[test]
    fn valida_valor_pagamento_acima_de_dez_deve_retornar_verdadeiro() {
        let valor_valido = integracao_de_pagamento::valor_eh_valido(10.0);
        assert!(valor_valido);
    }

    #[test]
    fn valida_valor_pagamento_abaixo_de_zero_deve_retornar_falso() {
        let valor_valido = integracao_de_pagamento::valor_eh_valido(-10.0);
        assert!(!valor_valido);
    }

    #[test]
    fn valida_valor_pagamento_acima_de_mil_deve_retornar_falso() {
        let valor_valido = integracao_de_pagamento::valor_eh_valido(1001.0);
        assert!(!valor_valido)
    }
}
```

NOTA: Como `tests`é um módulo ele deve importar os outros módulos
NOTA 2: O nome do módulo não necessariamente deve ser "tests"

Para executar o código acima, devemos usar o comando `cargo test`, este comando ira executar todos os testes do projeto. Como esperado todos os testes vão falhar! Vamos escrever o nosso código.

```rust
mod integracao_de_pagamento {

    pub fn valor_eh_valido(value: f64) -> bool {
        valor > 0.0
    }

}
```

Ao executar o comando `cargo test` temos a seguinte saída:

```bash
running 3 tests
test tests::valida_valor_pagamento_acima_de_dez_deve_retornar_verdadeiro ... ok
test tests::valida_valor_pagamento_abaixo_de_zero_deve_retornar_falso ... ok
test tests::valida_valor_pagamento_acima_de_mil_deve_retornar_falso ... FAILED

failures:

---- tests::valida_valor_pagamento_acima_de_mil_deve_retornar_falso stdout ----
thread 'tests::valida_valor_pagamento_acima_de_mil_deve_retornar_falso' panicked at 'assertion failed: !valor_valido', src/lib.rs:29:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::valida_valor_pagamento_acima_de_mil_deve_retornar_falso

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Dois testes passaram e um teste falhou, pelas regras, devemos validar se o valor é menor ou igual a 1000.00 para ser valido.
Vamos alterar a implementação.

```rust
mod integracao_de_pagamento {

    pub fn valor_eh_valido(value: f64) -> bool {
        valor > 0.0 && valor <= 1000.0
    }

}
```

E agora temos todos os testes funcionando.

```bash
    Finished test [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests (target/debug/deps/testes-83f2e46ed2822f78)

running 3 tests
test tests::valida_valor_pagamento_acima_de_mil_deve_retornar_falso ... ok
test tests::valida_valor_pagamento_acima_de_dez_deve_retornar_verdadeiro ... ok
test tests::valida_valor_pagamento_abaixo_de_zero_deve_retornar_falso ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests testes

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Usamos apenas uma opção de `assert`, vamos utilizar outro teste.

Vamos usar aquela conta que todo mundo ama fazer no ensino médio, encontrar os valores de uma equação de segundo grau, e para isso usaremos a [Fórmula de Bhaskara](https://www.infoescola.com/matematica/formula-de-bhaskara/) e também iremos aproveitar para explicar outro conceito que é quando um teste deve causar um `panic`.

Vamos às regras da Fórmula de Bhaskara:

- Para calcular Delta usaremos a fórmula `b²-4*a*c`
- Delta não pode ser negativo
- Precisamos ter três valores (a, b e c)
- Temos que devolver duas raízes
- Para calcular as raízes usaremos (-b +- √delta) / 2*a

Vamos escrever nossos testes:

```rust
mod calculadora {

    pub fn calcular(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
        todo!()
    }

    pub(super) fn calcula_delta(a: f64, b: f64, c: f64) -> Result<f64, String> {
        todo!()
    }

}

#[cfg(test)]
mod tests {

    use super::calculadora;

   
    #[test]
    #[should_panic]
    fn calcular_delta_negativo_esperado_error() {
        calculadora::calcula_delta(1.0, 2.0, 3.0).unwrap();
    }

    #[test]
    fn calcular_delta_nao_deve_haver_erro() {
        let delta = calculadora::calcula_delta(8.0, 7.0, -2.0).unwrap();
        assert_eq!(113.0, delta, "o valor de desta esta errado")
    }

    #[test]
    fn deve_calcular_as_duas_raizes() {
        let resultado = calculadora::calcular(1.0, 3.0, -2.0).unwrap();
        assert_eq!((0.5615528128088303, -3.5615528128088303), resultado);
    }

    #[test]
    #[should_panic(expected = "nao contem raiz real")]
    fn calcular_raizes_com_delta_negativo_esperado_error() {
        calculadora::calcular(1.0, 2.0, 3.0).unwrap();
    }
}
```

Ao executar o comando para os testes, teremos um teste que ira passar, que é o teste `calcular_delta_negativo_esperado_error`, isso acontece porque ele tem que causar um `panic`e o macro `todo!` causa o panic, porém não é o que queremos, então no último testes colocamos a mensagem que esperamos no `panic!`, que é "nao contem raiz real".

Vamos seguir com a implementação do nosso cálculo.

```rust
mod calculadora {

    pub fn calcular(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
        todo!()
    }

    pub(super) fn calcula_delta(a: f64, b: f64, c: f64) -> Result<f64, String> {
        let delta = (b * b) - 4.0 * a * c;
        if delta < 0.0 {
            return Err("nao contem raiz real".to_string())
        } 
        Ok(delta)
    }

}
```

Agora temos dois testes que passam e dois testes que falham. Veja a saída no console:

```bash
    Finished test [unoptimized + debuginfo] target(s) in 0.38s
     Running unittests (target/debug/deps/testes-83f2e46ed2822f78)

running 4 tests
test tests::calcular_delta_nao_deve_haver_erro ... ok
test tests::calcular_raizes_com_delta_negativo_esperado_error - should panic ... FAILED
test tests::calcular_delta_negativo_esperado_error - should panic ... ok
test tests::deve_calcular_as_duas_raizes ... FAILED

failures:

---- tests::calcular_raizes_com_delta_negativo_esperado_error stdout ----
thread 'tests::calcular_raizes_com_delta_negativo_esperado_error' panicked at 'not yet implemented', src/lib.rs:4:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"not yet implemented"`,
 expected substring: `"nao contem raiz real"`
---- tests::deve_calcular_as_duas_raizes stdout ----
thread 'tests::deve_calcular_as_duas_raizes' panicked at 'not yet implemented', src/lib.rs:4:9


failures:
    tests::calcular_raizes_com_delta_negativo_esperado_error
    tests::deve_calcular_as_duas_raizes

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Como esperado aquele método que esta esperando uma mensagem especifica do panic, continua falhando. Vamos implementa-lo.

```rust
mod calculadora {

    pub fn calcular(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
        let delta = calcula_delta(a, b, c)?;
        let menos_b = -b;
        let x1 = (menos_b + delta.sqrt()) / 2.0 * a;
        let x2 = (menos_b - delta.sqrt()) / 2.0 * a;
        Ok((x1, x2))
    }

}
```
Agora ao executar os testes teremos sucesso:

```bash
    Finished test [unoptimized + debuginfo] target(s) in 0.32s
     Running unittests (target/debug/deps/testes-83f2e46ed2822f78)

running 4 tests
test tests::calcular_delta_nao_deve_haver_erro ... ok
test tests::calcular_delta_negativo_esperado_error - should panic ... ok
test tests::deve_calcular_as_duas_raizes ... ok
test tests::calcular_raizes_com_delta_negativo_esperado_error - should panic ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests testes

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```