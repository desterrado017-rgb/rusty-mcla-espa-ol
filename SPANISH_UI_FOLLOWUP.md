# Seguimiento: coleccionables y transición de apertura

Base exacta: `5c740ad2c0c29528193ecd16fc6dfa4ecb4e1348`, rama
`feature/spanish-ui-completion-v0.14.0`. Este informe describe el conjunto de
diagnóstico y correcciones posteriores, separados en el historial. Las 120
entradas bilingües, la navegación de Henry, preferencias, recursos y partidas
se conservan. No se fusiona con estable ni se publica un ejecutable Windows.

## Coleccionables: causa y corrección

La ruta de recogida copiaba `COLLECTABLE_FOUND_MSG` sin expandir su variable y
añadía siempre otro contador. Con el marcador reportado por el usuario, el
resultado conservaba el marcador dinámico literal y un sufijo
de progreso adicional. Ese código ya existe en v0.14.0 oficial; no procede del
catálogo español.

El formato dinámico observado en el port es `<D>nombre</D>`: otros consumidores
ya rellenan las variables de tiempos o de dinero. `StringTable` selecciona y
devuelve el texto, sin conocer el estado que debe sustituir el consumidor.
El nuevo `collectable_text::found_message` sustituye únicamente la variable
nombrada por el número recogido y conserva el resto de la plantilla oficial,
incluido cualquier denominador, puntuación o marcador ajeno. No añade otro
contador cuando hay recurso. Si no existe la clave, conserva el fallback inglés
previo del port. La selección española y su respaldo inglés por clave no cambian.

No se han inspeccionado las tablas reales. El marcador está confirmado por el
reporte; la frase completa y su denominador original no se inventan. Los textos
de prueba son centinelas, no traducciones atribuidas a Rockstar.

Detalles y evidencia: [collectables.md](localization-audit/collectables.md).

El informe original aportado posteriormente corrigió el token a
`<D>NUM_COLLECTABLE_COLLECTED</D>` y acreditó `xTCO` y el formulario 94.
El marcador auténtico está corregido desde `05c25f6`. La variante con espacios
del diagnóstico inicial era incorrecta.

El backend de `COLLECTABLE-FIRST-TUTORIAL` está preparado y probado por separado;
el usuario confirma ahora que se cierra automáticamente sin pulsar un botón.
El panel no está activado en el pickup: faltan duración precisa y política de
pausa. El [análisis de autocierre](localization-audit/form94-autoclose-duration.md)
documenta esos dos datos pendientes. Estado y alcance en
[PENDING_XBOX360_FIDELITY.md](PENDING_XBOX360_FIDELITY.md) y
[contrato original](localization-audit/first-collectable-tutorial.md).

## Película inicial: discontinuidad negra demostrada

El usuario precisó que ocurre con **INTRO NORMAL**, solo durante la transición,
y no persiste en la selección de vehículos. Esa observación coincide con el
defecto de traspaso encontrado en código:

1. Mientras existe la película, la máscara pública está en cero: el vídeo cubre
   el mundo con su propio fondo.
2. Al terminar naturalmente o finalizar el desvanecimiento del salto,
   `Video::show` devuelve `false` antes de dibujar ese fondo.
3. La UI retira la película, marca su final y retorna antes de la máscara global.
   El mundo queda descubierto antes del siguiente tick de la transición.
4. Puede haber varios renders entre ticks de simulación; no basta confiar en
   que el próximo tick restituirá el negro.

La ruta original de v0.14.0 tiene el mismo retorno. Cámara, profundidad,
streaming y visibilidad examinados no muestran una regresión introducida por
la localización. Una película completa y una saltada convergen en el mismo
final, pero ofrecen tiempos de carga distintos.

La corrección queda limitada al traspaso de una película de nueva partida,
en modo de conducción libre y con selección todavía retenida. Mantiene el
negro que ya estaba retenido, lo pinta en el fotograma de finalización y lo
conserva en la máscara existente hasta que `opening_tick` vuelva a publicar su
desvanecimiento normal. No cambia la cámara, profundidad, geometría, oclusores,
streaming, colocación de vehículos ni condiciones de liberación.

Esto corrige una exposición prematura demostrable de la escena. **No sustituye
la confirmación visual en Windows del artefacto concreto reportado**. Sin esa
captura y los recursos reales no se identifica ningún sector/material culpable
ni se asegura la ausencia de otros problemas gráficos.

Detalles: [opening-transition.md](localization-audit/opening-transition.md) y
[selection-rendering.md](localization-audit/selection-rendering.md).
Los diagnósticos [opening-diagnostics.md](localization-audit/opening-diagnostics.md)
se activan solo con `MCLA_DIAG_OPENING=1`, reutilizan el registro existente y no
modifican decisiones ni archivos del juego. Permiten comparar película completa
y salto temprano mediante estado, cámara efectiva, streaming y starters.

## Subtítulos: alcance explícito

El valor predeterminado sigue siendo **`false`** tanto en `game::Options` como
en `mc4::Options`. La corrección de la fase anterior solo respeta la preferencia
que ya se ha cargado; no activa subtítulos ni cambia su persistencia.

**No se afirma que estén disponibles los subtítulos españoles.** El cargador
lee una pista literal de `subtitle.csv`, sin seleccionar idioma por StringTable.
Siguen pendientes los CSV originales de `cut_intro` y
`CUT_STORY_RACE_GH_06`, la tabla UI y el inventario de instalación descritos en
[subtitles.md](localization-audit/subtitles.md). No se modifican esos archivos.

## Verificación y estado de entrega

La verificación automatizada y sus límites se detallan en
[followup-validation.md](localization-audit/followup-validation.md).
Incluye cantidades e idiomas, final por salto/EOF, renders sin tick intermedio,
controles negativos y regresiones de la localización y de Henry.

La validación visual pendiente debe comparar película completa y salto temprano
en Windows con el mismo ejecutable, configuración y perfil de prueba aislado.
Se conservan la partida y los recursos habituales. Esta entrega no genera ese
ejecutable ni cambia Rust 1.99.0, WinLibs o las dependencias bloqueadas.
