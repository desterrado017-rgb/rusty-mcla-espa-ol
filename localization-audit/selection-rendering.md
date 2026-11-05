# Revisión independiente del renderizado en VEHICLE SELECT

Base examinada: `5c740ad2c0c29528193ecd16fc6dfa4ecb4e1348`. Comparación oficial: `30d8b1f3b54557664d81bb7cfc9cc33caea53647` (Rusty MCLA v0.14.0). Inspección de solo lectura; no se han ejecutado el juego, capturas ni compilaciones, ni se han leído/modificado recursos o partidas del usuario. Referencias de línea de la base, no de futuros commits.

## Resultado comprobado y límite

Se confirma un **hueco de cobertura negra al finalizar la película**, tanto por salto como por fin natural. Esto demuestra exposición de la escena antes de completar su traspaso; **no demuestra todavía que cada oclusión o vehículo parcialmente oculto reportado por el usuario tenga esa causa**. Sin la geometría, capturas y registros de esa sesión no se puede atribuir el resultado a un sector concreto, al z-buffer, al streaming o a una cámara dentro de un muro.

No se ha identificado un cambio de renderizado provocado por la localización: `camera.rs`, `world.rs`, `occluder.rs` y todo `render/` son idénticos al oficial comparado; los cambios de `vehicles.rs` son diagnósticos del rival y su preparación, sin alterar el dibujo de los starters. El estado de apertura y las constantes de cámara examinados son anteriores a la localización. La presencia de un defecto en ambos árboles no certifica su reproducción visual en ambos ejecutables.

## Prueba de la discontinuidad de negro

1. `main.rs:13079–13080`: mientras `game.movie` existe, `over_black` hace que el valor de máscara publicado a UI sea `game.intro_mask=0`; la película cubre el mundo con su propio fondo negro.
2. `game.rs:4735`: `ui_screens` toma la película con `self.movie.take()`. En un fotograma activo, `Video::show` pinta negro y después la imagen.
3. `video.rs:98–103`: al llegar al final de fotogramas o al terminar el desvanecimiento de salto, devuelve `false` **antes** de crear el painter o pintar el fondo negro (`video.rs:113–115`).
4. `game.rs:4763–4766`: el host marca `movie_done` y retorna `MenuAction::None`. No se vuelve a insertar la película ni se dibuja otra máscara en esa rama.
5. Ese retorno evita la máscara global final de `ui_screens`, `game.rs:10030–10034`. Las ramas previas tampoco la cubren: `start_warp_on` ya se ha liberado bajo negro en `opening_tick`, y `opening_black` es para stage4 o stage3 después de aceptar, no para stage2 mientras termina la película.
6. La escena 3D ya está preparada bajo la película. En el siguiente tick, `opening_tick` puede retener negro mientras espera o iniciar su fade; el fotograma inmediatamente anterior queda descubierto sin esperar a ese tick.

La reparación demostrable es conservar la cobertura en el fotograma de traspaso. No requiere alterar geometría, orden de dibujo, comparación de profundidad, colocar los coches delante del mundo ni mover la cámara. Debe probarse por el camino real de finalización de película y UI tanto para salto como para EOF, no solo mediante un helper que repita la condición.

## Profundidad y orden de presentación

- `camera.rs:81–90`: proyección infinita reverse-Z, profundidad 1 cerca y 0 lejos; near inicial 0,3 (`camera.rs:65`).
- `render/mod.rs:2117`, entre otros: pipelines de ciudad/vehículos usan `GreaterEqual`; las escrituras de profundidad dependen del material. No hay selección de pipeline condicionada al salto de la película.
- `main.rs:14467–14488`, `begin_city_pass`: el primer pase limpia color, profundidad a0 y stencil a0; los siguientes cargan y conservan esos targets.
- `main.rs:11337–11549`: los command buffers de ciudad se graban en paralelo, pero se insertan en el orden fijo A1 (cielo+sectores), A2 (resto de sectores), B1 (suelo+props), seguido por B2 del hilo principal (coches, personas, transparencias).
- `main.rs:11762`: una sola cola los envía en ese orden. El paralelo de grabación no implica orden aleatorio de ejecución de profundidad.
- `intro_mask` es una capa egui sobre la escena; no escribe ni reinicia la profundidad de la ciudad.

Esto descarta la hipótesis de que la localización haya añadido una rama de profundidad específica al salto. No es una prueba contra fallos de driver o de materiales que solo pueden estudiarse con la captura problemática. No se recomienda desactivar depth-test ni dibujar los coches por encima de la ciudad para ocultar la incidencia.

## Cámara, coches y visibilidad

