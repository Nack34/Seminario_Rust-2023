use crate::tp3::Fecha;
use std::{collections::{HashMap, HashSet}}; 
/* 1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de
números primos. Cree un trait para la determinación del número primo e impleméntelo
según corresponda. Utilice la función iter sobre el vector y aplique un closure para
resolverlo.
 */
pub trait chequear_es_primo {
    fn es_primo(&self) -> bool;
}
impl chequear_es_primo for i32{
    fn es_primo(&self) -> bool{
        for i in 2..self/2{
            if self%i==0 {return false}
        }
        return true
    }
}
pub fn cant_primos(vector:Vec<i32>) -> u32{
    vector.iter().filter(|x| x.es_primo()).count() as u32
}

/* 2- Dado el siguiente struct:
struct Persona<'a>{
nombre:&'a str,
apellido:&'a str,
direccion:&'a str,
ciudad:&'a str,
salario:f64,
edad:u8,
}
Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
Todos los ejercicios deben resolverse con iterator y closure.
 */
#[derive(PartialEq)]
 pub struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}
impl <'a>Persona<'a>{
    pub fn new( nombre:&'a str, apellido:&'a str, direccion:&'a str, ciudad:&'a str, salario:f64, edad:u8,)->Persona<'a>{      
        Persona{nombre,
        apellido,
        direccion,
        ciudad,
        salario,
        edad,}
    }
}
/* a- Escriba una función que reciba un vector de personas y otro parámetro que indica un
salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
*/
/*
pub fn personas_con_salario_mayor_a (personas:Vec<Persona<'a>>, salario:f64 ) -> Vec<Persona<'a>>{
    personas.into_iter().filter(|p| p.salario > salario).collect()
}
*/
pub fn personas_con_salario_mayor_a <'a> (personas:&'a Vec<&'a Persona<'a>>, salario:f64 ) -> Vec<&'a Persona<'a>>{
    personas.iter()
    .filter(|p| p.salario > salario)
    .map(|p| p.clone())
    .collect()
}
/*b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
ciudad. */
pub fn personas_mayores_a_y_que_viven_en<'a>(personas:&'a Vec<&'a Persona<'a>>, edad:u8, ciudad:&'a str) ->Vec<&'a Persona<'a>>{
    personas.iter().filter(|p| p.ciudad==ciudad && p.edad>edad).map(|p| p.clone()).collect()
}
/* c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
contrario. */
pub fn todas_las_personas_viven_en<'a>(personas:&'a Vec<&'a Persona<'a>>,ciudad:&'a str) ->bool{
    personas.iter().all(|p|p.ciudad == ciudad)
}
/* d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
contrario. */
pub fn al_menos_una_persona_vive_en<'a>(personas:&'a Vec<&'a Persona<'a>>,ciudad:&str) -> bool{
    personas.iter().any(|p| p.ciudad==ciudad)
}
/* e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
persona existe en el arreglo, false caso contrario. */
// ------------------------ ??????????????????????????????? No entiendo el objetivo de este punto (no se hace con iterator)
pub fn array_contiene<'a>(personas:&'a [&'a Persona], persona:&'a Persona) -> bool{ 
    personas.contains(&persona)
}
/* f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.*/
pub fn get_edades<'a, const N: usize>(personas:&'a [&'a Persona<'a>;N]) -> [u8;N]{
    personas.iter().map(|p| p.edad)
    .collect::<Vec<u8>>() 
    .try_into()
    .unwrap()
}
/* g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande. */
pub fn get_personas_min_y_max_salario <'a> (personas:&'a [&'a Persona<'a>]) -> PersonasMinMaxSalarios<'a>{
    let mut res = PersonasMinMaxSalarios{min:None, max: None};
    
    res.set_max(personas.iter()
    .max_by_key(|p| p.salario as i64)
    .unwrap());
    res.set_min(personas.iter().min_by_key(|p| p.salario as i64).unwrap());
    
    res
}
pub struct PersonasMinMaxSalarios <'a>{
    min:Option<&'a Persona<'a>>,
    max:Option<&'a Persona<'a>>,
}
impl <'a>PersonasMinMaxSalarios<'a> {
    pub fn set_min(&mut self, persona_min_salario:&'a Persona<'a>){
        self.min=Some(persona_min_salario);
    }
    pub fn set_max(&mut self, persona_max_salario:&'a Persona<'a>){
        self.max=Some(persona_max_salario);
    }
}

