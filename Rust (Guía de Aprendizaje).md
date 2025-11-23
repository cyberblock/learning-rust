# Tutoriales Rust (Guía de Aprendizaje)

Serie de pasos sugeridos para aprender Rust apoyándote en los videos del canal Píldoras Informáticas (https://www.youtube.com/@pildorasinformaticas). Contenido original de este README, usar los videos como refuerzo conceptual.

## Objetivos
- Aprender fundamentos de Rust de forma incremental.
- Practicar cada concepto con ejercicios breves.
- Adoptar buenas prácticas (ownership, borrowing, errores, concurrencia segura).

## Ruta de Aprendizaje

### 1. Preparación
- Instalar `rustup` (https://rustup.rs).
- Verificar: `rustc --version`, `cargo --version`.
- Crear primer proyecto: `cargo new hola_rust`.

Ejercicio: Imprime "Hola Rust" y compila con `cargo build`, ejecuta con `cargo run`.

### 2. Fundamentos Sintácticos
Temas: `fn main`, comentarios, `let`, mutabilidad, tipos primitivos, `println!`.

Ejercicio: Declara variables con y sin `mut`, imprime tipos usando debug `{:?}`.

### 3. Control de Flujo
Temas: `if`, `else if`, `loop`, `while`, `for`, rangos.

Ejercicio: Tabla de multiplicar (1..=10) con `for`.

### 4. Propiedad (Ownership)
Conceptos: Movimiento, `Copy`, pila vs heap, `String` vs `&str`.

Ejercicio: Escribe funciones que consumen `String` y otras que reciben `&str`. Observa qué compila.

### 5. Préstamos (Borrowing) y Referencias
Temas: `&T`, `&mut T`, reglas de préstamos.

Ejercicio: Función que calcula la longitud de una cadena sin mover la propiedad.

### 6. Slices
Temas: `&str`, slices de arreglos, fat pointers.

Ejercicio: Función que devuelve primer palabra de una frase usando slice sobre `&str`.

### 7. Structs y Impl
Temas: Definición, métodos (`impl`), `derive(Debug)`.

Ejercicio: Struct `Usuario { id: u32, nombre: String }` con método `fn nombre_mayus(&self)`.

### 8. Enums y Pattern Matching
Temas: `enum`, `match`, `if let`.

Ejercicio: Enum `ResultadoRed { Ok(u16), Error(String) }` y función que imprime distinto según el caso.

### 9. Option y Result
Temas: Manejo seguro de ausencia, propagación de errores (`?`).

Ejercicio: Función que lee un número desde `String` y devuelve `Result<u32, String>`.

### 10. Módulos y Organización
Temas: `mod`, `pub`, estructura en carpetas, `lib.rs`.

Ejercicio: Separa lógica de parseo en `mod parser`.

### 11. Genéricos y Traits
Temas: Parámetros genéricos, `trait`, `impl<T>`.

Ejercicio: Trait `Resumen` con método `resumen()`. Implementar para dos structs.

### 12. Lifetimes (Intro)
Temas: Referencias con duración, elixir mínimo: `'a`.

Ejercicio: Función que devuelve la referencia más larga entre dos `&str`.

### 13. Colecciones Estándar
Temas: `Vec`, `HashMap`, `String`, iteradores.

Ejercicio: Contar frecuencia de palabras en una frase usando `HashMap`.

### 14. Iteradores y Cierres
Temas: `iter()`, `map`, `filter`, `collect`, closures.

Ejercicio: Filtrar números pares y elevar al cuadrado.

### 15. Concurrencia
Temas: `std::thread`, `move`, `join`, `Mutex`, `Arc`.

Ejercicio: Contador compartido incrementado por múltiples hilos.

### 16. Errores Personalizados
Temas: `thiserror` (opcional), enum de errores, `From`.

Ejercicio: Enum `AppError` para IO y Parse.

### 17. Macros (Vista Inicial)
Temas: Diferencia `macro_rules!` vs funciones.

Ejercicio: Macro simple que envuelva `println!` con prefijo `[DEBUG]`.

### 18. Cargo y Publicación
Temas: `Cargo.toml`, features, workspaces, `cargo fmt`, `cargo clippy`.

Ejercicio: Añade `clippy` y corrige avisos.

### 19. Testing
Temas: Tests unitarios, integración, `#[should_panic]`.

Ejercicio: Test para función de parseo y uno que verifique error.

### 20. Proyecto Final
Construir mini CLI: lectura de archivo, conteo de líneas, búsqueda de patrón.

## Ejercicios Extra
- Implementar `FizzBuzz` genérico sobre cualquier tipo que implemente `Display`.
- Simular caja registradora con `HashMap<String, f32>`.
- Parser simple para una mini expresión matemática (suma y multiplicación).

## Buenas Prácticas
- Usa `cargo fmt` y `cargo clippy` regularmente.
- Prefiere `Result` sobre `panic!` en librerías.
- Documenta con `///` y genera docs: `cargo doc --open`.

## Recursos Complementarios
- The Rust Book: https://doc.rust-lang.org/book/
- Rustlings: https://github.com/rust-lang/rustlings
- Playground: https://play.rust-lang.org

## Estructura Sugerida de Carpetas (Ejemplo)