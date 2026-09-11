# 02 · Características de Rust e instalación del software

- **Vídeo:** https://www.youtube.com/watch?v=81Yr-xwEr-A
- **Duración:** 19:22

## Resumen

Primera mitad teórica (qué es Rust, dónde se usa, comparación con C++) y segunda mitad práctica:
instalar Rust en Windows, preparar VS Code con rust-analyzer y crear/ejecutar el primer proyecto con Cargo.

## ¿Qué es Rust?

- Logo oficial: una **R dentro de un engranaje**. El cangrejo (**Ferris**) es la mascota de la comunidad.
- Creado por **Mozilla** (~2010); hoy lo mantiene la **Rust Foundation**.
- Pensado para programas **rápidos, seguros y estables**, donde el rendimiento es crítico.
- Usos típicos: sistemas operativos, servidores web, herramientas de consola, sistemas embebidos
  y microcontroladores, motores de videojuegos, criptografía/seguridad, servicios cloud, drivers, WebAssembly.
- Es un lenguaje **compilado**: primero se compila y luego se ejecuta.

## Rust vs C++

**En común**

- Programan muy cerca del hardware (bajo nivel).
- Control total de la memoria, **sin garbage collector** (a diferencia de Java).
- Sirven para desarrollar sistemas, drivers, etc.
- Concurrencia de bajo nivel optimizada: trabajan directamente con hilos y sincronización, sin capas
  intermedias que ralenticen.

**Diferencias**

| C++ | Rust |
|-----|------|
| Libertad total con la memoria | Te "acompaña": controla cómo usas la memoria |
| Confía en el **programador** | Confía en el **compilador** y en las reglas de **ownership** |
| Un error de memoria puede ser el caos | Muchos errores se detectan **en tiempo de compilación** |
| Permite crear *data races* | Un *data race* **no compila** |

- **Ownership (adelanto):** cada variable es *dueña* de un valor, y Rust impone reglas estrictas sobre
  cómo ese valor se usa, se mueve o se destruye. Se ve en detalle en el módulo 3.
- **Data race:** dos o más hilos acceden al mismo dato a la vez, al menos uno lo modifica y no hay
  ningún mecanismo de sincronización que coordine esos accesos. En C++ es posible; en Rust el compilador lo impide.
- Opinión del profesor: Rust llega al mismo objetivo que C++ de una forma más moderna, a cambio de
  someterte a unas reglas (la libertad total de C++ también tiene sus ventajas).

**Empresas que usan Rust:** Microsoft (componentes del kernel de Windows), AWS, Cloudflare, Google
(Android), Dropbox (motor de sincronización de archivos). Ya está en el top 20 del índice TIOBE.

## Instalación (Windows)

1. Buscar "Rust" → web oficial → **Install** (https://www.rust-lang.org/tools/install).
2. Descargar el instalador `rustup-init.exe` según tu arquitectura (32 bits, 64 bits o ARM).
3. Ejecutarlo: se abre una consola → pulsar **Enter** para la instalación estándar → al terminar, Enter para cerrar.
4. Instalar **Visual Studio Code** (instalación típica "siguiente, siguiente…").
5. Crear una carpeta para el curso (p. ej. `Curso Rust`) y abrirla en VS Code: *File → Open Folder*.
6. Instalar la extensión **rust-analyzer** (la oficial) desde el panel de extensiones.

## Comprobar que todo funciona

Abrir una terminal en VS Code (*Terminal → New Terminal*):

```bash
rustc --version        # debe mostrar la versión del compilador

cargo new hola_rust    # crea un proyecto nuevo
cd hola_rust
cargo run              # compila y ejecuta
```

Salida esperada:

```
   Compiling hola_rust v0.1.0 (...\Curso Rust\hola_rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target\debug\hola_rust.exe`
Hello, world!
```

Estructura que genera `cargo new`:

```
hola_rust/
├── src/          # código fuente (main.rs)
├── target/       # aparece al compilar
├── .gitignore
├── Cargo.lock
└── Cargo.toml
```

## Problemas comunes

- **`rustc` no se reconoce (texto en rojo):** normalmente pasa si instalaste Rust con VS Code abierto.
  Cierra VS Code, vuelve a abrirlo y repite el comando.
- Si sigue fallando, copia el error y búscalo (Google / un asistente de IA).
