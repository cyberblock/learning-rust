# 14 · Iteradores VI: funciones útiles para cálculos

- **Vídeo:** https://www.youtube.com/watch?v=IC-GOBEF0Vc
- **Duración:** 15:19

## Resumen

`filter`, `map` y `collect` **transforman** datos. Este vídeo presenta funciones que **obtienen
resultados**: `sum`, `find`, `count`, `any`, `all`, `max` y `min` (parecidas a las de Excel). La lección
transversal es que hay que fijarse en **qué tipo devuelve cada una**: un número, un `bool` o un `Option`.

Vector de partida en todos los ejemplos:

```rust
let numeros: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
```

## `sum`: sumar

```rust
let resultado: i32 = numeros
    .iter()
    .filter(|n| *n % 2 == 0)   // 2, 4, 6
    .sum();                     // 12

let resultado: i32 = numeros
    .iter()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * 2)             // 4, 8, 12
    .sum();                     // 24
```

⚠️ Error típico (le pasa al profesor al reutilizar el código anterior): `sum` devuelve **un número**, no un
vector. Si la variable está declarada como `Vec<i32>`, no compila. Tiene que ser `i32`.

## `find`: buscar el primer elemento que cumple una condición

```rust
let resultado = numeros.iter().find(|n| **n > 3);  // resultado: Option<&i32>
println!("{:?}", resultado);                        // Some(4)

let resultado = numeros.iter().find(|n| **n > 7);
println!("{:?}", resultado);                        // None
```

- **¿Por qué `**n`?** `iter()` produce `&i32` y `find` recibe una **referencia a ese elemento**, así que a
  la closure le llega un **`&&i32`**. Para comparar con `3` hace falta el valor real, así que se
  desreferencia **dos veces**.
- **Devuelve `Option`** porque puede encontrar algo… o no:
  - `Some(valor)` → lo ha encontrado.
  - `None` → no hay ninguno.
- Por eso **no se puede guardar en un `i32`**: si no encuentra nada, "la nada" no es un `i32`. Hay que
  quitar la anotación de tipo (o poner `Option<&i32>`) e imprimir con `{:?}`.

## `count`: contar cuántos cumplen una condición

```rust
let resultado = numeros.iter().filter(|n| *n % 2 == 0).count();
println!("{}", resultado); // 3  (2, 4 y 6)
```

## `any`: ¿**alguno** cumple la condición?

```rust
let resultado = numeros.iter().any(|n| n % 2 == 0);
println!("{}", resultado); // true (hay pares)
```

Devuelve un **`bool`**, así que se imprime con `{}`.

## `all`: ¿**todos** cumplen la condición?

```rust
let resultado = numeros.iter().all(|n| n % 2 == 0);
println!("{}", resultado); // false (no todos son pares)

let pares = vec![2, 4, 6];
println!("{}", pares.iter().all(|n| n % 2 == 0)); // true
```

## `max` y `min`: valor máximo y mínimo

```rust
let maximo = numeros.iter().max();   // Option<&i32>
println!("{:?}", maximo);            // Some(6)

let minimo = numeros.iter().min();
println!("{:?}", minimo);            // Some(1)
```

Devuelven un **`Option`**, así que se imprimen con `{:?}` y salen como `Some(...)`.

> **Nota (corrección):** en el vídeo se dice que `max` devuelve `Option` porque, si todos los valores son
> iguales, "no hay máximo". No es así: con `[6, 6, 6]`, `max()` devuelve `Some(6)`. Devuelve **`None` solo
> cuando el vector está vacío**, porque entonces no hay ningún elemento que devolver.

## Tabla resumen

| Función | Qué hace | Devuelve | Imprimir con |
|---------|----------|----------|--------------|
| `sum()` | Suma los elementos | Número (hay que indicar el tipo: `i32`…) | `{}` |
| `find(cond)` | Primer elemento que cumple la condición | `Option<&T>` | `{:?}` |
| `count()` | Cuántos elementos hay (tras filtrar) | `usize` | `{}` |
| `any(cond)` | ¿Alguno cumple? | `bool` | `{}` |
| `all(cond)` | ¿Todos cumplen? | `bool` | `{}` |
| `max()` / `min()` | Mayor / menor valor | `Option<&T>` | `{:?}` |

Se pueden **combinar y encadenar** para construir expresiones más complejas.