- `main.rs:13032` inicializa la cámara del lote al comenzar stage2, antes de que acabe la película; `sel_cam` se interpola en stage3, pasos de30ms (`main.rs:13000–13006`).
- Constantes `SEL_EYE0`, `SEL_LOOK0`, `SEL_EYE` y `SEL_LOOK` en `main.rs:15359–15362`; son las mismas en el oficial comparado.
- `main.rs:8312`: la cámara de selección se ofrece al selector de vista cuando no hay una escena narrativa activa; otras prioridades posteriores (warp, lugar, título, cámara fija de diagnóstico, etc.) pueden sustituirla. La cámara efectiva se copia a `cam.pos` en el mismo recorrido (`main.rs:8997` y siguientes). Un log únicamente de `sel_cam` no demostraría qué cámara se dibujó finalmente.
- Los starters son tres vehículos distintos, `driver="starter"`, fijados y desactivados, creados bajo negro (`main.rs:4592–4621`). `hide_player` oculta el vehículo del jugador en la apertura; no es una máscara parcial sobre estos tres.
- `main.rs:9683–9689`: frustum y oclusores CPU se recalculan desde el VP de la vista final y se eligen LODs. `vehicles.rs:5984–6012` decide LOD/visibilidad; un rechazo suele retirar la instancia, mientras que la prueba de profundidad puede producir una ocultación por píxel. Esto orienta la investigación, pero no prueba por sí solo el origen de una ocultación parcial; también hay piezas, materiales y ruedas separados.
- `main.rs:11493` y `11520`: los coches opacos y transparentes comparten las pruebas de escena normales. No se encontró una rama especial que los dibuje detrás de todo por haberse saltado la película.

## Streaming: por qué cero pendientes no basta como diagnóstico

`World::streaming` devuelve exclusivamente `pending.len()` (`world.rs:1348–1350`). `World::ready_at(p)` comprueba que los sectores a menos de60m estén residentes (`world.rs:1333–1335`). Son observaciones diferentes.

El foco de `world.update` se calcula al principio del tick (`main.rs:2834–2853`): jugador, cámara de escena y título, en ese orden de prioridades; no selecciona expresamente `sel_cam`. Con el jugador ya en el lote puede ser suficiente, pero no debe inferirse sin registrar las posiciones reales. Además hay presupuesto de uploads e inflight máximo12 (`world.rs:1368–1424`): cero pendientes en una observación aislada no prueba que no queden sectores solicitables o recursos fallidos.

Stage2 comprueba `intro_t>3/60` y cero pendientes (`main.rs:13038`), pero ese tiempo empezó al comenzar la película (`13030`), no al terminarla. La película completa ofrece más tiempo de carga que un salto temprano. La condición no equivale a medir tres presentaciones después de terminar. Esta diferencia es comprobable en código; que cambiarla resuelva el escenario reportado requiere evidencia adicional. No se debe añadir una espera arbitraria ni suprimir sectores como parche visual.

## Diagnóstico mínimo recomendado, optativo

Un único interruptor, p.ej. `MCLA_DIAG_OPENING=1`, debe generar líneas D de log y no cambiar dibujo, tiempos ni preferencias. Registrar eventos al iniciar película, pedir salto, finalizar (razón `skip`/`eof`/`attract-input`), cambiar stage y liberar máscara; durante el traspaso basta una muestra por cambio de stage/estado y, si persiste, cada60fotogramas.

Campos de evento:

- frame/tick, dt no escalado, intro_stage, intro_t, movie presente, movie tiempo/duración y razón de salida; cuts y cine presentes, fresh_game, starter_pick, starters_made.
- máscara interna `intro_mask` (fase/alpha/full si la API lo expone), `game.intro_mask`, `over_black`, `opening_black`, `start_warp_on`, `hold_starter`.
- pending, loaded, ready_at(centro del lote), ready_at(cámara efectiva), foco efectivo de world.update. Registrar los fallos de lectura de sectores que ya existan, no provocar nuevas lecturas.

Muestra del render final (después de **todas** las prioridades de cámara, con VP final disponible):

- cámara efectiva eye/look/up, FOV, near, viewport; fuente elegida (`sel_cam`/cine/title/warp/override/etc.) y los candidatos de cámara activos.
- para cada `driver="starter"`: uid/modelo, state.pos/rot y render_pose.pos/rot, finitud, deactivated/pinned/is_player/hide_player, LOD elegido, batches/índices disponibles y draws enviados, frustum y oclusor CPU.
- raycast de colisión cámara→centro de coche, si se necesita, con distancia del primer impacto. Es solo evidencia de colisión: no toda geometría visual tiene la misma malla física, y un hit no demuestra por sí solo un fragmento dibujado delante.

Comparar dos sesiones con el mismo ejecutable, configuración y perfil de prueba: película completa y salto temprano. Conservar log y una captura/vídeo corto que incluya los frames del fin de película y el primer segundo de selección; no borrar ni migrar partidas. Solo si quedan diferencias, repetir en el oficial v0.14.0 con el mismo directorio de datos y opciones. Una futura captura GPU puede identificar el draw/sector concreto, pero no es necesaria para demostrar o corregir la continuidad negra ya probada.
