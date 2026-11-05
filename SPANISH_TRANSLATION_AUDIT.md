# Auditoría de localización española — Fase 1

Rama de trabajo: `feature/spanish-ui-completion-v0.14.0`.

Base de código y referencias de línea: `b68c4d02d384f611d176170bb893053a697503af`, validada previamente por el usuario en Windows: Henry aparece al iniciar directamente sin `--data` y la primera carrera se puede completar. Esta fase conserva esa corrección y no modifica la resolución de datos, creación del rival, misiones, dependencias, lockfiles, recursos del juego ni partidas.

**Estado:** auditoría estática, integración de textos propios y pruebas focalizadas completadas. No se ha demostrado una localización completa del contenido original de Rockstar: faltan los recursos para verificar sus textos y los subtítulos. No se publica un ejecutable ni se modifica la rama estable.

## Fuentes y entregables

- [`SPANISH_TRANSLATION_INVENTORY.tsv`](localization-audit/inventory.tsv): inventario semántico consolidado, con texto inglés o familia dinámica, ruta y línea de la base, situación, origen, categoría, clave oficial cuando se demuestra en el código, propuesta manual solo para C, evidencia, límites y estado de implementación.
- [`spanish-literals.tsv.gz`](localization-audit/spanish-literals.tsv.gz): inventario bruto de literales en la base exacta, comprimido. Preserva literales técnicos, tests y logs para evitar confundirlos con UI.
- [`spanish-literal-coverage.tsv`](localization-audit/spanish-literal-coverage.tsv): denominador y cobertura léxica por archivo.
- `tools/audit_spanish_strings.py`: genera el inventario bruto y su cobertura desde una revisión Git, sin ejecutar el juego ni acceder a partidas.
- [`port-catalog.tsv`](localization-audit/port-catalog.tsv): relación clave/origen/inglés/español y alias del catálogo propio final.
- [`subtitles.md`](localization-audit/subtitles.md): recorrido detallado de CSV, escenas, opción de subtítulos y recursos concretos necesarios.
- [`layout.md`](localization-audit/layout.md): revisión de glifos, anchuras, límites de los campos y recursos requeridos para validación visual.
- [`other-ui.md`](localization-audit/other-ui.md): detalle de Career, HUD, garage, mapas, teclado, modos de banderas y límites del análisis.
- [`AUDIT_METRICS.json`](localization-audit/AUDIT_METRICS.json): conteos y fecha de la instantánea. Cada registro conserva la procedencia de la auditoría independiente que lo aportó.
- [`validation.md`](localization-audit/validation.md): comandos, resultados, alcance y límites de las pruebas.
- [`code-review.md`](localization-audit/code-review.md): revisión independiente del diff final.

Los archivos de auditoría se entregan bajo `localization-audit/`, fuera de la ruta `docs/` excluida por el repositorio.

## Cobertura comprobada frente a cobertura desconocida

El inventario bruto cubre **195 archivos**: todos los 192 `.rs` de `game/src` de la base, más `config/src/keybinds.rs`, `formats/src/strtbl.rs` y `flash/src/player.rs`. Se han inventariado **27.445 literales** de código; los comentarios quedan fuera. El extractor independiente de módulos adicionales contó 26.833 en `game/src`, más 413/146/53 en las tres dependencias: coincide con ese total.

Esto demuestra cobertura **léxica** del ámbito indicado. No demuestra 27.445 textos visibles ni todas las cadenas del juego: incluye rutas, identificadores, tests, código de shader, números, errores y logs. Los contenidos de StringTable, películas Flash originales, CSV de subtítulos y nombres aportados por mods o guardados no están en el repositorio y no forman parte de ese denominador.

La revisión semántica reúne **1.801 registros**: 1.344 de Game/MC4/Main, 36 de Hooks, 28 de Flash/localización, 385 de los módulos adicionales y 8 del recorrido de subtítulos. Un registro puede representar una familia dinámica y distintos auditores pueden documentar el mismo candidato. Por tanto **no es un recuento de cadenas únicas** y no sirve para calcular un porcentaje de traducción del juego.

| Categoría | Registros de esta instantánea | Qué está comprobado |
|---|---:|---|
| A | 885 | Ruta de selección de recursos originales y claves consumidas. El contenido castellano y la fidelidad requieren los recursos reales. |
| B | 401 | Bypass inglés, reserva inglesa, defecto de selección o limitación de arquitectura documentada. Cada fila distingue defecto demostrado de posibilidad pendiente de recurso/ejecución. |
| C | 136 | Texto propio del port o su adaptador de presentación, con propuesta manual identificada. Las candidaturas sujetas a revisión conservan su calificador. |
| D | 379 | Identificadores, claves, tokens, controles físicos, logs, depuración o nombres técnicos que deben conservarse. |

