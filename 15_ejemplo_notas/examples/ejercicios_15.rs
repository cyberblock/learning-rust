// Ejercicios · Vídeo 15
// Ejecuta con:  cargo run --example ejercicios_15
// Soluciones:   cargo run --example soluciones_15
//
// Amplía el ejemplo de las notas. Con `notas = [7.5, 4.1, 8.2, 9.1, 3.8, 6.4]`:
//
// 1. Obtén los suspensos (< 5.0) en un Vec<f64> (valores, no referencias).
//    Pista: .copied() convierte un iterador de &f64 en uno de f64.
// 2. Calcula la nota mínima.
// 3. Cuenta cuántos sobresalientes hay (>= 9.0).
// 4. Implementa `calificacion(nota: f64) -> &'static str` que devuelva
//    "Suspenso" (<5), "Aprobado" (<7), "Notable" (<9) o "Sobresaliente".
//    Luego genera un Vec<&str> con la calificación de cada nota usando map.
// 5. Muestra la media redondeada a 1 decimal con {:.1}.

#![allow(unused_variables)]

fn calificacion(nota: f64) -> &'static str {
    todo!()
}

fn main() {
    let notas: Vec<f64> = vec![7.5, 4.1, 8.2, 9.1, 3.8, 6.4];
    println!("{}", calificacion(notas[0]));
}
