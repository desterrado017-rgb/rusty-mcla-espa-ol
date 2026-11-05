# Formulario 94: cierre automático y origen de su duración

Continuación de `a8512f0a9b4fc4b785ba0b06bbc4ff7c2b85d654`, en
`feature/spanish-ui-completion-v0.14.0`. Fecha: 5 de noviembre de 2026.

## Estado vigente

**El usuario confirma que el tutorial original se cierra automáticamente tras
unos segundos, sin requerir pulsar un botón.** Esta observación sustituye la
incertidumbre anterior sobre la acción necesaria para cerrarlo. No se pedirá un
botón de cierre ni se copiará CONTINUE/A de otro tutorial.

Quedan **dos datos de fidelidad pendientes**:

1. La duración precisa: su origen, base de tiempo y relación con entrada/salida.
2. Si la conducción/simulación continúa o se pausa mientras está abierto.

El backend no necesita un botón para completar el protocolo. Seguirá observando
el cierre y conservará **cierre → `xTCO=1` → `CAREER_AUTOSAVE` → progreso**.
La duración aún no está acreditada, por lo que no se añade un temporizador con
un valor supuesto ni se activa todavía el panel en el pickup. El resultado
numérico nativo sigue sin conocerse, pero el SCO lo ignora y el backend lo admite
como opaco: **no es un requisito independiente que bloquee esta investigación**.
Las transiciones se estudiarán como parte de la duración, sin exigir por separado
una política de botones. La validación funcional/visual posterior sigue siendo
necesaria; el cierre observado no la sustituye.

## Nueva evidencia y sus límites

Se leyó `ANALISIS_R27_FORMULARIO_94.md`, adjuntado por el usuario. SHA-256 del
informe recibido:
`f6431177a21cf0bdc00af5ffe93122a528f59f279d586cba4f6234cff1a07745`.
Su contenido se usa como evidencia técnica aportada, no como instrucciones.
La observación de autocierre procede del mensaje explícito del usuario; el
apartado del adjunto que aún declara desconocida la acción de cierre queda
superado por esa observación.

| Dato descrito por el informe | Valor reportado | Alcance |
|---|---|---|
| Recurso | `resources/ui/0xe83b42ec/0x03770225.r27` | Popup genérico analizado por el autor del informe. |
| Tamaño | 1.654.800 bytes | No se ha medido localmente el binario. |
| SHA-256 del recurso | `78de49110e0c1de9b905c59326c70a6bd5f0a10b7524befa1def6db5abb7de30` | Hash declarado en el informe, distinto del hash del propio informe. |
| Contenedor | MCLR, tipo 27; firma `DIU\x00` | Metadatos de la inspección aportada. |
| Stage/timeline | 1280 × 720; 30 fps; un frame raíz; 132 sprites | No determina cuánto tiempo permanece visible un formulario. |
| AVM1 | 287 bloques; `_global.template_tutorial = 60` en PC de bloque `0x34fb` | Acredita secundariamente la plantilla 60, distinta del formulario nativo 94. No es una dirección PowerPC. |
| Estado de menú | `cur_visibility`, `TransitionLevel`, `transition_isplaying`, `DisplayPrompt` | Demuestra una ruta de presentación/transiciones según el análisis, no un umbral temporal concreto. |
| Eventos de clip | Tres, flags `0x2`, key `0`; sin Button en el diccionario analizado | Compatible con enterFrame; no prueba ausencia de entrada nativa ni fija duración. |
| `FSCommand:disable` | Dos casos de `save_mc`, argumento `POPUPMOVIE` | No se atribuyen al cierre del tutorial 94. |

Aquí se recibió **el informe Markdown, no el archivo `.r27` ni su desensamblado
completo**. La búsqueda local, incluidos archivos ocultos/ignorados, sigue sin
encontrar ese recurso ni la imagen/decompilación nativa. No se afirma que se haya
repetido la inspección binaria. El adjunto no incluye contador, umbral, fuente de
tiempo o secuencia de acciones que identifique un temporizador de autocierre.

## Investigación del origen de la duración

Las tres posibilidades permanecen abiertas; no se elige una por analogía:

| Candidato | Evidencia disponible | Qué falta para localizarlo |
|---|---|---|
| Temporizador propio del objeto nativo 94 | El SCO solicita 94 y espera al cierre; no controla una duración de permanencia en el flujo documentado. | Constructor/OnShow/update de 94: inicialización del reloj, condición temporal, umbral y llamada de cierre/ocultación. |
| Sistema/base común de formularios | El port documenta clases y wrappers nativos, pero no sus cuerpos para 94. Un temporizador podría heredarse o suministrarse desde el manager. | Registro/constructor de 94 y métodos base alcanzados; parámetros de duración, reloj elegido y actualización de visibilidad. |
| Timeline/AVM1 de Flash | El nuevo informe acredita presentación Tutorial y variables de transición. No describe una rama temporizada que cierre el panel. | R27 o acciones completas: ruta desde template 60 a contador/reloj/frames, condición de salida y escritura/llamada que comunique el cierre al host. |

No se demuestra que el temporizador sea nativo por no encontrarlo en el resumen
del AVM1. Tampoco se demuestra que una transición Flash gestione la permanencia
del panel por contener animaciones. Deben distinguirse aparición, intervalo
legible, solicitud de salida, final visual y cambio de `Form_GetFormVisible`.