/* 3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
correspondientes a excepción de Efectivo.
Los usuarios solo pueden tener una suscripción activa a la vez.
*/
pub struct StreamingRust <'a>{
    subscripciones:HashMap<&'a Usuario,Subscripcion<'a>>,
}
#[derive(PartialEq,Eq)]
pub struct Usuario{
    nombre:String,
    id:u32
}
impl Hash for Usuario{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
pub struct Subscripcion<'a>{
    info_propia:InfoDeSubscripcion, 
    info_relacionada_al_usuario:InfoDeSubscripcionRelacionadaAlUsuario<'a>, 
}
pub struct InfoDeSubscripcionRelacionadaAlUsuario<'a> {
    usuario: &'a Usuario,
    medio_de_pago:MedioDePago,
}
//cargo add strum (https://crates.io/crates/strum)
use strum::{EnumCount};
use strum_macros::{EnumCount as EnumCountMacro,FromRepr};
/*pub fn test_strum(){
    print!("{}",MedioDePago::COUNT); // funciona bien
}*/
#[derive(EnumCountMacro,Copy, Clone,FromRepr)]
#[repr(usize)]
pub enum MedioDePago {
    Efectivo, MercadoPago(DataMedioDePago), TarjetaDeCredito(DataMedioDePago), TransferenciaBancaria(DataMedioDePago), Cripto(DataMedioDePago)
}
impl MedioDePago { 
    fn discriminant(&self) -> usize { // https://docs.rs/enum-repr/latest/enum_repr/ // https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<usize>() }
    }
    pub fn to_usize(&self) -> usize{ // por si encuentro manera de hacerlo sin usar codigo inseguro
        self.discriminant()
    }
    pub const fn tamanio() -> usize{ // https://crates.io/crates/strum
        MedioDePago::COUNT
    }
}
#[derive(Copy, Clone, Default)]
pub struct DataMedioDePago{
    nro_de_tarjeta:u32
}
pub struct InfoDeSubscripcion{
    tipo_de_subscripcion:TipoDeSubscripcion,
    costo_mensual:f64,
    duracion_en_meses:u32,
    fecha_de_inicio:Fecha,
}
#[derive(EnumCountMacro,Copy, Clone,FromRepr,PartialEq)]
#[repr(usize)]
pub enum TipoDeSubscripcion{
    Basic, Clasic, Super
}
impl TipoDeSubscripcion {
    fn discriminant(&self) -> usize {
        unsafe { *<*const _>::from(self).cast::<usize>() }
    }
    pub fn to_usize(&self) -> usize{ // por si encuentro manera de hacerlo sin usar codigo inseguro
        self.discriminant()
    }
    pub const fn tamanio() -> usize{ // https://crates.io/crates/strum
        MedioDePago::COUNT
    }
}
// ---------- Constructores
impl<'a> StreamingRust<'a>{
    pub fn new()-> StreamingRust<'a>{
        StreamingRust { subscripciones: HashMap::new() }
    }
}
impl Usuario{
    pub fn new (nombre:String, id:u32) -> Usuario{
        Usuario { nombre, id }
    }
}
impl <'a>Subscripcion<'a>{
    pub fn new(info_propia:InfoDeSubscripcion, usuario: &'a Usuario, medio_de_pago:MedioDePago ) ->Subscripcion<'a> {
        Subscripcion{info_propia,
                    info_relacionada_al_usuario: InfoDeSubscripcionRelacionadaAlUsuario{
                        usuario,
                        medio_de_pago} }
    }
}
impl InfoDeSubscripcion{
    pub fn new(tipo_de_subscripcion:TipoDeSubscripcion,costo_mensual:f64, duracion_en_meses:u32, fecha_de_inicio:Fecha )->InfoDeSubscripcion{
        InfoDeSubscripcion { tipo_de_subscripcion,  costo_mensual, duracion_en_meses, fecha_de_inicio }
    }
}
impl DataMedioDePago{
    pub fn new(nro_de_tarjeta:u32)->DataMedioDePago{
        DataMedioDePago { nro_de_tarjeta }
    }
}
/* Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
➢ Crear un usuario con una determinada suscripción y medio de pago. */
impl<'a> StreamingRust<'a>{
    // retorna true si el usuario no tenia otra subscripcion valida, false si no se pudo completar la accion
    pub fn agregar_subscripcion(&mut self, usuario: &'a Usuario, info_de_subscripcion:InfoDeSubscripcion, medio_de_pago:MedioDePago, hoy:Fecha) -> bool{ 
        if !self.subscripciones.contains_key(usuario) {
            self.subscripciones.insert(usuario,Subscripcion::new(info_de_subscripcion, usuario, medio_de_pago));
            return true
        }
     
        let subscripcion_guardada = self.subscripciones.get_mut(usuario).unwrap();
        if subscripcion_guardada.esta_vencida(&hoy){
            subscripcion_guardada.set_info(info_de_subscripcion);
            subscripcion_guardada.set_medio_de_pago(medio_de_pago);
            return true
        }
        
        return false
    }
}
impl<'a> Subscripcion<'a>{
    pub fn esta_vencida (&self, hoy:&'a Fecha)-> bool{
        self.info_propia.chequear_vencimiento(hoy)
    }
    pub fn set_info(&mut self, info_propia:InfoDeSubscripcion){
        self.info_propia=info_propia;
    }
    pub fn set_medio_de_pago(&mut self, medio_de_pago:MedioDePago){
        self.info_relacionada_al_usuario.medio_de_pago=medio_de_pago;
    }
}
impl  <'a> InfoDeSubscripcion{
    pub fn chequear_vencimiento (&self, hoy:&'a Fecha)->bool{
        self.fecha_de_inicio.diferencencia_de_meses(hoy) > self.duracion_en_meses
    }
}

/* ➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
pasa a Clasic y si está en Clasic pasa a Super. */
impl <'a> StreamingRust<'a> {
    pub fn upgrade_subscripcion_de(&mut self, user: &'a Usuario){
        if !self.subscripciones.contains_key(user) {panic!("USUARIO NO REGISTRADO")}

        self.subscripciones.get_mut(user).unwrap().upgrade_subscripcion();
    }
}
impl<'a> Subscripcion<'a>{
    pub fn upgrade_subscripcion(&mut self){
        self.info_propia.upgrade();
    }
}
impl InfoDeSubscripcion{
    pub fn upgrade(&mut self){
        match self.tipo_de_subscripcion{
            TipoDeSubscripcion::Basic => self.tipo_de_subscripcion = TipoDeSubscripcion::Clasic, 
            TipoDeSubscripcion::Clasic => self.tipo_de_subscripcion = TipoDeSubscripcion::Super, 
            TipoDeSubscripcion::Super => {}
        }
    }
}

/* ➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
suscripción es del tipo Basic al hacerlo se cancelará la suscripción. */
impl <'a> StreamingRust<'a> {
    pub fn downgrade_subscripcion_de(&mut self, user: &'a Usuario) {
        if !self.subscripciones.contains_key(user) {panic!("USUARIO NO REGISTRADO")}

        if self.subscripciones.get_mut(user).unwrap().downgrade_or_else_remove_subscripcion(){
            self.cancelar_subscripcion(user);
        }
    }

/* ➢ Dado un usuario cancelar la suscripción. */
    pub fn cancelar_subscripcion(&mut self, user: &'a Usuario){
        // Esto podria hacerse de otras maneras. Depende de si se quiere seguir manteniendo la informacion de ...
        //... las subs canceladas o no, yo asumo que eso no se quiere guardar asi que se elimina toda informacion
        self.subscripciones.remove(user); 
    }
}
impl<'a> Subscripcion<'a>{
    pub fn downgrade_or_else_remove_subscripcion(&mut self) -> bool{ // returns true si tiene que ser cancelada 
        if self.info_propia.es_el_peor_tipo_de_subscripcion() {return true}
        self.info_propia.downgrade();
        false
    }
}
impl InfoDeSubscripcion{
    pub fn es_el_peor_tipo_de_subscripcion(&self) -> bool{
        self.tipo_de_subscripcion == TipoDeSubscripcion::Basic
    }
    pub fn downgrade <'a> (&mut self){ 
        match self.tipo_de_subscripcion{
            TipoDeSubscripcion::Basic => {}, 
            TipoDeSubscripcion::Clasic => {self.tipo_de_subscripcion=TipoDeSubscripcion::Basic}, 
            TipoDeSubscripcion::Super => {self.tipo_de_subscripcion=TipoDeSubscripcion::Clasic}
        }
    }
}

/* ➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
suscripciones activas */
impl <'a> StreamingRust<'a> {
    pub fn medio_de_pago_mas_utilizado_de_subscripciones_activas(&self, hoy:Fecha) ->MedioDePago { // uso el crate strum, para un mejor manejo de enums

        const COUNT:usize = MedioDePago::tamanio(); // cantidad de enums = longitud del array
        let mut medios_de_pago: [u32; COUNT] = [0;COUNT]; // el index representa el medio de pago, el contenido representa la cantidad de veces que aparece ese enum

        self.subscripciones.iter()
        .filter(|s|!s.1.esta_vencida(&hoy)) //obtengo las subscripciones activas 
        .map(|s|s.1.get_medio_de_pago()) // obtengo los medios de pagos de las subs activas
        .for_each(|medio_de_pago|medios_de_pago[medio_de_pago.to_usize()]+=1); // cargo la cant de veces que se repite cada medio de pago

        let max_index = medios_de_pago.iter()
        .enumerate() // enlazo los indices a los valores
        .max_by_key(|x|x.1) // obtengo el maximo
        .unwrap().0; // obtengo el indice del valor maximo

        //let a:Vec<MedioDePago> = Vec::new();
        //a.iter().acumular();

        /* la idea seria: 
        impl <'a> StreamingRust<'a> {
            pub fn medio_de_pago_mas_utilizado(&self) ->MedioDePago { // uso el crate strum, para un mejor manejo de enums

                let max_medio_de_pago = self.subscripciones.iter()
                .filter(|s|!s.1.esta_vencida()) //obtengo las subscripciones activas 
                .map(|s|s.1.medio_de_pago) // obtengo los medios de pagos de las subs activas
                .acumular() // ---------- ESTO NO EXISTE, seria un iterator con <elem:MedioDePago,cant_repeticiones:u8> 
                .max_by_key(|x|x.1) // obtengo el máximo  
                .unwrap().0; // obtengo el MedioDePagp del valor máximo 

                max_medio_de_pago  // lo retorno
            }
        }
         */

        MedioDePago::from_repr(max_index).unwrap() // convierto el index obtenido al enum y lo retorno
    }
}
/*pub trait IAcumulable{
    fn acumular(&self) -> u32{
        5
    }
}
impl <T> IAcumulable for Iter<'_, T>{
    fn acumular(&self) -> u32{
        5
    }
}*/
impl<'a> Subscripcion<'a> {
    pub fn get_medio_de_pago(&self) -> &MedioDePago{
        self.info_relacionada_al_usuario.get_medio_de_pago()
    }
}
impl<'a> InfoDeSubscripcionRelacionadaAlUsuario <'a> {
    pub fn get_medio_de_pago(&self) -> &MedioDePago{
        &self.medio_de_pago
    }
}

/* ➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
activas. */
impl <'a> StreamingRust<'a> {
    pub fn tipo_de_subscripcion_mas_contratada_de_subscripciones_activas(&self, hoy:Fecha) ->TipoDeSubscripcion { 
        
        const COUNT:usize = TipoDeSubscripcion::tamanio(); // cantidad de enums = longitud del array
        let mut tipos_de_subscripciones: [u32; COUNT] = [0;COUNT]; // el index representa el medio de pago, el contenido representa la cantidad de veces que aparece ese enum

        self.subscripciones.iter()
        .filter(|s|!s.1.esta_vencida(&hoy)) //obtengo las subscripciones activas 
        .map(|s|s.1.get_tipo_de_subscripcion()) // obtengo los medios de pagos de las subs activas
        .for_each(|tipo_de_subscripcion|tipos_de_subscripciones[tipo_de_subscripcion.to_usize()]+=1); // cargo la cant de veces que se repite cada medio de pago

        let max_index = tipos_de_subscripciones.iter()
        .enumerate() // enlazo los indices a los valores
        .max_by_key(|x|x.1) // obtengo el maximo
        .unwrap().0; // obtengo el indice del valor maximo

        TipoDeSubscripcion::from_repr(max_index).unwrap()
    }
}
impl<'a> Subscripcion<'a> {
    pub fn get_tipo_de_subscripcion(&self) -> &TipoDeSubscripcion {
        self.info_propia.get_tipo_de_subscripcion()
    }
}
impl InfoDeSubscripcion  {
    pub fn get_tipo_de_subscripcion(&self) -> &TipoDeSubscripcion {
        &self.tipo_de_subscripcion
    }
}
/* ➢ Saber cuál fue el medio de pago más utilizado. */ //--------------- ???? No es casi lo mismo que la de arriba?
/* ➢ Saber cuál fue la suscripción más contratada. */ //--------------- ???? No es casi lo mismo que la de arriba?

