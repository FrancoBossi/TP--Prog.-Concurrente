# Trabajo Practico - Alquiler de bicicletas

## Estado del documento

Este documento describe una version preliminar del diseno de la solucion. La intencion es validar la arquitectura general, las decisiones principales y el uso de herramientas de concurrencia distribuida antes de avanzar con el informe final, los diagramas completos y la implementacion.

## Descripcion general

El sistema modela una red de estaciones autonomas para alquiler y devolucion de bicicletas. Cada estacion se ejecuta como un proceso independiente, mantiene estado local propio y puede seguir operando aun cuando pierda conectividad con el resto del sistema.

La aplicacion de usuario tambien se modela como un proceso separado. Su funcion es consultar estaciones cercanas, solicitar el retiro de una bicicleta y realizar devoluciones. Para retirar o devolver una bicicleta debe existir comunicacion local entre la app y la estacion. Esa comunicacion local se modelara con sockets TCP, como abstraccion de una conexion de cercania fisica.

El sistema no confia en la app del usuario como fuente de verdad. La app puede pedir una operacion, pero quien registra el inicio y el fin de un viaje es la estacion. Esto evita depender de que el usuario informe honestamente horarios, devoluciones o bicicletas no devueltas.

## Zonas geograficas

La ciudad se divide logicamente en zonas geograficas. Cada estacion tiene una ubicacion, por ejemplo coordenadas `(x, y)`, y pertenece a una zona configurada. La division en zonas permite reducir la cantidad de mensajes distribuidos: una estacion no necesita informar cada cambio a toda la ciudad, sino principalmente a las estaciones o coordinadores cercanos.

Cuando un usuario consulta disponibilidad, la app busca estaciones dentro de un radio de distancia. Si no encuentra ninguna, el radio puede ampliarse. Esta decision evita definir una cantidad fija de estaciones cercanas y refleja mejor la consigna: la cercania depende de la ubicacion del usuario.

Una estacion puede aparecer como cercana aunque este desconectada del resto de la red. En ese caso la app puede mostrar su ubicacion y, si tiene informacion previa, su ultimo estado conocido, aclarando que podria estar desactualizado.

## Descubrimiento de estaciones

El descubrimiento de estaciones no es el foco principal del trabajo. Por eso se propone resolverlo mediante configuracion inicial.

Cada proceso de aplicacion y estacion leera un archivo de configuracion con:

- identificador de cada estacion;
- direccion IP y puerto;
- ubicacion `(x, y)`;
- zona a la que pertenece;
- capacidad maxima;
- vecinos o estaciones conocidas.

La cantidad total de estaciones existentes sera constante y estara definida por configuracion. Durante la ejecucion se podran levantar o matar procesos de estaciones, pero no se requiere crear estaciones nuevas dinamicamente.

## Arquitectura propuesta

La solucion se compone de los siguientes procesos:

- **Aplicacion de usuario**: consulta estaciones cercanas, solicita retiros y devoluciones.
- **Estacion**: administra bicicletas, capacidad, viajes, estado local, comunicacion con usuarios y comunicacion con otras estaciones.
- **Lider de zona**: rol asumido por una estacion de la zona. Coordina informacion regional y operaciones distribuidas.
- **Banco o pasarela de pagos simulada**: proceso simple que recibe operaciones de cobro y las registra.

No se propone un servidor central unico para todo el sistema. En su lugar, cada zona tiene un lider elegido entre sus estaciones. Si el lider deja de responder, las estaciones de la zona ejecutan un algoritmo de eleccion para elegir un nuevo lider. Esto evita que un unico punto central deje inutilizable a todo el sistema.

## Modelo de estaciones

Cada estacion mantiene la autoridad inmediata sobre las bicicletas fisicamente presentes en ella. Si una bicicleta esta en una estacion, esa estacion sabe si esta disponible o no. Si un usuario retira una bicicleta, la estacion actualiza su estado local en el momento y no vuelve a asignar esa misma bicicleta.

No se modelara cada slot como una aplicacion o proceso independiente. Para este trabajo alcanza con que la estacion conozca:

- su capacidad maxima;
- cuantas bicicletas tiene disponibles;
- cuantos lugares libres tiene;
- que identificadores de bicicletas estan presentes;
- que viajes fueron iniciados desde ella;
- que operaciones quedaron pendientes de sincronizacion.

El concepto de slot puede existir como parte del estado interno de la estacion, pero no como una entidad distribuida separada. El foco del trabajo esta en la coordinacion entre procesos, no en modelar hardware de bajo nivel.

## Estados principales

Las bicicletas podran estar en alguno de estos estados logicos:

- **DisponibleEnEstacion**: la bicicleta esta fisicamente en una estacion y puede ser retirada.
- **EnViaje**: la bicicleta fue retirada por un usuario y todavia no fue devuelta.
- **DevueltaPendienteDeSync**: la bicicleta fue devuelta en una estacion que no pudo sincronizar inmediatamente.
- **PosiblementePerdida**: la bicicleta esta en viaje hace mas tiempo que el limite esperado o no fue devuelta.

