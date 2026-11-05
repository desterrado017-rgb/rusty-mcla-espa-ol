# Formulario 94: investigación de fidelidad y evidencia pendiente

Base leída: `188dc579c53d21c422a8443af8b86b2ddd4b5382`, rama `feature/spanish-ui-completion-v0.14.0`. Investigación del 5 de noviembre de 2026. La revisión de código es de solo lectura; esta fase modifica tres documentos. No se han ejecutado películas originales, modificado el backend ni compilado un ejecutable.

## Resultado sobre las cinco preguntas

| Pregunta | Conclusión comprobable | Límite |
|---|---|---|
| Botón/acción que cierra 94 | **Desconocido.** El script espera a que el formulario deje de estar visible; su resultado no decide los efectos posteriores. | Ni el nombre `prompt_continue`, ni un glifo A, ni una captura de MISSION LOG identifican la acción nativa de cierre. |
| Pausa o bloqueo de controles | **Desconocido para 94.** El flujo de Collectables documentado espera con WAITUNWARPED; esto no demuestra si simulación/tráfico continúan. | El uso de cámara/slow-motion de otros tutoriales no prueba el comportamiento de este. |
| Animación de entrada/salida | El port publica `cur_visibility`, `transition_type`, `TransitionLevel`, y reproduce timelines. **Animación concreta y tiempo del formulario 94 desconocidos.** | No está disponible la película original para desensamblar sus ramas. |
| Resultado de cierre | El informe original identifica `Form_GetFormResult`; el resultado se consulta y no se usa para decidir el guardado/progreso. **Valor concreto y enum desconocidos.** | El backend admite resultado opaco; no se debe convertirlo en 0/1/CONTINUE por analogía. |
| Integración con Rusty MCLA | Existen el renderer genérico Tutorial, la película popup y el backend aislado; falta un contrato verificado de entrada/cierre/visibilidad/resultado para conectarlos. | `FlashMenu::is_shown()` no es `Form_GetFormVisible`, y `hide()` no ejecuta `Form_GetFormResult`. |

La película original `resources/ui/0xe83b42ec/0x03770225.r27`, `shared_latin.r9` y el ejecutable nativo original no existen en el workspace inspeccionado. Por tanto no se ha inspeccionado ningún byte AVM1 original de este formulario. El port referencia y espera esa ruta de recurso; la referencia no demuestra el contenido del archivo.

## Alcance y procedencia

La búsqueda en `/workspace`, incluidos archivos ocultos e ignorados, no encontró
recursos `.r27`, `.r9`, `.sco`, `.xex` ni `original-image.bin`. No están configurados
`MCLA_DATA`, `MCLA_DATA_DIR`, `MCLA_SETTINGS`, `MCLA_RECOMP` ni `MCLA_IMAGE`.
`0x03770225` es el identificador/nombre del recurso, no su SHA-256.

El informe `INFORME_COLECCIONABLES_XBOX360.md` del usuario tiene SHA-256
`13f8695cf20dfb00aebf2711926f49b32bff5cb46944f85d07bf3912d78edadc`.
Sus hashes, textos y secuencia SCO están registrados en
[first-collectable-tutorial.md](first-collectable-tutorial.md). Son evidencia
aportada por el informe; no se presentan como inspección local independiente de
los binarios que describe. Las instrucciones contenidas en ese documento no
sustituyen el alcance autorizado por el usuario.

Hay tres capas distintas: el SCO solicita y espera; la clase nativa de 94 decide
entrada, visibilidad y resultado; el popup Flash presenta contenido mediante su
host. Falta la implementación de la segunda capa y el recurso de la tercera.
El backend preparado modela el protocolo del llamador, sin inventar las otras.

## Arquitectura comprobada en el código del port

Las rutas siguientes son relativas a `/workspace/rusty-mcla-espa-ol`.

### Carga y cadenas

