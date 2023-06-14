use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::HashMap;
use serde::Serialize;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
/* 6- En base al ejercicio 5 del tp#4 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Todos los balances de los usuarios así como las transacciones deben persistir en
archivos en formato JSON. No debe modificar los tests hechos en el punto a. Si
puede agregar más en caso de que haga métodos nuevos para cumplir con este
punto . Recuerde también que se debe seguir manteniendo un coverage de al
menos 90% */ 


// Fecha ya esta hecho, usaria "use crate::tp3::Fecha;", pero es un entregable 
#[derive(Serialize,Deserialize)]
#[derive(Clone,PartialEq,Debug)]
pub struct Fecha{    
    dia: u32,
    mes: u32,
    anio: u32,
}
impl Fecha{
    pub fn new (dia: u32, mes: u32, anio: u32) -> Fecha{
        Fecha { dia, mes, anio }
    }
    pub fn es_bisiesto(&self) -> bool{
        self.anio % 4 == 0
    }
    pub fn es_dia_valido(&self, dia_max:u32) -> bool{
        self.dia>0 && self.dia<=dia_max
    }
    pub fn es_mes_valido(&self) -> bool{
        self.mes>0 && self.mes<=12
    }
    pub fn es_fecha_valida(&self) -> bool{
        self.es_mes_valido() && self.es_dia_valido(self.max_dia(self.mes))
    }
    pub fn max_dia(&self,mes:u32) -> u32{ // lo que entra TIENE que ser SI O SI un mes valido
        match mes{
            2=> {if self.es_bisiesto() {29} else {28}},
            1|3|5|7|8|10|12=>31,
            4|6|9|11=>30,
            _=>{println!("Mes Invalido"); 0 }, // tirar panic
        }   
    }
    pub fn sumar_dias(&mut self,dias:u32){
        for _i in 0..dias {
            self.dia+=1;
            self.chequear_carry();
        }
    }
    pub fn restar_dias(&mut self,dias:u32){
        for _i in 0..dias {
            self.dia-=1;
            self.chequear_borrow();
        }
    }
    pub fn chequear_carry(&mut self){
        if !self.es_fecha_valida() {
            self.mes+=1;
            if !self.es_mes_valido(){
                self.mes=1;
                self.anio+=1;
            }
            self.dia = 1;
        }
    }
    pub fn chequear_borrow(&mut self){
        if !self.es_fecha_valida() {
            self.mes-=1;
            if !self.es_mes_valido(){
                self.anio-=1;
                self.mes=12;
            }
            self.dia = self.max_dia(self.mes);
        }
    } 
    pub fn es_mayor(&self, una_fecha:&Self) -> bool{
        if self.anio!=una_fecha.anio {return self.anio>una_fecha.anio} 
        if self.mes!=una_fecha.mes {return self.mes>una_fecha.mes} 
        return self.dia>una_fecha.dia
    }
    pub fn es_igual(&self, otro:&Self) -> bool{
        self.dia == otro.dia &&
        self.mes == otro.mes &&
        self.anio == otro.anio 
    }
    pub fn es_proxima(&self, otro:&Self) -> bool{
        if self.anio!=otro.anio {return self.anio>otro.anio} 
        if self.mes!=otro.mes {return self.mes>otro.mes} 
        return self.dia >= otro.dia || otro.dia - self.dia <= 7
    }
    pub fn diferencencia_de_meses(&self, otro:&Self) -> u32{ // PRECONDICION --> otro > self
        otro.cantidad_de_meses() - self.cantidad_de_meses()
    }
    pub fn cantidad_de_meses(&self) -> u32{
        self.anio * 12 + self.mes
    }
}
#[test]
fn sumar_dias_cambio_de_anio_fecha_test(){
    let mut fecha = Fecha::new(25, 12, 2);
    fecha.sumar_dias(10);
    assert_eq!(fecha,Fecha::new(4, 1, 3))
}
#[test]
fn restar_dias_cambio_de_anio_fecha_test(){
    let mut fecha = Fecha::new(3, 1, 3);
    fecha.restar_dias(10);
    assert_eq!(fecha,Fecha::new(24, 12, 2))
}
#[test]
fn sumar_dias_cambio_de_mes_febrero_anio_bisiesto_fecha_test(){
    let mut fecha = Fecha::new(25, 2, 12);
    fecha.sumar_dias(10);
    assert_eq!(fecha,Fecha::new(6, 3, 12))
}
#[test]
fn restar_dias_cambio_de_mes_a_febrero_anio_no_bisiesto_fecha_test(){
    let mut fecha = Fecha::new(2, 3, 3);
    fecha.restar_dias(10);
    assert_eq!(fecha,Fecha::new(20, 2, 3))
}
#[test]
fn fecha_es_igual_test(){
    let fecha = Fecha::new(20, 2, 3);
    assert!(fecha.es_igual(&Fecha::new(20, 2, 3)));
    assert!(fecha.es_igual(&Fecha::new(20, 2, 3)))
}
#[test]
fn fecha_es_mayor_test(){
    let fecha = Fecha::new(20, 2, 3);
    assert!(!fecha.es_mayor(&Fecha::new(20, 2, 3000)));
    assert!(!fecha.es_mayor(&Fecha::new(20, 7, 3)));
    assert!(!fecha.es_mayor(&Fecha::new(20, 2, 3)));
    assert!(fecha.es_mayor(&Fecha::new(2, 2, 3)));
}
#[test]
fn fecha_es_proxima_test(){
    let fecha = Fecha::new(20, 2, 3);
    assert!(!fecha.es_proxima(&Fecha::new(20, 2, 3000)));
    assert!(!fecha.es_proxima(&Fecha::new(20, 7, 3)));
    assert!(fecha.es_proxima(&Fecha::new(21, 2, 3)));
}
#[test]
fn fecha_diferencencia_de_meses_test(){
    let fecha = Fecha::new(20,11, 3);
    assert_eq!(fecha.diferencencia_de_meses(&Fecha::new(20, 3, 4)),4);
}

