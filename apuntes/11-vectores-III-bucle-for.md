# 11 · Vectores III: recorrer un vector con `for`

- **Vídeo:** https://www.youtube.com/watch?v=VfS3EKlMvzQ
- **Duración:** 15:43

## Resumen

Cómo recorrer un vector con `for` y por qué, a diferencia de Java o C#, **después del bucle el vector
puede dejar de existir** (*move*). Hay tres formas de recorrerlo: por valor (`v`), por referencia (`&v`)
y por referencia mutable (`&mut v`), esta última con el operador de desreferencia `*`.

## Sintaxis básica

```rust
fn main() {
    let numeros = vec![1, 2, 3];

    for n in numeros {       // "por cada n en numeros"
        println!("{}", n);   // 1, 2, 3
    }
}
```

Es la misma idea que el *for-each* de otros lenguajes (Java, C#…).

## Caso 1: `for n in v` → el vector se MUEVE

```rust
let numeros = vec![1, 2, 3];

for n in numeros {
    println!("{}", n);
}

println!("{:?}", numeros); // ❌ borrow of moved value: `numeros`
```

¿Por qué falla? Al empezar un `for`, internamente se crea un **iterador** (en Java, C# o Python también).
La diferencia en Rust:

- El dueño del vector era `numeros`.
- Al crearse el iterador, **la propiedad pasa al iterador** (*move*).
- `numeros` queda **invalidada**, como si no existiera, así que no se puede usar después del bucle.

¿Por qué en Java no pasa? Porque Java no tiene ownership (tiene *garbage collector*): el bucle trabaja con
**referencias** a los elementos y el `ArrayList` sigue disponible después. Rust es más estricto.

## Caso 2: `for n in &v` → préstamo, el vector sigue vivo

```rust
fn main() {
    let numeros = vec![1, 2, 3];

    for n in &numeros {          // n: &i32 — se PRESTA el vector al iterador
        println!("{}", n);       // 1, 2, 3
    }

    println!("{:?}", numeros);   // ✅ [1, 2, 3]
}
```

Con `&` no se transfiere la propiedad: se hace **borrowing**. `numeros` sigue siendo la dueña y se puede
usar después del bucle.

## Caso 3: `for n in &mut v` → modificar los elementos

Para modificar cada elemento (por ejemplo, sumarle 1):

```rust
fn main() {
    let mut numeros = vec![1, 2, 3];   // 1) el vector tiene que ser mut

    for n in &mut numeros {            // 2) referencia mutable → n: &mut i32
        *n += 1;                       // 3) desreferenciar con *
    }

    println!("{:?}", numeros);         // [2, 3, 4]
}
```

Hacen falta **tres cosas**:

1. Declarar el vector como **`mut`**.
2. Recorrerlo con **`&mut`**.
3. Usar el operador de **desreferencia `*`**.

**¿Qué es `*`?** El **operador de desreferencia**: significa "dame el valor al que apunta esta referencia"
(igual que en C++). `n` es una referencia y **no se le puede sumar 1 a una referencia**, solo al valor
apuntado. Sin `*` da error.

## Resumen de los tres casos

| Forma | Tipo de `n` | ¿Se puede usar el vector después? | ¿Se pueden modificar los elementos? |
|-------|-------------|-----------------------------------|-------------------------------------|
| `for n in v` | `T` | ❌ No (*move* al iterador) | — |
| `for n in &v` | `&T` | ✅ Sí (préstamo) | ❌ No |
| `for n in &mut v` | `&mut T` | ✅ Sí | ✅ Sí, con `*n` (el vector debe ser `mut`) |

> El profesor admite que es un vídeo denso, pero clave para entender en qué se diferencia Rust de otros lenguajes.
