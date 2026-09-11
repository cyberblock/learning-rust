//! Vídeo 04 · Declaración de variables. Binding
//! Apuntes: ../apuntes/04-declaracion-de-variables-binding.md

fn main() {
    declarar_variables();
    inmutabilidad();
    binding_y_ownership();
    primitivos_se_copian();
    declaracion_diferida_y_tipo_explicito();
}

fn declarar_variables() {
    println!("== Declarar variables ==");
    // Con `let` creamos variables. Rust es de tipado ESTÁTICO: el tipo de una variable
    // no puede cambiar. Puede ser explícito o inferido (lo que muestra VS Code en gris
    // son "inlay hints": ayudas visuales que no forman parte del código).
    let edad = 25; // i32
    let mensaje = "Hola"; // &str
    let precio = 19.99; // f64
    let activa = true; // bool

    println!("{edad} {mensaje} {precio} {activa}");
}

fn inmutabilidad() {
    println!("\n== Inmutable por defecto ==");
    let edad = 30;
    // edad = 18; // ❌ error: cannot assign twice to immutable variable `edad`

    // Para poder cambiar el valor hay que usar `mut` (vídeo 05)
    let mut edad_mutable = edad;
    edad_mutable = edad_mutable + 1;
    println!("edad: {edad}, edad_mutable: {edad_mutable}");
}

fn binding_y_ownership() {
    println!("\n== Binding y ownership ==");
    // `let` crea un BINDING: un vínculo que hace a la variable DUEÑA del valor.
    // El texto "Jose" vive en el heap; `nombre` (en el stack) apunta a él.
    let nombre = String::from("Jose");

    // Al asignar una variable a otra, la propiedad del valor se transfiere (MOVE):
    // el dueño pasa a ser `dato` y `nombre` deja de ser válida.
    let dato = nombre;
    println!("Nombre: {}", dato);

    // println!("{}", nombre);
    // ❌ error[E0382]: borrow of moved value: `nombre`
    //    move occurs because `nombre` has type `String`, which does not implement the `Copy` trait
    //
    // ¿Por qué? Solo puede haber UN dueño por valor. Así Rust sabe exactamente quién
    // debe liberar la memoria y evita errores como la doble liberación (double free).

    // Si de verdad necesitas dos copias independientes, clónalo (tiene coste: duplica el heap)
    let original = String::from("Ana");
    let copia = original.clone();
    println!("original: {original}, copia: {copia}"); // ✅ ambas válidas
}

fn primitivos_se_copian() {
    println!("\n== Los primitivos se copian ==");
    // Los tipos primitivos (i32, f64, bool, char...) implementan `Copy`:
    // se copian en el stack y la variable original sigue siendo válida.
    let a = 10;
    let b = a;
    println!("a: {a}, b: {b}"); // ✅ sin error, a diferencia de String
}

fn declaracion_diferida_y_tipo_explicito() {
    println!("\n== Declarar ahora, inicializar después ==");
    let x; // declarada...
    x = 30; // ...e inicializada más tarde (una sola vez)
    let nombre_persona: &str = "Marta"; // snake_case + tipo explícito
    let y: i32 = 30;
    println!("x: {x}, y: {y}, nombre_persona: {nombre_persona}");
}
