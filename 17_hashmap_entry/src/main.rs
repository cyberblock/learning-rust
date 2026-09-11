//! Vídeo 17 · HashMap II: claves duplicadas y actualizar valores
//! Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=zBWrq4RYQNk
//! Apuntes: ../apuntes/17-hashmap-II-claves-duplicadas.md
//!
//! | Método                  | Clave YA existe  | Clave NO existe | Devuelve                  |
//! |-------------------------|------------------|-----------------|---------------------------|
//! | insert(k, v)            | REEMPLAZA        | inserta         | Option<V> (valor anterior)|
//! | entry(k).or_insert(v)   | no hace nada     | inserta         | &mut V (para modificarlo) |

use std::collections::HashMap;

fn main() {
    let mut notas = HashMap::new();
    notas.insert("Ana", 8.5);
    notas.insert("Luis", 4.2);

    // insert con clave repetida: reemplaza y devuelve el valor anterior
    let valor_anterior = notas.insert("Ana", 9.5);
    println!(
        "insert(Ana, 9.5) → anterior: {:?}, ahora: {:?}",
        valor_anterior, notas["Ana"]
    );
    println!(
        "insert(Marta, 7.0) → anterior: {:?}",
        notas.insert("Marta", 7.0)
    ); // None

    // entry().or_insert(): solo inserta si la clave NO existe
    notas.entry("Ana").or_insert(1.0); // Ana existe → no cambia (sigue 9.5)
    notas.entry("Marcos").or_insert(6.0); // Marcos no existe → se inserta
    println!("{:?}", notas);

    // or_insert devuelve &mut V → se puede modificar el valor dentro del mapa
    let nota = notas.entry("Ana").or_insert(0.0);
    *nota = 2.7; // desreferencia (vídeo 11)
    println!("Ana ahora tiene: {}", notas["Ana"]);

    contar_palabras();
}

// Extra: el patrón más famoso de entry → contar apariciones
fn contar_palabras() {
    println!("\n== Contar palabras ==");
    let texto = "el perro y el gato y el loro";
    let mut conteo: HashMap<&str, i32> = HashMap::new();
    for palabra in texto.split_whitespace() {
        *conteo.entry(palabra).or_insert(0) += 1;
    }
    println!("{:?}", conteo); // {"el": 3, "y": 2, ...} (orden variable)
}
