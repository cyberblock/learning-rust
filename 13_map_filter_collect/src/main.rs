//! Vídeo 13 · Iteradores V: map(), filter() y collect()
//! Apuntes: ../apuntes/13-iteradores-map-filter-collect.md
//!
//! Problema: quedarse con los pares de un vector y multiplicarlos por 2.
//!   filter  → filtra      map → transforma      collect → construye una colección nueva

fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6];

    println!("Imperativo (for + if):   {:?}", imperativo(&numeros));
    println!("Funcional (closures):    {:?}", funcional(&numeros));
    println!("Con funciones con nombre: {:?}", con_funciones(&numeros));
    println!("Original intacto:        {:?}", numeros);

    closures();
}

// Nota: las funciones reciben &[i32] (un slice, vídeo 08) en vez de &Vec<i32>:
// es más flexible porque acepta vectores, arrays y trozos de ellos.

/// Estilo IMPERATIVO: das órdenes y controlas cada paso.
fn imperativo(numeros: &[i32]) -> Vec<i32> {
    let mut resultado = Vec::new();
    for n in numeros {
        if n % 2 == 0 {
            resultado.push(n * 2);
        }
    }
    resultado
}

/// Estilo FUNCIONAL/declarativo: describes QUÉ quieres.
fn funcional(numeros: &[i32]) -> Vec<i32> {
    numeros
        .iter() //                   &i32
        .filter(|n| *n % 2 == 0) //  a filter le llega &&i32
        .map(|n| n * 2) //           i32
        .collect() //                el tipo de retorno (Vec<i32>) le dice qué construir
}

/// Sin closures: hay que declarar funciones aparte (fíjate en la doble referencia).
fn con_funciones(numeros: &[i32]) -> Vec<i32> {
    numeros.iter().filter(es_par).map(duplicar).collect()
}

fn es_par(n: &&i32) -> bool {
    *n % 2 == 0
}

fn duplicar(n: &i32) -> i32 {
    n * 2
}

fn closures() {
    println!("\n== Closures (funciones anónimas) ==");
    let doble = |x: i32| x * 2; // se pueden guardar en una variable
    let suma = |a: i32, b: i32| a + b; // varios parámetros
    let saludar = |nombre: &str| {
        // cuerpo con varias líneas → llaves
        let texto = format!("Hola, {nombre}");
        texto.to_uppercase()
    };
    println!("{} {} {}", doble(4), suma(2, 3), saludar("ana"));

    // Una closure puede usar variables de su entorno (las "captura")
    let limite = 3;
    let mayores: Vec<i32> = vec![1, 5, 2, 8]
        .into_iter()
        .filter(|n| *n > limite)
        .collect();
    println!("mayores que {limite}: {:?}", mayores);
}
