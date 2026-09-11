//! Vídeo 15 · Ejemplo práctico: aprobados, nota media y nota más alta
//! Apuntes: ../apuntes/15-ejemplo-practico-notas.md

fn main() {
    let notas: Vec<f64> = vec![7.5, 4.1, 8.2, 9.1, 3.8, 6.4];

    // 1) Aprobados: iter() da &f64 → el resultado es Vec<&f64>.
    //    iter() NO consume `notas`, así que podemos seguir usándolo abajo.
    let aprobados: Vec<&f64> = notas
        .iter()
        .filter(|nota| **nota >= 5.0) // &&f64 → doble desreferencia; comparar con 5.0, no 5
        .collect();

    // 2) Media: len() es usize → hay que convertirlo con `as f64` para dividir
    let suma: f64 = notas.iter().sum();
    let media = suma / notas.len() as f64;
    // En una línea hace falta el turbofish para decirle a sum() el tipo:
    let media_una_linea = notas.iter().sum::<f64>() / notas.len() as f64;

    // 3) Nota máxima: max() no funciona con f64 porque f64 no implementa Ord (por NaN).
    //    max_by + partial_cmp (devuelve Option<Ordering>) + unwrap (seguro: no hay NaN)
    let nota_maxima = notas.iter().max_by(|a, b| a.partial_cmp(b).unwrap());

    println!("Aprobados:   {:?}", aprobados);
    println!("Media:       {:.2}", media); // {:.2} = 2 decimales
    println!("Media (1 línea): {:.2}", media_una_linea);
    println!("Nota máxima: {:?}", nota_maxima);

    // Extra: alternativas modernas para el máximo de f64
    let maxima_fold = notas.iter().copied().fold(f64::MIN, f64::max);
    let maxima_total_cmp = notas.iter().max_by(|a, b| a.total_cmp(b)); // no necesita unwrap
    println!(
        "Máxima con fold: {maxima_fold}, con total_cmp: {:?}",
        maxima_total_cmp
    );
}
