use crate::teclado;
/* 1-Definir la función llamada es_par que recibe como parámetro un número entero y retorna
true si el número es par, false caso contrario. */
pub fn ej1(){
    println!("{}",es_par(teclado::ingresar_nro_i32()));
}
fn es_par(num:i32) -> bool {
    num%2==0
}

/* 2- Definir la función llamada es_primo que recibe un número entero positivo mayor a 1 y
retorna true si es primo, false caso contrario */
pub fn ej2(){
    println!("{}",es_primo(teclado::ingresar_nro_i32()));
}
fn es_primo(num:i32) -> bool{        
    for i in 2..num-1{
        if num%i==0{
            return false
        }
    }
    return true
}

/* 3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de
números enteros y retorna la suma de los números pares. */
const N_ej3:usize = 4; // longitud del array
pub fn ej3(){
    let mut array:[i32;N_ej3] = [0;N_ej3]; // array de long N lleno de 0
    for i in 0..N_ej3{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    println!("{}",suma_pares(array));
}
fn suma_pares(array:[i32;N_ej3]) -> i32{
    let mut suma = 0;
    for i in array {
        if es_par(i){
            suma+=i;
        }
    }
    return suma
}

/* 4- Definir la función llamada cantidad_impares que recibe como parámetro un arreglo de
números enteros y retorna la cantidad de números impares.
 */
const N_ej4:usize = 4; // longitud del array
pub fn ej4(){
    let mut array:[i32;N_ej4] = [0;N_ej4]; // array de long N lleno de 0
    for i in 0..N_ej4{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    println!("{}",cantidad_impares(array));
}
fn cantidad_impares(array:[i32;N_ej4]) -> i32{
    let mut suma = 0;
    for i in array {
        if !es_par(i){
            suma+=1;
        }
    }
    return suma
}

/* 5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro.
 */
const N_ej5:usize = 4; // longitud del array
pub fn ej5(){
    let mut array:[f32;N_ej5] = [0.0;N_ej5]; // array de long N lleno de 0
    for i in 0..N_ej5{ // carga del array
        array[i] = teclado::ingresar_nro_i32() as f32;
    }

    println!("{:?}",duplicar_valores(array));
}
fn duplicar_valores(array:[f32;N_ej5]) -> [f32;N_ej5] {
    let mut array2:[f32;N_ej5] = [0.0;N_ej5];
    
    for i in 0..array.len() {
        array2[i] = array[i]*2.0;
    }
    return array2
}

/* 6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
arreglo. */
const N_ej6:usize = 5; // longitud del array
pub fn ej6(){

    let mut array:[&str;N_ej6] =   ["a",
                                    "bb",
                                    "ccc",
                                    "ddddd",
                                    "eeee"];


    // ---------------------------------------- PREGUNTAR -------------------------------------------
    /*let mut array:[&str;N_ej6] = ["";N_ej6]; // array de long N lleno de 0
    for i in 0..N_ej6{ // carga del array
        let mut ingresar_por_teclado= String::from("");

        array[i] = teclado::ingresar_str(&mut ingresar_por_teclado);
    }*/ 
    // ---------------------------------------- PREGUNTAR -------------------------------------------

    println!("{:?}",longitud_de_cadenas(array));
}
fn longitud_de_cadenas(array:[&str;N_ej6]) -> [i32;N_ej6] {
    let mut array2:[i32;N_ej6] = [0;N_ej6];
    
    for i in 0..array.len() {
        array2[i] = array[i].len() as i32;
    }
    return array2
}

/* 7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo.
 */
const N_ej7:usize = 4; // longitud del array
pub fn ej7(){
    let mut array:[i32;N_ej7] = [0;N_ej7]; // array de long N lleno de 0
    for i in 0..N_ej7{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    println!("{}",cantidad_de_mayores(array,teclado::ingresar_nro_i32()));
}
fn cantidad_de_mayores(array:[i32;N_ej7],limite:i32) -> i32{
    let mut suma = 0;
    for i in array {
        if i>limite{
            suma+=1;
        }
    }
    return suma
}

/* 8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
arreglos pasados por parámetro, correspondiendose el resultado con cada posición de los
arreglos pasados por parámetro.
 */
const N_ej8:usize = 4; // longitud del array
pub fn ej8(){
    let mut array1:[f32;N_ej8] = [0.0;N_ej8]; // array de long N lleno de 0
    let mut array2:[f32;N_ej8] = [0.0;N_ej8]; // array de long N lleno de 0
    for i in 0..N_ej8{ // carga del array
        array1[i] = teclado::ingresar_nro_i32() as f32;
        array2[i] = teclado::ingresar_nro_i32() as f32;
    }

    println!("{:?}",sumar_arreglos(array1,array2));
}
fn sumar_arreglos(array1:[f32;N_ej8],array2:[f32;N_ej8]) -> [f32;N_ej8] {
    let mut array3:[f32;N_ej8] = [0.0;N_ej8];
    
    for i in 0..array1.len() {
        array3[i] = array1[i]+array2[i];
    }
    return array3
}

/* 9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive.
 */
const N_ej9:usize = 4; // longitud del array
pub fn ej9(){
    let mut array:[i32;N_ej9] = [0;N_ej9]; // array de long N lleno de 0
    for i in 0..N_ej9{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    println!("{}",cantidad_en_rango(array,teclado::ingresar_nro_i32(),teclado::ingresar_nro_i32()));
}
fn cantidad_en_rango(array:[i32;N_ej9],inferior:i32,superior:i32) -> i32{
    let mut suma = 0;
    for i in array {
        if (i>=inferior) && (i<=superior) {
            suma+=1;
        }
    }
    return suma
}

/* 10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite. */
const N_ej10:usize = 5; // longitud del array
pub fn ej10(){

    let mut array:[&str;N_ej10] =   ["a",
                                    "bb",
                                    "ccc",
                                    "ddddd",
                                    "eeee"];


    // ---------------------------------------- PREGUNTAR -------------------------------------------
    /*let mut array:[&str;N_ej10] = ["";N_ej10]; // array de long N lleno de 0
    for i in 0..N_ej10{ // carga del array
        let mut ingresar_por_teclado= String::from("");

        array[i] = teclado::ingresar_str(&mut ingresar_por_teclado);
    }*/ 
    // ---------------------------------------- PREGUNTAR -------------------------------------------

    println!("{:?}",cantidad_de_cadenas_mayor_a(array,teclado::ingresar_nro_i32()));
}
fn cantidad_de_cadenas_mayor_a(array:[&str;N_ej10],limite:i32) -> i32 {
    let mut cant = 0;

    for i in array {
        if i.len() as i32 >limite{
            cant+=1;
        }
    }
    return cant
}

/* 11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo */
const N_ej11:usize = 4; // longitud del array
pub fn ej11(){
    let mut array:[i32;N_ej11] = [0;N_ej11]; // array de long N lleno de 0
    for i in 0..N_ej11{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    array = multiplicar_valores(array,teclado::ingresar_nro_i32());

    println!("{:?}",array);
}
fn multiplicar_valores(mut array:[i32;N_ej11], factor:i32) -> [i32;N_ej11]{    
    for i in 0..array.len() {
        array[i] = array[i]*factor;
    }
    return array;
}

/* 12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
reemplaza todos los números pares por -1.
 */
const N_ej12:usize = 5; // longitud del array
pub fn ej12(){
    let mut array:[i32;N_ej12] = [0;N_ej12]; // array de long N lleno de 0
    for i in 0..N_ej12{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    array = reemplazar_pares(array);

    println!("{:?}",array);
}
fn reemplazar_pares(mut array:[i32;N_ej12]) -> [i32;N_ej12]{
    for i in 0..array.len() {
        if es_par(array[i]){
            array[i]=-1;
        }
    }
    return array
}

/* 13-Definir una función llamada ordenar_nombres que recibe un arreglo de String y los
ordena en orden alfabético */
const N_ej13:usize = 5; // longitud del array
pub fn ej13(){

    let mut array:[&str;N_ej13] =  ["aa",
                                    "cc",
                                    "dd",
                                    "bb",
                                    "ee"];

    // ---------------------------------------- PREGUNTAR -------------------------------------------
    /*let mut array:[&str;N_ej13] = ["";N_ej13]; // array de long N lleno de 0
    for i in 0..N_ej13{ // carga del array
        let mut ingresar_por_teclado= String::from("");

        array[i] = teclado::ingresar_str(&mut ingresar_por_teclado);
    }*/ 
    // ---------------------------------------- PREGUNTAR -------------------------------------------

    array = ordenar_nombres(array);
    println!("{:?}",array);
}
fn ordenar_nombres(mut array:[&str;N_ej13]) -> [&str;N_ej13] {
    array.sort();
    return array;
}


/* 14-Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor */
pub fn ej14(){
    let mut num = teclado::ingresar_nro_i32() as f32;
    num = incrementar(num);
    println!("{}",num);
}
fn incrementar(num: f32) -> f32{
    num+1.0
}















