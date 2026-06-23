# Rust
**En un futuro lo usaré:** Framework web (Tauri)

*Apuntes tomados por mi, Rodrigo.*

Es un lenguaje de bajo nivel:
- Propósito general y multiparadigma.
- Su sintaxis es parecida a C, C++.
- Rendimiento, demasiado rápido y eficiente.
- Cargo es su herramienta para compilar.

## Usos habituales:
- Línea de comando.
- WebAssembly (formato binario que permite ejecutar código de forma casi nativa).
- Redes (Bajo consumo de recursos).
- Dispositivos Integrados.
- Blockchain, seguridad.

---

## 1. Mi primer Hola mundo en Rust
```rust
fn main() {
    //Hola mundo
    println!("Hola, Rust!");
}
```

---

## 2. Variables y Tipos de Cadenas (Mutabilidad)
Ejemplo práctico de mutabilidad de cadenas usando referencias a literales (`&str`) y cadenas dinámicas (`String`):

```rust
// Declaración mutable usando &str (Aquí designamos la mayor cantidad de memoria)
let mut my_string: &str = "Esto es una cadena de texto";
println!("{my_string}"); // Imprime la primer declaracion

my_string = "Aqui se cambio el valor de la cadena de texto";
println!("{my_string}"); // Imprime el cambio 

my_string = "Prueba de mutacion";
println!("{my_string}");

// Creación de una cadena dinámica usando String (Aquí solo designamos una cantidad limitada de memoria)
let my_string2: String = String::from("Esta es otra cadena de texto");
println!("{my_string2}");
```

---

## 3. Enteros y Decimales (Integers y Floats)
Ejemplos de definición de tipos de datos numéricos y operaciones sobre variables mutables:

```rust
// Integer (Entero de 32 bits)
let mut my_int: i32 = 7;
my_int = my_int + 4;

// Usamos la mutacion de la variable en ejecucion
println!("{}", my_int - 1); 

println!("Este es el valor de la variable my_int: {}", my_int);

// Entero de 64 bits (my_int64)
let my_int64: i64 = 7;
println!("{my_int64}");

// Float (Decimal de 64 bits)
let my_float: f64 = 6.5;
println!("{my_float}");
```
