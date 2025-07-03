use std::io;

struct Persona {
    nombre: String,
    edad: u32,
    email: String,
    genero: String,
}

fn validar_nombre() -> String {
   let mut entrada: String = String::new();
   println!("Por favor, ingresa tu nombre: ");

   io::stdin()
    .read_line(&mut entrada)
    .expect("Error al ingresar nombre");
    
    let nombre = entrada.trim().to_string();
    println!("Hola, {}!", nombre);
    nombre
}

fn validar_edad() -> u32 {
    loop {
        let mut entrada = String::new();

        println!("Por favor, ingresa tu edad: ");
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al ingresar edad");

        // Intentamos convertir la entrada a un número entero sin signo (u32)
        match entrada.trim().parse::<u32>() {
            Ok(edad) => {
                if edad < 18 {
                    println!("Eres menor de edad");
                } else {
                    println!("Eres mayor de edad");
                }

                return edad; // ✅ Aquí devolvemos la edad si fue válida
            }
            Err(_) => {
                println!("Edad inválida. Por favor, ingresa un número."); // ✅ Este mensaje es necesario
                continue; // Vuelve a pedir la entrada
            }
        }
    }
}


fn validar_email() -> String {
   loop {
        let mut entrada: String = String::new();
        println!("Por favor, ingresa tu email: ");

        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al ingresar email");
        
        let email = entrada.trim();

        if email.contains("@") && email.contains(".") {
            return email.to_string()
        } else {
            println!("Email invalido.")
        }
    }
}

fn validar_genero() -> String {
    loop {
        let mut entrada: String = String::new();
        println!("Por favor, ingresa tu género (M/F): ");
        
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al ingresar género");

        

        let genero = entrada.trim().to_uppercase();

        if genero == "M" {
            return "Masculino".to_string();
        } else if genero == "F" {
            return "Femenino".to_string();
        } else {
            println!("Género no válido. Usa 'M' o 'F'.");
            continue; //repite la pregunta
        }
    }
}


fn pedir_datos(){

    let nombre = validar_nombre();
    let edad = validar_edad();
    let email = validar_email();
    let genero = validar_genero();


    let persona = Persona {
        nombre,
        edad,
        email,
        genero,
    };

    println!("Persona registrada: ");
    println!("{}", persona.nombre);
    println!("edad: {}", persona.edad);
    println!("Email {}", persona.email);
    println!("Genero: {}", persona.genero);
}

//main
fn main() {
    println!("Bienvenido al sistema interactivo");
    println!("Si no aparece la primera pregunta, presiona Enter para comenzar...");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    pedir_datos();
}


