use std::io::stdin;
use std::str::FromStr;

pub fn ingresar_str<'a>(buf: &'a mut  String) -> &'a str {
    stdin().read_line(buf).expect("Error al leer el buf.");
    let y: &str = &buf.trim();
    //println!("{}",y);
    return y
}
pub fn ingresar_string() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Error al leer el buf.");
    let y = buf.trim().to_string();
    //println!("{}",y);
    return y
}
pub fn ingresar_nro_i32() -> i32 {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Error al leer el buf.");
    let y: i32 = i32::from_str(&buf.trim()).unwrap();
    //println!("{}",y);
    return y
}
pub fn ingresar_bool() -> bool {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Error al leer el buf.");
    let y: bool = bool::from_str(&buf.trim()).unwrap();
    //println!("{}",y);
    return y
}