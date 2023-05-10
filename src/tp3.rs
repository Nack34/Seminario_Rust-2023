/* 1- Escribir un programa que defina una estructura Persona que tenga campos para el
nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una
persona). Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
➢ imprimir: que imprime los datos de la persona sobre el mensaje ejecutado por ej:
person.imprimir() , donde person es una variable del tipo Persona.
➢ obtener_edad: retorna la edad de la persona.
➢ actualizar_direccion(nueva_direccion)
 */
pub struct Persona{
    nombre: String,
    direccion: Option<String>,
    edad: i32
}
impl Persona{
    pub fn new (nombre: String, direccion: Option<String>, edad: i32) -> Persona{
        Persona { nombre, direccion, edad }
    }
    pub fn imprimir(&self){
        print!("nombre: {}, direccion: ",self.nombre); 

        if let Some(n) = &self.direccion {
            print!("{}, ", n); 
        } else {print!("no ingresada, ")};

        println!("edad: {} ",self.edad); 
    }
    pub fn obtener_edad(&self) -> i32{
        self.edad
    }
    pub fn actualizar_direccion(&mut self, nueva_direccion:Option<String>){
        self.direccion = nueva_direccion;
    }
}

/* 2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la
longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
retorna.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
➢ es_cuadrado: retorna true si es cuadrado, false caso contrario
 */
pub struct Rectangulo{    
    longitud: f32,
    ancho: f32
}
impl Rectangulo{
    pub fn new (longitud: f32, ancho: f32) -> Rectangulo{
        Rectangulo { longitud, ancho }
    }
    pub fn calcular_area(&self) -> f32{
        self.ancho*self.longitud
    }
    pub fn calcular_perimetro(&self) -> f32{
        self.ancho*2.0+self.longitud*2.0
    }
    pub fn es_cuadrado(&mut self) -> bool{
        self.ancho==self.longitud
    }
}

/* 3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
mes y el año. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
➢ es_fecha_valida: retorna true si es una fecha valida, false caso contrario.//tenga en
cuenta los años bisiestos también.
➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
la fecha pasada por parámetro.. */
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
        self.anio>una_fecha.anio || self.mes>una_fecha.mes || self.dia>una_fecha.dia 
    }
    // metodos auxiliares para punto 10
    pub fn es_igual(&self, otro:&Self) -> bool{
        self.dia == otro.dia &&
        self.mes == otro.mes &&
        self.anio == otro.anio 
    }
    pub fn es_proxima(&self, otro:&Self) -> bool{
        self.anio - otro.anio > 0 && 
        self.mes - otro.mes > 0 &&
        self.dia - otro.dia > 7 
    }
}

/* 4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
isósceles o escaleno.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
 */
pub struct Triangulo{
    l1: f32,
    l2: f32,
    l3: f32,
}
impl Triangulo{
    pub fn new (l1: f32, l2: f32, l3: f32,) -> Triangulo{
        Triangulo { l1, l2, l3 }
    }
    pub fn determinar_tipo(&self) -> String{
        let mut lados_iguales = 0;
        
        if self.l1==self.l2 { lados_iguales+=1; } 
        if self.l2 == self.l3 { lados_iguales+=1; }
        if self.l1 == self.l3 { lados_iguales+=1; }
            
        match lados_iguales{
            3=>"equilátero".to_string(),
            1=>"isósceles".to_string(),
            0=>"escaleno".to_string(),
            _=>"ERROR".to_string() // tirar panic
        }
    }
    // h = (2 / a)​ * RAIZ(S(S−a)(S−b)(S−c)​), siendo S = (a+b+c)/2
    pub fn calcular_area(&self) -> f32{
        let h = self.altura();  // se toma a l1 como la base
        self.l1*h
    }
    pub fn semiperimetro (&self) -> f32{
        (self.l1+self.l2+self.l3)/2.0
    }
    pub fn altura(&self) -> f32 { // se toma a l1 como la base
        let s = self.semiperimetro();
        (2.0 / self.l1) * (s*(s-self.l1)*(s-self.l2)*(s-self.l3)).sqrt()
    }
    pub fn calcular_perimetro(&self) -> f32{
        self.l1+self.l2+self.l3
    }
}

