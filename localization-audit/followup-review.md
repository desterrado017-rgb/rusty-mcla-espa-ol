# Revisión independiente de coleccionables y transición de apertura

> Errata posterior: el informe original acredita el marcador
> `<D>NUM_COLLECTABLE_COLLECTED</D>`. La variante con espacios de esta revisión
> histórica era incorrecta; se corrigió en `05c25f6`. Véase
> [el contrato actualizado](first-collectable-tutorial.md).


Base: `5c740ad2c0c29528193ecd16fc6dfa4ecb4e1348`.
Revisión de fuentes y diff, sin compilar ni ejecutar el juego y sin acceder a partidas ni recursos originales. Este revisor no escribe fuentes. Los resultados de ejecución de pruebas los coordina el agente principal.

## Coleccionables

`collectable_text::found_message` resuelve la misma clave de `StringTable` y en aquella base reemplazaba el token supuesto con espacios, posteriormente demostrado incorrecto. La corrección `05c25f6` utiliza `<D>NUM_COLLECTABLE_COLLECTED</D>`. El tratamiento por consumidor concuerda con `mc4b::fill_d` y las sustituciones nombradas de dinero y reputación existentes. No se ha supuesto un denominador ni una frase española original.

La rama con recurso deja de añadir el contador exterior; una plantilla sin la variable se mantiene intacta. Las variables ajenas y los marcadores incompletos se preservan, evitando tratar los tags como HTML para borrarlos. El respaldo sin recurso conserva el mensaje inglés anterior completo. La selección de idioma y los respaldos por clave pertenecen a la `StringTable` existente, no se reimplementan en el helper.

En `main.rs` solo cambian las dos líneas de presentación. No cambian las condiciones de recogida, el índice introducido, la longitud del contador, el total de barriles, efectos, sonido ni persistencia. El paso a mayúsculas ya existía. Las cuatro pruebas usan centinelas sintéticos explícitos; ninguna frase se atribuye a Rockstar. Cubren idiomas, cantidades, denominador inmutable, duplicación evitada, texto sin marcador y respaldos ausentes/vacíos.

## Película y traspaso

La extracción de `movie_ui` mantiene el orden y las condiciones de la rama anterior: toma la película, consume Enter/Y/Start/Back, solicita el fade de salto solo para la película que no usa `any_key`, presenta el subtítulo bajo la preferencia existente, vuelve a insertar una película activa y marca el fin de una terminada. La llamada sigue situada tras las mismas puertas de UI. `transition_masks_ui` reutiliza exactamente el dibujo de máscara anterior; no modifica geometría ni profundidad.

La nueva factoría de pruebas de `Video` solo existe bajo `cfg(test)`. Usa memoria anónima, un track MCV sintético y tiempos deterministas, sin JPEG, ficheros originales ni sleeps. `Playback::Complete` y `Playback::Skipped` toman ramas de terminación diferentes del `Video::show` real. Un track activo dura1000s, evitando carreras con el reloj de la prueba. `skip_requested` solo observa si se solicitó el salto; no adelanta fade ni tiempo.

El defecto de cobertura negra identificado existe en el código oficial comparado y en la base: al terminar el vídeo, `show` retorna antes de pintar su fondo y la UI retorna antes de pintar la máscara global. `game.intro_mask` estaba publicado como0 mientras la película cubría el mundo. Puede quedar más de un render sin tick fijo. La corrección debe conservar el negro solo para el contexto de apertura y dejar al siguiente `opening_tick` el control del fade normal. Este hallazgo no prueba qué sector concreto ocultó un coche en la sesión del usuario.

## Diagnósticos

`opening_diag` es optativo mediante `MCLA_DIAG_OPENING=1`, usa el log existente y no escribe recursos ni partidas. Estado y muestreo tienen mutex de alcance corto, liberados antes de emitir logs; no hay adquisición anidada detectada. Los campos de render se observan desde la cámara/VP finales después de las prioridades de vista. `ready_at` y `pending` se registran por separado. Las poses y LODs de los starters se leen sin modificarlos.

El límite de muestreo de render/foco es por cambio de stage o cada60frames; stage0 reinicia el muestreo y no produce esas muestras. Los eventos de petición de salto pueden repetirse si llegan inputs repetidos durante el fade corto; es ruido potencial de diagnóstico, no un cambio de flujo.

No se alteran clipping, cámara, comparación/escritura de profundidad, orden de pases, streaming, física, IA ni la creación del rival. No se debe presentar el diagnóstico como validación de la geometría original ni garantizar que todo el fallo visual haya quedado resuelto sin revisión Windows.

## Preservación

El catálogo de120 entradas y los módulos de navegación de Henry no forman parte de este diff. Las opciones de subtítulos mantienen `false` por defecto. Los documentos aclaran que respetar la preferencia no demuestra disponibilidad de subtítulos españoles: faltan los CSV originales.

## Corrección de continuidad revisada tras RED

El agente principal confirmó el RED real: los dos casos de fin natural y salto no encontraban cobertura negra; los dos negativos previos pasaban. Tras esa comprobación se aplicó y congeló el cambio pequeño en el final de `movie_ui`.

La máscara solo se restaura cuando concurren `fresh_game`, `hold_starter`, `Mode::FreeRoam` y `!movie.any_key`, después de que el `Video::show` real haya terminado. Se publica `intro_mask=1` y se dibuja el mismo helper existente de máscara. El `CutMask` interno de `App` permanece intacto y el siguiente tick mantiene la decisión de streaming y desvanecimiento; el valor publicado también conserva negro si hay varios renders sin tick fijo.

Los casos negativos cubren carreras existentes, UI no retenida por apertura, modo MainMenu y vídeos de atracción `any_key`, incluso en FreeRoam. No se cambia un vídeo activo ni un frame sin vídeo. El test adicional conserva las reglas de Enter/Y/Start/Esc para pedir el fade de la película y el final por Enter de la atracción. Las preferencias de subtítulos y el texto siguen pasando por la rama anterior idéntica.

## Estado de cierre

No se ha identificado otro defecto de producción en el diff final revisado. Se revisaron `collectable_text.rs`, los consumidores y observaciones de `main.rs`, la extracción y corrección pequeña de `game.rs`, la factoría y getter de `video.rs`, y `opening_diag.rs`. `git diff --check 5c740ad` pasa en esta revisión.

La ejecución GREEN y la validación visual corresponden al agente principal y al usuario, respectivamente. Esta revisión estática no las certifica. La corrección demostrada es continuidad de cobertura; queda pendiente contrastar las capturas Windows para concluir si explica toda la incidencia gráfica reportada.
