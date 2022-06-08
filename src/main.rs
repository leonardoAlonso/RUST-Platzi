fn main() {
    print!("Por favor introduce tu nombre: ");
    
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();
    // Obtener edad desde consola
    print!("Por favor introduce tu edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    // convertir la edad a numero
    let edad_int: u8 = edad.trim().parse().unwrap();

    print!("Hola, bienvenido {} de {} años", nombre, edad_int);

    if edad_int > 18 && edad_int != 30 {
        print!("Puedes pasar");
    } else if edad_int == 30 {
        print!("Tu no");
    } else {
        print!("No puedes pasar");
    }

}
