// Soluciones · Vídeo 04

fn main() {
    // 1.
    let ciudad = "Madrid";
    let habitantes = 3_300_000;
    let capital = true;
    println!("{ciudad} tiene {habitantes} habitantes. ¿Es capital? {capital}");

    // 2a. Clonar: cada variable tiene su propio String
    let curso = String::from("Rust");
    let otro = curso.clone();
    println!("{} {}", curso, otro);

    // 2b. Usar `curso` ANTES de mover la propiedad
    let curso = String::from("Rust"); // shadowing: nueva variable con el mismo nombre
    println!("{}", curso);
    let otro = curso; // a partir de aquí `curso` ya no es válida
    println!("{}", otro);

    // 3. Compila porque i32 es un tipo primitivo que implementa `Copy`:
    //    `let m = n;` COPIA el valor en lugar de moverlo, así que `n` sigue siendo válida.
    let n = 7;
    let m = n;
    println!("{} {}", n, m);

    // 4.
    let total;
    total = 100 * 3;
    println!("total: {total}");
}
