# 10 · Vectores II: eliminar y acceder a elementos

- **Vídeo:** https://www.youtube.com/watch?v=2g2yv2hv9uM
- **Duración:** 18:21

## Resumen

Eliminar el último elemento con `pop()`, que devuelve un **`Option<T>`**, y acceder a posiciones de dos
formas: con **corchetes `v[i]`** (hace *panic* si el índice no existe) y con **`.get(i)`** (devuelve `Some`
o `None` y el programa sigue). Conclusión: **Rust evita el `null`**.

## Noticia comentada al inicio

El profesor comenta una noticia de Computer Hoy (Linux 7.0): el experimento de usar Rust en el kernel de
Linux se da por concluido con éxito y Rust se queda. El kernel está escrito mayoritariamente en C, y la
mayoría de las vulnerabilidades graves son de memoria (en torno al 70 %), justo el tipo de fallo que Rust
elimina. La idea no es sustituir C, sino que Rust lo acompañe poco a poco.

## Código de partida

```rust
fn main() {
    let mut numeros: Vec<i32> = Vec::new();
    numeros.push(25);
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);
    println!("{:?}", numeros); // [25, 10, 20, 30]
}
```

## Eliminar el último elemento: `pop()`

```rust
let ultimo = numeros.pop();   // ultimo: Option<i32>
println!("{:?}", numeros);    // [25, 10, 20]
println!("{:?}", ultimo);     // Some(30)
```

- `pop()` **elimina el último elemento** y lo devuelve envuelto en un **`Option<T>`**.
- **`Option`** significa "puede haber un valor o puede no haberlo":
  - **`Some(valor)`** → había algo (*some* = "algo").
  - **`None`** → no había nada.
- Con el vector **vacío**, `pop()` devuelve **`None`** **sin error**: el programa no se cae.
  En otros lenguajes tendrías que controlarlo con excepciones. Rust nunca devuelve basura ni valores inválidos.

## Acceder a posiciones

Los índices empiezan en **0**: en `[25, 10, 20, 30]`, la posición 2 es `20`.

### 1. Con corchetes `v[i]`

```rust
println!("{}", numeros[2]); // 20
```

- Aquí se usa `{}` (no `{:?}`) porque un **elemento suelto** es un `i32`, que sí implementa `Display`.
  El vector completo no.
- ⚠️ Si el índice **no existe**, el programa **se cae en tiempo de ejecución** (*panic*):

```
index out of bounds: the len is 3 but the index is 3
```

(Después del `pop()` solo quedan 3 elementos, posiciones 0 a 2, así que ni siquiera existe la 3).

### 2. Con `.get(i)`: la forma segura

```rust
println!("{:?}", numeros.get(3)); // Some(30)
println!("{:?}", numeros.get(5)); // None → el programa CONTINÚA
```

- `get` también devuelve un **`Option<&T>`**: `Some(valor)` si la posición existe y `None` si no.
- Como devuelve un `Option`, hay que imprimirlo con **`{:?}`**.
- **Nunca hace caer el programa.**

### Comparativa

| | `v[i]` | `v.get(i)` |
|---|---|---|
| Devuelve | El valor directamente | `Option` (`Some(&valor)` / `None`) |
| Índice inexistente | 💥 *panic*: `index out of bounds` | `None` y el programa sigue |
| Cuándo usarlo | Cuando estás seguro de que el índice existe | Cuando el índice puede no existir |

## Rust evita el `null`

Con un índice inexistente o una colección vacía:

- **Java** podría devolver `null`, el caldo de cultivo de los `NullPointerException`.
- **C** podría devolver algo indefinido.
- **Python** lanzaría una excepción.
- **Rust** te **obliga a manejar explícitamente** la posibilidad de que no haya valor mediante `Option`.
  Así se evitan los accesos inválidos y los comportamientos inesperados.

## Conclusiones

- `Some()` es una variante de `Option`.
- `pop()` devuelve un `Option<T>`.
- `Some(x)` indica que hay valor; `None`, que no lo hay.
- **Rust evita `null`.**
