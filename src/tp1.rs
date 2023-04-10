use crate::teclado;
/*1- Escribir un programa que defina una variable de tipo flotante con algún valor, y luego 
permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y 
restar su valor. Se deben imprimir los resultados. 
*/
pub fn ej1() {
    let x = 3.0;
    let y = teclado::ingresar_nro_i32();
    println!("Los nros son: {}, {}",x,y);
    println!("Su multiplicacion da: {}",x*(y as f64));
    println!("Su divicion da: {}",x/(y as f64));
    println!("Su suma da: {}",x+(y as f64));
    println!("Su resta da: {}",x-(y as f64));


}

/* 2- Escribir un programa que defina una variable de tipo entero sin signo, y luego imprima su
valor en hexadecimal */
pub fn ej2(){
    let x :u32 = 17;
    println!("{:x}",x);
}

/* 3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
and y or. Se deben imprimir ambos resultados.
 */
pub fn ej3(){
    let b = false;
    let c = teclado::ingresar_bool();
    println!("{}",b && c) ;
    println!("{}",b || c) ;
}

/* 4- Escribir un programa que defina una tupla que contenga una cadena, un número entero
con signo y un valor booleano, y luego imprima cada valor de la tupla
 */
pub fn ej4(){
    let a:(String,i32,bool) = ("hola".to_string(),4,true);
    println!("{:?}",a);
}

/* 5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
cadena en mayúsculas.
 */
pub fn ej5(){
    let mut cadena:String= "Hola ".to_string();
    let mut nombre = String::new();
    cadena += teclado::ingresar_str(&mut nombre); 
    println!("{}",cadena.as_str().to_uppercase());
}

/* 6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
usuario ingresar un número entero por teclado para sumarse con la variable definida. El
programa debe imprimir el valor del número elevado al cuadrado.
 */
pub fn ej6(){
    let x: u32 = 5;
    let x = (x as i32) + teclado::ingresar_nro_i32();

    println!("{}",x.pow(2))
}

/* 7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números
enteros, y luego multiplique cada valor del arreglo por un valor constante definido,
modificando el contenido del arreglo.
 */
pub fn ej7(){
    let mut array:[i32;6] =  [0,1,2,3,4,5];
    for x in array{
        println!("{}",x);
    }

    let constante = teclado::ingresar_nro_i32();

    for i in 1..(array.len()) {
        array[i] *= constante;
    }

    for x in array{
        println!("{}",x);
    }
       
}

/* 8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el
número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
Se debe imprimir el resultado.
 */
pub fn ej8(){
    let cadena:&str = "kudytszsfdghtyc";
    let char_vec: Vec<char> = cadena.chars().collect();

    let mut caracter_buscado:String = "".to_string();
    let caracter_buscado: Vec<char> = (teclado::ingresar_str(&mut caracter_buscado)).chars().collect();
    let caracter_buscado = caracter_buscado[0];

    let mut contador =0;
    
    for x in char_vec{
        if x == caracter_buscado{
            contador += 1;
        }
    }
    println!("{}",contador);
}

/* 9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
suma de los valores del arreglo. */
pub fn ej9(){
    let array = [0,1,2,3,4];
    let mut cont = 0;
    for x in array{
        cont+=x;
    }
    println!("{}",cont);
}

/* 10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos
originales.
 */
pub fn ej10(){
    let array1 = [5,1,2,3,4];
    let array2 = [0,1,2,3,4];
    let mut array3 = [0,1,2,3,4];
    for i in 0..array1.len(){
        array3[i] = array1[i] + array2[i];
    }
    println!("{:?}",array3);
}

/* 11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
ingresada por el usuario se encuentra en el arreglo.
 */
pub fn ej11(){
    let array:[&str;5] =   ["aa",
                            "bb",
                            "cc",
                            "dd",
                            "ee"];

    let mut str_buscado = "".to_string();
    let str_buscado = teclado::ingresar_str(&mut str_buscado);
    for x in array{
        if str_buscado == x{
            println!("{}",str_buscado);
        }
    }
}

/* 12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de
enteros, y luego imprima la cadena y la suma de los valores en el arreglo. */
pub fn ej12(){
    let tupla = ("holaa",[0,1,2,3,4,5]);
    println!("{}",tupla.0);
    let mut cont = 0;
    for x in tupla.1{
        cont+=x;
    }
    println!("{}",cont);

}
