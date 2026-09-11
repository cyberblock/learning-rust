//! Vídeo 12 · Vectores IV: iteradores explícitos
//! Apuntes: ../apuntes/12-vectores-IV-iteradores.md
//!
//! |                | Mueve (ownership)   | Préstamo inmutable | Préstamo mutable      |
//! |----------------|---------------------|--------------------|-----------------------|
//! | Sin iterador   | for n in v          | for n in &v        | for n in &mut v       |
//! | Con iterador   | v.into_iter()       | v.iter()           | v.iter_mut()          |

fn main() {
    iter();
    iter_mut();
    into_iter();
    iterador_a_mano();
}

fn iter() {
    println!("== iter()  ≡  &v ==");
    let numeros = vec![1, 2, 3];
    for n in numeros.iter() {
        println!("{n}"); // n: &i32
    }
    println!("sigue disponible: {:?}", numeros);
}

fn iter_mut() {
    println!("\n== iter_mut()  ≡  &mut v ==");
    let mut numeros = vec![1, 2, 3];
    for n in numeros.iter_mut() {
        *n *= 10; // n: &mut i32
    }
    println!("{:?}", numeros); // [10, 20, 30]
}

fn into_iter() {
    println!("\n== into_iter()  ≡  v  (consume el vector) ==");
    let nombres = vec![String::from("Ana"), String::from("Luis")];
    for nombre in nombres.into_iter() {
        println!("{nombre}"); // nombre: String (somos dueños de cada elemento)
    }
    // println!("{:?}", nombres); // ❌ borrow of moved value
}

// Extra: un iterador es un objeto con un método next() que devuelve Option.
// Eso es exactamente lo que hace el for por dentro.
fn iterador_a_mano() {
    println!("\n== Qué hace un for por dentro ==");
    let numeros = [1, 2, 3];
    let mut it = numeros.iter();
    println!("{:?}", it.next()); // Some(1)
    println!("{:?}", it.next()); // Some(2)
    println!("{:?}", it.next()); // Some(3)
    println!("{:?}", it.next()); // None → el for se detiene aquí
}
