# Operadores

Antes de entrarmos nas estruturas condicionais, precisamos saber os operadores.

## Operadores matemáticos

| Tipo                 | Simbolo | Ação                         |
| ---                  | ---     | ---                          |
| Soma                 | +       | Soma dois valores            |
| Subtração            | -       | Subtrai dois valores        |
| Multiplicação        | *       | Multiplica dois valores      |
| Divisão              | /       | Divide dois valores          |
| Mod                  | %       | Resto de divisão          |
| Soma atribui         | +=      | Soma e atribui o valor       |
| Subtrai e atribui    | -=      | Subtrai e atribui o valor    |
| Multiplica e atribui | *=      | Multiplica e atribui o valor |
| Divide e atribui     | /=      | Divide e atribui o valor     |

No capítulo sobre os [tipos de dados](./04-data-types.md) mostramos como declarar variáveis e também sobre o caso do Rust precisar de uma palavra extra para essas variáveis serem modificadas, mas não a utilizamos, neste capítulo iremos utilizar, nos exemplos dos usos dos operadores matemáticos.

```rust
fn main() {
    let mut n = 10 - 1; //n = 9
    n = 1 + 2; //n = 3
    n = 10 * 2; //n = 20
    n = 10 / 2; //n = 5
    /*
    Agora temos o valor de n = a 5, iremos realizar operações de atribuições com base neste valor.
    */
    n += 1; //n = 6
    n -= 2; //n = 4
    n *= 3; //n = 12
    n /= 4; //n = 3
}
```

## Operadores lógicos
| Tipo        | Simbolo | Ação                                          |
| ---         | ---     | ---                                           |
| Igual       | ==      | Compara se dois valores são iguais            |
| Diferente   | !=      | Verifica se dois valores são diferentes       |
| Maior       | >       | Verifica se um valor é maior que outro        |
| Menor       | <       | Verifica se um valor é menor que outro        |
| Maior igual | >=      | Verifica se um valor é maior ou igual a outro |
| Maior igual | <=      | Verifica se um valor é menor ou igual a outro |

O uso dos operadores lógicos iremos ver na [próxima](./06-conditions.md) parte



