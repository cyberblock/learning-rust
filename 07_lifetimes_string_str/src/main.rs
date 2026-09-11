//! Vídeo 07 · Lifetimes de variables y referencias. String vs &str
//! Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=qC661SO0WjI
//! Apuntes: ../apuntes/07-lifetimes-string-vs-str.md

fn main() {
    lifetime_de_un_bloque();
    non_lexical_lifetimes();
    string_vs_str();
    funciones_con_str();
}

fn lifetime_de_un_bloque() {
    println!("== Lifetime y bloques ==");
    let r;
    {
        let s = String::from("Hola"); // `s` empieza a vivir aquí
        r = &s;
        println!("Dentro del bloque: {r}"); // ✅ se usa mientras `s` vive
    } // ← `s` muere aquí

    // println!("Fuera del bloque: {r}");
    // ❌ error[E0597]: `s` does not live long enough
    // `r` sería una referencia colgante: apuntaría a algo que ya no existe.
}

fn non_lexical_lifetimes() {
    println!("\n== Non-Lexical Lifetimes (NLL) ==");
    // Desde ~2018 el compilador no mira solo las llaves: un préstamo dura
    // hasta su ÚLTIMO USO real.
    let mut numero = 5;
    let r = &numero;
    println!("Préstamo: {r}"); // último uso de `r` → el préstamo termina aquí
    numero += 1; // ✅ con lifetimes léxicos (antes de 2018) esto no compilaba
    println!("Número modificado: {numero}");
}

fn string_vs_str() {
    println!("\n== String vs &str ==");
    // String: DUEÑO del texto, vive en el heap, puede crecer y modificarse
    let mut s = String::from("hola");
    s.push_str(" mundo");
    println!(
        "String: {s} (longitud {}, capacidad {})",
        s.len(),
        s.capacity()
    );

    // &str: solo una REFERENCIA a un texto, inmutable, no libera memoria
    let t: &str = "adiós"; // los literales son &'static str: viven en el ejecutable
    let parte: &str = &s[0..4]; // también puede apuntar a una parte de un String
    println!("&str: {t} / {parte}");

    // Conversiones
    let de_str_a_string: String = t.to_string(); // o String::from(t)
    let de_string_a_str: &str = &s; // o s.as_str()
    println!("{de_str_a_string} / {de_string_a_str}");
}

fn funciones_con_str() {
    println!("\n== ¿Qué recibe una función? ==");
    // Regla práctica: si solo necesitas LEER texto, recibe &str.
    // Así la función acepta tanto literales como Strings (prestados).
    let nombre = String::from("Ana");
    saludar("Luis");
    saludar(&nombre);
    println!("`nombre` sigue siendo nuestro: {nombre}");
}

fn saludar(nombre: &str) {
    println!("Hola, {nombre}");
}
