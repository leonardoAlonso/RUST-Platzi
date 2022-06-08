fn main() {
    let numero_1 = 123;
    let numero_2 = 321;
    let suma = numero_1 + numero_2;

    loop {
        // Mostrar los 2 numeros en pantalla
        print!("Escribir la suma de: {} + {}", numero_1, numero_2);
    
        // Obtener del usuario el numero que representa la suma
    
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();
        let suma_usuario_int: i32 = suma_usuario.trim().parse().unwrap();
    
        if suma_usuario_int == suma {
            print!("Lo has conseguido ✅");
            break;
        } else {
            print!("El resultado no es correcto intenta de nuevo ❌");
        }
    }

}