impl <'a> StreamingRust<'a> {
    pub fn medio_de_pago_mas_utilizado(&self) ->MedioDePago { 

        const COUNT:usize = MedioDePago::tamanio(); // cantidad de enums = longitud del array
        let mut medios_de_pago: [u32; COUNT] = [0;COUNT]; // el index representa el medio de pago, el contenido representa la cantidad de veces que aparece ese enum

        self.subscripciones.iter()
        .map(|s|s.1.get_medio_de_pago()) // obtengo los medios de pagos de las subs activas
        .for_each(|medio_de_pago|medios_de_pago[medio_de_pago.to_usize()]+=1); // cargo la cant de veces que se repite cada medio de pago

        let max_index = medios_de_pago.iter()
        .enumerate() // enlazo los indices a los valores
        .max_by_key(|x|x.1) // obtengo el maximo
        .unwrap().0; // obtengo el indice del valor maximo

        MedioDePago::from_repr(max_index).unwrap() // convierto el index obtenido al enum y lo retorno
    }
}
impl <'a> StreamingRust<'a> {
    pub fn tipo_de_subscripcion_mas_contratada(&self) ->TipoDeSubscripcion { 
        
        const COUNT:usize = TipoDeSubscripcion ::tamanio(); // cantidad de enums = longitud del array
        let mut tipos_de_subscripciones: [u32; COUNT] = [0;COUNT]; // el index representa el medio de pago, el contenido representa la cantidad de veces que aparece ese enum

        self.subscripciones.iter()
        .map(|s|s.1.get_tipo_de_subscripcion()) // obtengo los medios de pagos de las subs activas
        .for_each(|tipo_de_subscripcion|tipos_de_subscripciones[tipo_de_subscripcion.discriminant()]+=1); // cargo la cant de veces que se repite cada medio de pago

        let max_index = tipos_de_subscripciones.iter()
        .enumerate() // enlazo los indices a los valores
        .max_by_key(|x|x.1) // obtengo el maximo
        .unwrap().0; // obtengo el indice del valor maximo

        TipoDeSubscripcion::from_repr(max_index).unwrap()
    }
}

/* 4 -Se requiere implementar un sistema de ventas de productos. De cada producto se
conoce el nombre, una categoría y un precio base, y algunos productos pueden tener
descuentos aplicables dependiendo de la categoría. Además, se debe registrar al vendedor
que realizó la venta y al cliente. De ellos se conoce nombre, apellido, dirección, dni y del
vendedor nro de legajo, antigüedad y salario. Los clientes pueden tener un beneficio de
descuento si tienen suscripción al newsletter, de ser así se tiene el correo electrónico del
mismo.
El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado.
Los medios de pago aceptados son: tarjeta de crédito, tarjeta de débito, transferencia
bancaria y efectivo.
*/
pub struct SistemaDeVentas<'a>{
    ventas:Vec<Venta<'a>>
}
pub struct Venta<'a >{
    fecha:Fecha,
    productos_vendidos:Vec<&'a Producto>,
    vendedor:&'a Vendedor,
    cliente:&'a Cliente,
    medio_de_pago:OpcionDePago
}
pub struct Producto{
    nombre:String, categoria:String, precio_base:f32,
}
pub struct Categorias{ // se usa en un metodo mas adelante
    categorias_con_sus_descuentos:HashMap<String,f32>
}
#[derive(Eq, Hash, PartialEq)]
pub struct DataPersona{
    nombre:String, apellido:String, dirección:String, dni:String
}
#[derive(Eq, Hash, PartialEq)]
pub struct Vendedor{
    data_persona:DataPersona,
    nro_de_legajo:u32, antigüedad:u32, salario:u32
}
pub struct Cliente{
    data_persona:DataPersona,
    subscripcion_newsletter:Option<InformacionNecesariaParaNewsletter>
}
pub struct InformacionNecesariaParaNewsletter{
    descuento:f32, correo:String
}
pub enum OpcionDePago{
    TarjetaDeCredito, TarjetaDeDebito, TransferenciaBancaria, Efectivo
}
/*
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de
productos.
*/
// ------------------------ constructores
impl <'a> SistemaDeVentas<'a>{
    pub fn new () -> SistemaDeVentas<'a>{
        SistemaDeVentas { ventas: Vec::new() }
    }
    pub fn set_ventas(&mut self, ventas:Vec<Venta<'a>>){
        self.ventas = ventas;
    }
}
impl <'a> Venta<'a>{
    pub fn new(fecha:Fecha,productos_vendidos:Vec<&'a Producto>, vendedor:&'a Vendedor, cliente:&'a Cliente, medio_de_pago:OpcionDePago) -> Venta<'a> {
        Venta { fecha, productos_vendidos, vendedor, cliente, medio_de_pago }
    }    
}
impl Producto{
    pub fn new(nombre:String, categoria:String, precio_base:f32) -> Producto {
        Producto { nombre, categoria, precio_base }
    }    
}
impl Categorias{
    pub fn new() -> Categorias {
        Categorias { categorias_con_sus_descuentos:HashMap::new() }
    }    
}
impl DataPersona{
    pub fn new(nombre:String, apellido:String, dirección:String, dni:String) -> DataPersona {
        DataPersona { nombre, apellido, dirección, dni }
    }    
}
impl Vendedor{
    pub fn new(data_persona:DataPersona, nro_de_legajo:u32, antigüedad:u32, salario:u32) -> Vendedor {
        Vendedor { data_persona, nro_de_legajo, antigüedad, salario }
    }    
}
impl Cliente{
    pub fn new(data_persona:DataPersona, subscripcion_newsletter:Option<InformacionNecesariaParaNewsletter>) -> Cliente {
        Cliente { data_persona, subscripcion_newsletter }
    }    
}
impl InformacionNecesariaParaNewsletter{
    pub fn new(descuento:f32, correo:String) -> InformacionNecesariaParaNewsletter {
        InformacionNecesariaParaNewsletter { descuento, correo }
    }    
}
/*
➢ Calcular el precio final de una venta en base a los productos que hay en ella. Para
calcularlo tenga en cuenta que pueden haber determinados productos de alguna
categoría donde debería aplicarse un descuento. Tanto la categoría como el
porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el
sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe
aplicar un porcentaje de descuento general si el cliente tiene suscripción al
newsletter.
*/ 
impl Categorias{
    pub fn get_porcentaje_de_descuento(&self,categoria:&String) -> Option<&f32>{
        self.categorias_con_sus_descuentos.get(&categoria.clone())
    }
}
impl<'a> Venta <'a> { // 
    pub fn calcular_precio_final(&self, categorias:Categorias) ->f32{
        let descuento_personal = if let Some(valor) = &self.cliente.subscripcion_newsletter{ valor.descuento} else {0.0};  

        self.productos_vendidos.iter()
        .map(|p| (p.precio_base *(100.0 - *categorias.get_porcentaje_de_descuento(&p.categoria).unwrap()) /100.0)) // obtengo el precio de los productos ya con descuento 
        .sum::<f32>() * ((100.0 - descuento_personal )/100.0) // sumo los valores y aplico el descuento si es que tiene newsletter
    }
}
/*
➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que
permita visualizar las ventas totales por categoría de producto y otro por vendedor. */
impl<'a> SistemaDeVentas<'a>{
    pub fn realizar_reporte(&self) -> Visualizacion{
        let mut res = Visualizacion::new();
        self.ventas.iter().for_each(|v| {
            res.agregar_venta_por_vendedor(v.vendedor, v);
            v.productos_vendidos.iter().for_each(|p| res.agregar_venta_por_categoria_producto(p.categoria.clone(), v)); // --------------------- Es asi??? :(
        });
        res
    }
}
pub struct Visualizacion <'a>{
    ventas_ordenadas_por_categoria_de_producto:HashMap</*categoria:*/String,Vec<&'a Venta<'a>>>, 
    ventas_ordenadas_por_vendedor:HashMap</*vendedor:*/&'a Vendedor,Vec<&'a Venta<'a>>>, 
}
impl <'a> Visualizacion <'a>{
    pub fn new() -> Visualizacion<'a>{
        Visualizacion { ventas_ordenadas_por_categoria_de_producto: HashMap::new(), ventas_ordenadas_por_vendedor: HashMap::new() }
    }
    pub fn agregar_venta_por_categoria_producto(&mut self, categoria:String,venta:&'a Venta){
        if self.ventas_ordenadas_por_categoria_de_producto.contains_key(&categoria){
            let ventas = self.ventas_ordenadas_por_categoria_de_producto.get_mut(&categoria).unwrap();
            ventas.push(venta);
        } else {
            let mut ventas = Vec::new();
            ventas.push(venta);
            self.ventas_ordenadas_por_categoria_de_producto.insert(categoria, ventas);
        }
    }
    pub fn agregar_venta_por_vendedor(&mut self, vendedor:&'a Vendedor,venta:&'a Venta){
        if self.ventas_ordenadas_por_vendedor.contains_key(vendedor){
            let ventas = self.ventas_ordenadas_por_vendedor.get_mut(vendedor).unwrap();
            ventas.push(venta);
        } else {
            let mut ventas = Vec::new();
            ventas.push(venta);
            self.ventas_ordenadas_por_vendedor.insert(vendedor, ventas);
        }
    }
}

