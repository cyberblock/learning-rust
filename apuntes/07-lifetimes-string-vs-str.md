# 07 · Lifetimes de variables y referencias. `String` vs `&str`

- **Vídeo:** https://www.youtube.com/watch?v=qC661SO0WjI
- **Autor:** [Píldoras Informáticas](https://www.youtube.com/@pildorasinformaticas)
- **Duración:** 13:08

## Resumen

Profundiza en el **lifetime** (ciclo de vida) de variables y referencias con un ejemplo de bloques `{}`.
Explica los **Non-Lexical Lifetimes (NLL)**: desde ~2018 Rust mira el **último uso real** de una
referencia y no solo las llaves. Termina comparando las dos formas de manejar texto: **`String`** y **`&str`**.

## Lifetime y bloques

Una variable declarada dentro de un bloque `{ }` **vive desde que se declara hasta la llave de cierre**.

```rust
fn main() {
    let r;                          // r: &String (declarada fuera del bloque)

    {
        let s = String::from("Hola"); // s empieza a vivir aquí
        r = &s;                        // r toma prestado s
    }                                  // ← s muere aquí

    // println!("{}", r);  // ❌ si se descomenta: `s` does not live long enough
}
```

- Tal y como está (sin usar `r` fuera del bloque), **compila**. VS Code solo avisa (subrayado amarillo)
  de que `r` no se usa.
- En memoria: `s` está en el stack y apunta a `"Hola"` en el heap, que es de su propiedad; `r` apunta a `s`.
  Si `s` desaparece, `r` sería una **referencia colgante**, y Rust no lo permite.

## Non-Lexical Lifetimes (NLL)

- **Antes (lifetimes léxicos, anteriores a ~2018):** el compilador solo miraba las **llaves** del bloque.
  Si una referencia "parecía" salir del bloque del valor, daba error **aunque luego no se usara**.
  El código de arriba no compilaba.
- **Ahora (NLL):** el lifetime de una referencia llega **hasta su último uso real**. El compilador comprueba:
  - ¿Se usa `r` fuera del bloque? No.
  - ¿Hay algún uso de `r` cuando `s` ya no vive? No.
  - Entonces el préstamo es válido y **compila**.

El error aparece **solo si usas la referencia** cuando el valor ya ha muerto:

```rust
let r;
{
    let s = String::from("Hola");
    r = &s;
    println!("{}", r); // ✅ se usa ANTES de que s muera
}
println!("{}", r);     // ❌ se usa DESPUÉS: s ya no existe → no compila
```

En C++ el equivalente compilaría y dejaría un puntero colgante con resultados inesperados.
Rust es más flexible gracias a NLL sin perder seguridad.

## `String` vs `&str`

```rust
let s = String::from("hola"); // String: propietario del texto
let t: &str = "adiós";        // &str: referencia a un texto (no es dueña)
```

**Diferencia principal:** `String` es **dueño** (tipo propietario); `&str` es solo una **referencia** a un texto.

| Característica | `String` | `&str` |
|----------------|----------|--------|
| ¿Es dueño? | Sí | No |
| ¿Vive en el heap? | Sí | No (solo es una referencia) |
| ¿Puede mutar? | Sí (con `mut`) | No |
| ¿Tiene ownership? | Sí | No |
| ¿Sigue las reglas del borrowing? | No (es el dueño) | Sí (es un préstamo) |
| ¿Libera memoria? | Sí | No |
| ¿Puede crecer? | Sí | No |

- **`String`**: es propietario; puede crecer y modificarse (con `mut`) y sigue las reglas del ownership.
- **`&str`**: solo apunta al texto; es inmutable, no reserva ni libera memoria y **vive mientras viva el
  texto al que apunta**.

### ¿Cuándo usar cada uno?

- **`String`** → cuando necesitas **almacenar, modificar o poseer** el texto (ser su dueño).
- **`&str`** → cuando solo necesitas **leer o prestar** el texto.

¿Por qué Rust separa las dos cosas? Porque distingue claramente **poseer** los datos de **usarlos**.
Así consigue más seguridad, menos copias, más rendimiento y control total de la memoria.

> **Nota (precisión técnica, no aparece en el vídeo):** los literales de texto como `"adiós"` no están en
> el heap: se guardan dentro del propio ejecutable (datos de solo lectura) y su tipo es `&'static str`
> (viven durante todo el programa). Lo que sí está en el heap es el contenido de un `String`.
> Un `&str` también puede apuntar a una parte de un `String` (se verá con los *slices* en el vídeo 8).
