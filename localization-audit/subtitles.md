# Auditoría de subtítulos — fase 1

Base auditada: `b68c4d02d384f611d176170bb893053a697503af` (rama validada en Windows `fix/first-race-rival-v0.14.0`). Referencias de línea de esa base, salvo las nuevas pruebas de `video.rs`. Trabajo sin ejecutar el juego, sin abrir partidas del usuario y sin disponer de los datos extraídos de Xbox 360.

## Conclusión comprobada

El port sabe cargar y temporizar un único texto de cada `subtitle.csv`, pero **no tiene una selección de subtítulos por `game.language` ni una consulta de sus textos a `StringTable`**. Por tanto no se puede demostrar con este checkout que exista una ruta funcional hacia los subtítulos españoles originales. Esto tampoco demuestra que los datos del usuario no los contengan: falta inspeccionarlos. No se ha supuesto una columna española, un nombre alternativo ni una traducción oficial.

Se ha probado además por lectura de flujo un defecto distinto: la película inicial ignoraba el interruptor guardado de subtítulos. Las escenas de historia sí lo respetaban. La corrección de fase 1 añade a `Video::subtitle(enabled)` el control de visibilidad, usando el mismo valor `game.opts.subtitles` en su consumidor. **No cambia el valor predeterminado, la persistencia, el idioma de los datos, sus textos ni sus tiempos.** No se han tocado el parser CSV ni las rutas de recursos.

## Recorrido de los datos

1. `main.rs:13658` carga `Cutscenes::load(&self.opt.data_dir, audio_root)`; `self.opt.data_dir` es la raíz cache resuelta por el arranque ya validado. No se modifica esa resolución.
2. `cutscene.rs:65–73` enumera `tune/cutscene/Story`, ordena los subdirectorios y solo admite los que empiezan por `CUT_` al convertir el nombre a mayúsculas. Lee exclusivamente el archivo literal `subtitle.csv` de cada uno. Si no puede leerlo, omite la escena sin un aviso individual.
3. `cutscene.rs:72–87` interpreta cada byte como carácter Latin-1, omite la primera línea, separa comas fuera de comillas y admite comillas dobles escapadas. Primera columna: inicio decimal; segunda: texto; última: final si es mayor que inicio, si no usa `inicio + 2,5 s`. Omite texto vacío y filas de control cuyo segundo campo sea exactamente `time correction`, `position` o `size`. No interpreta cabeceras para descubrir idiomas. El parser trabaja por líneas y no implementa registros CSV con saltos de línea dentro de comillas. Esto describe su comportamiento, no demuestra que el CSV original necesite otro.
4. `cutscene.rs:89–106` estima la duración por el último subtítulo y, si encuentra audio convertido, por su duración. Ruta opcional de diálogo: `data/audio_pc/0xf615164c/<nombre_en_minúsculas>.mp3`; el `.mp3.txt` aporta duración. Ese audio no elige el idioma de los subtítulos. La animación real puede sustituir la duración al empezar (`main.rs:3600–3605`).
5. `cutscene.rs:118–135` resuelve misiones como `CUT_STORY_<ID>` o su variante `_NIGHT`; conserva alias concretos para misiones. `RACE_GH_06` resuelve `CUT_STORY_RACE_GH_06`. `by_name` compara nombres sin distinguir mayúsculas.
6. Las escenas se ponen en cola y avanzan con su reloj. `main.rs:3592–3656` coordina escena, carga, audio y salto a marcador; `cutscene.rs:177–179` devuelve el primer intervalo que cumpla `inicio <= tiempo < final`. No se cambia nada de este flujo ni de los eventos musicales de `CUT_STORY_RACE_GH_06` (`cutscene.rs:30`).
7. `main.rs:4501–4502` copia el texto a `game.cine` solo si `game.opts.subtitles` está activado. `game.rs:9660–9684` pinta el texto y las barras según la UI. No pide una traducción a las tablas.
8. Para la película de apertura, `main.rs:13017–13018` copia las líneas de `CUT_INTRO` a `start_movie("intro", subs, false)`. Si falta ese recurso, usa una lista vacía. `main.rs:12883–12895` abre `data/video_pc/intro.mcv` y su MP3 y asigna las líneas sin modificarlas. `video.rs` solo selecciona el intervalo según su reloj. La película de atracción se abre con una lista vacía (`main.rs:3590`); el conversor de vídeo no añade una pista de subtítulos (`tools/convert_video.py:1–14`).
9. Antes de la corrección, `game.rs:4747` pintaba `m.subtitle()` de la película sin comprobar la opción. Ahora se pasa el interruptor a `m.subtitle(self.opts.subtitles)`; el selector no reescribe la pista. El flujo de las escenas de historia ya hacía la comprobación.

