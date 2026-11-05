# Diagnósticos optativos de la transición inicial

Base: `5c740ad2c0c29528193ecd16fc6dfa4ecb4e1348`. Se conserva el mecanismo de logging existente; `opening_diag.rs` no abre ni guarda recursos/partidas, no modifica cámara, profundidad, streaming, visibilidad ni temporizadores. No se compila ni publica un ejecutable Windows como parte de este cambio.

## Activación

En una futura compilación de prueba que incluya estos cambios, añadir en la misma ventana de PowerShell antes del arranque de prueba:

```powershell
$env:MCLA_DIAG_OPENING = '1'
```

Usar el perfil de prueba aislado existente y los mismos datos/configuración para ambas sesiones. No borrar ni migrar partidas. La bandera solo activa registros; no corrige la escena ni cambia el idioma. Para desactivarla, retirarla del entorno antes del siguiente arranque.

## Comparación

Comparar una sesión INTRO NORMAL con película completa y otra saltándola temprano, desde el mismo punto de una partida de prueba nueva. Conservar los logs existentes de ambas y, si sigue habiendo coches ocultos, una captura/vídeo corto que incluya la salida de la película y el primer segundo de selección. El usuario ha concretado que el fallo es temporal durante la transición y ocurre con INTRO NORMAL: eso concuerda con la discontinuidad negra demostrada, sin identificar aún ningún draw/sector responsable de una oclusión por píxel.

## Campos

Todas las líneas empiezan por `opening-diag event=`:

- `state`: cambios discretos de etapa/condiciones (incluida cola vacía y selección), versión/SHA de compilación si existe, frame/tick, etapa previa y actual, `intro_t`, dt, película y tiempo/duración observados, cola/cine, pendientes, máscara interna full/on, máscara cacheada para UI, hold/fresh/starter y warp. El tiempo no se usa como clave de repetición.
- `movie-skip-request` y `movie-end`: emitidos por la ruta real de UI de película; permiten distinguir fin natural, petición de salto y su estado de cobertura. La razón se observa, sin imponerla ni adelantar la reproducción.

En `movie-end`, el campo `mask` captura el valor anterior a restituir el negro
del traspaso. Un cero en esa línea no demuestra que el fotograma corregido
haya quedado descubierto; la máscara se conserva y pinta inmediatamente después.
- `stream-focus`: foco realmente usado en `world.update`, pendientes/residentes y `ready_at(focus)`. Se observa después de la llamada existente; no solicita otras cargas.
- `render-view`: ojo/dirección de la vista final usada para escoger LOD, FOV, near de proyección y LOD, viewport, finitud del VP, candidato `sel_cam`, presencia de cine, pendientes/residentes, `ready_at(eye)`, número de starters e `hide_player`.
- `starter`: UID/modelo, posición de estado y pose renderizada, rotación, finitud, pinned/deactivated/is_player, `ready_at(car)`, LOD/batches disponibles y pruebas de esfera contra frustum y oclusores CPU.

El ojo/dirección final se obtiene después de todas las prioridades de cámara y del VP efectivo. `selection_candidate` es solo el candidato de selección; no se declara que haya ganado la prioridad. Las muestras de streaming/render se producen al entrar/cambiar etapa y cada 60 frames, con cadencias independientes; no se emiten en etapa 0. Las transiciones discretas se registran al cambiar, no cada tick. Una prueba pura comprueba la cadencia y el reinicio al salir.

## Límites

`streaming()==0` solo significa cero pendientes, mientras `ready_at` comprueba residencia cercana; ambos se registran sin convertirlos en nuevas condiciones de liberación. `intro_t` sigue contando desde el inicio de la película y no se añade una espera arbitraria.

`drawable_batches` son lotes disponibles según el LOD seleccionado, no un contador de draw calls GPU completados ni una prueba de que sus píxeles sean visibles. Las pruebas de esfera frustum/oclusor describen el criterio CPU; los modelos sin LOD propio pueden usar otro recorrido de dibujo. La profundidad puede ocultar parcialmente un coche aunque esas pruebas sean favorables. Los registros no identifican por sí solos un sector o material culpable, ni sustituyen una captura con recursos reales. No se han desactivado oclusores, cambiado cámaras o dibujado coches por encima del escenario.
