# Transición de película inicial a selección de vehículos: diagnóstico

Base investigada: `5c740ad2c0c29528193ecd16fc6dfa4ecb4e1348`, rama `feature/spanish-ui-completion-v0.14.0`. Comparación de código con Rusty MCLA oficial v0.14.0, `30d8b1f3b54557664d81bb7cfc9cc33caea53647`. La investigación inicial es estática: no se han ejecutado datos originales ni reproducido una captura del defecto visual del usuario. Después de identificar el retorno defectuoso se prepararon una extracción del bloque de UI y pruebas headless. El agente principal confirmó RED: las regresiones de EOF y salto fallaron por ausencia de negro, mientras los dos controles pasaron. Solo entonces se aplicó la corrección mínima de handoff; el agente principal confirmó GREEN después: las cinco pruebas finales pasan. Los resultados completos están en [followup-validation.md](followup-validation.md).

## Conclusiones y grado de demostración

1. **Demostrado por el flujo de código: hay un hueco sin película ni máscara negra al finalizar la película.** Ocurre al saltarla y también al llegar a EOF. `Game::ui_screens` retorna desde el bloque de película después de que `Video::show` devuelve `false`, sin dibujar la máscara de transición. El tick anterior había publicado `game.intro_mask = 0` precisamente porque la película aún existía.
2. Ese hueco deja ver el mundo ya renderizado, antes de que el siguiente tick reanude la máscara retenida y decida liberar la selección según el streaming. Si la película se salta pronto, el mundo dispone de menos tiempo de carga. Es una explicación técnicamente compatible con una exposición fugaz de geometría, **pero no demuestra qué geometría tapó qué coche en la sesión reportada**.
3. **No se ha demostrado un defecto de profundidad, de geometría, de posición de cámara o de visibilidad como causa de la oclusión reportada.** No corresponde cambiar near plane, comparar profundidad, desactivar escenario/culling ni mover coches/cámaras por suposiciones.
4. El núcleo de esta transición y su cámara ya son idénticos al oficial v0.14.0. Por ello el hueco no se puede atribuir a la traducción española ni a la fase de interfaz.

El usuario ha precisado que el efecto aparece solo durante la transición, no persiste en la selección, y utiliza INTRO NORMAL con la película completa disponible. Esa precisión es coherente con el hueco detectado; sigue sin equivaler a una reproducción visual aquí.

## Recorrido exacto

Las referencias de línea se refieren al `5c740ad` de partida; los cambios posteriores pueden desplazarlas.

- `main.rs:5814–5823`: cuando la partida es nueva, las cajas terminan y el vuelo de inicio se mantiene, empieza `intro_stage = 1`, se reinicia `intro_t` y se inicia una máscara de un segundo.
- `main.rs:4592–4622`: bajo negro se crean y posan los tres starters en las coordenadas del original, quedan desactivados y fijados. La creación termina antes de que la película pueda arrancar.
- `main.rs:13011–13034`: con máscara completa y starters hechos se libera el vuelo retenido; se inicia la película normal, o la ruta opcional FAST, se entra a etapa 2, se reinicia `intro_t`, se inicializa `sel_cam` en `SEL_EYE0`/`SEL_LOOK0` y se solicita la hora 17.
- `main.rs:13079–13080`: mientras hay película o la escena de llegada FAST, el valor visible `game.intro_mask` es 0; la `CutMask` interna se conserva completamente negra y no avanza en ese camino. La película rellena de negro el fondo de su imagen mientras `Video::show` devuelve `true` (`video.rs:97–122`).
- `game.rs:4735–4765`: la UI toma temporalmente `self.movie`. Un botón válido solicita `Video::skip`. Si la película continúa se repone en `self.movie`; al finalizar, se marca `movie_done` y se retorna inmediatamente. En esa rama no se repone la película y tampoco se pinta negro.
- `video.rs:97–103`: `Video::show` devuelve `false` **antes de crear el painter y pintar su rectángulo negro** cuando el índice de imagen ya terminó o transcurrieron los once pasos del salto.
- `game.rs:10030–10035`: el pintado global de máscaras está después y nunca se alcanza en el retorno del bloque de película.
- Las ramas anteriores `start_warp_on` y `opening_black` no cubren el EOF normal de etapa 2: el vuelo se retiró al iniciar la película, y `opening_black` se activa en etapa 4 o al aceptar la selección en etapa 3 (`main.rs:13081`). La segunda revisión de renderizado confirmó que no hay otra capa negra en ese recorrido.
- El mundo de ese frame ya fue renderizado antes de la UI, con la cámara de selección que existe desde etapa 2 (`main.rs:8312–8330`). Por tanto el retorno sin imagen ni negro expone ese mundo.
- En el siguiente tick, etapa 2 solo se libera si no hay película ni escena, `intro_t > 3/60` y `streaming == 0` (`main.rs:13038–13041`). Hasta que se cumpla, la máscara interna sigue retenida y vuelve a publicarse opaca. Cuando se cumple, empieza su fade up de un segundo.

