# 09 · Vectores I: creación y uso

- **Vídeo:** https://www.youtube.com/watch?v=OKm246BwuFg
- **Duración:** 19:08

## Resumen

Qué es un `Vec<T>`, cómo se guarda en memoria (cabecera en el stack con `ptr`/`len`/`cap` y elementos en
el heap), cuándo usar un vector en vez de un array y las tres formas de crearlo: `vec![...]`,
`Vec::new()` + `push` y `Vec::with_capacity(n)`.

## ¿Qué es un `Vec<T>`?

- Una **colección de elementos del mismo tipo** `T` (enteros, strings, decimales…).
- Los elementos se guardan en el **heap** (los arrays normalmente van en el stack).
- **Puede cambiar de tamaño dinámicamente**: se añaden o quitan elementos en tiempo de ejecución.
  Esta es la gran diferencia con los arrays.

## ¿Qué guarda un `Vec` en memoria?

En el **stack** guarda una **cabecera** con 3 datos. Los **elementos** van en el **heap**.

| Campo | Significado |
|-------|-------------|
| `ptr` | Puntero al heap, donde están los elementos |
| `len` | Cuántos elementos hay **ahora** |
| `cap` | Cuántos elementos **caben sin recolocar** |

**Recolocación (*reallocation*):** el vector reserva un espacio en el heap. Si lo llenas y sigues
añadiendo, Rust reserva un espacio **más grande** y **mueve todos los elementos** allí.

> Analogía: una caja llena. Para meter más cosas, las pasas todas a una caja más grande.

Esta operación **consume recursos**. `cap` indica cuántos elementos caben antes de que haga falta.

## ¿Cuándo usar un vector en lugar de un array?

Cuando **no sabes de antemano cuántos datos** vas a almacenar:

- Usuarios de una aplicación.
- Resultados de una búsqueda (¿20 o 20 000?).
- Líneas de un archivo externo.
- Registros que devuelve una consulta a una base de datos.

Si sabes con certeza que siempre serán, por ejemplo, 5 elementos, usa un **array**.

## 1. Crear un vector con valores: `vec!`

```rust
fn main() {
    let numeros = vec![1, 2, 3];   // numeros: Vec<i32>  (tipo inferido)
    println!("{:?}", numeros);     // [1, 2, 3]
}
```

- `vec!` es una **macro** (termina en `!`).
- Los vectores **no implementan `Display`**, sino **`Debug`**, así que se imprimen con `{:?}` (vídeo 8).

## 2. Crear un vector vacío y añadir con `push`

```rust
fn main() {
    let mut numeros: Vec<i32> = Vec::new(); // vacío
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);
    println!("{:?}", numeros);              // [10, 20, 30]
}
```

- Tiene que ser **`mut`**, porque va a crecer.
- `push(valor)` añade un elemento al final.

### Inferencia del tipo

El tipo se puede omitir si Rust puede deducirlo del **primer `push`**:

```rust
let mut numeros = Vec::new(); // Rust infiere Vec<i32> gracias al push de abajo
numeros.push(10);
```

Pero si **no hay ningún `push`**, no puede inferirlo:

```rust
let mut numeros = Vec::new(); // ❌ type annotations needed (Vec<{unknown}>)
```

Solución: indicar el tipo explícitamente con `let mut numeros: Vec<i32> = Vec::new();`.

- Si declaras `mut` y no añades nada, sale un aviso amarillo, **no un error**:
  *variable does not need to be mutable*. Desaparece en cuanto haces un `push`.

## 3. Reservar capacidad: `Vec::with_capacity`

```rust
fn main() {
    let mut numeros: Vec<i32> = Vec::with_capacity(500);
    numeros.push(25);
    println!("{:?}", numeros); // [25]
}
```

- Reserva de entrada espacio en el heap para `n` elementos, lo que **evita recolocaciones** mientras no
  se supere esa cantidad. Mejora el rendimiento cuando conoces aproximadamente el tamaño.
- Con 5 elementos apenas se nota; con cientos o miles tiene más sentido.
- Según el profesor no es tan frecuente: lo habitual es `Vec::new()`, porque normalmente no sabes cuántos
  elementos habrá.

> **Nota (precisión):** la capacidad **no es un límite máximo**. Si añades más elementos de los
> reservados, el vector sigue creciendo (recolocando). Puedes consultar los valores con `numeros.len()`
> y `numeros.capacity()`.
