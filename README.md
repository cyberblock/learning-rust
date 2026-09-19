# Aprendiendo Rust · Guía de aprendizaje

![Rust Learning](Rust-Learning.png)

Repositorio de estudio para aprender Rust siguiendo el
[Curso Rust](https://www.youtube.com/playlist?list=PLU8oAlHdN5BmhyZZQN0IyYUgimevrPL-i) de
[Píldoras Informáticas](https://www.youtube.com/@pildorasinformaticas). Los vídeos explican los conceptos;
aquí están los **apuntes propios**, el **código de cada vídeo** listo para ejecutar y
**ejercicios con soluciones** para practicar.

> **Estado:** 17 vídeos procesados · Rust edición 2024 · el workspace compila y pasa `cargo clippy` sin avisos.

## Contenido

- [Objetivos](#objetivos)
- [Preparación del entorno](#preparación-del-entorno)
- [Estructura del repositorio](#estructura-del-repositorio)
- [Cómo estudiar con este repositorio](#cómo-estudiar-con-este-repositorio)
- [Módulos del curso](#módulos-del-curso)
- [Ruta de aprendizaje](#ruta-de-aprendizaje)
- [Ejercicios extra](#ejercicios-extra)
- [Chuleta de Cargo](#chuleta-de-cargo)
- [Buenas prácticas](#buenas-prácticas)
- [Recursos complementarios](#recursos-complementarios)
- [Créditos](#créditos)

---

## Objetivos

- Aprender los fundamentos de Rust de forma incremental, un concepto por vídeo.
- Entender el modelo de memoria de Rust: *ownership*, *borrowing* y *lifetimes*.
- Practicar cada concepto con ejercicios breves y graduados, incluidos ejercicios de "arregla este código".
- Adoptar buenas prácticas desde el principio: leer los errores del compilador, `cargo fmt` y `cargo clippy`.

## Preparación del entorno

1. **Instala Rust** con [rustup](https://rustup.rs). En Windows, el instalador te pedirá las
   *Build Tools* de Visual Studio. Si falla al generar el ejecutable, cambia al *toolchain* GNU
   (se explica en el [apunte 03](apuntes/03-estructura-proyecto-y-tipos.md)).
2. **Comprueba la instalación:**
   ```bash
   rustc --version
   cargo --version
   ```
3. **Editor recomendado:** VS Code con la extensión
   [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
   (autocompletado, tipos inferidos y errores mientras escribes).
4. **Clona el repositorio y compila todo:**
   ```bash
   git clone https://github.com/cyberblock/learning-rust.git
   cd learning-rust
   cargo build
   ```

## Estructura del repositorio

```
learning-rust/
├── Cargo.toml                  # workspace: agrupa todos los proyectos (un único target/)
├── README.md                   # esta guía
├── apuntes/                    # un apunte .md por vídeo + índice y chuleta (README.md)
├── 02_hola_rust/               # un proyecto de Cargo por vídeo
│   ├── src/main.rs             #   código del vídeo (ejecutable)
│   └── examples/
│       ├── ejercicios_02.rs    #   enunciados + todo!() para que los resuelvas
│       └── soluciones_02.rs    #   soluciones
├── 03_estructura_y_tipos/
├── ...
└── 17_hashmap_entry/
```

El vídeo 01 es de presentación y no tiene código.

## Cómo estudiar con este repositorio

Desde la **raíz** del repositorio (el nombre del paquete es el de la carpeta sin el número):

```bash
cargo run -p borrowing                            # ejecuta el código del vídeo 06
cargo run -p borrowing --example ejercicios_06    # tus ejercicios (fallan con todo!() hasta que los resuelvas)
cargo run -p borrowing --example soluciones_06    # las soluciones
cargo build                                       # compila TODOS los proyectos
```

O entrando en la carpeta, como siempre:

```bash
cd 06_borrowing
cargo run
cargo run --example ejercicios_06
```

**Método recomendado para cada vídeo:**

1. Mira el vídeo.
2. Lee el apunte en `apuntes/`: resumen, conceptos, errores típicos y notas de precisión.
3. Ejecuta y modifica `src/main.rs`. Descomenta las líneas marcadas con ❌ para ver el error del compilador.
4. Resuelve `examples/ejercicios_XX.rs` sustituyendo cada `todo!()`.
5. Compara con `examples/soluciones_XX.rs`.

---

## Módulos del curso

El índice completo, con la chuleta final, está en [`apuntes/README.md`](apuntes/README.md).

### Módulo 1 · Introducción y fundamentos

| # | Tema | Duración | Código | Apuntes |
|:-:|------|:--------:|--------|---------|
| 01 | Temario y presentación | 12:57 | — | [01](apuntes/01-temario-y-presentacion.md) |
| 02 | Características de Rust e instalación | 19:22 | [02_hola_rust](02_hola_rust/src/main.rs) | [02](apuntes/02-caracteristicas-e-instalacion.md) |
| 03 | Estructura de un proyecto y tipos de datos | 22:13 | [03_estructura_y_tipos](03_estructura_y_tipos/src/main.rs) | [03](apuntes/03-estructura-proyecto-y-tipos.md) |

### Módulo 2 · Variables

| # | Tema | Duración | Código | Apuntes |
|:-:|------|:--------:|--------|---------|
| 04 | Declaración de variables. Binding | 19:39 | [04_variables_binding](04_variables_binding/src/main.rs) | [04](apuntes/04-declaracion-de-variables-binding.md) |
| 05 | Inmutables vs mutables vs constantes | 19:35 | [05_mutables_constantes](05_mutables_constantes/src/main.rs) | [05](apuntes/05-inmutables-mutables-constantes.md) |

### Módulo 3 · Ownership y borrowing

| # | Tema | Duración | Código | Apuntes |
|:-:|------|:--------:|--------|---------|
| 06 | Borrowing (préstamos) | 20:20 | [06_borrowing](06_borrowing/src/main.rs) | [06](apuntes/06-borrowing.md) |
| 07 | Lifetimes. `String` vs `&str` | 13:08 | [07_lifetimes_string_str](07_lifetimes_string_str/src/main.rs) | [07](apuntes/07-lifetimes-string-vs-str.md) |
| 08 | Slices | 17:11 | [08_slices](08_slices/src/main.rs) | [08](apuntes/08-slices.md) |

### Módulo 4 · Colecciones e iteradores

| # | Tema | Duración | Código | Apuntes |
|:-:|------|:--------:|--------|---------|
| 09 | Vectores I: creación | 19:08 | [09_vectores_creacion](09_vectores_creacion/src/main.rs) | [09](apuntes/09-vectores-I-creacion.md) |
| 10 | Vectores II: eliminar y acceder | 18:21 | [10_vectores_acceso](10_vectores_acceso/src/main.rs) | [10](apuntes/10-vectores-II-eliminar-y-acceder.md) |
| 11 | Vectores III: bucle `for` | 15:43 | [11_vectores_for](11_vectores_for/src/main.rs) | [11](apuntes/11-vectores-III-bucle-for.md) |
| 12 | Vectores IV: iteradores | 11:20 | [12_iteradores](12_iteradores/src/main.rs) | [12](apuntes/12-vectores-IV-iteradores.md) |
| 13 | Iteradores: `map`, `filter`, `collect` | 19:38 | [13_map_filter_collect](13_map_filter_collect/src/main.rs) | [13](apuntes/13-iteradores-map-filter-collect.md) |
| 14 | Iteradores: funciones de cálculo | 15:19 | [14_funciones_calculo](14_funciones_calculo/src/main.rs) | [14](apuntes/14-iteradores-funciones-de-calculo.md) |
| 15 | Ejemplo práctico: notas, media y máxima | 22:41 | [15_ejemplo_notas](15_ejemplo_notas/src/main.rs) | [15](apuntes/15-ejemplo-practico-notas.md) |
| 16 | HashMap I: clave y valor | 20:11 | [16_hashmap](16_hashmap/src/main.rs) | [16](apuntes/16-hashmap-I.md) |
| 17 | HashMap II: claves duplicadas y `entry` | 11:08 | [17_hashmap_entry](17_hashmap_entry/src/main.rs) | [17](apuntes/17-hashmap-II-claves-duplicadas.md) |

---

## Ruta de aprendizaje

Mapa completo de Rust, más allá de lo que cubre el curso por ahora. Cada paso indica qué vídeos y
carpetas lo trabajan.

**Leyenda:** ✅ cubierto en el curso · 🟡 cubierto en parte · ⏳ pendiente (próximos vídeos)

| Paso | Tema | Estado | Vídeos |
|:----:|------|:------:|:------:|
| 1 | [Preparación](#1-preparación-) | ✅ | 02–03 |
| 2 | [Fundamentos sintácticos](#2-fundamentos-sintácticos-) | ✅ | 03–05 |
| 3 | [Control de flujo](#3-control-de-flujo-) | 🟡 | 11, 13 |
| 4 | [Propiedad (ownership)](#4-propiedad-ownership-) | ✅ | 04, 07 |
| 5 | [Préstamos (borrowing) y referencias](#5-préstamos-borrowing-y-referencias-) | ✅ | 06 |
| 6 | [Slices](#6-slices-) | ✅ | 08 |
| 7 | [Structs e `impl`](#7-structs-e-impl-) | ⏳ | — |
| 8 | [Enums y pattern matching](#8-enums-y-pattern-matching-) | ⏳ | — |
| 9 | [`Option` y `Result`](#9-option-y-result-) | 🟡 | 10, 14, 15 |
| 10 | [Módulos y organización](#10-módulos-y-organización-) | ⏳ | — |
| 11 | [Genéricos y traits](#11-genéricos-y-traits-) | 🟡 | 08, 15 |
| 12 | [Lifetimes (introducción)](#12-lifetimes-introducción-) | ✅ | 07 |
| 13 | [Colecciones estándar](#13-colecciones-estándar-) | ✅ | 09–11, 16–17 |
| 14 | [Iteradores y closures](#14-iteradores-y-closures-) | ✅ | 12–15 |
| 15 | [Concurrencia](#15-concurrencia-) | ⏳ | — |
| 16 | [Errores personalizados](#16-errores-personalizados-) | ⏳ | — |
| 17 | [Macros (vista inicial)](#17-macros-vista-inicial-) | 🟡 | 03 |
| 18 | [Cargo y publicación](#18-cargo-y-publicación-) | 🟡 | 02–03 |
| 19 | [Testing](#19-testing-) | ⏳ | — |
| 20 | [Proyecto final](#20-proyecto-final-) | ⏳ | — |

### 1. Preparación ✅
📺 Vídeos 02–03 · 📁 `02_hola_rust`, `03_estructura_y_tipos`

- Instalar `rustup` y verificar con `rustc --version` y `cargo --version`.
- Crear el primer proyecto: `cargo new hola_rust`.
- Estructura de un proyecto: `src/main.rs`, `Cargo.toml`, `.gitignore`, `target/`.
- Windows: si falla al generar el ejecutable, cambia al *toolchain* GNU (ver apunte 03).

**Ejercicio:** imprime "Hola Rust", compila con `cargo build` y ejecuta con `cargo run`. → `ejercicios_02.rs`

### 2. Fundamentos sintácticos ✅
📺 Vídeos 03–05 · 📁 `03_estructura_y_tipos`, `04_variables_binding`, `05_mutables_constantes`

`fn main`, comentarios, `let`, mutabilidad (`mut`), constantes (`const`), tipos primitivos
(enteros, decimales, `bool`, `char`), tuplas y arrays, inferencia de tipos y macros (`println!`).

**Ejercicio:** declara variables con y sin `mut` e imprime valores compuestos con `{:?}`.
→ `ejercicios_03.rs`, `ejercicios_04.rs`, `ejercicios_05.rs`

### 3. Control de flujo 🟡
📺 Vídeos 11, 13 (se usan `for` e `if`; `loop`, `while` y `match` llegarán más adelante)

`if`, `else if`, `loop`, `while`, `for` y rangos (`1..10`, `1..=10`).

**Ejercicio:** tabla de multiplicar (1..=10) con `for`. → `ejercicios_09.rs` (3) y `ejercicios_11.rs` (4)

### 4. Propiedad (ownership) ✅
📺 Vídeos 04, 07 · 📁 `04_variables_binding`, `07_lifetimes_string_str`

*Binding*, movimiento (*move*), `Copy` vs `clone()`, stack vs heap, `String` vs `&str`.

**Ejercicio:** escribe funciones que consumen un `String` y otras que reciben `&str`, y observa qué compila.
→ `ejercicios_04.rs` (2 y 3), `ejercicios_07.rs`

### 5. Préstamos (borrowing) y referencias ✅
📺 Vídeo 06 · 📁 `06_borrowing`

`&T`, `&mut T`, reglas de los préstamos y cómo el préstamo vive hasta su último uso.

**Ejercicio:** función que calcula la longitud (o las vocales) de una cadena sin mover la propiedad.
→ `ejercicios_06.rs`

### 6. Slices ✅
📺 Vídeo 08 · 📁 `08_slices`

`&str`, slices de arrays (`&[T]`), rangos, *fat pointers* (puntero + longitud), `Display` vs `Debug`.

**Ejercicio:** función que devuelve la primera palabra de una frase usando un slice sobre `&str`.
→ `ejercicios_08.rs` (2)

### 7. Structs e `impl` ⏳
Definición de structs, métodos (`impl`) y `#[derive(Debug)]`.

**Ejercicio:** struct `Usuario { id: u32, nombre: String }` con el método `fn nombre_mayus(&self)`.

### 8. Enums y pattern matching ⏳
`enum`, `match` e `if let`. Hay un adelanto de `match` e `if let` con `Option` en `10_vectores_acceso`.

**Ejercicio:** enum `ResultadoRed { Ok(u16), Error(String) }` y una función que imprima algo distinto en cada caso.

### 9. `Option` y `Result` 🟡
📺 Vídeos 10, 14, 15 (`Option`, `Some`, `None`, `unwrap`) · `Result` y `?` pendientes

Manejo seguro de la ausencia de valor (Rust no tiene `null`) y propagación de errores con `?`.

**Ejercicio:** función que lee un número desde un `String` y devuelve `Result<u32, String>`.
Ya puedes practicar `Option` con `ejercicios_10.rs` (3).

### 10. Módulos y organización ⏳
`mod`, `pub`, estructura en carpetas, `lib.rs` y workspaces (este repositorio ya es uno).

**Ejercicio:** separa la lógica de parseo en un `mod parser`.

### 11. Genéricos y traits 🟡
📺 Vídeos 08, 15 (qué es un trait: `Display`, `Debug`, `Ord`) · genéricos pendientes

Parámetros genéricos, `trait` e `impl<T>`.

**Ejercicio:** trait `Resumen` con el método `resumen()`, implementado para dos structs.

### 12. Lifetimes (introducción) ✅
📺 Vídeo 07 · 📁 `07_lifetimes_string_str`

Duración de las referencias, *Non-Lexical Lifetimes* y la sintaxis mínima de anotación `'a`.

**Ejercicio:** función que devuelve la referencia más larga entre dos `&str`. → `ejercicios_07.rs` (3)

### 13. Colecciones estándar ✅
📺 Vídeos 09–11, 16–17 · 📁 `09_…` a `11_…`, `16_hashmap`, `17_hashmap_entry`

`Vec` (`push`, `pop`, `get`, `len`, `capacity`), `HashMap` (`insert`, `get`, `entry().or_insert()`) y `String`.

**Ejercicio:** cuenta la frecuencia de las palabras de una frase con un `HashMap`. → `ejercicios_17.rs` (2)

### 14. Iteradores y closures ✅
📺 Vídeos 12–15 · 📁 `12_iteradores` a `15_ejemplo_notas`

`iter()`, `iter_mut()`, `into_iter()`, `map`, `filter`, `collect`, `sum`, `find`, `count`,
`any`, `all`, `max`/`min`, `max_by` + `partial_cmp` y closures.

**Ejercicio:** filtra los números pares y elévalos al cuadrado. → `ejercicios_13.rs` (1)

### 15. Concurrencia ⏳
`std::thread`, `move`, `join`, `Mutex` y `Arc`.

**Ejercicio:** contador compartido que incrementan varios hilos.

### 16. Errores personalizados ⏳
Enum de errores, el trait `From` y, opcionalmente, el crate `thiserror`.

**Ejercicio:** enum `AppError` para errores de E/S y de parseo.

### 17. Macros (vista inicial) 🟡
📺 Vídeo 03 (qué es una macro: genera código al compilar) · `macro_rules!` pendiente

Diferencias entre `macro_rules!` y las funciones.

**Ejercicio:** macro sencilla que envuelva `println!` con el prefijo `[DEBUG]`.

### 18. Cargo y publicación 🟡
📺 Vídeos 02–03 (`cargo new`, `run`, `build`, `clean`, `Cargo.toml`)

`Cargo.toml`, *features*, *workspaces*, `cargo fmt` y `cargo clippy`.

**Ejercicio:** ejecuta `cargo clippy` en el workspace y corrige los avisos (este repositorio ya pasa sin avisos).

### 19. Testing ⏳
Tests unitarios y de integración, y `#[should_panic]`.

**Ejercicio:** un test para una función de parseo y otro que compruebe el caso de error.

### 20. Proyecto final ⏳
Una mini CLI que lea un archivo, cuente sus líneas y busque un patrón.

---

## Ejercicios extra

- ✅ Simular una caja registradora con `HashMap<String, f32>`. → `ejercicios_16.rs` (4)
- ✅ Calificar notas (Suspenso/Aprobado/Notable/Sobresaliente) con `map`. → `ejercicios_15.rs` (4)
- ⏳ Implementar `FizzBuzz` genérico sobre cualquier tipo que implemente `Display`.
- ⏳ Parser sencillo para una mini expresión matemática (suma y multiplicación).

## Chuleta de Cargo

| Comando | Qué hace |
|---------|----------|
| `cargo new <nombre>` | Crea un proyecto nuevo |
| `cargo run` | Compila y ejecuta |
| `cargo build` | Solo compila (`target/debug/`) |
| `cargo build --release` | Compila optimizado (`target/release/`) |
| `cargo check` | Comprueba errores sin generar el ejecutable (más rápido) |
| `cargo clean` | Borra `target/` (útil si un archivo queda bloqueado) |
| `cargo run --example <nombre>` | Ejecuta un archivo de `examples/` |
| `cargo run -p <paquete>` | Ejecuta un paquete concreto del workspace |
| `cargo fmt` | Formatea el código |
| `cargo clippy` | Revisa el código y sugiere mejoras |
| `cargo doc --open` | Genera y abre la documentación |

## Buenas prácticas

- Usa `cargo fmt` y `cargo clippy` con regularidad.
- Lee los errores del compilador completos: casi siempre explican el problema y proponen la solución.
- Para solo **leer** datos, recibe préstamos (`&str`, `&[T]`) en lugar de `&String` o `&Vec<T>`.
- Prefiere `get()` (devuelve `Option`) a los corchetes `v[i]` cuando el índice puede no existir.
- Usa `unwrap()` solo cuando estés seguro de que hay valor; en librerías, prefiere `Result` a `panic!`.
- Documenta con `///` y genera la documentación con `cargo doc --open`.

## Recursos complementarios

- The Rust Book (oficial): https://doc.rust-lang.org/book/ · en español: https://book.rustlang-es.org/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rustlings (ejercicios guiados): https://github.com/rust-lang/rustlings
- Exercism, pista de Rust: https://exercism.org/tracks/rust
- Documentación de la librería estándar: https://doc.rust-lang.org/std/
- Playground: https://play.rust-lang.org

## Créditos

- **Autor del curso:** Juan, de [Píldoras Informáticas](https://www.pildorasinformaticas.es)
- **Canal de YouTube:** [@pildorasinformaticas](https://www.youtube.com/@pildorasinformaticas)
- **Lista original:** [Curso Rust](https://www.youtube.com/playlist?list=PLU8oAlHdN5BmhyZZQN0IyYUgimevrPL-i)

> Este repositorio contiene apuntes de estudio propios basados en su curso. No tiene relación oficial
> con el autor ni con el canal. Los vídeos no se incluyen en el repositorio: se enlazan desde cada apunte.