/* 5- Escribir un programa que defina una estructura Producto que tenga campos para el
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
el precio bruto
➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
descuento sobre el precio bruto
➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
parámetros son opcionales.
 */
pub struct Producto{
    nombre: String,
    precio_bruto: f32,
    numero_identificatorio: i32
}
impl Producto{
    pub fn new (nombre: String, precio_bruto: f32, numero_identificatorio: i32) -> Producto{
        Producto { nombre, precio_bruto, numero_identificatorio }
    }
    pub fn calcular_impuestos(&self, porcentaje_de_impuestos:f32) -> f32{
        self.precio_bruto*porcentaje_de_impuestos/100.0
    }
    pub fn aplicar_descuento(&self, porcentaje_de_descuento:f32) -> f32{
        self.precio_bruto*porcentaje_de_descuento/100.0
    }
    pub fn calcular_precio_total(&self, porcentaje_de_impuestos:f32, porcentaje_de_descuento:f32) -> f32{ // entiendo por "opcional" que pasan 0
        self.precio_bruto - self.aplicar_descuento(porcentaje_de_descuento) + self.calcular_impuestos(porcentaje_de_impuestos)
    }
}

/* 6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
métodos:
❖ Examen:
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo
retorna.
❖ Estudiante:
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo
retorna.
➢ obtener_promedio: retorna el promedio de las notas.
➢ obtener_calificacion_mas_alta: retorna la nota más alta.
➢ obtener_calificacion_mas_baja: retorna la nota más baja.
Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen. */
use std::collections::LinkedList;
pub struct Examen{
    nombre_materia: String,
    nota: f32
}
impl Examen{
    pub fn new (nombre_materia: String, nota: f32) -> Examen{
        Examen { nombre_materia, nota }
    }
}
pub struct Estudiante{
    nombre: String,
    examenes: LinkedList<Examen>,
    nro_id: i32
}
impl Estudiante{
    pub fn new (nombre: String, examenes: LinkedList<Examen>, nro_id: i32) -> Estudiante{
        Estudiante { nombre, examenes, nro_id }
    }
    pub fn obtener_promedio(&self) -> f32{
        let l=self.examenes.len();
        if l > 0 {
            let mut sum_notas:f32 = 0.0;
            for examen in &self.examenes {
                sum_notas+=examen.nota;
            }
            sum_notas/(l as f32)
        } else {-1.0}
    }  
    pub fn obtener_calificacion_mas_alta(&self) -> f32{
        let mut max:f32=-1.0;
        for examen in &self.examenes{
            max = max.max(examen.nota);
        }
        max
    }
    pub fn obtener_calificacion_mas_baja(&self) -> f32{
        let mut min:f32=999.0;
        for examen in &self.examenes{
            min = min.min(examen.nota);
        }
        min
    }
}