/* 5- La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los
usuarios comprar y vender distintas criptomonedas. La plataforma permite el registro de
usuarios y la gestión de sus balances en distintas criptomonedas y en dinero fíat. De los
usuarios se conoce nombre, apellido, email, dni, y si está validada su identidad o no. Cada
usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma. De las
criptomonedas se conoce: nombre, prefijo y un listado de blockchains donde se pueden
enviar o recibir. De cada blockchain se conoce el nombre, prefijo.
*/ 
use std::hash::{Hash, Hasher};

pub struct Plataforma<'a>{
    criptomonedas:HashSet<Criptomoneda<'a>>,
    blockchains:HashSet<BlockChain>,
    datos_usuarios:HashMap<&'a User, DatosDeUsuarioEnLaPlataforma<'a>>,     
}

#[derive(Eq, PartialEq)]
pub struct User{ 
    nombre:String, apellido:String, email:String, dni:u32,
}
impl Hash for User{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dni.hash(state);
    }
}

#[derive(Eq, PartialEq,Clone)]
pub struct Criptomoneda <'a>{ 
    nombre:String, prefijo:String,
    blockchains_accedibles:HashSet<&'a BlockChain>
}
impl<'a> Hash for Criptomoneda<'a>{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nombre.hash(state);
        self.prefijo.hash(state);
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct BlockChain{ 
    nombre:String, prefijo:String
}

pub struct DatosDeUsuarioEnLaPlataforma<'a> { 
    usuario: &'a User,
    identidad_validada:bool,        
    monto_fiat:f32,
    balances:Balances<'a>,
    transacciones:Transacciones<'a>
} 
#[derive(Default)]
pub struct Balances<'a>{ 
    data:HashMap<&'a Criptomoneda<'a>,Balance<'a>>
}
#[derive(Clone)]
pub struct Balance<'a>{ 
    usuario: &'a User, criptomoneda: &'a Criptomoneda<'a>,
    monto:f32
}
impl<'a> Hash for Balance<'a>{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.usuario.hash(state);
        self.criptomoneda.hash(state);
    }
}
impl<'a> PartialEq for Balance<'a>{
    fn eq(&self, other: &Self) -> bool {
        self.usuario == other.usuario && self.criptomoneda == other.criptomoneda
    }
}
impl<'a> Eq for Balance<'a>{} 

