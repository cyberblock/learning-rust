//! Vídeo 14 · Iteradores VI: funciones útiles para cálculos
//! Apuntes: ../apuntes/14-iteradores-funciones-de-calculo.md
//!
//! Lo importante: fíjate en QUÉ DEVUELVE cada una (número, bool u Option).

fn main() {
    let numeros: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    // sum → número (hay que indicar el tipo)
    let suma_pares: i32 = numeros.iter().filter(|n| *n % 2 == 0).sum();
    let suma_dobles: i32 = numeros.iter().filter(|n| *n % 2 == 0).map(|n| n * 2).sum();
    println!("sum de pares: {suma_pares}, de sus dobles: {suma_dobles}"); // 12, 24

    // find → Option<&i32>: el PRIMERO que cumple la condición (a la closure le llega &&i32)
    println!("find > 3: {:?}", numeros.iter().find(|n| **n > 3)); // Some(4)
    println!("find > 7: {:?}", numeros.iter().find(|n| **n > 7)); // None

    // count → usize
    println!(
        "count pares: {}",
        numeros.iter().filter(|n| *n % 2 == 0).count()
    ); // 3

    // any / all → bool
    println!("any par: {}", numeros.iter().any(|n| n % 2 == 0)); // true
    println!("all par: {}", numeros.iter().all(|n| n % 2 == 0)); // false

    // max / min → Option<&i32> (None solo si el vector está vacío)
    println!(
        "max: {:?}, min: {:?}",
        numeros.iter().max(),
        numeros.iter().min()
    );
    println!("max de [6, 6, 6]: {:?}", [6, 6, 6].iter().max()); // Some(6)
    let vacio: Vec<i32> = Vec::new();
    println!("max de un vector vacío: {:?}", vacio.iter().max()); // None

    // Extra: otras funciones muy usadas
    println!("\n== Extras ==");
    println!("position del 4: {:?}", numeros.iter().position(|n| *n == 4)); // Some(3)
    println!("product: {}", numeros.iter().product::<i32>()); // 720
    println!("contains 5: {}", numeros.contains(&5)); // true (método de Vec, no de iterador)
    let (pares, impares): (Vec<i32>, Vec<i32>) = numeros.iter().partition(|n| *n % 2 == 0);
    println!("partition: pares {:?}, impares {:?}", pares, impares);
}
