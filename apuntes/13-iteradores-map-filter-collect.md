# 13 · Iteradores V: `map()`, `filter()` y `collect()`

- **Vídeo:** https://www.youtube.com/watch?v=5T-x8m6b-98
- **Autor:** [Píldoras Informáticas](https://www.youtube.com/@pildorasinformaticas)
- **Duración:** 19:38

## Resumen

Se resuelve el mismo problema de tres formas: **extraer los números pares de un vector y multiplicarlos
por 2**. Primero con un `for` clásico (estilo **imperativo**), después con `iter().filter().map().collect()`
y **closures** (estilo **funcional/declarativo**) y, por último, con funciones con nombre en lugar de
closures, para ver lo que simplifican.

## Las tres funciones

| Función | Qué hace |
|---------|----------|
| `filter` | **Filtra** los datos (se queda con los que cumplen una condición) |
| `map` | **Transforma** los datos |
| `collect` | **Construye una nueva colección** (por ejemplo, un `Vec`) |

- Todo esto se puede hacer sin ellas, con bucles `for`. ¿Por qué usarlas? Por **simplicidad**.
- **Estilo imperativo** (`for` + `if`): das órdenes y controlas el proceso paso a paso.
- **Estilo funcional/declarativo** (iteradores): **describes lo que quieres** y la sintaxis queda más clara y breve.

## Versión 1: bucle `for` (imperativo)

```rust
fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6];
    let mut resultado = Vec::new();

    for n in &numeros {
        if n % 2 == 0 {             // % = módulo (resto de la división): si es 0, es par
            resultado.push(n * 2);
        }
    }

    println!("{:?}", resultado);    // [4, 8, 12]
}
```

## Versión 2: iteradores + closures (funcional)

```rust
fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6];

    let resultado: Vec<i32> = numeros
        .iter()                     // 1) iterador sobre &i32
        .filter(|n| *n % 2 == 0)    // 2) nos quedamos con los pares
        .map(|n| n * 2)             // 3) los multiplicamos por 2
        .collect();                 // 4) los reunimos en un Vec nuevo

    println!("{:?}", resultado);    // [4, 8, 12]
}
```

- Lo habitual es poner **cada método encadenado en su propia línea**.
- `filter` y `map` **no modifican el vector original** (`numeros` sigue siendo `[1, 2, 3, 4, 5, 6]`).
  Por eso hace falta `collect()` para guardar el resultado en un vector nuevo.
- Hay que indicar el tipo de destino (`Vec<i32>`) para que `collect` sepa qué colección construir.

### Closures (funciones anónimas)

```rust
|n| n * 2      // parámetros entre barras verticales | |, y a continuación la expresión
```

- Son funciones **sin nombre**, escritas en línea.
- Equivalen a las *arrow functions* de JavaScript (`n => n * 2`) o a las *lambdas* de otros lenguajes.
- Los tipos que muestra VS Code en gris (`n: &&i32`) son solo ayudas visuales.

## Versión 3: con funciones con nombre (para comparar)

Sin closures hay que declarar una función para cada paso y pasarlas por nombre:

```rust
fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6];

    let resultado: Vec<i32> = numeros
        .iter()
        .filter(es_par)
        .map(duplicar)
        .collect();

    println!("{:?}", resultado);    // [4, 8, 12]
}

fn es_par(n: &&i32) -> bool {   // ¡doble referencia!
    *n % 2 == 0
}

fn duplicar(n: &i32) -> i32 {
    n * 2                        // sin punto y coma: es el valor de retorno
}
```

**Detalle importante:** `filter` siempre recibe una **referencia al elemento** del iterador. Como
`iter()` ya produce referencias (`&i32`), a `filter` le llega una **referencia de referencia (`&&i32`)**.
Si declaras `es_par(n: &i32)`, el compilador da un error de tipos (*mismatched types*) y sugiere envolverla
en una closure.

La conclusión del profesor: las closures **simplifican muchísimo** el código. La versión 2 es la más elegante.

## Flujo del ejemplo

```
Vec [1,2,3,4,5,6]
   │ .iter()             → recorre los elementos
   ▼
   │ .filter(|n| par)    → 2, 4, 6
   ▼
   │ .map(|n| n * 2)     → 4, 8, 12
   ▼
   │ .collect()          → Vec [4, 8, 12]
```