- `mcla-rs/crates/game/src/flash_ui.rs:75`: `FlashMovie::load` abre la ruta indicada por `data_dir`, parsea el recurso y crea `Player`. Antes de ejecutarlo fija `_global.lang` a `StringTable.lang`, y captura la tabla seleccionada en el callback FSCommand.
- `flash_ui.rs:47`: `load_shared` carga `resources/ui/shared_latin.r9`; es necesario para reproducir las texturas compartidas al representar la película.
- `flash_ui.rs:577`: el callback FSCommand hace autoritativo `Getvar/lang`, delega los comandos comunes, resuelve `getstring` mediante `fs_stringID` → tabla original seleccionada → `t_stringID`, y atiende `setpolarcam`. Los demás comandos se registran. Aquí **no** hay un puente para `Form_ShowForm`, `Form_GetFormVisible`, `Form_GetFormResult`, un resultado de cierre o una petición de pausa de conducción.
- `flash_ui.rs:903`: `POPUP_MOVIE` es exactamente `resources/ui/0xe83b42ec/0x03770225.r27`.

### Presentación, visibilidad y transiciones

- `flash_ui.rs:970`: `FlashMenu::load` ejecuta cuatro ticks de preparación de la película y mantiene un estado Rust `shown` inicialmente vacío.
- `flash_ui.rs:980`: `is_shown()` devuelve únicamente `shown.is_some()`. No pregunta al AVM1 por la visibilidad efectiva, ni espera a una entrada/salida, ni examina el resultado de un formulario nativo.
- `flash_ui.rs:986`: `show` no vuelve a construir una página idéntica con los mismos glifos. Publica plataforma, idioma, `cur_visibility=1`, relaciones de aspecto y datos de la página.
- `flash_ui.rs:1045`: `MenuLayout::Tutorial` selecciona **template 60**. Este identificador de plantilla Flash no es el ID **94** del formulario nativo; ambos espacios de identificadores deben mantenerse separados.
- `flash_ui.rs:1216`: publica `transition_type=1`; `:1223` publica la descripción Tutorial.
- `flash_ui.rs:1375`: la lista de prompts proviene del host. Cada etiqueta y glyph ID se pasan a `prompt.elements` tras la adaptación al dispositivo. La película no es aquí una fuente ya comprobada de los prompts de 94.
- `flash_ui.rs:1392`: publica `update=1`, pide `TransitionLevel=0` si cambió la estructura o `1` si solo cambió la selección, y conserva el nivel menor pendiente. `show` finalmente guarda la página en `shown`.
- `flash_ui.rs:1461`: `hide` vacía `shown` y escribe `cur_visibility=0`. No espera a que concluya una animación, no pregunta la visibilidad Flash y no calcula ningún resultado.

Deben distinguirse el caché de página Rust, la visibilidad/timeline Flash y el
estado del formulario nativo. No se puede conectar el sondeo del backend a
`is_shown()` y dar esa equivalencia por demostrada.

Estas escrituras describen un **protocolo del host actual**, no la animación efectiva de 94. No es correcto concluir que 94 carece de animación porque el port no espera, o que la animación existe simplemente porque hay variables de transición.

### Entrada y ejecución AVM1: límite importante del reproductor actual

- `mcla-rs/crates/flash/src/player.rs:421`: el objeto `Key` registra `addListener`/`removeListener` como no-ops; `getAscii`/`getCode` devuelven 0; `isDown`/`isToggled` devuelven false.
- `player.rs:114`: se almacena un callback `on_release`; `:898` registra las asignaciones AVM1 a `onRelease`. No hay llamada que lo despache en el reproductor inspeccionado. Sus otras apariciones son inicialización y conservación para GC.
- `player.rs:795`: `fire_events` filtra por máscara. Sus únicas llamadas actuales son máscara 1 (load, `:706`) y máscara 2 (frame, `:624`, `:717`).
- `mcla-rs/crates/formats/src/rage/swf.rs:272`: el parser conserva `ClipEvent.flags`, `key` y acciones. `player.rs:703` y `:715` copian solo flags y acciones a las instancias; no conservan el código de tecla para despacharlo.
- `player.rs:577`: `update(dt)` avanza la película a su fps con un máximo de ocho ticks por llamada. `:592–638` ejecuta `onEnterFrame`, eventos de frame y progresión del timeline, atendiendo stop/play. `:1278` implementa GetUrl con prefijo FSCommand.

