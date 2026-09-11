# 03 · Estructura de un proyecto y tipos de datos

- **Vídeo:** https://www.youtube.com/watch?v=roQHqh4lqM4
- **Autor:** [Píldoras Informáticas](https://www.youtube.com/@pildorasinformaticas)
- **Duración:** 22:13

## Resumen

Tras resolver dos dudas de instalación (WSL y *toolchain* GNU en Windows), el profesor crea un segundo
proyecto (`fundamentos`), recorre cada archivo que genera Cargo, explica qué es una **macro** y termina
con los **tipos primitivos** de Rust (escalares y compuestos).

## Dudas de instalación

**WSL (Linux dentro de Windows):** no se usa el `.exe`. En la misma página de instalación hay un comando
para terminal Linux; se ejecuta dentro de WSL y se siguen las instrucciones:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Toolchains en Windows: MSVC vs GNU.** Rust tiene dos versiones del compilador para Windows:

- **MSVC** (Microsoft Visual C++): la que instala por defecto. **Necesita las Build Tools de Visual
  Studio** (Visual Studio, no VS Code). Si no las tienes, falla al generar el ejecutable.
- **GNU**: funciona sin depender de las herramientas de Microsoft.

Si `cargo run` no llega a mostrar "Hello, world!", cambia a GNU:

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

## Estructura de un proyecto

```bash
cargo new fundamentos
```

```
fundamentos/
├── src/
│   └── main.rs     # punto de entrada del programa
├── .gitignore      # evita subir target/ a Git
├── Cargo.toml      # manifiesto del proyecto
└── (target/)       # solo aparece tras compilar/ejecutar
```

- **`target/`**: no existe hasta que compilas o ejecutas. Guarda binarios, compilaciones temporales,
  versiones *debug* y *release* y cachés, para recompilar rápido. Suele pesar mucho.
- **`src/main.rs`**: contiene la función `main`, el **punto de entrada** (como `main` en Java).
- **`Cargo.toml`**: probablemente el archivo más importante. Cargo lo lee para saber el nombre, la versión,
  la configuración y las **dependencias**. El formato TOML es parecido a un JSON de configuración.
- **`.gitignore`**: Cargo lo añade automáticamente para que `target/` no se suba al repositorio.

`Cargo.toml` típico recién creado:

```toml
[package]
name = "fundamentos"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`src/main.rs` por defecto:

```rust
fn main() {
    println!("Hello, world!");
}
```

## Cargo

Es el **gestor oficial de proyectos y dependencias** de Rust. Sirve para crear proyectos, compilar,
ejecutar, descargar librerías externas, generar documentación… Equivale a `npm` (JavaScript),
`pip` (Python) o Maven/Gradle (Java).

| Comando | Qué hace |
|---------|----------|
| `cargo new <nombre>` | Crea un proyecto nuevo |
| `cargo run` | Compila y ejecuta (crea `target/`) |
| `cargo clean` | Borra la carpeta `target/` |

## Macros

- `println!` **no es una función, es una macro**: toda llamada que termina en `!` es una macro.
- Una macro **genera código en tiempo de compilación**. Las funciones reciben *valores*; las macros
  reciben *código* y lo expanden en más código.
- Analogía del profesor (suya, no oficial): una macro es como un **ZIP de código**. Tú ves una línea
  simple, pero al compilar se "descomprime" en mucho más código.
- ¿Por qué existen?
  1. **Simplicidad**: escribes poco y por detrás se genera todo lo necesario.
  2. **Eficiencia**: el código expandido es más detallado y el procesador lo ejecuta de forma muy eficiente.
- Es solo una idea general; se trabajará mucho más adelante.

## Error "archivo bloqueado" al ejecutar

Si guardas los proyectos en una carpeta sincronizada (Dropbox, Google Drive, OneDrive), o un
antivirus/firewall muy agresivo bloquea archivos, `cargo run` puede compilar pero fallar al ejecutar. Solución:

```bash
cargo clean   # borra target/
cargo run     # vuelve a compilar y ejecutar
```

Mejor aún: guarda los proyectos en una carpeta local no sincronizada.

## Tipos primitivos

Se dividen en **escalares** (un único valor) y **compuestos** (varios valores).

### Escalares

| Categoría | Tipos | Por defecto (inferencia) |
|-----------|-------|--------------------------|
| Enteros con signo | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | **`i32`** |
| Enteros sin signo | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | — |
| Decimales (flotantes) | `f32`, `f64` | **`f64`** |
| Booleanos | `bool` (`true` / `false`) | — |
| Caracteres | `char` | — |

- El número es el **tamaño en bits** (8, 16, 32, 64, 128).
- **`i` = con signo** (admite negativos), **`u` = *unsigned*, sin signo**.
  `5` cabe en un `u32`, pero `-5` necesita un tipo con signo, como `i32`.
- **`isize` / `usize`**: su tamaño **depende de la arquitectura**. En una máquina de 64 bits equivalen
  a `i64` / `u64`.
- **Inferencia de tipos**: Rust deduce el tipo sin que lo escribas (Java, en cambio, te obliga).
  Un entero sin más se infiere como `i32` y un decimal como `f64`.
- ¿Por qué tantos tamaños? Porque Rust, como C++, trabaja a muy bajo nivel con memoria, tamaños y
  direcciones, y ahí estos tipos son imprescindibles.
- **`String` no es un tipo primitivo** (igual que en Java, donde `String` es un objeto).

### Compuestos

- **Tuplas**
- **Arrays**

Hay más primitivos (relacionados con punteros y memoria) que se verán más adelante.