Las estaciones podran tener operaciones pendientes cuando no tengan conectividad:

- retiros realizados sin informar todavia al lider;
- devoluciones realizadas sin informar todavia al lider;
- cobros pendientes de enviar al banco;
- actualizaciones de disponibilidad pendientes.

Para tolerar reinicios, cada estacion persistira su estado local en archivos simples. Como minimo deberia guardar bicicletas presentes, viajes activos, operaciones pendientes y pagos pendientes. De esta forma, si se mata y vuelve a levantar el proceso de una estacion, puede recuperar su ultimo estado conocido.

## Retiro de bicicletas

Para retirar una bicicleta, la app del usuario se comunica localmente con una estacion cercana y solicita un retiro.

La estacion valida:

- que el usuario este dentro del radio permitido;
- que exista al menos una bicicleta disponible;
- que la bicicleta no este ya asignada a otro viaje;
- que la estacion pueda registrar localmente la operacion.

Si la estacion esta conectada con el lider de zona, informa el retiro y actualiza la vista regional. Si no tiene conectividad, igualmente puede liberar la bicicleta porque la disponibilidad fisica es controlada localmente. En ese caso registra el retiro como operacion pendiente y lo sincroniza cuando vuelva la conexion.

El monto de seguridad o preautorizacion tambien puede quedar pendiente si no hay conectividad con el banco. Cuando se restablece la conexion, la estacion envia la operacion pendiente a la pasarela de pagos.

## Devolucion de bicicletas

Para devolver una bicicleta, el usuario se comunica localmente con una estacion y solicita la devolucion. La estacion valida que tenga capacidad disponible. Si puede aceptar la bicicleta, registra la devolucion, incrementa su disponibilidad local y cierra el viaje.

Si la estacion tiene conectividad, informa la devolucion al lider de zona y envia el cobro al banco. Si no tiene conectividad, guarda la devolucion y el cobro como operaciones pendientes.

El cobro final puede hacerlo la estacion de origen, la estacion de destino o el componente que reciba la sincronizacion correspondiente. La decision de diseno propuesta es que la estacion donde se devuelve cierre el viaje si tiene informacion suficiente; si falta informacion, deja el cierre pendiente hasta sincronizar con la zona.

## Bicicletas en viaje, perdidas o robadas

La comunicacion entre estaciones se justifica especialmente para seguir bicicletas que estan en viaje. Si una bicicleta fue retirada de una estacion y nunca vuelve a aparecer como devuelta, el sistema debe poder detectarlo.

Para eso, cada retiro genera un viaje activo con:

- identificador de bicicleta;
- estacion de origen;
- usuario;
- horario registrado por la estacion;
- estado del cobro;
- estado de sincronizacion.

Cuando una bicicleta se devuelve en otra estacion, esa estacion informa la devolucion para cerrar el viaje. Si el viaje permanece abierto mas tiempo que un umbral definido, la bicicleta puede marcarse como posiblemente perdida. No se modelara una logica compleja de robo, pero si se contemplara el estado de bicicleta no devuelta.

## Banco y pagos

El banco o pasarela de pagos sera un proceso independiente y simple. No se modelaran saldos, fondos insuficientes ni rechazos de pago, porque no forman parte del foco del trabajo.

El comportamiento esperado es:

- la estacion envia una operacion de preautorizacion o cobro;
- el banco la registra;
- la estacion guarda localmente si el cobro fue informado;
- si no hay conexion, el cobro queda pendiente y se reintenta luego.

En esta primera version asumimos que los pagos no son rechazados. El objetivo es representar la comunicacion distribuida con un servicio externo, no implementar un sistema bancario real.

## Herramientas de concurrencia distribuida

La solucion usara al menos dos herramientas de concurrencia distribuida vistas en la materia.

### Eleccion de lider

Cada zona tendra un lider. El lider sera una estacion de la zona y se encargara de coordinar informacion regional. Como el conjunto de estaciones esta definido por configuracion, cada estacion conoce a las demas estaciones de su zona.

Se propone usar el algoritmo Bully:

- si una estacion detecta que el lider no responde, inicia una eleccion;
- envia mensajes `ELECTION` a estaciones con identificador mayor;
- si nadie responde, se declara lider;
- si responde una estacion con identificador mayor, esa estacion continua la eleccion;
- el nuevo lider anuncia `COORDINATOR` al resto de la zona.

Esta eleccion permite que la zona siga funcionando aunque el lider anterior haya caido.

### Exclusion mutua distribuida centralizada

El lider de zona actuara como coordinador de exclusion mutua distribuida para operaciones criticas de la zona.

La exclusion mutua distribuida no se usa para decidir si una bicicleta fisica esta o no en una estacion: eso lo sabe la estacion local. Se usa para coordinar informacion compartida entre estaciones, por ejemplo:

- actualizar la vista regional de disponibilidad;
- registrar o cerrar viajes que involucran mas de una estacion;
- sincronizar operaciones pendientes;
- evitar que dos estaciones modifiquen al mismo tiempo el estado regional de una misma bicicleta en transito.

