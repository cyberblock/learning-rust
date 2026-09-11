//! Vídeo 11 · Vectores III: recorrer un vector con for
//! Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=VfS3EKlMvzQ
//! Apuntes: ../apuntes/11-vectores-III-bucle-for.md
//!
//! | Forma             | n es     | ¿vector usable después? | ¿modifica? |
//! |-------------------|----------|-------------------------|------------|
//! | for n in v        | T        | ❌ (move al iterador)   | —          |
//! | for n in &v       | &T       | ✅ (préstamo)           | ❌         |
//! | for n in &mut v   | &mut T   | ✅                      | ✅ con *n  |

fn main() {
    por_valor_mueve();
    por_referencia();
    por_referencia_mutable();
    con_indice();
}

fn por_valor_mueve() {
    println!("== for n in v  (MUEVE el vector) ==");
    let numeros = vec![1, 2, 3];
    for n in numeros {
        println!("{n}");
    }
    // println!("{:?}", numeros);
    // ❌ borrow of moved value: `numeros` — la propiedad pasó al iterador del for
}

fn por_referencia() {
    println!("\n== for n in &v  (préstamo) ==");
    let numeros = vec![1, 2, 3];
    for n in &numeros {
        println!("{n}"); // n: &i32
    }
    println!("después del bucle: {:?}", numeros); // ✅
}

fn por_referencia_mutable() {
    println!("\n== for n in &mut v  (modificar) ==");
    let mut numeros = vec![1, 2, 3]; // 1) el vector debe ser mut
    for n in &mut numeros {
        // 2) préstamo mutable
        *n += 1; // 3) * desreferencia: suma 1 al VALOR apuntado, no a la referencia
    }
    println!("{:?}", numeros); // [2, 3, 4]
}

// Extra: si además necesitas la posición, usa enumerate()
fn con_indice() {
    println!("\n== Índice + valor con enumerate() ==");
    let frutas = vec!["manzana", "pera", "uva"];
    for (i, fruta) in frutas.iter().enumerate() {
        println!("{i}: {fruta}");
    }
}
