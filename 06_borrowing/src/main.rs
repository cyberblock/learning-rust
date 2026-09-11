//! Vídeo 06 · Borrowing (préstamos)
//! Apuntes: ../apuntes/06-borrowing.md
//!
//! Reglas del borrowing:
//!   ✅ muchas referencias inmutables (&T) a la vez
//!   ✅ O una única referencia mutable (&mut T)
//!   ❌ nunca mezcladas al mismo tiempo
//!   ❌ no se puede destruir/mover un valor mientras un préstamo siga en uso

// En este vídeo se usa &String (referencia al String que hemos creado). En el vídeo 07
// verás que para solo leer texto es mejor recibir &str; Clippy lo sugiere (ptr_arg).
#![allow(clippy::ptr_arg)]

fn main() {
    prestamo_inmutable();
    no_se_puede_destruir_lo_prestado();
    prestamo_mutable();
    el_orden_importa();
    prestar_a_funciones();
}

fn prestamo_inmutable() {
    println!("== Préstamo inmutable ==");
    let saludo = String::from("hola");
    let referencia = &saludo; // se presta, la dueña sigue siendo `saludo`
    let otra = &saludo; // se pueden tener varias referencias inmutables

    println!("Original: {saludo}");
    println!("Préstamo: {referencia} / {otra}");
}

fn no_se_puede_destruir_lo_prestado() {
    println!("\n== No se puede destruir un valor prestado ==");
    let saludo = String::from("hola");
    let referencia = &saludo;
    println!("Préstamo: {referencia}");

    // std::mem::drop(saludo);
    // println!("{referencia}");
    // ❌ error[E0505]: cannot move out of `saludo` because it is borrowed
    // En C++ esto dejaría un puntero colgante; Rust ni siquiera compila.

    drop(saludo); // ✅ aquí sí: el préstamo ya no se usa más abajo
    println!("`saludo` liberado");
}

fn prestamo_mutable() {
    println!("\n== Préstamo mutable ==");
    // Hacen falta DOS cosas: variable `mut` + préstamo `&mut`
    let mut saludo = String::from("hola");
    let referencia = &mut saludo;
    referencia.push_str(" mundo cruel");
    println!("Préstamo: {referencia}");

    // let solo_lectura = &saludo; // si después se usara `referencia`...
    // ❌ cannot borrow `saludo` as immutable because it is also borrowed as mutable
}

fn el_orden_importa() {
    println!("\n== El préstamo vive hasta su ÚLTIMO USO ==");
    let mut saludo = String::from("hola");
    let referencia = &mut saludo;
    referencia.push_str(" mundo cruel");

    println!("Préstamo: {referencia}"); // ← último uso: aquí termina el préstamo
    println!("Original: {saludo}"); // ✅ el original vuelve a estar disponible

    // Si intercambias el orden de estos dos println! → error de compilación.
}

// Extra: el uso más habitual del borrowing es pasar datos a funciones sin perderlos.
fn prestar_a_funciones() {
    println!("\n== Préstamos y funciones ==");
    let mut texto = String::from("rust");
    let n = longitud(&texto); // préstamo inmutable
    gritar(&mut texto); // préstamo mutable
    println!("'{texto}' tenía {n} letras"); // `texto` sigue siendo nuestro
}

fn longitud(s: &String) -> usize {
    s.len()
}

fn gritar(s: &mut String) {
    *s = s.to_uppercase();
    s.push('!');
}