El juego usa ticks fijos de 60 Hz y render puede ejecutarse sin tick (`main.rs:8060–8076`). El hueco puede durar más de un frame de UI si hay renders consecutivos sin un tick entre ellos.

## Saltar frente a reproducir completa

| Camino | Condición de `Video::show` para terminar | Siguiente paso de UI | Tiempo de carga disponible |
|---|---|---|---|
| Película completa | `i >= frames` | `movie_done = true`, `return` | Toda la duración del vídeo |
| Película saltada | Los once pasos de su fade terminaron | Exactamente la misma rama | Desde el inicio hasta el salto y su fade |
| Película FAST no disponible | No arranca película; camino etapa 2 normal | No entra al retorno defectuoso de vídeo | Dependiente de la escena/ruta alternativa |

La oclusión persistente durante la selección, si existe tras terminar el fade y asentarse la carga, no queda explicada por un hueco fugaz de vídeo: se necesita una captura y las medidas del frame concreto.

## Cámara, visibilidad y profundidad

- `sel_cam` empieza en un valor determinista al entrar a etapa 2. Su interpolación hacia el coche seleccionado solo se ejecuta en etapa 3, cada 30 ms (`main.rs:13000–13006`). Saltar la película no tiene una función de cámara alternativa.
- `SEL_EYE0`, `SEL_LOOK0`, los tres targets y `sel_cam_step` son idénticos al oficial. No se ha cambiado su lente 25,5°.
- La vista de selección reemplaza las vistas de replay y se aplica antes de la lógica normal de cámara. `world`, `camera` y `boomcam` completos son idénticos a los archivos oficiales v0.14.0.
- El análisis independiente de renderizado confirmó el borrado de profundidad 0 y las comparaciones GreaterEqual de reverse-Z, y ausencia de una rama de profundidad distinta para el salto. La falta de una captura no permite identificar un material, batch o LOD concreto responsable de ocluir un coche.
- `hide_player` oculta el coche del jugador mientras se escoge entre los starters; no oculta por esa condición a los tres starters. Cambiarla no está justificado por la descripción.

## Dos límites del streaming que deben diagnosticarse

- `World::streaming()` (`world.rs:1348–1350`) devuelve solo el número de solicitudes pendientes, no una afirmación de residencia de todos los sectores de la cámara o el lote. `World::ready_at(p)` es una consulta distinta (`world.rs:1335–1337`). No se ha demostrado que el frame reportado libere la máscara sin los sectores necesarios; esa diferencia por sí sola no demuestra un fallo.
- `intro_t` se reinicia al **empezar** la etapa 2/película, no al acabar. La condición documentada como tres frames después de la película puede cumplirse en el primer tick posterior a un vídeo largo o a un salto que dure más de 50 ms. Es una discrepancia estática entre comentario y temporizador, ya presente en v0.14.0; no debe cambiarse sin verificar el protocolo original y medir su relación con el defecto visual.
- El centro de streaming del tick (`main.rs:2834–2853`) es el jugador, sobreescrito por la cámara cinematográfica o de título; no incorpora expresamente `sel_cam`. La cercanía real entre centros y sectores del lote necesita recursos. Tampoco es prueba suficiente para cambiar ese centro.

## Regresión propuesta sin recursos ni GPU

Extraer el bloque existente de vídeo de `Game::ui_screens` a un método pequeño que devuelva si la película consumió la UI de ese frame. El método no necesita `World` ni `Vehicles`; así puede ejecutarse el flujo real de presentación en un `egui::Context` sin GPU.

