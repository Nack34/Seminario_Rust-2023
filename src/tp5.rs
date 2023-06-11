/* 6- En base al ejercicio 5 del tp#4 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Todos los balances de los usuarios así como las transacciones deben persistir en
archivos en formato JSON. No debe modificar los tests hechos en el punto a. Si
puede agregar más en caso de que haga métodos nuevos para cumplir con este
punto . Recuerde también que se debe seguir manteniendo un coverage de al
menos 90% */ 


/* 5- La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los
usuarios comprar y vender distintas criptomonedas. La plataforma permite el registro de
usuarios y la gestión de sus balances en distintas criptomonedas y en dinero fíat. De los
usuarios se conoce nombre, apellido, email, dni, y si está validada su identidad o no. Cada
usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma. De las
criptomonedas se conoce: nombre, prefijo y un listado de blockchains donde se pueden
enviar o recibir. De cada blockchain se conoce el nombre, prefijo.
*/ 
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::HashMap;
use crate::tp3::Fecha;

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