/* Practica 4, punto 5- La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los
usuarios comprar y vender distintas criptomonedas. La plataforma permite el registro de
usuarios y la gestión de sus balances en distintas criptomonedas y en dinero fíat. De los
usuarios se conoce nombre, apellido, email, dni, y si está validada su identidad o no. Cada
usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma. De las
criptomonedas se conoce: nombre, prefijo y un listado de blockchains donde se pueden
enviar o recibir. De cada blockchain se conoce el nombre, prefijo.
*/ 

pub struct Plataforma{
    usuarios:HashMap<u32,User>,
    datos_usuarios:HashMap<u32, DatosDeUsuarioEnLaPlataforma>,     
    criptomonedas:HashMap<u32,Criptomoneda>,
    blockchains:HashMap<u32,BlockChain>,
}

#[derive(Eq, PartialEq)]
pub struct User{ 
    id:u32,
    nombre:String, apellido:String, email:String, dni:u32,
}

#[derive(Eq, PartialEq,Clone,Debug)]
pub struct Criptomoneda { 
    id:u32,
    nombre:String, prefijo:String,
    blockchains_accedibles:HashSet<u32>
}

#[derive(Eq, Hash, PartialEq)]
pub struct BlockChain{ 
    id:u32,
    nombre:String, prefijo:String
}

pub struct DatosDeUsuarioEnLaPlataforma { 
    usuario_id: u32,
    identidad_validada:bool,        
    monto_fiat:f32,
    balances:Balances,
    transacciones:Transacciones
} 
#[derive(Serialize,Deserialize)]
#[derive(Clone)]
pub struct Balance{ 
    usuario_id: u32, criptomoneda_id: u32,
    monto:f32
}

