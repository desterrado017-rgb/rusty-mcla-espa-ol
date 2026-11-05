# Primer coleccionable: contrato original y backend preparado

Base solicitada: `df1671bc2029adbd6eb97ef9f31fb267566a6c79`, rama
`feature/spanish-ui-completion-v0.14.0`. Corrección inmediata del marcador:
`05c25f63564a257a8f4fe518978d1fd8c8106e65`.

**Estado:** el marcador real y el protocolo verificable están implementados.
El backend y su puente a estadísticas se prueban directamente, pero **no están
conectados al pickup activo**. El formulario aún no se muestra en el juego.
Sus controles, pausa, validación visual y adaptador de autoguardado de App siguen
pendientes, conforme a la aclaración del usuario. No se considera cerrada la tarea.

## Evidencia recibida

Se leyó `INFORME_COLECCIONABLES_XBOX360.md`, aportado por el usuario el 5 de noviembre
de 2026. SHA-256 del informe leído:
`13f8695cf20dfb00aebf2711926f49b32bff5cb46944f85d07bf3912d78edadc`.
El informe describe la descompilación del SCO y la lectura de los recursos:

| Recurso original analizado en el informe | Tamaño | SHA-256 reportado |
|---|---:|---|
| `Collectables.sco` | 1.868 bytes | `621fe074089ae1a1dae34d9d28d9e8c0c861331ab6fc5e7b38ba862a45d754ad` |
| `ui/0xa4c1e8fa` | 2.157.137 bytes | `c01f84edf4563d21084140839b9ead10405f97ae01949ad4abc90b5fb8a3965d` |

Los binarios no se recibieron ni se copiaron al repositorio en esta continuación.
La procedencia de los textos y del flujo se acredita mediante el informe, sin
presentar su lectura como una inspección local independiente de esos binarios.
Se incluyen únicamente fixtures textuales reducidos para las regresiones solicitadas.

El informe y la aclaración posterior confirman:

- `tutorial_collectables_title`: `COLLECTABLES` / `OBJETOS COLECCIONABLES`.
- `tutorial_collectables_desc`: descripción original que explica el desbloqueo de
  trucos; los fixtures conservan el doble espacio inglés y `Éstos` en español.
- `COLLECTABLE_FOUND_MSG`: usa la variable `NUM_COLLECTABLE_COLLECTED` dentro de
  `<D>…</D>`. En español se conserva el doble espacio después de `COLECC.`.
- La presencia de `prompt_continue` no demuestra qué prompt ni botón usa el 94.
- La ausencia de red, conexión y editor; tutoriales habilitados; `xTCO == 0`.
- `Form_ShowForm` solicita el 94; se consulta `Form_GetFormVisible`; entre
  reconsultas se hace `WAITUNWARPED(100)`. Tras cerrarse se obtiene
  `Form_GetFormResult`, cuyo valor no decide los efectos posteriores.
- Después: asignar `xTCO/0 = 1` con operación 3, pedir `CAREER_AUTOSAVE` y mostrar
  el contador. El script también contempla `CHEAT_UNLOCKED` en múltiplos de diez;
  esa presentación deberá mantenerse al completar la integración de App.

## Corrección del marcador y de la interpretación de estadísticas

La implementación y sus primeras pruebas usaban incorrectamente
`<D>NUM COLLECTABLE COLLECTED</D>`. Es una errata del código y de esas pruebas,
no un token alternativo del juego. `collectable_text.rs` ahora sustituye solo
`<D>NUM_COLLECTABLE_COLLECTED</D>` y conserva el resto del recurso. El token con
espacios se trata como desconocido; no se normalizan identificadores.

Se añadieron las cadenas reales EN/ES de progreso del informe. La regresión con
el token antiguo falla mostrando el marcador literal; con el correcto pasa.
No se reescribe el denominador oficial ni se añade un contador adicional.

La descripción previa de `xTCO` en `tools/mc4save/small.py` como «all collectables
found flag» era incorrecta para este flujo. Se corrige el metadato descriptivo;
no se modifica ningún valor del guardado. `gCOL` sigue siendo otro contador.

## Protocolo implementado, sin asignar controles nativos

`game/src/collectable_tutorial.rs` ofrece:

1. Elegibilidad con el valor real de `xTCO` y los cuatro predicados explícitos.
   No se deriva de la cantidad recogida ni se marca visto al solicitar el panel.
