/* 5- En base al ejercicio 3 del tp#4 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Todas las suscripciones deben almacenarse en un archivo en formato JSON,
implemente lo necesario para que toda la funcionalidad de las suscripciones se realice
guardando, leyendo o modificando archivos..No debe modificar los tests hechos en el punto
a. Si puede agregar más en caso de que haga métodos nuevos para cumplir con este
punto. Recuerde también que se debe seguir manteniendo un coverage de al menos 90%. */
/*
use std::{collections::{HashMap}}; 
use std::hash::{Hash, Hasher};
use crate::tp3::Fecha;
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
*/