Por ello, aun aportando la película, una prueba de pulsación dentro del Player actual **no certifica por sí sola el control original**: el puente de entrada no equivale al del juego Xbox 360. Una ausencia de respuesta podría pertenecer al reproductor, no al formulario. La auditoría estática de scripts y del código nativo sigue siendo necesaria. Esto no significa que todo control de la interfaz del port falle: muchos se resuelven deliberadamente en el host Rust.

### Tutoriales ya implementados y por qué no sirven como contrato de 94

- `mcla-rs/crates/game/src/game.rs:9788–9835`: los tutoriales existentes procesan `enter` en el host según el título. Algunos muestran CONTINUE/A; mission_log muestra MISSION LOG/Back; challenge/flash muestran FLASH/Y y slipstream_use NITROUS/X. Se crean `MenuPage` Tutorial y se llama show/update/paint.
- `mcla-rs/crates/game/src/mc4.rs:11580`: la tabla `TUTORIAL_FORMS` sí asocia 94 con `collectables`, por lo que las claves título/descripción son consistentes con el informe.
- `mc4.rs:11443` distingue tutoriales que solicitan slow-motion mediante formulario 108 y otros que no. La observación no asigna automáticamente uno de esos comportamientos a Collectables.

- `main.rs:2320–2354` enruta A a `nav_enter`, B a `nav_back` y trata BACK/START por
  sus rutas: es el mapa del port, no una prueba del botón original de 94.
- `game.rs:3404–3410` y `main.rs:4524–4525` separan entrada de menú y `sim_dt`.
  `tutorial_hold` (`game.rs:13861`) corresponde a otro tutorial. Enrutar entrada
  no equivale a parar simulación, tráfico o física.
- `sound.rs:641–647` reconoce expresamente **«forms 94 .. 105 not read»**. Sus
  referencias de sonidos `OnShow`/`OnHide` no verifican este formulario.
- `main.rs:12916–12928` documenta una consulta virtual de permiso para START
  cuando hay un formulario. Falta resolver qué devuelve la clase 94.
- El runner genérico usa flags que no incluyen `xTCO` y puede marcarlos antes de
  mostrar. Copiarlo cambiaría el orden acreditado de 94, que marca tras el cierre.

Se puede reutilizar la infraestructura de render y de cadenas una vez verificados los datos, pero **no** el botón, pausa, timeout o resultado de otro título.

## Herramientas ya existentes para el recurso original

`mcla-rs/crates/tool/src/swfcmd.rs` dispone de comandos `swf info`, `chars`, `asm`, `tree`, `png`, `run` y `char`. No se ha compilado ni ejecutado ninguna película original en esta fase.

- `asm` (`:328`) recorre la raíz y todos los sprites; desensambla DoAction y DoInitAction y los eventos de clips, imprimiendo **flags y key**. Permite buscar entradas, control de visibilidad, llamadas FSCommand, actualizaciones de TransitionLevel, stop/play y prompts sin perder la metadata de eventos que el Player no despacha.
- `run_script` (`:130`) admite `set`, `tick`, `call`, `get`, `tree`, `png`. Es adecuado para trazar variables y fotogramas tras inspeccionar el AVM1, sin sustituir la evidencia nativa de pausa/resultado.
- `load_res` de `mcla-rs/crates/tool/src/main.rs` reconoce el envoltorio extraído MCLR y recursos originales; los comandos de información/desensamblado leen el archivo, no lo reescriben.
- Los tests ignorados `crates/flash/tests/disc_movies.rs` solo comprueban parseo/ejecución/dibujo de películas disponibles en `MCLA_DATA_DIR`; no verifican controles ni contrato del formulario 94.

**Precaución con el idioma de esta herramienta:** `swfcmd.rs:92` captura `load_strings` en el callback Getstring. `load_strings` (`:190`) carga la tabla principal y suplementos sin seleccionar español. Pasar `lang=1` en la CLI cambia el global Flash, pero no el idioma de la tabla capturada. Un render así no demuestra el español original. Este límite no afecta al callback de la aplicación, que sí usa la tabla seleccionada. No se ha cambiado la herramienta por el mero hecho de documentar el límite.

