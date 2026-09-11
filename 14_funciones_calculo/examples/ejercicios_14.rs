// Ejercicios · Vídeo 14
// Autor: Píldoras Informáticas · https://www.youtube.com/watch?v=IC-GOBEF0Vc
// Ejecuta con:  cargo run --example ejercicios_14
// Soluciones:   cargo run --example soluciones_14
//
// Con el vector de edades [15, 22, 17, 34, 41, 19, 12] calcula, sin bucles for:
//
// 1. La suma de todas las edades.
// 2. La primera edad mayor de 30 (find).
// 3. Cuántos son menores de edad (< 18).
// 4. ¿Hay alguien mayor de 40? (any)
// 5. ¿Son todos mayores de 10? (all)
// 6. La edad máxima y la mínima. Imprime el valor SIN el Some(...).
//    Pista: if let Some(x) = ... o .unwrap() si sabes que el vector no está vacío.

fn main() {
    let edades = vec![15, 22, 17, 34, 41, 19, 12];
    todo!("resuelve los ejercicios con {:?}", edades);
}
