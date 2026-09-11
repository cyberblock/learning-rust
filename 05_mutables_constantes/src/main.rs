//! Vídeo 05 · Variables inmutables vs mutables vs constantes
//! Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=faj7967df4Q
//! Apuntes: ../apuntes/05-inmutables-mutables-constantes.md

// Una constante puede declararse FUERA de cualquier función (ámbito global).
// Reglas: `const`, nombre en MAYÚSCULAS, tipo OBLIGATORIO y valor conocido al compilar.
const SALUDO: &str = "Hola";
// Mejor usar la constante de la librería estándar que escribir 3.1416 a mano:
const PI: f64 = std::f64::consts::PI;
const MAX_INTENTOS: u32 = 3;

// let global = 5; // ❌ error: una variable (`let`) no puede declararse fuera de una función

fn main() {
    contador_mutable();
    constantes_globales();
    cualquiera();
    inmutable_no_es_constante();
}

fn contador_mutable() {
    println!("== Inmutable vs mutable ==");
    // let contador = 0;
    // contador = contador + 1; // ❌ cannot assign twice to immutable variable

    let mut contador = 0; // con `mut` sí puede cambiar en tiempo de ejecución
    contador = contador + 1;
    contador += 1; // forma abreviada
    println!("Contador después de incrementar: {contador}");
}

fn constantes_globales() {
    println!("\n== Constantes ==");
    println!("{SALUDO}, PI vale {PI} y hay {MAX_INTENTOS} intentos");

    let radio = 2.0;
    println!(
        "Área de un círculo de radio {radio}: {}",
        PI * radio * radio
    );
}

fn cualquiera() {
    // La constante global también es visible desde cualquier otra función
    println!("Desde función cualquiera: {SALUDO}");
}

fn inmutable_no_es_constante() {
    println!("\n== Inmutable ≠ constante ==");
    // const RESULTADO: i32 = obtener_numero(10);
    // ❌ error[E0015]: cannot call non-const function `obtener_numero` in constants
    // Una constante debe conocerse AL COMPILAR; el resultado de una función normal no.

    // Una variable inmutable SÍ puede guardar un valor calculado en ejecución:
    let resultado = obtener_numero(10);
    println!("resultado: {resultado}"); // 10 (y ya no puede cambiar)

    // Extra: las funciones marcadas como `const fn` sí se pueden usar en constantes
    const DOBLE: i32 = doble(21);
    println!("DOBLE: {DOBLE}");
}

fn obtener_numero(n: i32) -> i32 {
    n // sin punto y coma = valor de retorno
}

const fn doble(n: i32) -> i32 {
    n * 2
}