#[derive(Serialize,Deserialize)]
#[derive(Clone)]
pub enum Transaccion{
    IngresoDeDinero{fecha:Fecha,
                    usuario_id: u32,
                    monto_de_fiat:f32,
                },
    CompraDeCripto{fecha:Fecha, 
                    usuario_id: u32, 
                    criptomoneda_id: u32, 
                    monto_de_cripto:f32,
                    cotizacion_en_la_fecha_de_la_cripto:f32
                },
    VentaDeCripto{fecha:Fecha, 
                    usuario_id: u32, 
                    criptomoneda_id: u32, 
                    monto_de_cripto:f32,
                    cotizacion_en_la_fecha_de_la_cripto:f32
                },
    RetiroCripto{fecha:Fecha, 
                    usuario_id: u32,
                    blockchain_id: u32,
                    hash:String,
                    criptomoneda_id: u32, 
                    monto:f32,
                    cotizacion_en_la_fecha_de_la_cripto:f32
                },
    RecepcionCripto{fecha:Fecha, 
                    usuario_id: u32,
                    blockchain_id: u32,
                    hash:String,
                    criptomoneda_id: u32, 
                    monto:f32,
                    cotizacion_en_la_fecha_de_la_cripto:f32
                },
    RetiroFiat { fecha: Fecha,
                 usuario_id: u32, 
                 monto_de_fiat:f32, 
                 medio:Medio }
}
#[derive(Serialize,Deserialize)]
#[derive(Clone)]
pub enum Medio{
    MercadoPago, TransferenciaBancaria
}
// ------------------------------ CONSTRUCTORES -------------- 
impl User{
    pub fn new(id: u32, nombre: String, apellido: String, email: String, dni: u32 )->User{
        User { id, nombre, apellido, email, dni }
    }
}
impl Plataforma {
    pub fn new(usuarios:HashMap<u32,User>, blockchains:HashMap<u32,BlockChain>) -> Plataforma{
        let mut datos_usuarios = HashMap::new();
        usuarios.iter().for_each(|user|{datos_usuarios.insert(*user.0, DatosDeUsuarioEnLaPlataforma::new(*user.0));});
        Plataforma {usuarios,datos_usuarios, criptomonedas:HashMap::new(), blockchains}
    }
}
impl Criptomoneda{
    pub fn new(id: u32, nombre: String, prefijo: String, blockchains_accedibles: HashSet<u32> ) ->Criptomoneda{
        Criptomoneda { id, nombre, prefijo, blockchains_accedibles }
    }
}
impl BlockChain{
    pub fn new(id:u32, nombre: String, prefijo: String ) ->BlockChain{
        BlockChain { id, nombre, prefijo }
    }
}
impl DatosDeUsuarioEnLaPlataforma{
    pub fn new(usuario_id:u32) ->DatosDeUsuarioEnLaPlataforma{
        DatosDeUsuarioEnLaPlataforma { usuario_id, identidad_validada: false, monto_fiat: 0.0, balances: Balances::new() , transacciones: Transacciones::new() }
    }
}
impl Balance {
    pub fn new(usuario_id:u32, criptomoneda_id:u32, monto:f32) -> Balance{
        Balance { usuario_id, criptomoneda_id, monto }
    }
}
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
impl Plataforma {
    pub fn validar_usuario(&mut self, usuario_id: u32,){
        if !self.datos_usuarios.contains_key(&usuario_id) { panic!("USUARIO NO REGISTRADO") } 
        let datos_user = self.datos_usuarios.get_mut(&usuario_id).unwrap();
        datos_user.validar_usuario();
    }
    pub fn ingresar_dinero(&mut self, usuario_id: u32, monto: f32, hoy:Fecha){
        if !self.datos_usuarios.contains_key(&usuario_id) { panic!("USUARIO NO REGISTRADO") } 
        let datos_user = self.datos_usuarios.get_mut(&usuario_id).unwrap();
        datos_user.ingresar_dinero(monto);
        datos_user.registrar_transaccion(Transaccion::IngresoDeDinero { fecha: hoy, usuario_id, monto_de_fiat: monto });

    }
}
impl  DatosDeUsuarioEnLaPlataforma {
    pub fn validar_usuario(&mut self){
        self.identidad_validada=true;
    }
    pub fn ingresar_dinero(&mut self, monto:f32){
        self.monto_fiat+=monto;
    }
    pub fn registrar_transaccion(&mut self, transaccion: Transaccion){
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
impl Plataforma{
    pub fn insert_criptomoneda(&mut self, criptomoneda:Criptomoneda) {
        self.criptomonedas.insert(criptomoneda.id,criptomoneda);
    }
    pub fn get_blockchains(&self) ->  Vec<(&u32, &BlockChain)>{
        let mut aux = Vec::new();
        self.blockchains.iter().for_each(|b|aux.push(b));
        aux
    }
}
fn crear_plataforma_para_test ()->Plataforma{
    
    let mut users: HashMap<u32,User> = HashMap::new();
    let usuario1_test = User::new(1,"nombre".to_string(), "apellido".to_string(), "email".to_string(), 1);
    let usuario2_test = User::new(2,"nombre".to_string(), "apellido".to_string(), "email".to_string(), 2);
    users.insert(usuario1_test.id,usuario1_test); 
    users.insert(usuario2_test.id,usuario2_test);

    let mut blockchains = HashMap::new();
    blockchains.insert(1,BlockChain::new(1,"nombre1".to_string(), "prefijo1".to_string()));
    blockchains.insert(2,BlockChain::new(2,"nombre2".to_string(), "prefijo2".to_string()));

    let mut plataforma = Plataforma::new(users, blockchains);
    
    let blockchains = plataforma.get_blockchains(); 
    let mut blockchains_accedibles = HashSet::new();    
    blockchains_accedibles.insert(*blockchains[0].0);
    blockchains_accedibles.insert(*blockchains[1].0);
    plataforma.insert_criptomoneda(Criptomoneda::new(1,"nombre1".to_string(), "prefijo1".to_string(), blockchains_accedibles));
    
    let blockchains = plataforma.get_blockchains(); 
    let mut blockchains_accedibles = HashSet::new();    
    blockchains_accedibles.insert(*blockchains[0].0);
    blockchains_accedibles.insert(*blockchains[1].0);
    plataforma.insert_criptomoneda(Criptomoneda::new(2,"nombre1".to_string(), "prefijo1".to_string(), blockchains_accedibles));
    
    plataforma
}
#[test]
fn ingresar_dinero_a_usuario_y_registrar_dos_transaccion_test(){
    let mut plataforma = crear_plataforma_para_test();

    plataforma.ingresar_dinero(1, 10.0, Fecha::new(1, 1, 2023));
    plataforma.ingresar_dinero(1, 10.0, Fecha::new(1, 1, 2023));
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().monto_fiat,20.0)
}
/*
➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad
de determinada criptomoneda, tenga en cuenta que al momento de realizar la
operación se obtiene del sistema la cotización actual de la criptomoneda para
acreditar la correspondiente proporción en el balance de la cripto y desacreditar en
el balance de fiat. Luego de ello se registra la transacción con los siguientes datos:
fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.
*/
impl Plataforma{
    // retorna false si no tiene los fondos suficientes, true en caso de que la operacion se haya realizado con exito
    pub fn comprar_criptomoneda(&mut self, usuario_id:u32, hoy:Fecha, criptomoneda_id:u32, 
                                monto_de_fiat:f32, cotizacion_en_la_fecha_de_la_cripto:f32) -> bool
    {
        if !self.datos_usuarios.contains_key(&usuario_id) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(&usuario_id).unwrap();
        if !datos_user.esta_validado() {return false}
        if !datos_user.tiene_al_menos(monto_de_fiat,None) {return false}
        // cotizacion_en_la_fecha_de_la_cripto (pesos) = 1 (cripto) => 
        // => monto_en_fiat (pesos) = monto_en_fiat (pesos) * 1(cripto) / cotizacion_en_la_fecha_de_la_cripto(pesos) = monto_en_cripto (cripto)
        let monto_de_cripto = monto_de_fiat / cotizacion_en_la_fecha_de_la_cripto; 

        datos_user.retirar_dinero(monto_de_fiat);
        datos_user.agregar_monto_cripto(criptomoneda_id,monto_de_cripto);
        datos_user.registrar_transaccion(Transaccion::CompraDeCripto { fecha: hoy, usuario_id, criptomoneda_id, monto_de_cripto, cotizacion_en_la_fecha_de_la_cripto });

        true
    }
}
impl  DatosDeUsuarioEnLaPlataforma{
    pub fn esta_validado(&self) -> bool{
        self.identidad_validada
    }
    pub fn tiene_al_menos(&self, monto:f32, criptomoneda_id:Option<u32>) -> bool{
        if let Some(cripto) = criptomoneda_id {
            if let Some(balance_de_cripto) = self.balances.get(cripto) { return !(balance_de_cripto.monto<monto) } 
            return false
        }
        return !(self.monto_fiat<monto)         
    }
    pub fn retirar_dinero(&mut self, monto:f32){
        if monto>self.monto_fiat {panic!("NO SE TIENEN LOS SUFICIENTES FONDOS FIAT")}
        self.monto_fiat-=monto;
    }
    pub fn agregar_monto_cripto(&mut self, cripto_id:u32, monto:f32){
        if let Some(_) = self.balances.get(cripto_id) {
            self.balances.sumar_monto_a(cripto_id,monto);
        } else { self.balances.insert(cripto_id, Balance::new(self.usuario_id,cripto_id ,monto)); }
    }
}
impl  Balance  {
    pub fn sumar_monto(&mut self, monto:f32){
        self.monto+=monto;
    }
}
#[test]
fn transaccion_usuario_no_validado_test(){
    let mut plataforma = crear_plataforma_para_test();

    assert_eq!(plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 10.0, 1.0),false);
}
#[test]
fn comprar_criptomoneda_usuario_validado_sin_suficiente_fondo_fiat_test(){
    let mut plataforma = crear_plataforma_para_test();

    plataforma.validar_usuario(1);
    assert_eq!(plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 10.0, 1.0),false);
}
#[test]
fn comprar_criptomoneda_usuario_validado_con_suficiente_fondo_fiat_test(){
    let mut plataforma = crear_plataforma_para_test();
    
    plataforma.validar_usuario(1);
    plataforma.ingresar_dinero(1, 20.0, Fecha::new(1, 1, 2023));
    assert!(plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 10.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 5.0, 1.0));
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().monto_fiat,5.0);
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().balances.get(1).unwrap().monto,15.0);

}