## Tablas de cadenas y arranque

`boot.rs:158–160` carga las mismas tablas que el resto del juego mediante `localization::load`, y `boot.rs:76` pasa `shared.strings.lang` al global Flash `lang`. Esa ruta controla las pantallas Flash de arranque, no los CSV. `StringTable` contiene bloques de idioma: `strtbl.rs:16` documenta 0 inglés / 1 español; `strtbl.rs:104–106` hace respaldo inglés por clave cuando falta el texto seleccionado. `localization.rs:8–32` lee `ui/0xa4c1e8fa`, combina las tablas adicionales válidas del directorio y elige el idioma al final. El consumidor de subtítulos no usa esa API. No se deben inventar claves españolas a partir de nombres de escena.

`names.txt` conoce `cut_intro` (línea 429), `CUT_STORY_RACE_GH_06` (492), `Story` (2550), `subtitle.csv` (2552) y `subtitle.txt` (2553). El diccionario recupera nombres por hash; **la presencia de un nombre no prueba la existencia de un archivo, su idioma ni su formato**. `subtitle.txt` no se carga en la ruta actual. Los ficheros de referencia de cinemáticas del repositorio tratan parámetros de escenas, no proporcionan diálogos españoles.

## Preferencia y persistencia

- Estado del juego: `game::Options.subtitles: bool` (`game.rs:67`), predeterminado `false` (`game.rs:96`). La ausencia de `opts` en `SaveData` usa `Options::default` (`game.rs:1297–1298`).
- Estado MC4: `mc4::Options.subtitles: bool` (`mc4.rs:173`), también predeterminado `false` (`mc4.rs:221`).
- El lector de partida del port carga `sd.opts` (`game.rs:1765`) y después importa el bloque MC4 si es válido (`game.rs:1793`; `mc4.rs:1889–1890`). La importación MC4 copia `o.subtitles` al interruptor (`mc4.rs:2445`), por lo que ese bloque tiene precedencia si ambos valores difieren. No se cambia este orden.
- La UI usa la clave oficial `option_subtitles`, con respaldo `SUBTITLES` (`game.rs:5604,5671`); es una opción del original, no candidata a traducción manual. Las rutas de navegación y confirmación alternan el mismo bool (`game.rs:5850,6117,6311`) y marcan cambios con `dirty`.
- `write_save` guarda `opts: self.opts` y `mc4: self.mc4_save()` (`game.rs:1851`). `mc4_store` llama `mc4_options_out` (`mc4.rs:2167`), que conserva/copia el valor de subtítulos (`mc4.rs:2468–2469`). El guardado respeta `no_save`; no se añaden escrituras ni migraciones en esta fase.
- El importador Rust del guardado original obtiene el valor de LocalOptions `u16` en el offset 1238 (`savedecode/src/options.rs:159`); el exportador del bloque usa el mismo offset (`mc4.rs:18444`). Son datos técnicos que no se traducen.
- No se aumenta ni se reduce la preferencia predeterminada. La nueva comprobación de película solo evita mostrar texto cuando la preferencia existente dice que no.

## Clasificación de los textos de este recorrido

| Categoría | Hallazgo | Verificación |
| --- | --- | --- |
| A, recurso oficial conectado | Etiqueta de subtítulos `option_subtitles`, tablas de Boot y texto de diálogo cargado del CSV | Se verifican clave/cargadores/rutas. Sin recursos no se valida el contenido español ni su fidelidad. |
| B, inglés oficial por selección/carga/respaldo | Potencial en todo diálogo CSV si los bytes aportados son ingleses, porque el idioma no participa en esa carga | Limitación de arquitectura demostrada; no hay diálogo español o inglés real en este checkout para probar una fila concreta. No se presenta como regresión española confirmada. |
| C, texto del port | `PRESS ANY KEY`, superposición de atracción (`game.rs:4756`), y `{button} SKIP`, barras alternativas (`game.rs:9684`) | Literales visibles del port; no se ha probado una clave oficial equivalente. El auditor global debe revisar origen antes de escoger traducción o reutilización. |
| D, interno | `CUT_INTRO`, `CUT_STORY_RACE_GH_06`, `time correction`, `position`, `size`, nombres de ruta/archivo, globales Flash, mensajes `log::*`, claves de guardado `subtitles` | Identificadores, controles del parser o diagnósticos. Deben mantenerse. `video.rs` no contiene textos ingleses fijos para mostrar al jugador; entrega el diálogo recibido. |

