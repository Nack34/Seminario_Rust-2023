mod teclado;

mod tp1;
mod tp2;
mod test_tp2;
fn main() {
   test_tp2::test_ej6();
}

/* -------------------------------------- DUDAS: ------------------------------------------- 


- Array de Strings? (no str) -> usar = Default::default(); para inicializar
- Punto 6 y 10 practica 2: Como hacer para leer x teclado? -> ingresar Strings (solucion a eso en la anterior pregunta). Mas adelante fijarse como hacer para usar correctamente los &str 
- Por que hay q usar crates para comunicar entre modulos y no simplemente mod?

*/