/* 7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la
dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se
conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo,
verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos:

❖ ConcesionarioAuto:
➢ new: que pasando los parámetros correspondientes, crea un
ConcesionarioAuto y lo retorna.
➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
no lo agrega y retorna false.
➢ eliminar_auto(auto): elimina un auto de la lista de autos.
➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.

❖ Auto:
➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
retorna.
➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
■ si es de color primario le aplica un recargo del 25%, sino le aplica un
descuento del 10%.
■ si la marca es BMW le aplica un recargo del 15%-
■ si el año es menor a 2000 le aplica un descuento del 5%. */
pub struct ConcesionarioAuto{
    nombre: String,
    direccion: String,
    capacidad_maxima_autos:u32,
    autos:Vec<Auto>,  

}
impl ConcesionarioAuto{
    pub fn new (nombre: String, direccion: String, capacidad_maxima_autos:u32, autos:Vec<Auto>) -> ConcesionarioAuto{
        if autos.len()as u32>capacidad_maxima_autos {println!("autos.len()as u32>capacidad_maxima_autos");/* tirar panic */}
        ConcesionarioAuto { nombre, direccion, capacidad_maxima_autos, autos }
    }
    pub fn agregar_auto(&mut self,auto:Auto) -> bool{
        if self.autos.len() as u32>self.capacidad_maxima_autos {false}
        else{
            self.autos.push(auto);
            true
        }
    }
    pub fn eliminar_auto(&mut self){
        self.autos.pop();
    }
    pub fn buscar_auto(&self, auto:&Auto) -> Option<&Auto>{
        let mut auto_buscado:Option<&Auto>=None;
        for a in &self.autos {
            if auto.marca == a.marca && auto.modelo == a.modelo {
                auto_buscado = Some (&a);
                break;
            }
        }
        auto_buscado
    }
}
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Color{
    Rojo, Verde, Azul, Amarillo, Blanco, Negro
}
#[derive(Debug)]
pub struct Auto{ 
    marca: String,
    modelo: String,
    anio: i32,
    precio_bruto: f32,
    color: Color,
    
}
impl Auto{ 
    pub fn new (marca: String,  modelo: String, anio: i32, precio_bruto: f32, color: Color) -> Auto{
        Auto { marca,  modelo, anio, precio_bruto, color }
    }
    pub fn calcular_precio(&self) -> f32{
        self.precio_bruto
        + self.precio_bruto*25.0/100.0*self.es_de_color_primario()as i32 as f32 
        - self.precio_bruto*10.0/100.0*!self.es_de_color_primario()as i32 as f32 
        + self.precio_bruto*15.0/100.0*self.es_BMW()as i32 as f32 
        - self.precio_bruto*5.0/100.0*self.anio_menor_2000()as i32 as f32 
    }
    pub fn es_de_color_primario(&self) -> bool{
        self.color == Color::Rojo || 
        self.color == Color::Amarillo || 
        self.color == Color::Azul 
    }
    pub fn es_BMW(&self) -> bool{
        self.marca == "BMW".to_string()
    }
    pub fn anio_menor_2000(&self) -> bool{
        self.anio < 2000
    }
}

/* 8- Defina la estructura Cancion con campos para el título, el artista y el género. El género
puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
ella:
➔ agregar canción.
➔ eliminar canción.
➔ mover canción // mueve la canción a una determinada posición de la playlist.
➔ buscar canción por nombre.
➔ obtener las canciones de un determinado género.
➔ obtener las canciones de un determinado artista.
➔ modificar título de la playlist.
➔ eliminar todas las canciones. */
#[derive(PartialEq)]
pub enum Genero{
    Rock, Pop, Rap, Jazz, Otros
}
pub struct Cancion{
    titulo:String,
    artista:String,
    genero:Genero
}
impl Cancion {
    pub fn new(titulo:String, artista:String, genero:Genero) -> Cancion {
            Cancion {titulo, artista, genero}
    }
}
pub struct Playlist{
    nombre:String,
    canciones:Vec<Cancion>,
}
impl Playlist {
    pub fn new(nombre:String, canciones:Vec<Cancion>) -> Playlist {
            Playlist {nombre, canciones}
    }
    pub fn agregar_cancion (&mut self,cancion:Cancion){
        self.canciones.push(cancion);
    }
    pub fn eliminar_cancion (&mut self){
        self.canciones.pop();
    }
    pub fn mover_cancion (&mut self,index:usize,cancion:Cancion){
        self.canciones.insert(index, cancion);
    }
    pub fn buscar_cancion (&self,titulo:String) -> Option<&Cancion>{
        let mut cancion_buscada:Option<&Cancion> = None;
        for cancion in &self.canciones {
            if cancion.titulo == titulo {
                cancion_buscada = Some(&cancion)
            }
        }
        cancion_buscada
    }
    pub fn canciones_genero (&self,genero:Genero) -> Vec<&Cancion>{
        let mut canciones:Vec<&Cancion> = Vec::new();
        for cancion in &self.canciones {
            if cancion.genero == genero {
                canciones.push(&cancion);
            }
        }
        canciones
    }
    pub fn canciones_artista (&self,artista:String) -> Vec<&Cancion>{
        let mut canciones:Vec<&Cancion> = Vec::new();
        for cancion in &self.canciones {
            if cancion.artista == artista {
                canciones.push(&cancion);
            }
        }
        canciones
    }
    pub fn modificar_nombre (&mut self, nombre:String){
        self.nombre = nombre;
    }
    pub fn vaciar_playlist (&mut self){
        self.canciones = Vec::new();
    }
}