pub struct Transacciones<'a>{ 
    data:Option<Vec<Transaccion<'a>>>
}
#[derive(Clone)]
pub enum Transaccion<'a>{
    IngresoDeDinero{fecha:Fecha,
                    usuario: &'a User,
                    monto_de_fiat:f32,
                },
    CompraDeCripto{fecha:Fecha, 
                    usuario: &'a User, 
                    criptomoneda:&'a Criptomoneda<'a>, 
                    monto_de_cripto:f32,
                    cotizacion_en_la_fecha_de_la_cripto:f32
                },
    VentaDeCripto{fecha:Fecha, 
                    usuario: &'a User, 
                    criptomoneda:&'a Criptomoneda<'a>, 
                    monto_de_cripto:f32,
                    cotizacion_en_la_fecha_de_la_cripto:f32
                },
    RetiroCripto{fecha:Fecha, 
                    usuario: &'a User,
                    blockchain: &'a BlockChain,
                    hash:String,
                    criptomoneda:&'a Criptomoneda<'a>, 
                    monto:f32,
                    cotizacion_en_la_fecha_de_la_cripto:f32
                },
    RecepcionCripto{fecha:Fecha, 
                    usuario: &'a User,
                    blockchain: &'a BlockChain,
                    hash:String,
                    criptomoneda:&'a Criptomoneda<'a>, 
                    monto:f32,
                    cotizacion_en_la_fecha_de_la_cripto:f32
                },
    RetiroFiat { fecha: Fecha,
                 usuario: &'a User, 
                 monto_de_fiat:f32, 
                 medio:Medio }
}
#[derive(Clone)]
pub enum Medio{
    MercadoPago, TransferenciaBancaria
}
// ------------------------------ CONSTRUCTORES -------------- 
impl<'a> Plataforma <'a>{
    pub fn new(users:&'a HashSet<&'a User>, criptomonedas:HashSet<Criptomoneda<'a>>,blockchains:HashSet<BlockChain>) -> Plataforma<'a>{
        let mut datos_usuarios = HashMap::new();
        users.iter().for_each(|&user|{datos_usuarios.insert(user, DatosDeUsuarioEnLaPlataforma::new(user));});
        let mut p = Plataforma {datos_usuarios, criptomonedas, blockchains};
        //p.add_criptos_all_balances();
        p
    }
    fn add_criptos_all_balances(&'a mut self){ // ------------------------------------------------------------------------------------------------ COMO SACO ESA 'a ESTA MALLLLLLLLLLL
        self.datos_usuarios.iter_mut().for_each(|datos_usuario|datos_usuario.1.balances.add_criptos(&self.criptomonedas, datos_usuario.0))
    }
}

impl<'a> DatosDeUsuarioEnLaPlataforma<'a>{
    pub fn new(usuario:&'a User) ->DatosDeUsuarioEnLaPlataforma<'a>{
        DatosDeUsuarioEnLaPlataforma { usuario, identidad_validada: false, monto_fiat: 0.0, balances: Balances::new() , transacciones: Transacciones::new() }
    }
}
impl <'a> Balances<'a>{
    pub fn new() -> Balances<'a>{
        Balances{data:HashMap::new()}
    }
    pub fn add_criptos(&mut self, criptomonedas:&'a HashSet<Criptomoneda<'a>>, usuario:&'a User){
        criptomonedas.iter().for_each(|c|{ self.data.insert(c, Balance::new(usuario, c, 0.0)); });        
    }
}
impl <'a> Balance <'a>{
    pub fn new(usuario:&'a User, criptomoneda:&'a Criptomoneda, monto:f32) -> Balance<'a>{
        Balance { usuario, criptomoneda, monto }
    }
}
impl <'a> Transacciones <'a>{
    pub fn new() -> Transacciones <'a>{
        Transacciones {data:None}
    }
}
// Plataforma, User, Criptomoneda, BlockChain, DatosDeUsuarioEnLaPlataforma, Balances, Balance, Transacciones
/* 
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones relacionadas al usuario:

Nota: Tanto para comprar. vender, retirar el usuario debe estar validado.
Se debe validar siempre que haya balance suficiente para realizar la operación en los casos
de compra, venta, retiro.

➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de
fiat de dicho usuario. Además se crea una transacción del hecho donde los datos
que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.
*/
impl<'a> Plataforma <'a>{
    pub fn ingresar_dinero(&mut self, user:&'a User, monto: f32, hoy:Fecha){
        if !self.datos_usuarios.contains_key(user) { panic!("USUARIO NO REGISTRADO") } 
        let datos_user = self.datos_usuarios.get_mut(user).unwrap();
        datos_user.ingresar_dinero(monto);
        datos_user.registrar_transaccion(Transaccion::IngresoDeDinero { fecha: hoy, usuario: user, monto_de_fiat: monto });

    }
}
impl <'a> DatosDeUsuarioEnLaPlataforma <'a>{
    pub fn ingresar_dinero(&mut self, monto:f32){
        self.monto_fiat+=monto;
    }
    pub fn registrar_transaccion(&mut self, transaccion: Transaccion<'a>){
        let new_data;
        if let Some(mut t) = self.transacciones.get_data() {
            t.push(transaccion);
            new_data=t;
        } else {
            new_data = vec![transaccion];
        }
        self.transacciones.set_data(Some(new_data));

    }
}
impl <'a> Transacciones <'a>{
    pub fn get_data(&self) -> Option<Vec<Transaccion<'a>>>{
        self.data.clone() // le hago clone ya que despues va a pasar a ser archivo
    }
    pub fn set_data(&mut self, new_data:Option<Vec<Transaccion<'a>>>) {
        self.data=new_data;
    }
}
/*
➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad
de determinada criptomoneda, tenga en cuenta que al momento de realizar la
operación se obtiene del sistema la cotización actual de la criptomoneda para
acreditar la correspondiente proporción en el balance de la cripto y desacreditar en
el balance de fiat. Luego de ello se registra la transacción con los siguientes datos:
fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.
*/
impl<'a> Plataforma<'a>{
    // retorna false si no tiene los fondos suficientes, true en caso de que la operacion se haya realizado con exito
    pub fn comprar_criptomoneda(&mut self, user:&'a User, hoy:Fecha, criptomoneda:&'a Criptomoneda, 
                                monto_de_fiat:f32, cotizacion_en_la_fecha_de_la_cripto:f32) -> bool
    {
        if !self.datos_usuarios.contains_key(user) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(user).unwrap();
        
        if !datos_user.tiene_al_menos(monto_de_fiat,None) {return false}
        // cotizacion_en_la_fecha_de_la_cripto (pesos) = 1 (cripto) => 
        // => monto_en_fiat (pesos) = monto_en_fiat (pesos) * 1(cripto) / cotizacion_en_la_fecha_de_la_cripto(pesos) = monto_en_cripto (cripto)
        let monto_de_cripto = monto_de_fiat / cotizacion_en_la_fecha_de_la_cripto; 

        datos_user.retirar_dinero(monto_de_fiat);
        datos_user.agregar_monto_cripto(criptomoneda,monto_de_cripto);
        datos_user.registrar_transaccion(Transaccion::CompraDeCripto { fecha: hoy, usuario: user, criptomoneda, monto_de_cripto, cotizacion_en_la_fecha_de_la_cripto });

        true
    }
}
impl <'a> DatosDeUsuarioEnLaPlataforma<'a>{
    pub fn tiene_al_menos(&self, monto:f32, criptomoneda:Option<&'a Criptomoneda>) -> bool{
        if let Some(cripto) = criptomoneda {
            if let Some(balance_de_cripto) = self.balances.get(cripto) { return !(balance_de_cripto.monto<monto) } 
            return false
        }
        return !(self.monto_fiat<monto)         
    }
    pub fn retirar_dinero(&mut self, monto:f32){
        if monto>self.monto_fiat {panic!("NO SE TIENEN LOS SUFICIENTES FONDOS FIAT")}
        self.monto_fiat-=monto;
    }
    pub fn agregar_monto_cripto(&mut self, cripto:&'a Criptomoneda, monto:f32){
        if let Some(_) = self.balances.get(cripto) {
            self.balances.sumar_monto_a(cripto,monto);
        } else { self.balances.insert(cripto, Balance::new(self.usuario,cripto,monto)); }
    }
}
impl <'a> Balances<'a>{
    pub fn get(&self, cripto: &'a Criptomoneda) -> Option<Balance>{
        if let Some(b) = self.data.get(cripto){
            return Some(b.clone()) // le hago clone ya que despues va a pasar a ser archivo
        }
        None
    }
    pub fn insert(&mut self, cripto:&'a Criptomoneda, balance:Balance<'a>) {
        self.data.insert(cripto, balance);
    }
    pub fn sumar_monto_a(&mut self, cripto:&'a Criptomoneda, monto:f32) {
        
        if let Some(b) = self.data.get_mut(cripto){
            b.sumar_monto(monto)
        }else{
            panic!("NO EXISTE EL BALANCE DE LA CRIPTOMONEDA {} DEL USUARIO SOLICITADO",cripto.nombre)
        }
    }
}
impl <'a> Balance <'a> {
    pub fn sumar_monto(&mut self, monto:f32){
        self.monto+=monto;
    }
}