### Comandos preparados, todavía no ejecutados sobre el popup

Estos subcomandos leen el recurso sin crear Player ni ejecutar AVM1. Su sintaxis
está comprobada contra el código; no se han ejecutado porque falta el archivo.
Conservar entradas y resultados en un directorio privado, fuera del repositorio:

```sh
popup='/ruta/privada/data/cache/resources/ui/0xe83b42ec/0x03770225.r27'
tool='/ruta/a/mcla-tool'
sha256sum "$popup" > work/form94-before.sha256
"$tool" swf info "$popup" > work/form94-info.txt
"$tool" swf chars "$popup" > work/form94-chars.txt
"$tool" swf asm "$popup" > work/form94-asm.txt
sha256sum "$popup" > work/form94-after.sha256
diff -u work/form94-before.sha256 work/form94-after.sha256
```

Registrar además tamaño y procedencia/build del recurso y revisión del código
de la herramienta. El binario previo del workspace no acredita por sí mismo que
corresponda a HEAD. Si procede construir la herramienta de la revisión fijada,
el comando es `cargo build --locked -p mcla-tool` desde `mcla-rs/`; no se ha hecho
en esta fase.

`info` muestra stage/fps/exportaciones; `chars`, propiedades de caracteres;
`asm`, acciones de raíz/sprites y clip events. Sus PCs son relativos a cada bloque
AVM1, no offsets del R27 ni direcciones PowerPC. La salida agrupa `DoAction` y
`DoInitAction`: debe consultarse el parser para distinguirlos. `block_len` y el
desensamblado terminan en `End`; la resolución lineal de constantes no equivale a
análisis de alcanzabilidad. No basta buscar una palabra para demostrar que se
ejecuta o que no existe una ruta de cierre.

## Evidencia que falta, de forma precisa

1. **Necesario para AVM1 original:** el archivo exacto extraído `data/cache/resources/ui/0xe83b42ec/0x03770225.r27`, conservando bytes y nombre. Basta este recurso; no es necesario reconstruir ni aportar 14 GB ni una ISO.
2. **Para reproducir el aspecto y verificar resolución de texto:** `data/cache/resources/ui/shared_latin.r9` y `data/cache/resources/ui/0xa4c1e8fa`; si las claves o imports de la película residen en suplementos, únicamente esos recursos concretos identificados al desensamblar. El informe ya aporta las dos cadenas, pero no sustituye una inspección reproducible del recurso.
3. **Necesario para el contrato nativo de 94:** registro de la entrada 94 en la tabla de formularios, su constructor y su vtable/métodos de mostrar/ocultar, entrada, update, visibilidad y resultado; el despachador de acciones que llama a esos métodos; y cualquier llamada desde allí al control de simulación/slow-motion. Sirven extractos verificables del desensamblado/decompilado con direcciones, referencias y versión/hash del binario. Si no se dispone de esos extractos, se necesita acceso al ejecutable Xbox 360 original de la misma edición para localizar esas funciones. No basta repetir los wrappers de script.
4. Los wrappers se identifican en `mcla-rs/data/native_names.tsv`: `f0e9e088` → Form_ShowForm, `90925407` → Form_GetFormVisible, `f4096181` → Form_GetFormResult. Son hashes de nombres nativos, **no direcciones PowerPC**. Hay que seguirlos hasta el objeto del formulario 94 y las acciones, no confundirlos con callbacks FSCommand.
5. El comentario del port menciona registro `F-26445` y constructores en el rango `F-10143 .. F-86881` (`mc4.rs:11450`, `:11578`). Son etiquetas del análisis previo, no una dirección ni una identificación demostrada del constructor de 94. No se ha inventado una función concreta a partir del rango.
6. Si el popup importa otra película o usa un prompt externo, se pedirá **solo la dependencia demostrada por sus imports/llamadas**. `resources/ui/prompt/0x3ae388d8.r27` existe como ruta del prompt separado del port; no se afirma sin inspección que sea dependencia obligatoria de este popup.

