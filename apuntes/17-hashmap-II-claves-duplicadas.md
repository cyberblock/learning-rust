# 17 · HashMap II: claves duplicadas y actualizar valores

- **Vídeo:** https://www.youtube.com/watch?v=zBWrq4RYQNk
- **Duración:** 11:08

## Resumen

¿Qué pasa si insertas una clave que ya existe? El vídeo muestra dos herramientas:

- **`insert`**: **reemplaza** el valor y además **devuelve el valor anterior** (para no perderlo).
- **`entry(clave).or_insert(valor)`**: **solo inserta si la clave no existe**, y devuelve una
  **referencia mutable** al valor, con la que se puede modificar.

Código de partida (del vídeo 16):

```rust
use std::collections::HashMap;

fn main() {
    let mut notas = HashMap::new();
    notas.insert("Ana", 8.5);
    notas.insert("Luis", 4.2);
    notas.insert("Marta", 9.1);
    notas.insert("Pedro", 6.4);
}
```

## `insert` con una clave repetida: reemplaza

```rust
notas.insert("Ana", 9.5);   // "Ana" ya existía con 8.5
```

- **No da error** y **no duplica** la clave: Ana aparece una sola vez, con el **nuevo valor (9.5)**.
- Ocurre porque `insert` **reemplaza** el valor cuando la clave existe, no por ser la última línea.

## `insert` devuelve el valor anterior

```rust
let valor_anterior = notas.insert("Ana", 9.5);   // Option<f64>

for (alumno, nota) in &notas {
    println!("{} tiene una nota de {}", alumno, nota); // Ana tiene una nota de 9.5 ...
}
println!("{:?}", valor_anterior);                     // Some(8.5)
```

- `insert` hace **dos cosas**: reemplaza el valor y **devuelve el anterior**.
- Lo devuelve como **`Option`**: `Some(anterior)` si la clave ya existía y `None` si es nueva.
- Es útil en HashMaps grandes, donde puedes reemplazar un valor sin darte cuenta y perderlo si no lo guardas.

## `entry().or_insert()`: insertar solo si la clave NO existe

```rust
notas.entry("Ana").or_insert(9.5);    // Ana ya existe → no hace nada (sigue con 8.5)
notas.entry("Marcos").or_insert(9.5); // Marcos no existe → lo inserta con 9.5
```

- **`entry(clave)`** busca esa clave.
- **`.or_insert(valor)`** inserta el valor **solo si la clave no existe**. Si ya existe, **no reemplaza**.

## `or_insert` devuelve una referencia mutable

```rust
let nota = notas.entry("Ana").or_insert(9.5);   // nota: &mut f64 → apunta al 8.5 de Ana
*nota = 2.7;                                    // modificamos el valor dentro del HashMap

for (alumno, nota) in &notas {
    println!("{} tiene una nota de {}", alumno, nota); // Ana tiene una nota de 2.7 ...
}
```

- Como "Ana" existe, no se inserta el 9.5, pero `or_insert` devuelve una **referencia mutable (`&mut f64`)**
  al valor que ya hay (8.5).
- Con el operador de desreferencia **`*`** (vídeo 11) se cambia ese valor **directamente dentro del HashMap**.
- Si la clave no existiera, la referencia apuntaría al valor recién insertado.

## Recapitulando

| Método | Si la clave **ya existe** | Si la clave **no existe** | Devuelve |
|--------|---------------------------|---------------------------|----------|
| `insert(k, v)` | **Reemplaza** el valor | Inserta el par | `Option<V>` con el valor **anterior** (`Some`/`None`) |
| `entry(k).or_insert(v)` | **No hace nada** | Inserta el par | `&mut V`: referencia **mutable** al valor que queda en el mapa |

> **Consejo extra (no aparece en el vídeo):** el patrón clásico para **contar apariciones** combina
> todo lo anterior:
>
> ```rust
> let texto = "hola mundo hola rust";
> let mut conteo = HashMap::new();
> for palabra in texto.split_whitespace() {
>     *conteo.entry(palabra).or_insert(0) += 1;
> }
> println!("{:?}", conteo); // {"hola": 2, "mundo": 1, "rust": 1} (el orden puede variar)
> ```
