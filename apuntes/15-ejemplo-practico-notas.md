# 15 · Vectores: ejemplo práctico con notas (media y nota más alta)

- **Vídeo:** https://www.youtube.com/watch?v=dq-ifVDamss
- **Duración:** 22:41

## Resumen

Ejercicio final de la sección de vectores. Dado un vector de notas decimales (0–10), hay que obtener:

1. Los **aprobados** (nota ≥ 5), en un nuevo vector.
2. La **nota media**.
3. La **nota más alta**.

Por el camino aparecen tres cosas nuevas: la conversión con **`as f64`**, por qué **`max()` no funciona
con `f64`** (el valor NaN y el trait `Ord`) y cómo resolverlo con **`max_by` + `partial_cmp` + `unwrap`**.

> 💡 Intenta resolverlo tú antes de leer la solución.

## Para profundizar: The Rust Book

El profesor recomienda **"The Rust Programming Language"** (el libro oficial):
https://doc.rust-lang.org/book/. El **capítulo 8 (*Common Collections*)**, apartado 8.1, trata los
vectores con más detalle: crear, actualizar, leer e iterar, guardar distintos tipos con `enum`, etc.

## Solución completa

```rust
fn main() {
    let notas: Vec<f64> = vec![7.5, 4.1, 8.2, 9.1, 3.8, 6.4];

    // 1) Aprobados
    let aprobados: Vec<&f64> = notas
        .iter()
        .filter(|nota| **nota >= 5.0)
        .collect();

    // 2) Media
    let suma: f64 = notas.iter().sum();
    let media: f64 = suma / notas.len() as f64;

    // 3) Nota más alta
    let nota_maxima = notas
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap());   // Option<&f64>

    println!("Aprobados: {:?}", aprobados);       // [7.5, 8.2, 9.1, 6.4]
    println!("Media: {:?}", media);               // 6.516666666666666
    println!("Nota máxima: {:?}", nota_maxima);   // Some(9.1)
}
```

## 1. Aprobados

- `notas.iter()` devuelve **referencias** (`&f64`), no los valores. Por eso el vector resultante es
  **`Vec<&f64>`**, no `Vec<f64>`.
- `iter()` **no consume** el vector: `notas` sigue existiendo y se puede volver a usar para la media y el
  máximo. Con `into_iter()` sí se consumiría.
- **`**nota`**: `iter()` da `&f64` y `filter` pasa a la closure una referencia a ese elemento, es decir,
  `&&f64`. Hay que desreferenciar **dos veces** para llegar al valor.
- Hay que comparar con **`5.0`**, no con `5`: Rust no mezcla `f64` con enteros.
- `collect()` guarda los elementos filtrados en un vector nuevo.

## 2. Media

```rust
let suma: f64 = notas.iter().sum();
let media: f64 = suma / notas.len() as f64;
```

- `len()` devuelve un **`usize`** (entero). No se puede dividir un `f64` entre un `usize`.
- **`as f64`** convierte el `usize` a `f64`.

**¿Por qué no funcionaba en una sola línea?** (el profesor lo deja como pregunta abierta). Si escribes
`notas.iter().sum() / notas.len() as f64`, falla porque **`sum()` es genérico**: puede devolver
distintos tipos y, dentro de una división, el compilador no puede deducir cuál quieres. Al separarlo en
`let suma: f64 = ...`, la anotación le dice el tipo. En una sola línea se resuelve con el **turbofish**:

```rust
let media = notas.iter().sum::<f64>() / notas.len() as f64;
```

## 3. Nota más alta: por qué `max()` no sirve con decimales

```rust
let nota_maxima = notas.iter().max(); // ❌ the trait bound `f64: Ord` is not satisfied
```

- `max()` necesita que el tipo implemente el trait **`Ord`** (**ordenación total**). Los enteros lo
  implementan; **`f32`/`f64` no**.
- ¿Por qué? Por el valor especial **`NaN`** (*Not a Number*), que aparece en operaciones matemáticas
  inválidas. `NaN` **no se puede comparar con nada, ni siquiera consigo mismo** (`NaN == NaN` es falso).
  Así que Rust no puede garantizar que dos `f64` siempre se puedan ordenar.

### La solución: `max_by` + `partial_cmp` + `unwrap`

```rust
.max_by(|a, b| a.partial_cmp(b).unwrap())
```

- **`max_by(closure)`**: calcula el máximo usando **la comparación que tú le indicas**.
- **`partial_cmp`** (*partial compare*): comparación **parcial**. Devuelve un **`Option<Ordering>`**:
  - `Some(Less)`, `Some(Equal)` o `Some(Greater)` si se pueden comparar.
  - `None` si no se pueden comparar (por ejemplo, si uno es `NaN`).
- `max_by` espera un `Ordering`, pero `partial_cmp` da un **`Option<Ordering>`**: *expected `Ordering`,
  found `Option<Ordering>`*.
- **`unwrap()`** saca el valor de dentro del `Some`. Aquí sabemos que no hay ningún `NaN`, así que es seguro.
- El resultado de `max_by` es un **`Option<&f64>`** (`None` si el vector estuviera vacío). Por eso se
  imprime como `Some(9.1)`.

> ⚠️ `unwrap()` hace que el programa **se caiga (*panic*)** si encuentra un `None`. Úsalo solo cuando
> estés seguro de que habrá un valor, como aquí.

## Conceptos nuevos del vídeo

| Concepto | Para qué |
|----------|----------|
| `as f64` | Convertir entre tipos numéricos (por ejemplo, `usize` → `f64`) |
| Trait `Ord` | Ordenación total. Lo tienen los enteros, no los `f64` |
| `NaN` | Valor especial de los decimales que no se puede comparar |
| `max_by(...)` | Máximo con un criterio de comparación propio |
| `partial_cmp` | Comparación parcial, devuelve `Option<Ordering>` |
| `unwrap()` | Extrae el valor de un `Some` (hace *panic* si es `None`) |