/* 9.-Dada una cadena de veterinarias se desea implementar un sistema de atención de
pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.
Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. Del
dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.
Dado todo lo mencionado anteriormente implemente los métodos para realizar las
siguientes acciones:
➔ crear una veterinaria.
➔ agregar una nueva mascota a la cola de atención de la veterinaria.
➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente
en atender porque tiene la máxima prioridad.
➔ atender la próxima mascota de la cola.
➔ eliminar una mascota específica de la cola de atención dado que se retira.
➔ registrar una atención.
➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el
teléfono.
➔ modificar el diagnóstico de una determinada atención.
➔ modificar la fecha de la próxima visita de una determinada atención.
➔ eliminar una determinada atención.
Nota: para la fecha utilice lo implementado en el punto 3.
 */
pub struct Duenio{
    nombre:String,
    direccion:String,
    teléfono:u32,
}
impl Duenio {
    pub fn new( nombre:String, direccion:String, teléfono:u32,) -> Duenio {
        Duenio { nombre, direccion, teléfono}
    }
}
#[derive(PartialEq)]
pub enum Animal {
    Perro, Gato, Caballo, Otros
}
pub struct Mascota{
    nombre:String,
    edad:u32,
    tipo_de_animal:Animal,
    duenio:Duenio,
}
impl Mascota {
    pub fn new( nombre:String, edad:u32, tipo_de_animal:Animal, duenio:Duenio) -> Mascota {
        Mascota {nombre, edad, tipo_de_animal, duenio}
    }
}
pub struct AtencionRealizada{
    mascota:Mascota,
    diagnostico_final:String,
    tratamiento:String,
    fecha_proxima_visita:Option<Fecha>,
}
impl AtencionRealizada {
    pub fn new( mascota:Mascota, diagnostico_final:String, tratamiento:String, fecha_proxima_visita:Option<Fecha>) -> AtencionRealizada {
        AtencionRealizada {mascota, diagnostico_final, tratamiento, fecha_proxima_visita}
    }
    pub fn soy_yo(&self, nombre_mascota:String,nombre_duenio:String,telefono:u32) -> bool{
        self.mascota.nombre == nombre_mascota && 
        self.mascota.duenio.nombre == nombre_duenio &&
        self.mascota.duenio.teléfono == telefono 
    }
    pub fn modificar_fecha(&mut self, fecha_proxima_visita:Option<Fecha>) {
        self.fecha_proxima_visita = fecha_proxima_visita;
    }
}
use std::collections::VecDeque;
pub struct Veterinaria{
    nombre:String,
    direccion:String,
    id:i32,
    proximas_atenciones:VecDeque<Mascota>,
    registro_atenciones:LinkedList<AtencionRealizada>,
}
impl Veterinaria {
    pub fn new( nombre:String, direccion:String, id:i32, proximas_atenciones:VecDeque<Mascota>, registro_atenciones:LinkedList<AtencionRealizada>) -> Veterinaria {
        Veterinaria {nombre, direccion, id, proximas_atenciones, registro_atenciones}
    }
    pub fn agregar_mascota_al_final(&mut self, mascota: Mascota) {
        self.proximas_atenciones.push_back(mascota);
    }
    pub fn agregar_mascota_al_inicio(&mut self, mascota: Mascota) {
        self.proximas_atenciones.push_front(mascota);
    }
    pub fn atender_mascota(&mut self) -> Mascota{
        let Some(mascota) = self.proximas_atenciones.pop_front() else { todo!() };
        mascota
    }
    pub fn eliminar_mascota(&mut self, mascota: Mascota)  {
        let mut index = 0;
        for m in &mut self.proximas_atenciones{
            if m.nombre == mascota.nombre && m.tipo_de_animal == mascota.tipo_de_animal{
                break;
            }
            index+=1;
        } 
        // if (index == self.proximas_atenciones.len() { panic })
        self.proximas_atenciones.remove(index);
    }
    pub fn mascota_atendida(&mut self, atencion_realizada: AtencionRealizada){
        self.registro_atenciones.push_front(atencion_realizada);
    }
    pub fn buscar_atencion(&mut self, nombre_mascota:String,nombre_duenio:String,telefono:u32) -> &AtencionRealizada {
        let mut res:Option<&AtencionRealizada> = None;
        for atencion in &self.registro_atenciones{
            if atencion.soy_yo(nombre_mascota.clone(),nombre_duenio.clone(),telefono){
                    res = Some(atencion);
                    break;
            }
        }
        let Some(r) = res else { todo!() };
        r
    }
    pub fn modificar_atencion(&mut self, atencionRealizada:AtencionRealizada) {
        for atencion in &mut self.registro_atenciones{
            if atencion.soy_yo(atencionRealizada.mascota.nombre.clone(),
                atencionRealizada.mascota.duenio.nombre.clone(),
                atencionRealizada.mascota.duenio.teléfono){
                    *atencion = atencionRealizada;
                    break;
            }
        }
    }
    pub fn eliminar_atencion(&mut self, atencionRealizada:AtencionRealizada) {
        let mut index = 0;
        for atencion in &mut self.registro_atenciones{
            if atencion.soy_yo(atencionRealizada.mascota.nombre.clone(),
                atencionRealizada.mascota.duenio.nombre.clone(),
                atencionRealizada.mascota.duenio.teléfono){
                    break;
            }
            index+=1;
        }

        // if (index == self.proximas_atenciones.len() { panic })

        let mut split_list = self.registro_atenciones.split_off(index);  // -------------------- IDK si funca, lo saque de internet xd
        split_list.pop_front();
        self.registro_atenciones.append(&mut split_list);
    }
}