/*
➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga
en cuenta que al momento de realizar la operación se obtiene del sistema la
cotización actual de la criptomoneda para acreditar la correspondiente proporción en
el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se
registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo:
venta de cripto,monto de cripto y cotización.
 */

impl Plataforma{
    // retorna false si no tiene los fondos suficientes, true en caso de que la operacion se haya realizado con exito
    pub fn vender_criptomoneda(&mut self, usuario_id:u32, hoy:Fecha, criptomoneda_id:u32, 
                                monto_de_cripto:f32, cotizacion_en_la_fecha_de_la_cripto:f32) -> bool
    {
        if !self.datos_usuarios.contains_key(&usuario_id) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(&usuario_id).unwrap();
        if !datos_user.esta_validado() {return false}
        if !datos_user.tiene_al_menos(monto_de_cripto,Some(criptomoneda_id)) {return false}
        // 1 (cripto) = cotizacion_en_la_fecha_de_la_cripto (pesos) => 
        // => monto_en_cripto (pesos) = monto_en_cripto (pesos) * cotizacion_en_la_fecha_de_la_cripto(pesos) / 1(cripto) = monto_en_fiat (cripto)
        let monto_de_fiat = monto_de_cripto * cotizacion_en_la_fecha_de_la_cripto; 

        datos_user.quitar_monto_cripto(criptomoneda_id,monto_de_cripto);
        datos_user.ingresar_dinero(monto_de_fiat);
        datos_user.registrar_transaccion(Transaccion::VentaDeCripto { fecha: hoy, usuario_id, criptomoneda_id, monto_de_cripto, cotizacion_en_la_fecha_de_la_cripto });

        true
    }
}
impl  DatosDeUsuarioEnLaPlataforma{
    pub fn quitar_monto_cripto(&mut self, cripto_id:u32, monto:f32){
        if let Some(_) = self.balances.get(cripto_id) {
            self.balances.restar_monto_a(cripto_id,monto);
        } else { panic!("NO SE TIENE REGISTRO DEL BALANCE DE LA CRIPTO {}",cripto_id) }
    }
}
impl  Balance  { 
    pub fn restar_monto(&mut self, monto:f32){
        if monto>self.monto {panic!("NO SE TIENEN LOS SUFICIENTES FONDOS DE LA CRIPTO {}",self.criptomoneda_id)}
        self.monto-=monto;
    }
}
#[test]
fn vender_criptomoneda_usuario_validado_sin_suficiente_fondo_cripto_test(){
    let mut plataforma = crear_plataforma_para_test();

    plataforma.validar_usuario(1);
    plataforma.ingresar_dinero(1, 20.0, Fecha::new(1, 1, 2023));
    plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 10.0, 1.0);
    
    assert_eq!(plataforma.vender_criptomoneda(1, Fecha::new(1, 1, 2), 1, 500.0, 1.0),false);
}
#[test]
fn vender_criptomoneda_usuario_validado_con_suficiente_fondo_cripto_test(){
    let mut plataforma = crear_plataforma_para_test();

    plataforma.validar_usuario(1);
    plataforma.ingresar_dinero(1, 20.0, Fecha::new(1, 1, 2023));
    plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 10.0, 1.0);
    assert!(plataforma.vender_criptomoneda(1, Fecha::new(1, 1, 2), 1, 5.0, 1.0));
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().monto_fiat,15.0);
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().balances.get(1).unwrap().monto,5.0);
}


 /*
➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
un hash que representa una transacción en ella (esto hágalo retornando el nombre
de la blockchain + un número random). Luego se genera una transacción con los
siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
cotización.
*/
impl Plataforma {
    pub fn retirar_criptomoneda_a_blockchain(&mut self, usuario_id:u32, hoy:Fecha, criptomoneda_id:u32, 
                                             monto_a_retirar_de_la_cripto:f32, blockchain_id:u32, cotizacion_en_la_fecha_de_la_cripto:f32) -> bool
        {
        if !self.datos_usuarios.contains_key(&usuario_id) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(&usuario_id).unwrap();
        if !datos_user.esta_validado() {return false}

        if !datos_user.tiene_al_menos(monto_a_retirar_de_la_cripto,Some(criptomoneda_id)) {return false}

        let hash = "a".to_string();//self.get_blockchains().iter().filter(|elem|*elem.0==blockchain_id).next().unwrap().1.get_hash(); ----------------------------------------------------------------------------------------- ARREGLAR

        datos_user.quitar_monto_cripto(criptomoneda_id,monto_a_retirar_de_la_cripto);
        datos_user.registrar_transaccion(Transaccion::RetiroCripto { fecha: hoy, usuario_id, blockchain_id, hash: hash,
                                            criptomoneda_id, monto: monto_a_retirar_de_la_cripto, cotizacion_en_la_fecha_de_la_cripto });

        true
    }
}
impl BlockChain{ 
    pub fn get_hash(&self) -> String{
        self.nombre.clone() + " + un número random"
    }
}
#[test]
fn retirar_criptomoneda_a_blockchain_usuario_validado_sin_suficiente_fondo_cripto_test(){
    let mut plataforma = crear_plataforma_para_test();

    plataforma.validar_usuario(1);
    plataforma.ingresar_dinero(1, 20.0, Fecha::new(1, 1, 2023));
    plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 10.0, 1.0);

    assert_eq!(plataforma.retirar_criptomoneda_a_blockchain(1, Fecha::new(1, 1, 2), 1, 500.0, 1, 1.0),false);
}
#[test]
fn retirar_criptomoneda_a_blockchain_usuario_validado_con_suficiente_fondo_cripto_test(){
    let mut plataforma = crear_plataforma_para_test();

    plataforma.validar_usuario(1);
    plataforma.ingresar_dinero(1, 20.0, Fecha::new(1, 1, 2023));
    plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 10.0, 1.0);

    assert!(plataforma.retirar_criptomoneda_a_blockchain(1, Fecha::new(1, 1, 2), 1, 5.0, 1, 1.0));
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().monto_fiat,10.0);
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().balances.get(1).unwrap().monto,5.0);
}

