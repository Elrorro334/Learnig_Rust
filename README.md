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

## Mi primer Hola mundo en Rust
```rust
fn main() {
    println!("Hola, Rust!");
}
```

## Variables y Tipos de Cadenas (Mutabilidad)
Ejemplo práctico de mutabilidad de cadenas usando referencias a literales (`&str`) y cadenas dinámicas en heap (`String`):

```rust
fn main() {
    // Declaración mutable usando &str
    let mut my_string: &str = "Esto es una cadena de texto";
    println!("{my_string}");
    
    my_string = "Aqui se cambio el valor de la cadena de texto";
    println!("{my_string}");
    
    my_string = "Prueba de mutacion";
    println!("{my_string}");

    // Creación de una cadena dinámica en heap usando String
    let my_string2: String = String::from("Esta es otra cadena de texto");
    println!("{my_string2}");
}
```

