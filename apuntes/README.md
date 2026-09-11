# Apuntes · Curso de Rust (Píldoras Informáticas)

Apuntes de estudio personales de la lista de reproducción
[Curso Rust](https://www.youtube.com/playlist?list=PLU8oAlHdN5BmhyZZQN0IyYUgimevrPL-i) de Píldoras Informáticas.
Son resúmenes con explicaciones propias y el código de cada vídeo. Para la explicación completa, mira los vídeos originales.

> Las **notas de precisión** marcadas en algunos apuntes son aclaraciones o correcciones técnicas añadidas que no aparecen en el vídeo.

## Índice

### Módulo 1 · Introducción y fundamentos
| # | Tema | Duración |
|---|------|----------|
| 01 | [Temario, a quién va dirigido y preguntas frecuentes](01-temario-y-presentacion.md) | 12:57 |
| 02 | [Características de Rust e instalación](02-caracteristicas-e-instalacion.md) | 19:22 |
| 03 | [Estructura de un proyecto y tipos de datos](03-estructura-proyecto-y-tipos.md) | 22:13 |

### Módulo 2 · Variables
| # | Tema | Duración |
|---|------|----------|
| 04 | [Declaración de variables. Binding](04-declaracion-de-variables-binding.md) | 19:39 |
| 05 | [Inmutables vs mutables vs constantes](05-inmutables-mutables-constantes.md) | 19:35 |

### Módulo 3 · Ownership y borrowing
| # | Tema | Duración |
|---|------|----------|
| 06 | [Borrowing (préstamos)](06-borrowing.md) | 20:20 |
| 07 | [Lifetimes. `String` vs `&str`](07-lifetimes-string-vs-str.md) | 13:08 |
| 08 | [Slices](08-slices.md) | 17:11 |

### Módulo 4 · Colecciones
| # | Tema | Duración |
|---|------|----------|
| 09 | [Vectores I: creación](09-vectores-I-creacion.md) | 19:08 |
| 10 | [Vectores II: eliminar y acceder](10-vectores-II-eliminar-y-acceder.md) | 18:21 |
| 11 | [Vectores III: bucle `for`](11-vectores-III-bucle-for.md) | 15:43 |
| 12 | [Vectores IV: iteradores](12-vectores-IV-iteradores.md) | 11:20 |
| 13 | [Iteradores: `map`, `filter`, `collect`](13-iteradores-map-filter-collect.md) | 19:38 |
| 14 | [Iteradores: funciones de cálculo](14-iteradores-funciones-de-calculo.md) | 15:19 |
| 15 | [Ejemplo práctico: notas, media y máxima](15-ejemplo-practico-notas.md) | 22:41 |
| 16 | [HashMap I](16-hashmap-I.md) | 20:11 |
| 17 | [HashMap II: claves duplicadas](17-hashmap-II-claves-duplicadas.md) | 11:08 |

## Chuleta rápida

```rust
let x = 5;                 // inmutable (i32 inferido)
let mut y = 5;             // mutable
const PI: f64 = 3.1416;    // constante: tipo obligatorio, MAYÚSCULAS

let s = String::from("hola");  // String: dueño
let r = &s;                     // préstamo inmutable
let t = &s[0..2];               // slice → "ho"

let mut v = vec![1, 2, 3];
v.push(4);                      // añadir
v.pop();                        // Option<T>
v.get(10);                      // Option<&T> (no hace panic)
for n in &v {}                  // préstamo: v sigue vivo
for n in &mut v { *n += 1; }    // modificar

let pares: Vec<i32> = v.iter().filter(|n| *n % 2 == 0).map(|n| n * 2).collect();

use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("Ana", 8.5);           // devuelve el valor anterior (Option)
*m.entry("Ana").or_insert(0.0) += 1.0;
```