El TSV adjunto enumera los candidatos con sus límites de verificación. El texto inglés de cada diálogo no puede enumerarse sin el CSV. No se transcribe ni se crea diálogo oficial.

## Archivos mínimos que debe aportar el usuario

Sin ISO, audio, vídeo, partidas ni reconstrucción de los 14 GB. Copias de lectura de estos recursos existentes, manteniendo bytes originales (no volver a guardar los CSV desde Excel):

1. `data/cache/tune/cutscene/Story/cut_intro/subtitle.csv` y `data/cache/tune/cutscene/Story/CUT_STORY_RACE_GH_06/subtitle.csv`. Usar la capitalización real de los directorios. El lector admite cualquier caso del nombre `CUT_*`, pero busca la hoja literal `subtitle.csv`. Adjuntar los archivos completos, incluida la primera línea y cualquier columna extra. Con dos escenas se puede comprobar cabecera, codificación, marcas y selección sin reunir todo el juego.
2. `data/.install/files.tsv`, si existe: es solo el inventario `ruta<TAB>tamaño` que el instalador genera (`install/src/layout.rs:36–37,152–160`; `install.rs:248–257`). Permite descubrir variantes de subtítulos o recursos adicionales por nombres conocidos y hashes. Si no existe, basta un listado recursivo de nombres y tamaños de `data/cache/tune/cutscene/Story` y un listado de archivos de `data/cache/ui`; conservar también los nombres `0x...` desconocidos.
3. `data/cache/ui/0xa4c1e8fa`, la tabla principal, sin editar. Para investigar una vinculación con diálogos y verificar textos españoles adicionales, el listado anterior permitirá pedir únicamente otras tablas identificadas. No es correcto afirmar sin inspección que una tabla desconocida contiene subtítulos. Se pueden aportar desde el principio las tablas adicionales de `ui` ya identificadas como tablas por la herramienta del proyecto; no hacen falta texturas o recursos Flash grandes.
4. Solo si el inventario muestra archivos alternativos en esas dos carpetas (por ejemplo `subtitle.txt`, variantes por idioma o nombres hash): aportar esos archivos pequeños tal cual. `subtitle.txt` es un nombre conocido, no una variante española demostrada.

Los alias de nombre calculados por el algoritmo del proyecto (`formats/src/hash.rs:20–35`), útiles si la extracción muestra hashes, son:

| Componente | Hash JOAAT de su nombre |
| --- | --- |
| `tune` | `0xa562a52d` |
| `cutscene` | `0xd454dc2b` |
| `Story` | `0xae6e9c22` |
| `cut_intro` | `0x8f54d607` |
| `CUT_STORY_RACE_GH_06` | `0xa451747a` |
| `subtitle.csv` | `0xcb466770` |
| `subtitle.txt` | `0x4e9030a3` |

Son hashes de componentes de nombre, **no comprobaciones SHA-256 del contenido ni evidencia de presencia en el disco**. `ui/0xa4c1e8fa` ya designa un hash de recurso conocido: no se debe volver a hashear su cadena hexadecimal.

Tras recibir los archivos se comprobarán bytes/codificación/cabecera, coincidencias de diálogo y tiempos, candidatos alternativos y bloques españoles reales; solo entonces se decidirá una selección de idioma. No se modifica el CSV original ni se traducen manualmente sus líneas.

## Pruebas de regresión de esta fase

`video.rs` incorpora dos pruebas sintéticas sobre el selector usado por producción:

- Preferencia desactivada: no se devuelve texto dentro de un intervalo válido; activada: vuelve el mismo texto, sin modificar la pista. Centinelas EN/ES con acentos y marcadores, sin copiar diálogo oficial.
- Intervalos `inicio <= t < final`, huecos y pista vacía: se conserva el comportamiento temporal previo.

Las pruebas no requieren recursos ni guardados. No se han ejecutado por este auditor para evitar compilar en paralelo; la validación consolidada la realiza el agente principal. La persistencia de `true` y `false` en memoria debe cubrirse con una prueba sintética del importador/exportador, conservando metadatos de idioma. La reproducción Windows con recursos españoles reales sigue pendiente para subtítulos; la primera carrera ya estaba validada en la base y no se toca su flujo.
