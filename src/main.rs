fn main() {

    let mut cadena = String::from("hola amigos del Rust");
    let palabra = primera_palabra(&cadena);
    cadena.clear();

    let hola = &cadena[0..4];
    let amigos = &cadena[5..11];

    println!("Hola: {}", hola);
    println!("Amigos: {}", amigos);

    let cadena2 String::from("homa amigos del Rust");
    println!("La primera palabra de la cadena es: {}", primera_palabra2(&cadena2));
    
    let a = [1, 2, 3, 4, 5];
    let segmento = &a[1..2];

    assert_eql(segmento, [2, 3]);

}

fn primera_palabra(cadena: &String) -> usize {
    let bytes = cadena.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    cadena.len()
}

fn primera_palabra2(cadena: &str) -> &str {
    let bytes = cadena.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &cadena[0..1];
        }
    }
    &cadena[..]
}