/* 10-Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
cada libro se conoce el título, autor, número de páginas, género(novela, infantil, técnico,
otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de vencimiento del
préstamo, la fecha de devolución y el estado que puede ser devuelto o en préstamo. Del
cliente se conoce el nombre, teléfono y dirección de correo electrónico.

Implemente los métodos necesarios para realizar las siguientes acciones:
➔ obtener cantidad de copias: dado un determinado libro retorna el retorna la
cantidad de copias a disposición que hay para prestar de dicho libro.
➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
la cantidad de copias de libros a disposición para prestar.
➔ incrementar cantidad de copias a disposición: dado un libro incremente en 1
la cantidad de copias del libro a disposición para ser prestado.
➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
“en préstamo” de un determinado cliente.
➔ ver la cantidad disponible de un determinado libro: retorna la cantidad de
libros disponibles del registro de “copias a disposición” de un determinado
libro.
➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
para un determinado cliente cumpliendo con lo siguiente
◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
◆ haya al menos una copia disponible en el registro de copias a
disposición.
De ser así descuenta 1 en el registro de “copias a disposición” y
retorna true, si no cumple con alguna de las condiciones retorna false.
➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a
vencer el los próximos días, el valor de días es pasado por parámetro.
➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en
préstamos” donde la fecha de vencimiento es menor a la fecha actual.
➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
existe.
➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
estado “devuelto”, se registra la fecha de devolución y se incrementa la
cantidad de libros en 1 del libro devuelto en el registro de copias a
disposición.
Nota: para la fecha utilice lo implementado en el punto 3. */

pub struct Biblioteca<'a>{
    nombre:String,
    direccion:String, 
    libros_a_disposición:LibrosADisposicion,
    prestamos_efectuados:PrestamosEfectuados<'a>
}
pub struct LibrosADisposicion{
    libros_a_disposición:Vec<LibroADisposicion>
}
pub struct LibroADisposicion{
    libro: Libro,
    cant_ejemplares:u32
}
#[derive(PartialEq)]
pub enum GeneroDeLibro{ Novela, Infantil, Tecnico, Otros }
pub struct Libro{
    título: String, 
    autor: String, 
    nro_paginas: u32, 
    genero: GeneroDeLibro
}
pub struct PrestamosEfectuados<'a>{
    prestamos:Vec<Prestamo<'a>>
}
#[derive(PartialEq)]
pub enum Estado{Devuelto, EnPrestamo}
pub struct Prestamo<'a> {
    libro:&'a Libro,
    cliente: Cliente,
    fecha_vencimiento:Fecha,
    fecha_devolución:Option<Fecha>,
    estado:Estado
}
pub struct Cliente{
    nombre:String, 
    telefono: String,
    email:String
}

