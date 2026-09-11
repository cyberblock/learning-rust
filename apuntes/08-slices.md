# 08 · Slices

- **Vídeo:** https://www.youtube.com/watch?v=CH2fpLz39Dg
- **Autor:** [Píldoras Informáticas](https://www.youtube.com/@pildorasinformaticas)
- **Duración:** 17:11

## Resumen

Qué es un *slice* ("rebanada" o "corte"), cómo se representa en memoria y cómo crear slices de un
`String` y de un array. Al imprimir el slice del array aparece un error que sirve para introducir los
**traits** (`Display` vs `Debug`) y el formato `{:?}`.

## ¿Qué es un slice?

> Una **referencia a una parte de una colección de datos que ya existe**.

- Un `String` es una colección de caracteres, así que un slice de un `String` apunta a un trozo del texto.
- De un array de 10 elementos se puede crear un slice que apunte a 3, 6 o 7 de ellos.
- Permite **ver parte de los datos sin copiarlos y sin ser su dueño**.
- Como es una referencia, sigue las reglas del **borrowing** y tiene **lifetime**.

**¿Para qué sirve?** Para ganar **eficiencia** (se evitan copias innecesarias en memoria) y **seguridad**.
Es clave para pasar datos a funciones de forma segura y para diseñar APIs flexibles (se verá más adelante).

## Sintaxis: referencia + rango

```rust
&coleccion[inicio..fin]
```

- `inicio` es **inclusivo** (se incluye) y `fin` es **exclusivo** (no se incluye).
- Las posiciones empiezan en **0**.

## Slice de un `String`

```rust
fn main() {
    let s = String::from("Hola alumnos");
    let saludo = &s[0..4];   // posiciones 0,1,2,3 → "Hola" (el 4, el espacio, se excluye)
    println!("{}", saludo);  // Hola
}
```

### En memoria

- `s` está en el stack, es la **dueña** y apunta a `"Hola alumnos"` en el heap.
- `saludo` también está en el stack, **no es dueña de nada** y apunta **directamente al heap**, pero solo
  al rango `"Hola"`. No apunta al objeto `s`.
- **No se copia ningún dato**: ahí está la eficiencia.
- Internamente un slice guarda **un puntero + una longitud**. El ownership sigue siendo de `s`.

## Slice de un array

```rust
fn main() {
    let numeros = [1, 2, 3, 4, 5];   // numeros: [i32; 5]
    let parte = &numeros[1..4];      // parte: &[i32] → posiciones 1,2,3 → [2, 3, 4]

    // println!("{}", parte);        // ❌ `[i32]` doesn't implement `std::fmt::Display`
    println!("{:?}", parte);         // ✅ [2, 3, 4]
}
```

## Introducción a los traits: `Display` y `Debug`

- Rust **no imprime cualquier cosa automáticamente**. Para usar `{}` en `println!`, el tipo tiene que
  **saber mostrarse como texto**.
- **Trait**: una **definición de comportamiento**. Dice **qué puede hacer** un tipo, no qué es.
  Es parecido a las **interfaces de Java** (una lista de habilidades).
- **`Display`** es el trait que usa `{}`. Lo implementan `String`, `&str` y los enteros, pero **no los
  arrays ni los slices de arrays**, de ahí el error.
- El propio compilador sugiere usar **`{:?}`**, que usa el trait **`Debug`** (formato de depuración).
  Los arrays y slices sí lo implementan, así que se pueden imprimir así.

| Formato | Trait | Uso |
|---------|-------|-----|
| `{}` | `Display` | Salida "bonita" para el usuario (texto, números) |
| `{:?}` | `Debug` | Salida de depuración (arrays, vectores, estructuras…) |

> Consejo extra (no aparece en el vídeo): `{:#?}` imprime en formato `Debug` "bonito", con saltos de
> línea. Es muy útil con estructuras grandes.
