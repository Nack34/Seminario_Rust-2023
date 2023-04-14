use crate::teclado;
use crate::tp2;

pub fn test_ej1(){
    println!("{}",tp2::es_par(teclado::ingresar_nro_i32()));
}

pub fn test_ej2(){
    println!("{}",tp2::es_primo(teclado::ingresar_nro_i32()));
}

pub fn test_ej3(){
    let mut array:[i32;tp2::N_ej3] = [0;tp2::N_ej3]; // array de long N lleno de 0
    for i in 0..tp2::N_ej3{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    println!("{}",tp2::suma_pares(array));
}

pub fn test_ej4(){
    let mut array:[i32;tp2::N_ej4] = [0;tp2::N_ej4]; // array de long N lleno de 0
    for i in 0..tp2::N_ej4{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    println!("{}",tp2::cantidad_impares(array));
}

pub fn test_ej5(){
    let mut array:[f32;tp2::N_ej5] = [0.0;tp2::N_ej5]; // array de long N lleno de 0
    for i in 0..tp2::N_ej5{ // carga del array
        array[i] = teclado::ingresar_nro_i32() as f32;
    }

    println!("{:?}",tp2::duplicar_valores(array));
}

pub fn test_ej6(){
    let mut array:[String;tp2::N_ej6] = Default::default();
    for i in 0..tp2::N_ej6{ // carga del array
        array[i] = teclado::ingresar_string();
    }

    println!("{:?}",tp2::longitud_de_cadenas(array));
}

pub fn test_ej7(){
    let mut array:[i32;tp2::N_ej7] = [0;tp2::N_ej7]; // array de long N lleno de 0
    for i in 0..tp2::N_ej7{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    println!("{}",tp2::cantidad_de_mayores(array,teclado::ingresar_nro_i32()));
}

pub fn test_ej8(){
    let mut array1:[f32;tp2::N_ej8] = [0.0;tp2::N_ej8]; // array de long N lleno de 0
    let mut array2:[f32;tp2::N_ej8] = [0.0;tp2::N_ej8]; // array de long N lleno de 0
    for i in 0..tp2::N_ej8{ // carga del array
        array1[i] = teclado::ingresar_nro_i32() as f32;
        array2[i] = teclado::ingresar_nro_i32() as f32;
    }

    println!("{:?}",tp2::sumar_arreglos(array1,array2));
}

pub fn test_ej9(){
    let mut array:[i32;tp2::N_ej9] = [0;tp2::N_ej9]; // array de long N lleno de 0
    for i in 0..tp2::N_ej9{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    println!("{}",tp2::cantidad_en_rango(array,teclado::ingresar_nro_i32(),teclado::ingresar_nro_i32()));
}

pub fn test_ej10(){
    let mut array:[String;tp2::N_ej10] =  Default::default(); 
    for i in 0..tp2::N_ej10{ // carga del array
        array[i] = teclado::ingresar_string();
        
    }

    println!("{:?}",tp2::cantidad_de_cadenas_mayor_a(array,teclado::ingresar_nro_i32()));
}

pub fn test_ej11(){
    let mut array:[i32;tp2::N_ej11] = [0;tp2::N_ej11]; // array de long N lleno de 0
    for i in 0..tp2::N_ej11{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    array = tp2::multiplicar_valores(array,teclado::ingresar_nro_i32());
    println!("{:?}",array);
}

pub fn test_ej12(){
    let mut array:[i32;tp2::N_ej12] = [0;tp2::N_ej12]; // array de long N lleno de 0
    for i in 0..tp2::N_ej12{ // carga del array
        array[i] = teclado::ingresar_nro_i32();
    }

    array = tp2::reemplazar_pares(array);
    println!("{:?}",array);
}

pub fn test_ej13(){
    let mut array:[String;tp2::N_ej13] =  Default::default(); 
    for i in 0..tp2::N_ej13{ // carga del array
        array[i] = teclado::ingresar_string();
    }

    array = tp2::ordenar_nombres(array);
    println!("{:?}",array);
}

pub fn test_ej14(){
    let mut num = teclado::ingresar_nro_i32() as f32;
    num = tp2::incrementar(num);
    println!("{}",num);
}
