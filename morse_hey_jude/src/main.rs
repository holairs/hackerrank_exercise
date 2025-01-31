mod preloaded;

use preloaded::MORSE_CODE;

fn decode_morse(encoded: &str) -> String {
    encoded
        .trim() // Eliminamos espacios innecesarios.
        .split("   ") // Dividimos en palabras Morse.
        .map(|word| {
            word.split(' ') // Dividimos en símbolos Morse.
                .filter_map(|symbol| MORSE_CODE.get(symbol)) // Buscamos en el HashMap.
                .cloned() // Clonamos los valores encontrados.
                .collect::<String>() // Formamos palabras.
        })
        .collect::<Vec<String>>() // Unimos todas las palabras.
        .join(" ") // Unimos con un espacio.
}

fn main() {
    let encoded = ".. ...- .- -.";
    let decoded = decode_morse(encoded);
    println!("{}", decoded);
    //println!("{:?}", MORSE_CODE); 
}