Si una estacion no puede contactar al lider, prioriza la operacion local y registra el evento como pendiente. Cuando vuelve la conectividad, solicita sincronizacion. Esto implica consistencia eventual: el estado local de la estacion es inmediato, pero la vista regional puede tardar en converger.

## Modelo de actores

Al menos la estacion se implementara usando modelo de actores, porque es la entidad con mayor concurrencia interna. Una estacion recibe eventos de usuarios, de otras estaciones, del lider, del banco y de su propio almacenamiento local.

Actores internos propuestos:

- **StationActor**: coordina el estado general de la estacion.
- **InventoryActor**: administra bicicletas disponibles, capacidad y lugares libres.
- **NetworkActor**: maneja conexiones TCP/UDP, mensajes entrantes y salientes.
- **ElectionActor**: detecta lider caido y participa en elecciones.
- **MutexActor**: solicita y libera permisos de exclusion mutua distribuida.
- **PaymentActor**: administra cobros enviados y pendientes.
- **PersistenceActor**: guarda y recupera estado local desde archivos.

La comunicacion entre actores sera mediante pasaje de mensajes. Esto evita compartir directamente estado mutable entre threads y permite razonar mejor sobre la concurrencia interna de la estacion.

## Comunicacion entre procesos

La comunicacion principal se realizara con sockets de la biblioteca estandar de Rust.

- **TCP** para operaciones que requieren respuesta confiable: retiro, devolucion, sincronizacion con lider, cobros con banco.
- **UDP o TCP corto** para heartbeats o mensajes livianos de disponibilidad, segun se defina durante la implementacion.

Los mensajes se representaran con un formato de texto simple, sin crates externos. La estructura exacta se definira mas adelante, pero conceptualmente habra mensajes como:

- `RENT_REQUEST`
- `RENT_ACCEPTED`
- `RETURN_REQUEST`
- `RETURN_ACCEPTED`
- `PAYMENT_CAPTURE`
- `ELECTION`
- `COORDINATOR`
- `MUTEX_REQUEST`
- `MUTEX_OK`
- `SYNC_PENDING`

## Casos de interes

### Caso feliz: retiro conectado

1. La app consulta estaciones cercanas.
2. El usuario elige una estacion con bicicletas disponibles.
3. La app solicita retirar una bicicleta.
4. La estacion valida distancia y disponibilidad.
5. La estacion registra el inicio del viaje.
6. La estacion informa al lider.
7. La estacion registra o envia la preautorizacion al banco.
8. La bicicleta queda en estado `EnViaje`.

### Caso feliz: devolucion conectada

1. El usuario se acerca a una estacion con lugares libres.
2. La app solicita devolver la bicicleta.
3. La estacion valida capacidad.
4. La estacion registra la devolucion.
5. La estacion informa al lider.
6. La estacion cierra el viaje y envia el cobro al banco.
7. La bicicleta queda disponible en la estacion de destino.

### Retiro sin conectividad externa

1. La app se comunica localmente con la estacion.
2. La estacion valida disponibilidad local.
3. La estacion libera una bicicleta.
4. El retiro queda guardado como pendiente de sincronizacion.
5. La preautorizacion queda pendiente.
6. Cuando vuelve la conexion, la estacion informa el retiro y envia el pago pendiente.

### Devolucion sin conectividad externa

1. La app se comunica localmente con la estacion.
2. La estacion valida que tenga lugar disponible.
3. La estacion acepta la bicicleta y actualiza su estado local.
4. La devolucion queda pendiente de sincronizacion.
5. El cobro queda pendiente si no pudo informarse al banco.
6. Al recuperar conectividad, la estacion sincroniza la operacion.

### Caida del lider

1. Las estaciones detectan que el lider no responde.
2. Una estacion inicia eleccion.
3. Se elige un nuevo lider de zona.
4. Las estaciones reenvian operaciones pendientes al nuevo lider.
5. La zona recupera coordinacion regional.

### Bicicleta no devuelta

1. Una estacion registra que una bicicleta fue retirada.
2. El viaje queda abierto.
3. Si pasa un tiempo maximo sin devolucion, el sistema marca la bicicleta como posiblemente perdida.
4. El estado se sincroniza dentro de la zona.

## Decisiones pendientes para validar

Todavia queremos validar con el corrector:

- Si la combinacion de eleccion de lider y exclusion mutua distribuida centralizada resulta suficiente como uso de herramientas distribuidas.
- Si el algoritmo Bully es una buena eleccion para zonas chicas y configuradas.
- Si la consistencia eventual propuesta para operaciones offline es aceptable.
- Si alcanza con implementar actores principalmente dentro de la estacion.
- Si el formato de mensajes de texto propio es aceptable sin usar crates externos.
- Si el umbral para marcar bicicletas como posiblemente perdidas debe ser configurable o puede quedar fijo para la simulacion.

## Pendiente para el README final

Para la entrega final de diseno todavia falta agregar:

- diagramas de procesos;
- diagramas de actores/threads internos;
- diagramas de entidades principales;
- pseudocodigo Rust de structs;
- detalle de payloads de mensajes;
- comandos de ejecucion;
- casos de prueba automatizados esperados.

