// Soluciones · Vídeo 07

fn presentar(nombre: &str, edad: u32) -> String {
    // format! funciona como println!, pero devuelve un String en vez de imprimir
    format!("Me llamo {nombre} y tengo {edad} años")
}

fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    // 1. `x` muere al cerrar el bloque, pero `r` se usa después → referencia colgante.
    //    Solución: que `x` viva lo suficiente (declararla fuera del bloque)
    let x = 5;
    let r;
    {
        r = &x;
    }
    println!("{}", r);

    // 2.
    println!("{}", presentar("Ana", 30));

    // 3.
    println!("{}", mas_larga("Rust", "Ferris el cangrejo"));

    // 4.
    let mut frase = String::from("Rust es seguro");
    frase.push_str(" y rápido");
    println!("{}", frase.to_uppercase());
}