2. Solicitud de formulario 94 y confirmación de que el adaptador lo ha mostrado.
   Una petición sin confirmar no equivale a un formulario cerrado.
3. Primera consulta de visibilidad inmediata; posteriores cada 100 ms de tiempo
   no escalado. Se espera la respuesta, sin ráfagas ni bloqueo del loop.
4. Visibilidad desconocida, otro formulario o un resultado fuera de orden no
   completan el tutorial. El cierre solicita el resultado real, sin interpretar
   su valor como un botón.
5. Token de finalización opaco y transitorio. El flujo continúa ocupado hasta
   confirmar los efectos, evitando otro disparo mientras `xTCO` todavía sea cero.

El payload `tutorial_page` usa `tutorial_keys(94)`, StringTable y la plantilla
`MenuLayout::Tutorial` existente. Solo prepara título y descripción originales.
Recibe los prompts de un futuro adaptador verificado: no presupone A, CONTINUE,
MISSION LOG ni ausencia de botones. Los prompts vacíos de una prueba significan
que no se suministró política de controles, no que el original carezca de ellos.

`game/src/collectable_tutorial_stats.rs` integra las APIs reales de Game:

- `collectable_tutorial_begin` consulta `mc4_stat("xTCO", 0)` con la precedencia
  existente. Ausente vale cero; no se modifica el estado al consultar o cargar.
- `collectable_tutorial_finish` valida que el token pertenece a ese flujo,
  asigna `xTCO` con el gestor existente, invoca el callback `CAREER_AUTOSAVE`,
  presenta el progreso auténtico y confirma los efectos para liberar el flujo.
- El callback es el contrato de petición, **no un dispatcher activo de App**.
  Cuando se conecte, deberá usar `write_save`, respetando `no_save` y los errores
  existentes. La petición precede al progreso; no se afirma que la escritura a
  disco deba completarse antes del splash original.
- Se conservan las restricciones del gestor de estadísticas, incluida capacidad
  y goal attack. Un rechazo no fuerza un registro ni inventa un rollback; se
  mantienen la petición y el progreso como en el flujo del script.

El estado no es serializable y no añade campos a SaveData. La persistencia sigue
estando en `career.stats` y `career.ostats`, sin añadir `xTCO` a `tutorials_shown`.
El MC4 tipado y el MC4 incompatible/raw se conservan. No se reparan discrepancias
entre tablas, lista de objetos o `gCOL` al cargar.

## Integración que queda pendiente

El informe y el usuario dejan explícitamente sin acreditar los controles y la
pausa. El código disponible no contiene la implementación nativa de esas partes.
El runner genérico marca estadísticas antes de mostrar y no permite inferir el
contrato del 94; añadir `xTCO` a `TUTORIAL_FLAGS` sería incorrecto.

Por ello `main.rs` solo registra los módulos preparados. No llama a este flujo
al recoger un objeto, no muestra un panel sin cierre conocido y no cambia los
controles, cámara, tiempo de simulación ni el guardado actual de la recogida.
La corrección del marcador sí actúa sobre el aviso normal existente.

Para terminar se necesita comprobar `0x03770225.r27` y la gestión nativa del 94,
o una observación específica Xbox 360/Xenia con pulsaciones, tráfico y reloj.
Después se enlazarán apertura/cierre, las respuestas de visibilidad/resultado,
los predicados reales y el evento de autoguardado al loop, sin bloquearlo.
Esa integración también conservará el orden previo del script: actualizar
`gCOL` y el estado local de recogidos antes de solicitar el formulario. Este
backend no altera la actualización del contador ni reemplaza esa parte de App.

## Pruebas y validación

Los resultados exactos se publican en [form94-validation.md](form94-validation.md).
Las pruebas nuevas usan texto auténtico reducido y las APIs reales del backend,
Game, Career y SaveData. Cubren primera/siguientes recogidas elegibles, supresión,
xTCO no cero, secuencia y sondeo, ausencia de dobles efectos y carga JSON.

Los guardados se serializan en memoria. Para probar el `Career::restore` real se
crea únicamente XML de misión sintético temporal, sin datos ni partidas del usuario.
Las pruebas no validan botones, pausa, render del panel original ni un ciclo de
recogida/UI/autoguardado activo en Windows. Los avisos de trucos y la presentación
final siguen formando parte de la aceptación pendiente al conectar App.

Se conservan las 120 entradas, Henry, la máscara de apertura, dependencias,
lockfiles y recursos originales. No se publica un ejecutable.