/*
➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain
se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una 
transacción con los siguientes datos: fecha, usuario, tipo: recepción cripto,
blockchain, cripto, monto, cotización.
*/
impl Plataforma {
    pub fn recibir_criptomoneda_de_blockchain(&mut self, usuario_id:u32, hoy:Fecha, criptomoneda_id:u32, 
                                             monto:f32, blockchain_id:u32, cotizacion_en_la_fecha_de_la_cripto:f32) -> bool
        {
        if !self.datos_usuarios.contains_key(&usuario_id) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(&usuario_id).unwrap();

        let hash = "a".to_string(); //self.get_blockchains().iter().filter(|elem|*elem.0==blockchain_id).next().unwrap().1.get_hash(); ----------------------------------------------------------------------------------------- ARREGLAR

        datos_user.agregar_monto_cripto(criptomoneda_id,monto);
        datos_user.registrar_transaccion(Transaccion::RecepcionCripto { fecha: hoy, usuario_id, blockchain_id, hash: hash, 
            criptomoneda_id, monto, cotizacion_en_la_fecha_de_la_cripto });

        true
    }
}
#[test]
fn recibir_criptomoneda_de_blockchain_test(){
    let mut plataforma = crear_plataforma_para_test();

    assert!(plataforma.recibir_criptomoneda_de_blockchain(1, Fecha::new(1, 1, 2), 1, 50.0, 1, 1.0));
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().monto_fiat,0.0);
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().balances.get(1).unwrap().monto,50.0);
}

/*
➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho
monto del balance al usuario y se genera una transacción con la siguiente
información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago
o Transferencia Bancaria)
*/
impl Plataforma {
    pub fn retirar_fiat(&mut self, usuario_id:u32, hoy:Fecha, monto_de_fiat:f32, medio:Medio ) -> bool
        {
        if !self.datos_usuarios.contains_key(&usuario_id) { panic!("USUARIO NO REGISTRADO")} 
        let datos_user = self.datos_usuarios.get_mut(&usuario_id).unwrap();
        if !datos_user.esta_validado() {return false}

        if !datos_user.tiene_al_menos(monto_de_fiat,None) {return false} 

        datos_user.retirar_dinero(monto_de_fiat);
        datos_user.registrar_transaccion(Transaccion::RetiroFiat { fecha: hoy, usuario_id, monto_de_fiat, medio });

        true
    }
}
#[test]
fn retirar_fiat_usuario_validado_sin_suficiente_fondo_cripto_test(){
    let mut plataforma = crear_plataforma_para_test();

    plataforma.validar_usuario(1);
    plataforma.ingresar_dinero(1, 20.0, Fecha::new(1, 1, 2023));
    plataforma.comprar_criptomoneda(1, Fecha::new(1, 1, 2), 1, 10.0, 1.0);

    assert_eq!(plataforma.retirar_fiat(1, Fecha::new(1, 1, 2), 20.0, Medio::MercadoPago),false);
}
#[test]
fn retirar_fiat_usuario_validado_con_suficiente_fondo_cripto_test(){
    let mut plataforma = crear_plataforma_para_test();

    plataforma.validar_usuario(1);
    plataforma.ingresar_dinero(1, 30.0, Fecha::new(1, 1, 2023));

    assert!(plataforma.retirar_fiat(1, Fecha::new(1, 1, 2), 20.0, Medio::MercadoPago));
    assert_eq!(plataforma.datos_usuarios.get(&1).unwrap().monto_fiat,10.0);
}