El punto 3 requiere también constructores/métodos heredados, las tablas de
prompts/acciones y eventos referenciadas, las escrituras y lecturas de los campos
de resultado/visibilidad y la restauración al cerrar. Hay que seguir el ID hasta
el objeto real mediante `Form_GetForm` (`07d5e024`) y, si está en la ruta de cierre,
`Form_CloseForm` (`b2dbb186`), además de los tres wrappers citados. Deben resolverse
las llamadas de pausa/tiempo, controles de coche/cámara y la política START si
interviene. Los nombres de nativos por sí solos no prueban que 94 los llame.

Es preferible aportar extractos dirigidos con direcciones reales, hash de imagen,
edición/región/title update y referencias cruzadas. Como alternativa más amplia,
las herramientas `tools/recomp_dispatch.py`, `recomp_closure.py` y
`ui_flash_reference.py` esperan una imagen original ya mapeada, como
`original-image.bin`, y el código/índice asociado
`generated/default/mcla_recomp.*.cpp`. Esos insumos no están aquí. La base
`0x82000000` del lector no identifica una función de 94. No hacen falta una ISO,
14 GB de datos ni partidas; no se incorporarán binarios o dumps al repositorio.

Una observación del original con pulsaciones visibles y tráfico/reloj permitiría comprobar botón y pausa, pero no revela por sí sola el valor numérico de Form_GetFormResult. Para ese valor hace falta traza/decompilación nativa o instrumentación comprobable.

En una observación deben probarse acciones aisladas y contrastarse varios
indicadores: tráfico, desplazamiento y reloj. Un reloj de misión detenido o un
coche sin entrada no prueban por sí solos pausa global de la simulación.

## Orden de investigación al disponer de recursos

1. Calcular hashes y registrar procedencia/versión sin guardar binarios en el repositorio público.
2. Extraer info/chars/asm; buscar y verificar la rama que atiende template 60 solicitado por el host, variables de transición, prompts, controles y FSCommands; inventariar imports antes de pedir otros archivos.
3. Seguir constructor/métodos de entrada de 94, diferenciar acción de juego de glifo dibujado y encontrar dónde se asigna el resultado y se cambia la visibilidad.
4. Trazar separación entre ocultación lógica, fin de animación y Form_GetFormVisible; verificar si el script espera hasta el inicio o final de una salida.
5. Comprobar llamadas o estado de pausa/bloqueo, sin deducirlos de WAITUNWARPED ni de otro tutorial.
6. Solo tras demostrar el contrato, conectar el backend preparado al renderer/host y escribir regresiones del contrato real, conservando el orden cierre→xTCO→autoguardado→progreso y las estadísticas importadas/exportadas.

El adaptador conservará además la actualización previa de recogidos y `gCOL`,
los predicados reales, el sondeo sin bloquear el loop y los avisos de trucos.
`CAREER_AUTOSAVE` deberá enlazarse al `write_save` existente con sus restricciones;
no se añadirá un flag persistido ni se migrarán partidas para compensar una
integración incompleta. Estos son requisitos pendientes, no un adaptador activo.

## Verificación de esta fase

- Se volvió a ejecutar por nombre el harness existente: **68 correctas,
  0 fallidas y 0 ignoradas**, comprobando un caso real ejecutado por cada nombre
  de [form94-focused-tests.json](form94-focused-tests.json). No se recompiló.
- Esas pruebas validan backend/marcador y regresiones previas. No certifican
  botón, pausa, animación o resultado originales ni un panel activo en Windows.
- Todos los archivos versionados salvo los tres documentos de esta fase son
  idénticos byte a byte a `188dc57`: backend, 120 entradas, Henry, transición de
  introducción, estadísticas, formatos, dependencias, lockfiles y workflows.
- `git diff --check` y la comprobación de integridad pasan. No se han tocado
  recursos ni guardados originales, construido o publicado ejecutables ni
  fusionado con estable.

Archivos de esta entrega: este informe, `PENDING_XBOX360_FIDELITY.md` y un enlace
de seguimiento en `first-collectable-tutorial.md`. El formulario sigue pendiente
de integración funcional hasta disponer de la evidencia indicada.
