// Soluciones · Vídeo 02
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=81Yr-xwEr-A
// Ejecuta con:  cargo run --example soluciones_02

fn main() {
    // Ejercicio 2
    println!("Me llamo Jose");
    println!("Estoy haciendo el curso de Rust");

    // Ejercicio 3: \" escapa las comillas dentro del texto
    println!("Rust es \"seguro\"");
    // Alternativa: un raw string r#"..."# no necesita escapar nada
    println!(r#"Rust es "rápido""#);

    // Ejercicio 4: `cargo build` deja el ejecutable en
    //   target/debug/hola_rust.exe  (en Windows)
    // En este repo, al ser un workspace, target/ está en la raíz del repositorio.
}