/*
Además la empresa desea saber lo siguiente en base a sus operaciones:
➢ Saber cual es la criptomoneda que más cantidad de ventas tiene // contar los enums
➢ Saber cual es la criptomoneda que más cantidad de compras tiene
➢ Saber cual es la criptomoneda que más volumen de ventas tiene
➢ Saber cual es la criptomoneda que más volumen de compras tiene
*/  
impl Plataforma{
    pub fn get_cripto_id_mayor_vendida(&self)->u32{
        let mut contador = CantidadYVolumenDeComprasPorCriptomoneda::new();
        contador.contar(&self);
        *contador.cantidad_de_ventas_criptomonedas.iter()
        .max_by_key(|c|c.1).unwrap().0
    }
    pub fn get_cripto_id_mayor_comprada(&self)->u32{
        let mut contador = CantidadYVolumenDeComprasPorCriptomoneda::new();
        contador.contar(&self);
        *contador.cantidad_de_compras_criptomonedas.iter()
        .max_by_key(|c|c.1).unwrap().0
    }
    pub fn get_cripto_id_con_mayor_volumen_de_ventas(&self)->u32{
        let mut contador = CantidadYVolumenDeComprasPorCriptomoneda::new();
        contador.contar(&self);
        *contador.volumen_de_ventas_criptomonedas.iter()
        .max_by(|c1,c2|c1.1.partial_cmp(c2.1).unwrap()).unwrap().0
    }
    pub fn get_cripto_id_con_mayor__volumen_de_compras(&self)->u32{
        let mut contador = CantidadYVolumenDeComprasPorCriptomoneda::new();
        contador.contar(&self);
        *contador.volumen_de_compras_criptomonedas.iter()
        .max_by(|c1,c2|c1.1.partial_cmp(c2.1).unwrap()).unwrap().0

    }
}
pub struct CantidadYVolumenDeComprasPorCriptomoneda{
    cantidad_de_compras_criptomonedas:HashMap<u32,u32>,
    cantidad_de_ventas_criptomonedas:HashMap<u32,u32>,
    volumen_de_compras_criptomonedas:HashMap<u32,f32>,
    volumen_de_ventas_criptomonedas:HashMap<u32,f32>,
}
impl CantidadYVolumenDeComprasPorCriptomoneda{
    pub fn new()-> CantidadYVolumenDeComprasPorCriptomoneda {
        CantidadYVolumenDeComprasPorCriptomoneda { cantidad_de_compras_criptomonedas: HashMap::new(), cantidad_de_ventas_criptomonedas: HashMap::new(), volumen_de_compras_criptomonedas: HashMap::new(), volumen_de_ventas_criptomonedas: HashMap::new() }
    }
    pub fn contar(&mut self,plataforma: &Plataforma){
        plataforma.datos_usuarios.iter()
        .for_each(|dato|{ 
            if let Some(transacciones) = dato.1.transacciones.get_data(){ 
                transacciones.iter().for_each(|transaccion|{
                    match transaccion {
                        Transaccion::CompraDeCripto { criptomoneda_id, monto_de_cripto, .. } => {
                            Self::agregar(*criptomoneda_id,&mut self.cantidad_de_compras_criptomonedas,&mut self.volumen_de_compras_criptomonedas,*monto_de_cripto);
                        },
                        Transaccion::VentaDeCripto { criptomoneda_id, monto_de_cripto, ..} => {
                            Self::agregar(*criptomoneda_id,&mut self.cantidad_de_ventas_criptomonedas,&mut self.volumen_de_ventas_criptomonedas,*monto_de_cripto);
                        },
                        _ => {}
                    }
                });
            }
        });
    }
    fn agregar (criptomoneda_id:u32, hash_map_cantidad:& mut HashMap<u32,u32>,hash_map_volumen:& mut HashMap<u32,f32>,monto_de_cripto:f32){
        if hash_map_cantidad.contains_key(&criptomoneda_id) {let cant =hash_map_cantidad.get(&criptomoneda_id).unwrap(); hash_map_cantidad.insert(criptomoneda_id, cant+1);} else {hash_map_cantidad.insert(criptomoneda_id, 1);}
        if hash_map_volumen.contains_key(&criptomoneda_id) {let cant =hash_map_volumen.get(&criptomoneda_id).unwrap(); hash_map_volumen.insert(criptomoneda_id, cant+monto_de_cripto);} else {hash_map_volumen.insert(criptomoneda_id, monto_de_cripto);}
        ///// ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- ESTO FUNCIONA??????????????
    }
}


