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
use crate::tp4::{Plataforma,User,Criptomoneda,BlockChain,DatosDeUsuarioEnLaPlataforma,Balance,Balances,Transaccion,Transacciones,Medio};