El catálogo propio contiene **120 entradas**, todas con inglés y español no vacíos: **120/120**. Incluye 40 etiquetas de teclas y cuatro entradas con alias descriptivos de ayudas. Se han comprobado consumidores directos o adaptadores para los **136/136 registros C auditados (100 % de esos registros)**, sin registros C pendientes de integración en este inventario. El denominador incluye registros/familias repetidos entre auditores: **no es un porcentaje de traducción del juego ni de todas las cadenas únicas del port**. La verificación visual con películas/fuentes originales y Windows sigue pendiente; no hay validación visual nueva que permita certificar ajuste, ausencia de recortes o traducción oficial de recursos no disponibles.

La cobertura visual nueva es **0 % del catálogo de esta fase**: ninguna de sus
120 entradas se ha validado aquí dentro del juego. No se ofrece una estimación
porcentual global: faltan el inventario de cadenas de los recursos originales y
sus valores españoles. Las estimaciones disponibles son de longitud de ayudas,
no de fidelidad ni de cobertura de traducción.

## Arquitectura y regla de origen

`game.language` selecciona español o inglés. `localization::load` comparte la selección entre Boot y Career: carga `ui/0xa4c1e8fa`, combina tablas UI suplementarias válidas en orden estable, excluye la tabla señalada por el proyecto y selecciona el bloque de idioma al final. `StringTable` conserva las cadenas originales y su respaldo inglés por clave cuando falta/vacía la entrada elegida; no se añaden traducciones españolas manuales a esas tablas.

El catálogo `port_text` se reserva para controles, opciones y mensajes escritos por Rusty MCLA. Cada entrada incluye origen; las palabras de configuración (`normal`, `fast`, `windowed`, etc.), claves, códigos de acción y valores serializados conservan su contrato. Las etiquetas inglesas del port continúan disponibles.

Las filas A describen una ruta original conectada, **no certifican bytes españoles no disponibles**. Las filas B no atribuyen el fallo a la localización cuando solo se ha demostrado una reserva de UI, y no reciben propuestas manuales de texto Rockstar. Los IDs convertidos en texto y los nombres derivados de mallas se registran expresamente como reservas pendientes.

## Cadenas oficiales: rutas ya resueltas

| Recorrido | Resolución comprobada en código y pruebas |
|---|---|
| Boot y Career | Cargador común, combinación de tablas antes de seleccionar español y respaldo inglés por clave. |
| Flash | `Getvar/lang` devuelve la selección efectiva; `Getstring` lee la misma tabla. |
| Dinero de Hangout | Se reutiliza `G_Money`, ya empleado por otras pantallas para dinero. |
| Controles originales | Las acciones originales usan sus claves; GPS y garaje usan `Navmap_GPSMap` y `pause_go_to_garage`. |
| Opción de subtítulos | Se mantiene `option_subtitles` y el bool existente, sin crear una traducción oficial alternativa. |

Son conexiones de recursos verificadas. Ninguna certifica el contenido español
real de una tabla ausente. Los textos del original quedan fuera del catálogo manual.

## Cambios de interfaz propios del port

Se han separado del texto original las páginas DISPLAY/PERFORMANCE y sus opciones añadidas: ventana, monitor, resolución, VSYNC, límite/contador FPS, escala de renderizado, filtrado, antialiasing, interruptores de efectos, multiplicadores de sombras/distancia/detalle, retrovisor, modo de inicio y zona muerta. El valor de detalle que habilita tráfico de mayor detalle mantiene sus índices y comportamiento.

También se han conectado al catálogo las ayudas y mensajes propios revisados: capturas F12, errores de inicialización gráfica, selector/editor de teclado, estado de mods ausentes, ayudas PC y mensajes de vehículo/editor que estén acreditados en el inventario. La traducción cambia presentación, no lógica ni valores persistidos.

El caso del teclado exige distinguir **página nueva** de **función original**. `ActDef.shared=false` no demuestra origen exclusivo: CAM_DOWN, MUSIC_PAUSE, PAUSE y cuatro WHEEL ya figuran en `CONTROL_ACTIONS` del original. GPS y garaje también tienen claves originales (`Navmap_GPSMap`, `pause_go_to_garage`). Esas nueve etiquetas usan lookup original; no se traducen a mano. Seis acciones adicionales acreditadas del port usan el catálogo. Los `KeyDef.name` se guardan también en partidas y siguen intactos. Los nombres descriptivos se localizan mediante `port_text::key_label` únicamente al mostrar la columna del editor, las ayudas y el indicador GPS; el token interno no cambia. Las pruebas nuevas recorren los 98 KeyDefs, preservan códigos/nombres, comprueban inglés exacto y el roundtrip JSON de bindings para las teclas asignables. El resultado de ejecución de esas pruebas debe constar en la sección de verificación final.

