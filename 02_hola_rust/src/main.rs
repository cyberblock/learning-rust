//! Vídeo 02 · Características de Rust e instalación
//! Apuntes: ../apuntes/02-caracteristicas-e-instalacion.md
//!
//! Primer proyecto creado con Cargo para comprobar que Rust está bien instalado.
//!
//! Comandos del vídeo:
//!   rustc --version        -> versión del compilador
//!   cargo new hola_rust    -> crea el proyecto
//!   cd hola_rust
//!   cargo run              -> compila y ejecuta
//!
//! Si `cargo run` no llega a imprimir el saludo en Windows, cambia al toolchain GNU (vídeo 03):
//!   rustup toolchain install stable-x86_64-pc-windows-gnu
//!   rustup default stable-x86_64-pc-windows-gnu

// `main` es el punto de entrada: el programa empieza a ejecutarse aquí.
fn main() {
    // println! es una MACRO (lleva `!`), no una función: genera código al compilar.
    println!("Hello, world!");
    println!("Hola Rust");
}