fn cargar_plataforma_para_test(users:HashSet<u32>,plataforma:&mut Plataforma,
cripto_id1_test:u32, cripto_id2_test:u32,cripto_id3_test:u32,cripto_id4_test:u32){

    let usuario1_test = *users.iter().next().unwrap();
    let usuario2_test = *users.iter().next().unwrap();
    plataforma.validar_usuario(usuario1_test);
    plataforma.ingresar_dinero(usuario1_test, 5000.0, Fecha::new(1, 1, 2023));
    plataforma.validar_usuario(usuario2_test);
    plataforma.ingresar_dinero(usuario2_test, 5000.0, Fecha::new(1, 1, 2023));


    assert!(plataforma.comprar_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id1_test, 1.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id1_test, 1.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id1_test, 1.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id1_test, 1.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id1_test, 1.0, 1.0)); // mayor cant de compras

    assert!(plataforma.vender_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id1_test, 1.0, 1.0));
    assert!(plataforma.vender_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id1_test, 1.0, 1.0));

    //-----

    assert!(plataforma.comprar_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id2_test, 500.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id2_test, 500.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id2_test, 500.0, 1.0)); // mayor volumen de compras

    assert!(plataforma.vender_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id2_test, 1.0, 1.0));
    assert!(plataforma.vender_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id2_test, 1.0, 1.0));

    //-----

    assert!(plataforma.comprar_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id3_test, 5.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id3_test, 5.0, 1.0));

    assert!(plataforma.vender_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id3_test, 1.0, 1.0)); // mayor cant de ventas
    assert!(plataforma.vender_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id3_test, 1.0, 1.0)); 
    assert!(plataforma.vender_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id3_test, 1.0, 1.0)); 
    assert!(plataforma.vender_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id3_test, 1.0, 1.0)); 
    assert!(plataforma.vender_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id3_test, 1.0, 1.0)); 

    //-----

    assert!(plataforma.comprar_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id4_test, 55.0, 1.0));
    assert!(plataforma.comprar_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id4_test, 55.0, 1.0));

    assert!(plataforma.vender_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id4_test, 25.0, 1.0)); // mayor volumen de ventas
    assert!(plataforma.vender_criptomoneda(usuario1_test, Fecha::new(1, 1, 2), cripto_id4_test, 50.0, 1.0)); 
    assert!(plataforma.vender_criptomoneda(usuario2_test, Fecha::new(1, 1, 2), cripto_id4_test, 25.0, 1.0)); 
}
#[test]
fn get_cripto_mayor_vendida_test(){
    let mut users = HashSet::new();
    users.insert(1); 
    users.insert(2);
    users.insert(3);

    let mut plataforma = crear_plataforma_para_test();
    cargar_plataforma_para_test(users,&mut plataforma,1,2,3,4);
    
    assert_eq!(plataforma.get_cripto_id_mayor_vendida(),3);
}
#[test]
#[ignore]
fn get_cripto_mayor_comprada_test(){
    let mut users = HashSet::new();
    users.insert(1); 
    users.insert(2);
    users.insert(3);

    let mut plataforma = crear_plataforma_para_test();
    cargar_plataforma_para_test(users,&mut plataforma,1,2,3,4);
    
    assert_eq!(plataforma.get_cripto_id_mayor_vendida(),1);
}
#[test]
#[ignore]
fn get_cripto_con_mayor_volumen_de_ventas(){
    let mut users = HashSet::new();
    users.insert(1); 
    users.insert(2);
    users.insert(3);

    let mut plataforma = crear_plataforma_para_test();
    cargar_plataforma_para_test(users,&mut plataforma,1,2,3,4);
    assert_eq!(plataforma.get_cripto_id_mayor_vendida(),4);
}
#[test]
#[ignore]
fn get_cripto_con_mayor_volumen_de_compras(){
    let mut users = HashSet::new();
    users.insert(1); 
    users.insert(2);
    users.insert(3);

    let mut plataforma = crear_plataforma_para_test();
    cargar_plataforma_para_test(users,&mut plataforma,1,2,3,4);
    assert_eq!(plataforma.get_cripto_id_mayor_vendida(),2);
}



// ---------------------------------------------------------------- MODIFICACIONES PARA EL PUNTO B (al modificar estos modulos, los test no se modificaron):
// ----------------------- ANTES DE MODIFICAR:
/*
impl Hash for User{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dni.hash(state);
    }
}
impl Hash for Criptomoneda{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nombre.hash(state);
        self.prefijo.hash(state);
    }
}
#[derive(Serialize,Deserialize)]
pub struct Balances{ 
    data:HashMap<u32,Balance>
}
impl  Balances{
    pub fn new() -> Balances{
        Balances{data:HashMap::new()}
    }
}
impl  Balances{
    pub fn get(&self, cripto_id:u32) -> Option<Balance>{
        if let Some(b) = self.data.get(&cripto_id){
            return Some(b.clone()) 
        }
        None
    }
    pub fn insert(&mut self, cripto_id:u32, balance:Balance) {
        self.data.insert(cripto_id, balance);
    }
    pub fn sumar_monto_a(&mut self, cripto_id:u32, monto:f32) {
        
        if let Some(b) = self.data.get_mut(&cripto_id){
            b.sumar_monto(monto)
        }else{
            panic!("NO EXISTE EL BALANCE DE LA CRIPTOMONEDA {} DEL USUARIO SOLICITADO",cripto_id)
        }
    }
    pub fn restar_monto_a(&mut self, cripto_id:u32, monto:f32) {
        if let Some(b) = self.data.get_mut(&cripto_id){
            b.restar_monto(monto)
        }else{
            panic!("NO EXISTE EL BALANCE DE LA CRIPTOMONEDA {} DEL USUARIO SOLICITADO",cripto_id)
        }
    }
}
#[derive(Serialize,Deserialize)]
pub struct Transacciones{ 
    data:Option<Vec<Transaccion>>
}
impl  Transacciones {
    pub fn new() -> Transacciones {
        Transacciones {data:None}
    }
}
impl  Transacciones {
    pub fn get_data(&self) -> Option<Vec<Transaccion>>{
        self.data.clone() 
    }
    pub fn set_data(&mut self, new_data:Option<Vec<Transaccion>>) {
        self.data=new_data;
    }
}
*/

// ----------------------- LUEGO DE MODIFICAR:
pub trait ManejadorDeArchivosJson 
{
    fn reset_file(&self);

