// Soluciones · Vídeo 17
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=zBWrq4RYQNk

use std::collections::HashMap;

fn frecuencia_palabras(frase: &str) -> HashMap<String, usize> {
    let mut conteo = HashMap::new();
    for palabra in frase.split_whitespace() {
        *conteo.entry(palabra.to_lowercase()).or_insert(0) += 1;
    }
    conteo
}

fn contar_letras(texto: &str) -> HashMap<char, usize> {
    let mut conteo = HashMap::new();
    for c in texto.chars().filter(|c| !c.is_whitespace()) {
        *conteo.entry(c).or_insert(0) += 1;
    }
    conteo
}

fn main() {
    // 1.
    let mut notas = HashMap::new();
    notas.insert("Ana", 8.5);
    if let Some(anterior) = notas.insert("Ana", 6.0) {
        println!("Se ha sobrescrito la nota {anterior}");
    }

    // 2.
    println!("{:?}", frecuencia_palabras("Hola hola mundo HOLA rust"));

    // 3. max_by_key busca el par (letra, veces) con más apariciones
    let letras = contar_letras("programar en rust");
    if let Some((letra, veces)) = letras.iter().max_by_key(|(_, veces)| **veces) {
        println!("La letra más repetida es '{letra}' ({veces} veces)");
    }

    // 4. El mismo código sirve exista o no el producto
    let mut inventario: HashMap<&str, u32> = HashMap::new();
    inventario.insert("tuercas", 20);
    *inventario.entry("tornillos").or_insert(0) += 5; // no existía → 5
    *inventario.entry("tornillos").or_insert(0) += 5; // ya existía → 10
    println!("{:?}", inventario);
}
