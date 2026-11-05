# Tutorial del primer coleccionable: investigación y pruebas pendientes

Base de las referencias: `48d2792ab84904dbfe3e5071d53f2bb0ad4afadb`, rama
`feature/spanish-ui-completion-v0.14.0`. Tarea `COLLECTABLE-FIRST-TUTORIAL`,
registrada en [PENDING_XBOX360_FIDELITY.md](../PENDING_XBOX360_FIDELITY.md).

**Estado real:** investigación inicial de código completada; no se ha
implementado el tutorial ni se han acreditado sus condiciones originales.
Las pruebas específicas de primera recogida y persistencia se describen abajo
como pendientes. No se atribuye ningún texto español no inspeccionado a Rockstar.

## Evidencia disponible

| Recorrido en la base | Comprobación |
|---|---|
| `game/src/main.rs:4961–4981` | La recogida añade el índice, llama a `collectable_text::found_message`, reproduce el sonido y usa el guardado existente. No solicita un tutorial. |
| `game/src/main.rs:13762–13774` | Lee `script_obj/game/MiscTasks/Collectables.sco` para extraer las 60 posiciones. No interpreta las ramas originales del tutorial. |
| `game/src/mc4.rs:11578–11643` | `TUTORIAL_FORMS` incluye `(94, "collectables")`; `tutorial_keys(94)` produce `tutorial_collectables_title` y `tutorial_collectables_desc`. |
| `game/src/mc4.rs:11614–11638` | Los 22 disparadores de `TUTORIAL_FLAGS` no incluyen el formulario 94. No se ha identificado su contrato de disparo en el port. |
| `game/src/mc4.rs:12046–12180` | Los tutoriales registrados usan estadísticas, cola y un estado activo. `mc4_tutorial` rechaza flags desconocidos; `mc4_tut_seen` consulta la estadística. |
| `game/src/game.rs:9788–9827` | El prompt CONTINUE y el cierre por entrada dependen de un estado reconocido. Abrir únicamente `tutorial_box` para el 94 no los garantiza. |
| `game/src/flash_ui.rs:903,1045,1223` | Existe la ruta de popup original, con `MenuLayout::Tutorial` y plantilla 60. El título y la descripción se leen de StringTable. |

Las rutas `game/src` son relativas a `mcla-rs/crates/`. Los mapeos de formulario
y claves también están en el v0.14.0 oficial
`30d8b1f3b54557664d81bb7cfc9cc33caea53647`. Esto prueba nombres y rutas de
consumo; no la existencia ni el contenido castellano de las entradas en una
instalación cuyos recursos no están disponibles aquí.

El texto «COLLECTABLES» y la explicación del desbloqueo de trucos forman parte
del comportamiento reportado por el usuario. No se ha redactado ni insertado
una supuesta versión oficial española.

## Condiciones y persistencia por demostrar

- `SaveData.collected` conserva índices. El importador original lee 60 bytes de
  LocalOptions desde `0x3DF84` (`savedecode/src/options.rs:16–17,244`) y conserva
  los índices no nulos (`savedecode/src/convert.rs:551–552`).
- `gCOL` es el contador estadístico; se sincroniza al alza desde la lista
  (`mc4.rs:12610–12614`). La lista y las estadísticas son canales distintos.
  Su actualización ocurre en momentos diferentes del tick: no se debe dar por
  hecho que siempre coinciden al guardar o cargar.
- `Mc4.tutorials_shown` conserva una lista de flags; el importador obtiene los
  `tT…` verdaderos, excepto `tTTA`. No se ha demostrado un flag de coleccionables.
  La ausencia de formulario 94 en la tabla del port tampoco prueba que el
  original necesite un flag separado: podría usar un contador u otro predicado.
- Collectables figura en `SCRIPTS_WITHOUT_DATA` (`mc4.rs:10143`), con cero
  palabras propias guardadas en ese registro. Esto no descarta Stats,
  LocalOptions, globals u otro canal de persistencia.
- `xTCO` está documentado como «todos los coleccionables encontrados»; no debe
  reutilizarse como «tutorial mostrado». No se introducirán `tTCO`, `tTCL` ni
  otro identificador o bool nuevo sin evidencia.

Por tanto, no se marcará automáticamente como visto a quien tenga progreso ni
se considerará pendiente en toda partida que carezca de un bool nuevo. Ambas
decisiones afirmarían un historial no acreditado. La implementación deberá
respetar el estado real de las partidas anteriores y los campos desconocidos.

