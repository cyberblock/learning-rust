//! Vídeo 03 · Estructura de un proyecto y tipos de datos
//! Apuntes: ../apuntes/03-estructura-proyecto-y-tipos.md
//!
//! Estructura que genera `cargo new`:
//!   src/main.rs   -> punto de entrada (fn main)
//!   Cargo.toml    -> nombre, versión, edición y dependencias
//!   .gitignore    -> evita subir target/ a Git
//!   target/       -> aparece al compilar (binarios, cachés, debug/release)
//!
//! Si un archivo queda bloqueado (Dropbox, OneDrive, antivirus...):  cargo clean && cargo run

fn main() {
    enteros();
    decimales();
    booleanos_y_caracteres();
    compuestos();
}

/// Enteros: `i` = con signo, `u` = sin signo. El número indica los bits que ocupa.
fn enteros() {
    println!("== Enteros ==");

    let por_defecto = 5; // sin anotación, Rust infiere i32
    let sin_signo: u32 = 5; // solo positivos
    let con_signo: i32 = -5; // admite negativos

    println!("{por_defecto} {sin_signo} {con_signo}");

    // Más bits = más rango (y más memoria). Cada tipo tiene su MIN y MAX:
    println!("i8   -> {} .. {}", i8::MIN, i8::MAX);
    println!("u8   -> {} .. {}", u8::MIN, u8::MAX);
    println!("i32  -> {} .. {}", i32::MIN, i32::MAX);
    println!("u64  -> {} .. {}", u64::MIN, u64::MAX);

    // isize / usize dependen de la arquitectura (64 bits en un PC actual).
    println!("usize ocupa {} bytes", std::mem::size_of::<usize>());

    // let pequeño: u8 = 300;  // ❌ error: literal out of range for `u8` (máx. 255)
}

/// Decimales: f32 y f64. Por defecto se infiere f64.
fn decimales() {
    println!("\n== Decimales ==");
    let precio = 19.99; // f64
    let temperatura: f32 = -3.5;
    println!("{precio} {temperatura}");
}

fn booleanos_y_caracteres() {
    println!("\n== bool y char ==");
    let activo: bool = true;
    let letra: char = 'R'; // char va con comillas SIMPLES
    let emoji: char = '🦀'; // un char es un carácter Unicode (4 bytes)
    println!("{activo} {letra} {emoji}");

    // String NO es un tipo primitivo (igual que en Java): se verá más adelante.
}

/// Tipos compuestos: agrupan varios valores.
fn compuestos() {
    println!("\n== Compuestos ==");

    // Tupla: tamaño fijo, cada elemento puede ser de un tipo distinto
    let persona: (&str, i32, f64) = ("Ana", 30, 1.68);
    println!("Tupla: {:?}", persona);
    println!("Nombre: {}, edad: {}", persona.0, persona.1); // acceso con .0, .1, ...

    // Desestructurar una tupla en variables
    let (nombre, edad, altura) = persona;
    println!("{nombre} tiene {edad} años y mide {altura}");

    // Array: tamaño fijo, todos los elementos del MISMO tipo
    let dias: [&str; 3] = ["lunes", "martes", "miércoles"];
    let ceros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("Array: {:?}, primer día: {}", dias, dias[0]);
    println!("Array de ceros: {:?} (longitud {})", ceros, ceros.len());
}