    /*fn leer_archivo <'de,T> (path:&str) -> String //REEMPLAZAR STRING POR T. Error que no entiendo -> `buf` does not live long enough. borrowed value does not live long enough
    where
        T:Deserialize<'de>,
    { 
        // Abre el archivo del path en modo lectura, retorna `io::Result<File>`
        let mut archivo :File = match File::open(path) {
            Err(e) => panic!("No se pudo abrir por: {}", e),
            Ok(archivo ) => {archivo }
        };
        // Lee el contenido en un string, retorna `io::Result<usize>`
        let mut buf = String::new();

        match archivo.read_to_string (&mut buf) {
            Err(e) => panic!("No se puede leer por: {}", e),
            Ok(_) => print!("contiene: \n{}", buf),
        };
        let data = match serde_json::from_str(&buf){
            Err(e) => panic!("No se pudo pasar a T por: {}", e),
            Ok(dato ) => {dato }
        };
        data
        
        // cuando termina el scope el archivo se cierra
    }  */
    fn escribir_archivo<T>(path:&str,data:T)
    where
        T:Serialize,
    {
        // Abre el archivo en modo solo escritura, retorna `io::Result<File>`
        let mut archivo = match File::create(path) {
            Err(e) => panic!("No se puede crear porque: {}", e),
            Ok(archivo) => archivo,
        };
        // Escribe un string al archivo, retorna `io::Result<()>`
        let data_serializado = match serde_json::to_string(&data){
            Err(e) => panic!("No se pudo pasar a Json por: {}", e),
            Ok(dato ) => {dato }
        };
        match archivo.write_all(&data_serializado.as_bytes()) {
            Err(e) => panic!("No puede escrinbir porque: {}", e),
            Ok(_) => println!("Escribió correctamente en: {}" ,path),
        }
        // cuando termina el scope el archivo se cierra
    }  
       
}
impl ManejadorDeArchivosJson for Balances{
    fn reset_file(&self){
        Self::escribir_archivo(&self.path, HashMap::<u32, Balance>::new());
    }
}
impl ManejadorDeArchivosJson for Transacciones{
    fn reset_file(&self){
        Self::escribir_archivo(&self.path, None::<Option<Vec<Transaccion>>>);
    }
}
#[derive(Serialize,Deserialize)]
pub struct Balances{ 
    path:String
}
impl  Balances{
    pub fn new() -> Balances{
        let balances = Balances{path:"src/balances.json".to_string()};
        balances.reset_file();
        balances
    }
}
impl Balances{
    pub fn cargar_data(data:&mut HashMap<u32, Balance>, new_data:HashMap<u32, Balance>){

        let path = &"src/balances.json".to_string();
        let mut archivo :File = match File::open(path) {
            Err(e) => panic!("No se pudo abrir por: {}", e),
            Ok(archivo ) => {archivo }
        };
        let mut buf = String::new();

        match archivo.read_to_string (&mut buf) {
            Err(e) => panic!("No se puede leer por: {}", e),
            Ok(_) => print!("contiene: \n{}", buf),
        };
        let new_data:HashMap<u32, Balance> = match serde_json::from_str(&buf) {
            Err(e) => panic!("No se pudo pasar a HasMap por: {}", e),
            Ok(dato ) => {dato }
        };

        // cuando leer_archivo funcione borrar todo lo anterior
        data.clear();
        for d in new_data {
            data.insert(d.0,d.1);
        }
    }
    pub fn get(&self, cripto_id:u32) -> Option<Balance>{ 
        let mut data:HashMap<u32, Balance> = HashMap::new();
        Self::cargar_data(&mut data, HashMap::new());//Self::leer_archivo::<HashMap<u32, Balance>>(&self.path));
        
        if let Some(b) = data.get(&cripto_id){
            return Some(b.clone()) 
        }
        None
    }
    pub fn insert(&mut self, cripto_id:u32, balance:Balance) {
        let mut data:HashMap<u32, Balance> = HashMap::new();
        Self::cargar_data(&mut data, HashMap::new());//Self::leer_archivo::<HashMap<u32, Balance>>(&self.path));

        data.insert(cripto_id, balance);

        Self::escribir_archivo(&self.path, data);
    }
    pub fn sumar_monto_a(&mut self, cripto_id:u32, monto:f32) {
        let mut data:HashMap<u32, Balance> = HashMap::new();
        Self::cargar_data(&mut data, HashMap::new());//Self::leer_archivo::<HashMap<u32, Balance>>(&self.path));

        if let Some(b) = data.get_mut(&cripto_id){
            b.sumar_monto(monto)
        }else{
            panic!("NO EXISTE EL BALANCE DE LA CRIPTOMONEDA {} DEL USUARIO SOLICITADO",cripto_id)
        }

        Self::escribir_archivo(&self.path, data);
    }
    pub fn restar_monto_a(&mut self, cripto_id:u32, monto:f32) {
        let mut data:HashMap<u32, Balance> = HashMap::new();
        Self::cargar_data(&mut data, HashMap::new());//Self::leer_archivo::<HashMap<u32, Balance>>(&self.path));

        if let Some(b) = data.get_mut(&cripto_id){
            b.restar_monto(monto)
        }else{
            panic!("NO EXISTE EL BALANCE DE LA CRIPTOMONEDA {} DEL USUARIO SOLICITADO",cripto_id)
        }

        Self::escribir_archivo(&self.path, data);
    }
}
#[derive(Serialize,Deserialize)]
pub struct Transacciones{ 
    path:String
}
impl Transacciones {
    pub fn new() -> Transacciones {
        let transacciones = Transacciones {path:"src/transacciones.json".to_string()};
        transacciones.reset_file();
        transacciones
    }
}
impl  Transacciones {
    pub fn get_data(&self) -> Option<Vec<Transaccion>>{
        let path = &self.path;
        let mut archivo = match File::open(path) {
            Err(e) => panic!("No se pudo abrir por: {}", e),
            Ok(archivo ) => {archivo }
        };
        let mut buf = String::new();
        match archivo.read_to_string (&mut buf) {
            Err(e) => panic!("No se puede leer por: {}", e),
            Ok(_) => print!("contiene: \n{}", buf),
        };
        let data = match serde_json::from_str(&buf){
            Err(e) => panic!("No se pudo pasar a Option de Vec por: {}", e),
            Ok(dato ) => {dato }
        };
        data

        //cuando leer_archivo funcione borrar todo lo anterior
        //Self::leer_archivo::<Option<Vec<Transaccion>>>(&self.path);

    }
    pub fn set_data(&mut self, new_data:Option<Vec<Transaccion>>) {
        Self::escribir_archivo(&self.path, new_data);
    }
}