// ----------------------------- Constructores
impl <'a> Biblioteca<'a>{
    pub fn new ( nombre:String, direccion:String, libros_a_disposición:LibrosADisposicion, prestamos_efectuados:PrestamosEfectuados<'a>) -> Biblioteca{
        Biblioteca{ nombre, direccion, libros_a_disposición, prestamos_efectuados }
    }
}
impl LibrosADisposicion{
    pub fn new ( libros_a_disposición: Vec<LibroADisposicion>) -> LibrosADisposicion{
        LibrosADisposicion{ libros_a_disposición }
    }
}
impl LibroADisposicion{
    pub fn new ( libro:Libro, cant_ejemplares:u32) -> LibroADisposicion{
        LibroADisposicion{ libro, cant_ejemplares }
    }
}
impl Libro{
    pub fn new ( título:String, autor:String, nro_paginas:u32, genero:GeneroDeLibro) -> Libro{
        Libro{ título, autor, nro_paginas, genero }
    }
}
impl <'a>PrestamosEfectuados<'a>{
    pub fn new ( prestamos: Vec<Prestamo<'a>>) -> PrestamosEfectuados{
        PrestamosEfectuados{ prestamos }
    }
}
impl <'a>Prestamo<'a>{
    pub fn new ( libro:&'a Libro, cliente:Cliente, fecha_vencimiento:Fecha, fecha_devolución:Option<Fecha>,estado:Estado) -> Prestamo{
        Prestamo{ libro, cliente, fecha_vencimiento, fecha_devolución , estado}
    }
}
impl Cliente{
    pub fn new ( nombre:String, telefono:String, email:String) -> Cliente{
        Cliente{ nombre, telefono, email}
    }
}

// ----------------------------------- metodos pedidos por el enunciado
impl <'a> Biblioteca<'a>{
    pub fn buscar_libro_a_disposicion(&mut self, libro:&Libro) -> Option<&mut LibroADisposicion>{
        self.libros_a_disposición.buscar(libro)
    }
    pub fn cant_ejemplares (&mut self, libro:&Libro) -> u32{
        let res;
        if let Some(l) = self.buscar_libro_a_disposicion(libro){
            res = l.cant_ejemplares;
        } else {res = 0;}
        res
    }
    pub fn libro_prestado(&mut self, libro:&Libro){
        if let Some(l) = self.buscar_libro_a_disposicion(libro){
            l.cant_ejemplares -= 1;
        }
    }
    pub fn libro_devuelto(&mut self, libro:&Libro){
        if let Some(l) = self.buscar_libro_a_disposicion(libro){
            l.cant_ejemplares += 1;
        }
    }
    pub fn cant_prestamos_de(&self, cliente:&Cliente) -> u32{
        self.prestamos_efectuados.cant_prestamos_de(cliente)
    }
    pub fn cant_ejemplares_disponibles(&self, libro:&Libro){
        // no es lo mismo que cant_ejemplares?
    }
    pub fn realizar_prestamos(&mut self, libro:&'a Libro, cliente:Cliente, fecha_vencimiento:Fecha, fecha_devolución:Option<Fecha>) -> bool{
        let mut res = true;
        if self.cant_prestamos_de(&cliente)<=5 && self.cant_ejemplares(libro)>=1 {
            self.prestamos_efectuados.agregar(Prestamo::new(libro, cliente, fecha_vencimiento, fecha_devolución, Estado::EnPrestamo));
            self.libro_prestado(libro);
            res = true;
        }
        res
    }
    pub fn prestamos_a_vencer_en_los_proximos_dias(&self, hoy:Fecha) -> LinkedList<&Prestamo<'a>>{
        self.prestamos_efectuados.prestamos_a_vencer_en_los_proximos_dias(&hoy)
    }
    pub fn prestamos_a_vencidos(&self, hoy:Fecha) -> LinkedList<&Prestamo>{
        self.prestamos_efectuados.prestamos_a_vencidos(&hoy)
    }
    pub fn buscar_prestamo(&mut self, libro:&Libro, cliente:&Cliente) -> Option<&mut Prestamo<'a>>{
        self.prestamos_efectuados.buscar_prestamo(libro,cliente)
    }
    pub fn devolver_libro(&mut self, libro:&Libro, cliente:&Cliente, hoy:Fecha){
        let Some(p) = self.buscar_prestamo(libro, cliente) else {todo!()}; // panic
        p.registrar_devolucion(hoy);
        self.libro_devuelto(libro);
    }
}