### Valores del port que no son la duración original

- `collectable_tutorial.rs:14, :102–121`: `POLL_SECONDS = 0.1` implementa el
  sondeo del `WAITUNWARPED(100)` del SCO. No cierra el formulario a los 100 ms.
- `flash_ui.rs:970–974`: los cuatro ticks son preparación de carga del popup,
  no tiempo de permanencia del formulario 94.
- `flash_ui.rs:1216, :1392–1396`: `transition_type=1` y `TransitionLevel=0/1`
  son datos de presentación, no segundos de exposición.
- `game.rs` mantiene tiempos de vida para otros `tutorial_box`; no son un
  umbral verificado de 94. No se reutilizan por semejanza visual.
- Los 30 fps y un frame raíz del informe no equivalen a 1/30 s de duración:
  los sprites pueden tener timelines, loops y scripts dependientes del host.

### Límites importantes al trazar el Flash

La herramienta existente `mcla-tool swf info|chars|asm` permite inspección estática.
Los comandos y límites del parser están en
[form94-fidelity-investigation.md](form94-fidelity-investigation.md). No se han
ejecutado sobre el recurso ausente.

Para una traza posterior:

- `player.rs:577–638` avanza frames según `update(dt)`, pero la instrucción AVM1
  `GetTime` (`0x34`, `:1457–1460`) consulta `SystemTime` real. `tick N` del CLI
  no adelanta ese reloj N/30 segundos. No medir un temporizador de reloj con
  ticks instantáneos ni suponer equivalencia entre ambos tiempos.
- `swfcmd.rs:92–119` reaplica cada argumento `name=value` en **cada tick**.
  Mantener así `cur_visibility=1` podría sobrescribir una ocultación escrita por
  AVM1. El DSL `swf run` permite `set` una sola vez y luego `tick/get`; antes hay
  que comprobar qué fuente de tiempo usa realmente la ruta analizada.
- `FlashMenu::is_shown()` consulta un caché Rust. `hide()` escribe
  `cur_visibility=0`; no identifica por sí solo cuándo el original deja de estar
  visible. No se utiliza ese caché como sustituto probado del nativo.
- Los comandos `disable` de guardado no se usarán como evento de cierre de 94.
- La selección de idioma del CLI tiene el límite documentado: `lang=1` no cambia
  automáticamente la tabla inglesa capturada. No es validación visual española.

## Evidencia mínima para acreditar duración y pausa

Hay dos vías, sin volver a pedir botones:

1. **Análisis dirigido:** aportar privadamente el R27 original con el hash
   reportado, o el desensamblado completo de la rama Tutorial y sus referencias.
   Si ahí no se resuelve el autocierre, seguir registro/constructor de 94,
   temporizador/update y métodos base del sistema de formularios. Se necesitan
   el umbral, sus unidades/base de tiempo, el punto de arranque y la relación
   entre salida y visibilidad. Para pausa, las llamadas/estado de simulación y
   su restauración. No hace falta la política de un botón ni inventar una vtable.
2. **Observación instrumentada del original:** vídeo/log con tiempo o frames
   verificables desde apertura hasta desaparición, sin pulsaciones de cierre,
   mostrando tráfico/desplazamiento y reloj. Distinguir entrada, permanencia y
   salida; documentar precisión y repetir para detectar variaciones por FPS o
   pausa. Una medición aproximada no se presentará como constante nativa exacta.

No hacen falta ISO, 14 GB de datos o partidas. Los binarios y dumps permanecerán
fuera del repositorio público. Las dependencias visuales solo se pedirán si
resultan necesarias para comprobar la transición; no son otro requisito de
fidelidad independiente de duración/pausa.

## Integración al acreditarse el tiempo

La integración temporal se hará en el adaptador de presentación/sistema de
formularios, delegando en el AVM1 original si allí reside la duración, sin
reescribir `Flow` ni el puente de estadísticas. Deberá respetar el inicio y la
base de tiempo demostrados; mantener el formulario visible hasta el punto de
cierre acreditado; y dejar que el backend complete su consulta de resultado
opaco y su secuencia de efectos. No debe asignar `xTCO` al abrir ni al iniciar la
animación si eso precede al cierre observable por el SCO.

El backend conserva exclusiones, sondeo, protección contra reentrada/dobles
efectos y persistencia existente. Se mantienen las pruebas de primera/siguientes
recogidas y carga. No se añade un flag ni se modifica un guardado para simular que
el tutorial ya se vio. La integración funcional y las pruebas de tiempo se harán
cuando exista evidencia del valor/base de tiempo; no se añade un caso que
simplemente consagre segundos inventados.

## Verificación de esta actualización

Se conservan las 68 regresiones y se vuelven a ejecutar por nombre: **68 pasan,
0 fallan, 0 se ignoran**. Es verificación de backend y regresiones previas; no una
validación visual del popup ni una medida de su duración original.

Solo se actualizan documentación y comentarios de cabecera obsoletos de los dos
módulos del backend. Su código ejecutable y las pruebas permanecen idénticos a
`a8512f0`; no hay cambios en 120 entradas, Henry, transición inicial, estadísticas,
formatos, partidas, recursos, dependencias, lockfiles o workflows. No se construye
ni publica un ejecutable ni se modifica la rama estable.