Los identificadores de teclas, nombres físicos de botones del mando, códigos de controles, APIs y unidades se preservan. Las etiquetas descriptivas propias del port se adaptan al mostrar: por ejemplo SPACE→ESPACIO, MOUSE→RATÓN y ARROWS→FLECHAS. ENTER, ESC, CTRL, ALT, NUM, letras, números y teclas de función se conservan como tokens técnicos donde corresponde. No se renombra un identificador global por aparecer en una ayuda.

## Defectos demostrados y reservas pendientes

- **Idioma Flash:** el host genérico respondía `lang=0` aunque el cargador hubiera seleccionado español. Una película sintética que ejecuta la llamada muestra el defecto; la corrección conserva el idioma suministrado y la ruta de juego responde desde la StringTable seleccionada. No se reescribe el contenido de las películas.
- **Etiqueta de dinero:** una ruta de Hangout usaba `MONEY` directamente; ahora reutiliza `G_Money` de la tabla original, conservando el override diagnóstico y el fallback exacto si falta la clave.
- **Preferencia de subtítulos en intro:** la película inicial mostraba la pista sin consultar la opción guardada. El control de presentación ahora recibe esa preferencia. El predeterminado, la lectura/escritura de partidas y el texto/tiempo de los CSV permanecen intactos.

El predeterminado de subtítulos sigue siendo **`false`** en las dos estructuras
de opciones. La corrección solo respeta la preferencia existente. **No demuestra
disponibilidad de subtítulos españoles**: los CSV originales no se han inspeccionado.

- **Reservas de interfaces originales:** resúmenes de Career, `RaceDef::title`, categorías/variantes de piezas, tipos de pintura y zonas de vinilos contienen inglés ensamblado. La auditoría demuestra sus consumidores y si solo se ven al fallar/no usar la película original. No se han traducido manualmente: requieren identificar las claves originales pertinentes.
- **Banderas offline:** la IA offline es una adición del port, pero los modos y objetivos proceden del multijugador original. Esa adición no demuestra que todo el texto sea nuevo. Sus frases quedan pendientes de contrastar con recursos.
- **Texto usado como estado:** banderas prioriza `GET`/`STOP`; un sonido detecta ganador con `YOU`. Traducir esas cadenas directamente puede cambiar lógica. Debe separarse el estado de la presentación antes de intervenir; esta entrega no altera esa lógica.

Ninguno de estos hallazgos vuelve a atribuir la aparición de Henry al idioma. La condición demostrada y corregida de aquella incidencia era la resolución de la ruta de datos al iniciar sin argumentos.

## Subtítulos: conclusión y archivos necesarios

`Cutscenes::load` lee `tune/cutscene/Story/<CUT_*>/subtitle.csv`, interpreta bytes Latin-1 y toma literalmente la segunda columna; primera/última columnas participan en los intervalos. Se han inspeccionado CUT_INTRO, escenas de historia, reproducción y opción/persistencia. El lector **no selecciona una columna/archivo por `game.language` ni resuelve el diálogo mediante StringTable**. Esta limitación está demostrada; el idioma real de los CSV del usuario no se conoce. No se inventa una columna española, una clave o un fichero alternativo.

Para continuar hacen falta copias de lectura de estos archivos pequeños originales, sin ISO, vídeo, audio ni partidas:

1. `data/cache/tune/cutscene/Story/cut_intro/subtitle.csv`.
2. `data/cache/tune/cutscene/Story/CUT_STORY_RACE_GH_06/subtitle.csv`.
3. `data/cache/ui/0xa4c1e8fa`.
4. `data/.install/files.tsv`, si existe; en su defecto, listado recursivo de nombres/tamaños de `data/cache/tune/cutscene/Story` y listado de `data/cache/ui`.
5. Solo si el listado revela variantes en esas dos escenas: los archivos alternativos pequeños tal cual, manteniendo nombre y bytes. No se da por hecho que `subtitle.txt` u otro nombre conocido contenga castellano.

Se conserva la capitalización real de carpetas. No abrir y volver a guardar los CSV desde una hoja de cálculo: se necesitan los bytes originales para comprobar codificación y columnas. El detalle del parser, alias/hash y persistencia está en la auditoría de subtítulos.

## Verificación de esta entrega

La preparación del inventario comprobó esquema, procedencia, categorías y que solo C tenga propuesta castellana. El inventario bruto quedó anclado a la revisión exacta y los dos extractores independientes coinciden en el total. Las marcas de implementación se basan en inspección de diff/adaptador; no sustituyen compilación, tests ni validación visual.