También falta verificar si se marca al solicitar, mostrar o cerrar el panel;
si la recogida durante una carrera lo aplaza; cómo afecta la supresión de
tutoriales; y el orden exacto entre tutorial, sonido, progreso y guardado.
La ruta actual admite recogidas en carrera, pero el tutorial genérico no tiene
por ello garantizada una presentación válida en ese modo.

## Presentación y controles

El popup actual es `resources/ui/0xe83b42ec/0x03770225.r27`. Las fuentes/arte
compartidos se cargan desde `resources/ui/shared_latin.r9`. El recurso de
prompt independiente no equivale al panel de tutorial.

Crear solo un `tutorial_box` con las dos claves no basta: sin el estado de
tutorial pertinente puede quedar sin CONTINUE, sin cierre por el botón
correcto y con el timeout genérico de 600 segundos. No se copiarán por analogía
la pausa, voz, duración o controles de otro tutorial. La lista de llamadas del
decodificador `Sco::calls` es lineal y no sigue ramas; deberá revisarse el flujo
del script para demostrar sus condiciones, no solo buscar constantes.

## Matriz de regresión que se añadirá con la implementación

**Todos estos casos están pendientes.** No son tests ejecutados ni una
certificación del tutorial a partir de un modelo supuesto. Se usarán partidas
sintéticas y el contrato persistente demostrado por el script.

| Caso | Resultado que debe verificarse |
|---|---|
| Partida nueva, primera recogida válida: 1/60 | Un único panel original 94, explicación obtenida de recursos y controles/cierre acreditados. Orden con progreso según el script. |
| Siguiente recogida: 2/60 | Solo progreso normal, sin tutorial repetido y sin marcador literal ni contador duplicado. |
| Tercera y posteriores: 3/60, 59/60, 60/60 | Progreso correcto; no repetir el tutorial ni alterar desbloqueos existentes. |
| Carga sin coleccionables y primera recogida posterior | Mantener la elegibilidad que indique el estado original; cargar por sí solo no equivale a recoger. |
| Carga de partida que ya mostró el tutorial | No repetirlo al cargar ni en la siguiente recogida. Preservar el estado persistente original. |
| Guardar/cargar antes y después de cerrar el panel | Reproducir el momento original de persistencia, sin deducirlo de otro tutorial. |
| Partida anterior del port o importada de Xbox 360 | Conservar índices, estadísticas, lista de tutoriales y campos ajenos; ninguna migración por un flag supuesto. |
| Lista y `gCOL` distintos o campos opcionales ausentes | Preservar ambos; aplicar únicamente la precedencia original demostrada, sin reparaciones automáticas. |
| Recogida en carrera, otro cuadro activo o tutoriales suprimidos | Reproducir el aplazamiento/supresión real, sin perder ni repetir el tutorial. |
| Inglés y español | Usar las mismas claves y recursos originales; sin traducción manual, con controles, saltos y ausencia de recortes verificados visualmente. |

En todos los casos se conservará el formateador de `COLLECTABLE_FOUND_MSG` ya
corregido. La cobertura automática y la validación visual se informarán por
separado. No se añadirán tests ignorados o simulaciones de un flag inventado
para presentar esta tarea como resuelta.

## Archivos mínimos para continuar

Copias de lectura de estos archivos ya presentes en la extracción del usuario,
sin editar sus bytes y conservando la capitalización real:

1. `data/cache/script_obj/game/MiscTasks/Collectables.sco`.
2. `data/cache/ui/0xa4c1e8fa`.
3. `data/.install/files.tsv`, si existe; en su defecto, un listado de nombres y
   tamaños de `data/cache/ui`, para localizar una tabla suplementaria si procede.

Para validar el panel visual después de resolver el script, podrán necesitarse
`data/cache/resources/ui/0xe83b42ec/0x03770225.r27` y
`data/cache/resources/ui/shared_latin.r9`. Si los controles dependen de código
nativo no descrito por esos recursos, se identificará la evidencia concreta que
falte después de leer el script; no se asignarán controles por suposición.

No se necesitan ISO, partidas reales, claves, audio/vídeo ni reconstruir los
14 GB. Se analizará el `.sco` ya extraído que utiliza el port; no se reescribirá
ni reconvertirá el archivo original por suposición.

## Alcance de este registro

Esta entrega añade documentación de trabajo pendiente y evidencia de la
investigación. No cambia código, guardados, recursos, las 120 entradas,
Henry ni la corrección del contador. No compila ni publica otro ejecutable.

Se reejecutaron las 51 regresiones focalizadas existentes del harness de
`48d2792`: 51 correctas, 0 fallidas y 0 ignoradas, incluidas las del marcador,
Henry y el traspaso de película. Esto comprueba la base conservada; no valida
el tutorial que aún no se ha implementado ni los casos pendientes de la matriz.