La factoría de prueba `video::test_support` construye una pista MCV1 puramente sintética de 1×1 y una imagen en memoria anónima: cabecera `MCV1`, width/height 1, fps 1 (0,001 FPS), frames 1, offsets 36/36, longitud 36. No abre archivos ni conserva datos del juego. Para EOF establece offset 2000 s; `show` retorna antes de decodificar JPEG. Para el salto establece `skipped` a un `Instant` anterior en el constructor `cfg(test)`; no se necesita dormir ni modificar el reloj del sistema. El tercer estado mantiene una película viva para comprobar que no se cubre ni finaliza antes de tiempo.

Se han ejecutado cinco pruebas `game::tests::opening_movie_*` que ejecutan en `Context::run` con pantalla 1280×720, película inicial no-attract, partida nueva y `hold_starter = true`. Las dos regresiones de EOF/salto comprueban que después de finalizar el vídeo la película falta, `movie_done` está activo **y existe un rectángulo negro opaco que cubre toda la pantalla**. El agente principal confirmó el fallo RED sobre el comportamiento extraído de `5c740ad` antes de aplicar la corrección: `opening_movie_eof_keeps_black_until_the_transition_tick` y `opening_movie_skip_keeps_black_until_the_transition_tick` fallaron en la aserción del rectángulo negro; los otros dos casos de control pasaron. Los casos cubren ambos caminos (salto y EOF), múltiples UI frames sin tick, liberación posterior de la máscara y controles negativos de attract/carrera existente/vídeo vivo para preservar su comportamiento.

La corrección aplicada se limita al handoff de la película inicial con selección retenida: `fresh_game && hold_starter && mode == FreeRoam && !any_key`. Cuando el vídeo termina, publica `intro_mask = 1.0` y pinta mediante el mismo helper de máscaras que usa la UI normal. La `CutMask` interna ya está completa en ese punto; mantener el valor visible de máscara en 1 hasta el siguiente tick conserva esa misma máscara, sin tocar cámaras, profundidad, geometría, streaming, guardados ni preferencias. El siguiente tick sigue decidiendo si espera o libera el fade normal. Se presenta como continuidad de transición demostrada, no como oclusión gráfica real ya validada.

La quinta prueba conserva las reglas de entrada extraídas: A/Enter, Y, Start y Back/Esc solicitan el fade de la película inicial y consumen los eventos, sin forzar su terminación inmediata. Enter sigue terminando el vídeo de atracción sin añadirle el nuevo negro. Hay controles independientes de `any_key = true` en FreeRoam para que esa exclusión no dependa únicamente del modo MainMenu.

## Validación Windows que aún hace falta

Registrar dos sesiones con el mismo ejecutable, configuración y perfil de prueba: película completa y salto temprano. Conservar los originales y no tocar la partida habitual. Recoger:

- Frame de fin de película, motivo EOF/salto, duración/tiempo, etapa, máscara interna y visible.
- Centro de streaming, pending, `ready_at` de cámara/lote, y stage 2→3.
- Cámara efectiva (eye/look/up/lente/near), coches starter/poses/LOD/visibilidad y origen de cámara.
- Captura de los primeros frames tras vídeo y después de asentarse la selección.

Si el defecto permanece durante la selección ya estable, las capturas permitirán distinguir geometría legítima en primer plano, modelo lejano temporal, cámara bloqueada o un fallo de profundidad. No se ha reproducido esa distinción en este entorno.

## Comparación verificable con oficial

Extracciones de código idénticas byte a byte a `30d8b1f3...`:

- `opening_tick`: SHA-256 `bbde223a5f65f1c1d620d1ebcf844cba517909ed8eb76bc9b0bfecdeb1fa51ce`.
- `sel_cam_step`: SHA-256 `de5ebd88293ece73bf1535f447c2ef508d53d03fd75c0c2b99d450d896870caa`.
- Constantes de cámara de selección: SHA-256 `9b8409e9c5d53efd9ea9c37eea2a92e1418fcba4371bc8197747c26711189e4e`.
- `world.rs`, `camera.rs`, `boomcam.rs`: archivos completos idénticos.

La diferencia de `video.rs` de fase 1 solo añade el control de preferencia de subtítulos y sus pruebas; no modifica `show`, `skip`, cámaras ni tiempos. El bloque de UI de fin de película ya tenía el retorno temprano en la base oficial. Estas son comparaciones de código, no una reproducción del binario oficial con recursos reales.
