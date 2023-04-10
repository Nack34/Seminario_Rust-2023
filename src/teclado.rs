use std::io::stdin;
use std::str::FromStr;

pub fn ingresar_str<'a>(nombre: &'a mut  String) -> &'a str {
    stdin().read_line(nombre).expect("Error al leer el nombre.");
    let y: &str = &nombre.trim();
    //println!("{}",y);
    return y
}
pub fn ingresar_nro_i32() -> i32 {
    let mut nombre = String::new();
    stdin().read_line(&mut nombre).expect("Error al leer el nombre.");
    let y: i32 = i32::from_str(&nombre.trim()).unwrap();
    //println!("{}",y);
    return y
}
pub fn ingresar_bool() -> bool {
    let mut nombre = String::new();
    stdin().read_line(&mut nombre).expect("Error al leer el nombre.");
    let y: bool = bool::from_str(&nombre.trim()).unwrap();
    //println!("{}",y);
    return y
}