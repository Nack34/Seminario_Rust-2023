use tp3::Auto;

mod teclado;

mod tp1;
mod tp2;
mod test_tp2;
mod tp3;
mod tp4;
mod tp5;
mod ricciardi_nicolas_e3_no_terminado;
fn main() { // tp3, fecha, es_proxima => esta mal hecho, corregir


/* test tp4
   tp4::test_strum();

   let array1 = [tp4::Persona::new("a","a","a","a",0.0,5),
   tp4::Persona::new("a","a","a","a",0.0,5),
   tp4::Persona::new("a","a","a","a",0.0,5),
   tp4::Persona::new("a","a","a","a",0.0,5)];
   let array2 = tp4::get_edades(&array1);
*/

/* test tp3
   let a:tp3::Auto = tp3::Auto::new("auto".to_string(),  "mo".to_string(), 40, 50.0, tp3::Color::Amarillo);
   let mut autos:Vec<tp3::Auto> = Vec::new();
   autos.push(a);
   let c = tp3::ConcesionarioAuto::new("aa".to_string(), "bb".to_string(), 5, autos);

   let a:tp3::Auto = tp3::Auto::new("auto".to_string(),  "mo".to_string(), 40, 50.0, tp3::Color::Amarillo);
   let j = c.buscar_auto(&a);


   let a:tp3::Auto = tp3::Auto::new("auto".to_string(),  "mo".to_string(), 40, 50.0, tp3::Color::Amarillo);
   let j = c.buscar_auto(&a);

   println!("{:#?}",j);
*/
}

/* -------------------------------------- DUDAS: ------------------------------------------- 


- Array de Strings? (no str) -> usar = Default::default(); para inicializar
- Punto 6 y 10 practica 2: Como hacer para leer x teclado? -> ingresar Strings (solucion a eso en la anterior pregunta). Mas adelante fijarse como hacer para usar correctamente los &str 
- Por que hay q usar crates para comunicar entre modulos y no simplemente mod?

*/