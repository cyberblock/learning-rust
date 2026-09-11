# 06 · Borrowing (préstamos)

- **Vídeo:** https://www.youtube.com/watch?v=bPVWshg1tC4
- **Duración:** 20:20

## Resumen

Qué es el *borrowing*, por qué hace a Rust más seguro que C++ (no hay punteros colgantes) y las reglas
de las referencias inmutables (`&`) y mutables (`&mut`). Cierra con un ejemplo que "vuela la cabeza":
el mismo código compila o no según el **orden de las líneas**, por el tiempo de vida del préstamo.

## ¿Qué es el borrowing?

Es el mecanismo por el que una variable **deja que otra parte del programa acceda a su valor sin
transferir la propiedad** (ownership).

> Analogía: te **presto un libro**. Puedes leerlo, pero **sigue siendo mío**.

- Se hace creando una **referencia** con `&`.
- Igual que las variables, las referencias son **inmutables por defecto** y pueden hacerse mutables (`&mut`).
- Sea mutable o no, **el dueño original no cambia**.

## Rust vs C++: punteros colgantes

- **C++:** `saludo` (en el stack) apunta a `"Hola"` (en el heap) y un **puntero** apunta también al heap.
  Si destruyes `saludo`, el puntero queda **colgando** (*dangling pointer*) y apunta a memoria liberada.
  El programa compila y se ejecuta con **comportamiento indefinido**. Evitarlo es responsabilidad del programador.
- **Rust:** la **referencia** apunta a `saludo` (en el stack), que a su vez apunta al heap. Si intentas
  destruir el original mientras hay un préstamo vivo, **no compila**. El compilador hace de **guardián**:
  no hay punteros colgantes ni accesos simultáneos peligrosos.
- Rust también tiene punteros, pero se usan mucho menos que en C++ y en casos distintos.

## Primer préstamo (inmutable)

```rust
fn main() {
    let saludo = String::from("hola");
    let referencia = &saludo;   // préstamo inmutable (tipo &String)

    println!("Original: {}", saludo);
    println!("Préstamo: {}", referencia);
}
```

```
Original: hola
Préstamo: hola
```

## Rust impide destruir un valor prestado

```rust
fn main() {
    let saludo = String::from("hola");
    let referencia = &saludo;

    println!("Original: {}", saludo);
    println!("Préstamo: {}", referencia);

    std::mem::drop(saludo);              // ❌ cannot move out of `saludo` because it is borrowed
    println!("Préstamo: {}", referencia); // el préstamo se sigue usando después del drop
}
```

- `std::mem::drop(x)` destruye (libera) un valor.
- Error en **tiempo de compilación**: no se puede mover ni destruir `saludo` mientras exista un préstamo
  que todavía se usa. En C++ esto compilaría y dejaría un puntero colgante.

## Préstamo mutable

Un préstamo inmutable solo permite **leer**:

```rust
let saludo = String::from("hola");
let referencia = &saludo;
referencia.push_str(" mundo cruel"); // ❌ cannot borrow `*referencia` as mutable
```

Para modificar a través del préstamo hacen falta **dos cosas**:

1. El préstamo tiene que ser mutable: `&mut saludo`.
2. La variable original también tiene que ser mutable: `let mut saludo` (no se puede prestar como mutable
   algo que es inmutable).

```rust
fn main() {
    let mut saludo = String::from("hola");
    let referencia = &mut saludo;       // tipo &mut String

    referencia.push_str(" mundo cruel"); // push_str añade texto a un String
    println!("Préstamo: {}", referencia); // hola mundo cruel
}
```

## Regla de exclusividad

```rust
let mut saludo = String::from("hola");
let referencia = &mut saludo;
referencia.push_str(" mundo cruel");

println!("Original: {}", saludo);     // ❌ cannot borrow `saludo` as immutable
                                      //    because it is also borrowed as mutable
println!("Préstamo: {}", referencia); // (el préstamo mutable se usa aquí después)
```

Mientras un **préstamo mutable** esté activo, tiene **acceso exclusivo**: no puedes usar el original
**para nada, ni siquiera para leerlo**. Así se evita que haya dos alias simultáneos al mismo valor.

### Las reglas del borrowing

- ✅ Puedes tener **muchas referencias inmutables** (`&T`) a la vez.
- ✅ **O** puedes tener **una única referencia mutable** (`&mut T`).
- ❌ **No se pueden mezclar** referencias mutables e inmutables al mismo tiempo.

## El orden importa: el préstamo vive hasta su último uso

Si solo cambias el orden de los dos `println!`, **compila**:

```rust
fn main() {
    let mut saludo = String::from("hola");
    let referencia = &mut saludo;
    referencia.push_str(" mundo cruel");

    println!("Préstamo: {}", referencia); // ← último uso del préstamo: aquí termina su vida
    println!("Original: {}", saludo);     // ✅ el original vuelve a estar disponible
}
```

- El **ciclo de vida de un préstamo termina en la última línea donde se usa**, no al final del bloque.
- Una vez que termina, puedes volver a usar el original sin problema.
- Si después vuelves a usar `referencia` otra vez (debajo del `println!` del original), el préstamo
  se alarga hasta esa línea y **vuelve el error**.

> El profesor reconoce que es un vídeo "duro": merece la pena repasarlo hasta tener claras estas reglas,
> porque son la base del ownership y el borrowing.