// ------------------------------------------ metodos auxiliares
impl LibrosADisposicion{
    pub fn buscar (&mut self, libro:&Libro) -> Option<&mut LibroADisposicion>{
        let mut res = None;
        for l in &mut self.libros_a_disposición {
            if l.libro.es_igual(libro) {res = Some(l)}; 
        }
        res
    }
}
impl Libro{
    pub fn es_igual (&self, otro:&Self) -> bool{
        self.título==otro.título && self.autor==otro.autor && self.nro_paginas==otro.nro_paginas && self.genero == otro.genero
    }
}
impl <'a>PrestamosEfectuados<'a>{
    pub fn cant_prestamos_de(&self, cliente:&Cliente) -> u32{
        let mut res = 0;
        for p in &self.prestamos {
            if p.estado == Estado::EnPrestamo && p.cliente.es_igual(&cliente) {res += 1}; 
        }
        res
    }
    pub fn agregar(&mut self, prestamo:Prestamo<'a>){
        self.prestamos.push(prestamo);
    }
    pub fn prestamos_a_vencer_en_los_proximos_dias(&self, hoy:&Fecha) -> LinkedList<&Prestamo <'a>>{
        let mut res:LinkedList<&Prestamo> = LinkedList::new();
        for p in &self.prestamos {
            if p.esta_proximo_a_vencer(hoy) {res.push_front(p)}
        }
        res
    }
    pub fn prestamos_a_vencidos(&self, hoy:&Fecha) -> LinkedList<&Prestamo <'a>>{
        let mut res:LinkedList<&Prestamo> = LinkedList::new();
        for p in &self.prestamos {
            if p.estado == Estado::EnPrestamo && p.esta_vencido(hoy) {res.push_front(p)}
        }
        res
    }
    pub fn buscar_prestamo(&mut self, libro:&Libro, cliente:&Cliente) -> Option<&mut Prestamo<'a>>{
        let mut res = None;
        for p in &mut self.prestamos {
            if p.es_igual(libro,cliente){ res = Some(p)}
        }
        res
    }
}
impl <'a>Prestamo<'a>{
    /*pub fn es_igual (&self, otro:&Self) -> bool{
        self.libro.es_igual(otro.libro) && self.cliente.es_igual(&otro.cliente) && 
        self.fecha_vencimiento.es_igual(&otro.fecha_vencimiento) && 
        self.fecha_devolución.es_igual(&otro.fecha_devolución) && 
        self.estado == otro.estado
    }*/
    pub fn es_igual (&self, libro:&Libro, cliente:&Cliente) -> bool{
        self.libro.es_igual(libro) && self.cliente.es_igual(cliente) 
    }
    pub fn esta_proximo_a_vencer(&self, hoy:&Fecha) -> bool{
        self.fecha_vencimiento.es_proxima(hoy)
    }
    pub fn esta_vencido(&self, hoy:&Fecha) -> bool{
        hoy.es_mayor(&self.fecha_vencimiento)
    }
    pub fn registrar_devolucion(&mut self, hoy:Fecha){
        self.fecha_devolución = Some(hoy);
        self.estado = Estado::Devuelto;
    }
}
impl Cliente{
    pub fn es_igual (&self, otro:&Self) -> bool{
        self.nombre==otro.nombre && self.telefono==otro.telefono && self.email==otro.email
    }
}