| Comprobación final en Linux | Resultado |
|---|---|
| Compilación de pruebas de `mcla`, `--locked` y la opción SDL de consola Linux | Correcta. |
| Pruebas focalizadas del juego, ejecutadas una a una | 41 correctas; 0 fallidas; 0 ignoradas. |
| `mcla-flash` | 28 correctas; 0 fallidas; 2 ignoradas. |
| `mcla-config` | 24 correctas; 0 fallidas; 0 ignoradas. |
| `mcla-formats` | 59 correctas; 0 fallidas; 8 ignoradas. |
| Archivos protegidos frente a `b68c4d0` | 95 idénticos byte a byte: manifiestos, lockfiles, workflows, configuración, parser y módulos esenciales de juego. |
| Inventario bruto regenerado | Gzip y cobertura idénticos byte a byte. |
| `git diff --check` | Correcto. |

Total: **152 correctas, 0 fallidas, 10 ignoradas** en este conjunto, que no es la
suite completa del juego. Las ignoradas no certifican recursos ausentes. Las
pruebas nuevas cubren selección AVM1, catálogo bilingüe, marcadores, glifos,
páginas de controles, teclas e IDs persistidos y la preferencia de subtítulos.
También pasan las regresiones existentes de la raíz de navegación y la solicitud
del mismo rival en ambos idiomas. No se usa la partida ni los datos del usuario.

Los comandos y la lista exacta están en `localization-audit/validation.md` y
`focused-tests.json`. La prueba de Flash falló antes de la corrección (`0` frente
a `1`) y pasó después. Se detectó y corrigió el glifo ausente `ª` de una etiqueta
propia, sustituyéndola por `(TECLA 2)`; la prueba de glifos no mide anchuras.

El compilador local disponible es Rust **1.99.1 en Linux**; la configuración de
GitHub Actions conserva **Rust 1.99.0**, WinLibs y las dependencias bloqueadas.
No se ha compilado ni distribuido un ejecutable Windows de esta fase.

Tras las pruebas de fuentes, sigue pendiente validar visualmente en Windows los menús y volver a iniciar `mcla.exe` sin `--data`, con el perfil de prueba ya usado, para comprobar el flujo de la primera carrera. No se modifican los datos ni la partida habitual para esa validación.

Prioridad manual: columnas y valores de PANTALLA/RENDIMIENTO, captura de teclas,
ayudas de mapa/editor/foto, rótulo de dinero y avisos HUD en 1280×720 y 1920×1080,
además de 4:3 o escalas que se utilicen. Las ayudas largas no tienen ajuste
automático; `layout.md` documenta posibles recortes. El contenido y los subtítulos
originales necesitan los archivos solicitados antes de considerarse verificados.

## Archivos modificados

- Documentación: `SPANISH_XBOX360.md`, `SPANISH_TRANSLATION_AUDIT.md` y los
  inventarios/informes de `localization-audit/`.
- Catálogo y presentación: nuevo `game/src/port_text.rs`; consumidores en
  `game.rs`, `main.rs`, `hooks.rs`, `mc4.rs`, `glyphs.rs` y `flash_ui.rs`.
- Selección de idioma: `flash/src/player.rs` y el callback de `flash_ui.rs`.
- Visibilidad de subtítulos: `game/src/video.rs` y su llamada en `game.rs`.
- Pruebas: módulos de pruebas de los archivos anteriores, nuevo
  `game/src/port_text_tests.rs`, `tools/test_spanish_ui.py` y evidencias de validación.
- Auditoría reproducible: `tools/audit_spanish_strings.py`.

Las rutas `game/src` y `flash/src` de esta lista son relativas a
`mcla-rs/crates/`. No se modifica ningún recurso original, guardado, dependencia,
lockfile ni workflow. Los cambios en opciones gráficas son exclusivamente sus
etiquetas/valores visibles: no se cambian ajustes, renderizado, físicas ni IA.

## Historial de entrega

La entrega se separa en tres commits desde la base validada: auditoría e
inventarios; catálogo/adaptadores y correcciones demostradas de presentación;
pruebas focalizadas y resultados. Este informe describe ese conjunto. Los SHA
exactos se proporcionan en la entrega y en el historial de la rama independiente.
No se fusiona con estable ni se publica un ejecutable nuevo.

El seguimiento posterior a `5c740ad` de marcadores de coleccionables, traspaso de
la película inicial y nuevas regresiones está en
[SPANISH_UI_FOLLOWUP.md](SPANISH_UI_FOLLOWUP.md). Los resultados de esta auditoría
describen su revisión inicial; el seguimiento registra las comprobaciones nuevas.
