fn main() {
    //Hola mundo
    println!("Hola, Rust!");

    //Variables
    let mut my_string: &str = "Esto es una cadena de texto"; //Aqui designamos la mayor cantidad de memmoria
    println!("{my_string}"); //Imprime la primer declaracion
    my_string = "Aqui se cambio el valor de la cadena de texto";
    println!("{my_string}"); //Implrime el cambio 
    my_string ="Prueba de mutacion";
    println!("{my_string}");

    let my_string2 : String = String :: from("Esta es otra cadena de texto"); //Aqui solo designamos una cantidad limitada de memoria
    println!("{my_string2}");
}