/*
➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga
en cuenta que al momento de realizar la operación se obtiene del sistema la
cotización actual de la criptomoneda para acreditar la correspondiente proporción en
el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se
registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo:
venta de cripto,monto de cripto y cotización.
 */

impl<'a> Plataforma<'a>{
    // retorna false si no tiene los fondos suficientes, true en caso de que la operacion se haya realizado con exito
    pub fn vender_criptomoneda(&mut self, user:&'a User, hoy:Fecha, criptomoneda:&'a Criptomoneda, 
                                monto_de_cripto:f32, cotizacion_en_la_fecha_de_la_cripto:f32) -> bool
    {
        if !self.datos_usuarios.contains_key(user) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(user).unwrap();

        if !datos_user.tiene_al_menos(monto_de_cripto,Some(criptomoneda)) {return false}
        // 1 (cripto) = cotizacion_en_la_fecha_de_la_cripto (pesos) => 
        // => monto_en_cripto (pesos) = monto_en_cripto (pesos) * cotizacion_en_la_fecha_de_la_cripto(pesos) / 1(cripto) = monto_en_fiat (cripto)
        let monto_de_fiat = monto_de_cripto * cotizacion_en_la_fecha_de_la_cripto; 

        datos_user.quitar_monto_cripto(criptomoneda,monto_de_cripto);
        datos_user.ingresar_dinero(monto_de_fiat);
        datos_user.registrar_transaccion(Transaccion::VentaDeCripto { fecha: hoy, usuario: user, criptomoneda, monto_de_cripto, cotizacion_en_la_fecha_de_la_cripto });

        true
    }
}
impl <'a> DatosDeUsuarioEnLaPlataforma<'a>{
    pub fn quitar_monto_cripto(&mut self, cripto:&'a Criptomoneda, monto:f32){
        if let Some(_) = self.balances.get(cripto) {
            self.balances.restar_monto_a(cripto,monto);
        } else { panic!("NO SE TIENE REGISTRO DEL BALANCE DE LA CRIPTO {}",cripto.nombre) }
    }
}
impl<'a> Balances<'a>{
    pub fn restar_monto_a(&mut self, cripto:&'a Criptomoneda, monto:f32) {
        if let Some(b) = self.data.get_mut(cripto){
            b.restar_monto(monto)
        }else{
            panic!("NO EXISTE EL BALANCE DE LA CRIPTOMONEDA {} DEL USUARIO SOLICITADO",cripto.nombre)
        }
    }
}
impl <'a> Balance <'a> { 
    pub fn restar_monto(&mut self, monto:f32){
        if monto>self.monto {panic!("NO SE TIENEN LOS SUFICIENTES FONDOS DE LA CRIPTO {}",self.criptomoneda.nombre)}
        self.monto-=monto;
    }
}

 /*
➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
un hash que representa una transacción en ella (esto hágalo retornando el nombre
de la blockchain + un número random). Luego se genera una transacción con los
siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
cotización. // --------------------- No entendi la consigna pero OK
*/
impl<'a> Plataforma<'a> {
    pub fn retirar_criptomoneda_a_blockchain(&mut self, user:&'a User, hoy:Fecha, criptomoneda:&'a Criptomoneda, 
                                             monto_a_retirar_de_la_cripto:f32, blockchain:&'a BlockChain, cotizacion_en_la_fecha_de_la_cripto:f32) -> bool
        {
        if !self.datos_usuarios.contains_key(user) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(user).unwrap();

        if !datos_user.tiene_al_menos(monto_a_retirar_de_la_cripto,Some(criptomoneda)) {return false}

        datos_user.quitar_monto_cripto(criptomoneda,monto_a_retirar_de_la_cripto);
        datos_user.registrar_transaccion(Transaccion::RetiroCripto { fecha: hoy, usuario: user, blockchain, hash: blockchain.get_hash(), //---- WTF
                                            criptomoneda, monto: monto_a_retirar_de_la_cripto, cotizacion_en_la_fecha_de_la_cripto });

        true
    }
}
impl BlockChain{ //---- WTF
    pub fn get_hash(&self) -> String{
        self.nombre.clone() + " + un número random"
    }
}
/*
➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain
se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una 
transacción con los siguientes datos: fecha, usuario, tipo: recepción cripto,
blockchain, cripto, monto, cotización. // --------------------- No entendi la consigna pero OK x2
*/
impl<'a> Plataforma<'a> {
    pub fn recibir_criptomoneda_de_blockchain(&mut self, user:&'a User, hoy:Fecha, criptomoneda:&'a Criptomoneda, 
                                             monto:f32, blockchain:&'a BlockChain, cotizacion_en_la_fecha_de_la_cripto:f32) -> bool
        {
        if !self.datos_usuarios.contains_key(user) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(user).unwrap();

        //if !datos_user.tiene_al_menos(monto_a_retirar_de_la_cripto,Some(criptomoneda)) {return false} // <- esto no va. No entiendo de donde sale la plata

        datos_user.agregar_monto_cripto(criptomoneda,monto);
        datos_user.registrar_transaccion(Transaccion::RecepcionCripto { fecha: hoy, usuario: user, blockchain, hash: blockchain.get_hash(), //---- WTF
                                            criptomoneda, monto, cotizacion_en_la_fecha_de_la_cripto });

        true
    }
}
/*
➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho
monto del balance al usuario y se genera una transacción con la siguiente
información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago
o Transferencia Bancaria)
*/
impl<'a> Plataforma<'a> {
    pub fn retirar_fiat(&mut self, user:&'a User, hoy:Fecha, monto_de_fiat:f32, medio:Medio ) -> bool
        {
        if !self.datos_usuarios.contains_key(user) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(user).unwrap();

        if !datos_user.tiene_al_menos(monto_de_fiat,None) {return false} 

        datos_user.retirar_dinero(monto_de_fiat);
        datos_user.registrar_transaccion(Transaccion::RetiroFiat { fecha: hoy, usuario: user, monto_de_fiat, medio });

        true
    }
}
/*
Además la empresa desea saber lo siguiente en base a sus operaciones:
➢ Saber cual es la criptomoneda que más cantidad de ventas tiene // contar los enums
➢ Saber cual es la criptomoneda que más cantidad de compras tiene
➢ Saber cual es la criptomoneda que más volumen de ventas tiene
➢ Saber cual es la criptomoneda que más volumen de compras tiene
*/  
impl<'a> Plataforma<'a>{
    pub fn get_cripto_mayor_vendida(&self)->&'a Criptomoneda <'a>{
        let mut contador = CantidadYVolumenDeComprasPorCriptomoneda::new();
        contador.contar(&self);
        contador.cantidad_de_ventas_criptomonedas.iter()
        .max_by_key(|c|c.1).unwrap().0
    }
    pub fn get_cripto_mayor_comprada(&self)->&'a Criptomoneda <'a>{
        let mut contador = CantidadYVolumenDeComprasPorCriptomoneda::new();
        contador.contar(&self);
        contador.cantidad_de_compras_criptomonedas.iter()
        .max_by_key(|c|c.1).unwrap().0
    }
    pub fn get_cripto_con_mayor_volumen_de_ventas(&self)->&'a Criptomoneda <'a>{
        let mut contador = CantidadYVolumenDeComprasPorCriptomoneda::new();
        contador.contar(&self);
        contador.volumen_de_ventas_criptomonedas.iter()
        .max_by(|c1,c2|c1.1.partial_cmp(c2.1).unwrap()).unwrap().0
    }
    pub fn get_cripto_con_mayor__volumen_de_compras(&self)->&'a Criptomoneda <'a>{
        let mut contador = CantidadYVolumenDeComprasPorCriptomoneda::new();
        contador.contar(&self);
        contador.volumen_de_compras_criptomonedas.iter()
        .max_by(|c1,c2|c1.1.partial_cmp(c2.1).unwrap()).unwrap().0

    }
}
pub struct CantidadYVolumenDeComprasPorCriptomoneda<'a>{
    cantidad_de_compras_criptomonedas:HashMap<&'a Criptomoneda<'a>,u32>,
    cantidad_de_ventas_criptomonedas:HashMap<&'a Criptomoneda<'a>,u32>,
    volumen_de_compras_criptomonedas:HashMap<&'a Criptomoneda<'a>,f32>,
    volumen_de_ventas_criptomonedas:HashMap<&'a Criptomoneda<'a>,f32>,
}
impl <'a>CantidadYVolumenDeComprasPorCriptomoneda<'a>{
    pub fn new()-> CantidadYVolumenDeComprasPorCriptomoneda <'a>{
        CantidadYVolumenDeComprasPorCriptomoneda { cantidad_de_compras_criptomonedas: HashMap::new(), cantidad_de_ventas_criptomonedas: HashMap::new(), volumen_de_compras_criptomonedas: HashMap::new(), volumen_de_ventas_criptomonedas: HashMap::new() }
    }
    pub fn contar(&mut self,plataforma: &Plataforma<'a>){
        plataforma.datos_usuarios.iter()
        .for_each(|dato|{ 
            if let Some(transacciones) = dato.1.transacciones.get_data(){ 
                transacciones.iter().for_each(|transaccion|{
                    match transaccion {
                        Transaccion::CompraDeCripto { criptomoneda, monto_de_cripto, .. } => {
                            Self::agregar(criptomoneda,&mut self.cantidad_de_compras_criptomonedas,&mut self.volumen_de_compras_criptomonedas,*monto_de_cripto);
                        },
                        Transaccion::VentaDeCripto { criptomoneda, monto_de_cripto, ..} => {
                            Self::agregar(criptomoneda,&mut self.cantidad_de_ventas_criptomonedas,&mut self.volumen_de_ventas_criptomonedas,*monto_de_cripto);
                        },
                        _ => {}
                    }
                });
            }
        });
    }
    fn agregar (criptomoneda:&'a Criptomoneda, hash_map_cantidad:& mut HashMap<&'a Criptomoneda<'a>,u32>,hash_map_volumen:& mut HashMap<&'a Criptomoneda<'a>,f32>,monto_de_cripto:f32){
        if hash_map_cantidad.contains_key(criptomoneda) {*hash_map_cantidad.get_mut(criptomoneda).unwrap() += 1;} else {hash_map_cantidad.insert(criptomoneda, 1);}
        if hash_map_volumen.contains_key(criptomoneda) {*hash_map_volumen.get_mut(criptomoneda).unwrap() += monto_de_cripto;} else {hash_map_volumen.insert(criptomoneda, monto_de_cripto);}
        ///// -------------------------------- ESTO FUNCIONA??????????????
    }
}