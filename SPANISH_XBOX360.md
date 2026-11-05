# Español original de Xbox 360 en Rusty MCLA v0.14.0

Base oficial: etiqueta `v0.14.0`, commit
`30d8b1f3b54557664d81bb7cfc9cc33caea53647`. La integración inicial está en
`feature/xbox360-spanish-original-v0.14.0`. La fase de interfaz se desarrolla en
`feature/spanish-ui-completion-v0.14.0`, desde la base validada en Windows
`b68c4d02d384f611d176170bb893053a697503af`, sin cambiar la rama estable.
La auditoría actual, sus inventarios y límites de verificación están en
[SPANISH_TRANSLATION_AUDIT.md](SPANISH_TRANSLATION_AUDIT.md).
El trabajo inicial de v0.13.0
se conserva en `feature/xbox360-spanish-original`.

## Configuración

En el `settings.toml` del juego (o en el archivo indicado con `--settings`):

```toml
[game]
language = "es"
```

`es` es el valor predeterminado de esta rama; `en` permite volver al inglés.
La selección se lee al arrancar y se conserva cuando el modelo de configuración
guarda el archivo. No depende de `launcher.language`, del idioma del sistema ni
de las opciones de una partida. El antiguo gancho de pruebas `MCLA_LANGUAGE`
ya no sustituye la configuración. No se añade un selector al lanzador.

## Lectura de textos

- Se lee la tabla principal `ui/0xa4c1e8fa` y se combinan las tablas adicionales
  de `ui/` en el mismo orden que v0.14.0 (South Central, consejos, policía).
  Se mantiene la prioridad de la tabla principal y la exclusión de `0xac5297d5`.
- Solo después de combinar todas las tablas se selecciona el bloque `1`, español
  en el formato original de Xbox 360. El inglés es el bloque `0`.
- Tanto el arranque como la carrera usan el mismo lector. Los textos que recibe
  Flash y su variable `lang` usan la selección resultante, también para las cifras.
- Una clave española ausente o vacía usa su entrada inglesa. Si no existe el
  bloque español, se usa el inglés. Si tampoco existe la clave inglesa, se
  mantienen los respaldos que ya tenía el port.
- El lector no reescribe, traduce ni normaliza el contenido de las tablas. Se
  conservan UTF-16, tildes, signos, saltos y marcadores como los recibe el parser
  existente. El formato visual que ya aplicaba cada pantalla se mantiene.
- La importación de opciones de una partida ya no cambia el idioma de las tablas.
  Los campos `language` y `language_name` de esa partida se conservan.

El índice español está documentado tanto en `formats/src/strtbl.rs` como en el
decodificador de referencia `tools/mc4save/options.py`. No se incluyen cadenas
del disco ni traducciones nuevas. Las pruebas nuevas usan datos sintéticos.

## Revisión de textos escritos en Rust

| Ruta | Resultado |
| --- | --- |
| `game/src/career.rs`, menús y HUD que consultan `career.strings` | Reciben las tablas combinadas con el idioma elegido. |
| `game/src/flash_ui.rs` y `boot.rs` | Se eliminan los índices ingleses forzados; se conserva el flujo de inicialización de v0.14.0. |
| `game/src/game.rs`: estados ON/OFF de trucos, listas musicales y neón | Se reutilizan `G_On` y `G_Off`, claves ya usadas para esos estados por el port. Se conserva el inglés si faltan. |
| `game/src/hooks.rs`: opciones propias del port y mensajes al jugador | Catálogo `port_text.rs` en inglés/español para adiciones verificadas del port; logs e identificadores técnicos intactos. No se presentan como traducciones de Rockstar. |
| `game/src/game.rs`: rótulos propios de pintura, modificaciones y pantallas alternativas | Permanecen en inglés cuando no hay una clave original comprobada; no se inventan traducciones ni se asignan claves de significado dudoso. |
| `game/src/catalog.rs` y `modcars.rs` | Los nombres añadidos por el port o por manifiestos siguen siendo respaldos ingleses; no sustituyen entradas originales. |

Esta integración activa los recursos españoles que lee el port; no afirma que
todos sus textos añadidos en Rust tengan traducción oficial.

## Investigación de subtítulos

`game/src/cutscene.rs` lee
`tune/cutscene/Story/<CUT_...>/subtitle.csv`, interpreta los bytes como Latin-1
y toma el texto de la segunda columna, con el inicio en la primera y el final
en la última. Esa ruta no consulta `StringTable` ni selecciona una columna o
archivo por idioma. El nombre `subtitle.csv` también está en `mcla-rs/names.txt`.

`game/src/main.rs` pasa los subtítulos de `CUT_INTRO` a la película inicial;
`game/src/video.rs` solo muestra los textos y tiempos recibidos. La fase de
interfaz corrige la película inicial para respetar la opción de subtítulos ya
cargada, como las escenas de historia. No cambia su valor predeterminado ni su
persistencia. La opción controla la visualización, no la localización del CSV.

No hay en el checkout los CSV originales, las tablas extraídas ni la ISO. Por
tanto no se ha podido comprobar dónde están los subtítulos españoles originales,
si son columnas, archivos alternativos o claves de otro recurso. No se cambia
el lector a partir de suposiciones y no se generan traducciones. Para completar
esa parte hace falta inspeccionar de solo lectura los recursos de una copia del
usuario y comprobar sus identificadores y tiempos contra el original.

## Diferencias relevantes entre v0.13.0 y v0.14.0

Se conserva v0.14.0 completa, incluidas las ampliaciones de tráfico, peatones,
física y audio. Su soporte `static-dxc`, `build.rs` y el adaptador C del runtime
UCRT siguen intactos. En `career.rs` se conserva el formato monetario ampliado de
v0.14.0. En `cutscene.rs` se conserva el despacho nuevo de eventos musicales;
no se reemplaza por el archivo de v0.13.0. La configuración y el parser de tablas
eran compatibles; el cambio de idioma se adapta sobre ellos.

## Verificación sin datos del juego

Las pruebas de configuración comprueban lectura, escritura, esquema, valores
inválidos e independencia del idioma del lanzador. Las de tablas comprueban
selección, caracteres UTF-16, marcadores intactos y respaldo inglés por clave.
Las del lector común generan tablas binarias temporales y comprueban que el
español aportado por una tabla adicional se activa incluso si falta en la principal,
manteniendo los bytes de los archivos de entrada. La regresión de importación
de opciones usa una partida sintética en memoria, sin escribir partidas.

El juego sigue necesitando los datos de la copia del usuario. No se ha ejecutado
contra una ISO ni se ha comprobado visualmente la edición española original.

Resultados históricos de la integración inicial en Linux x64, con Rust 1.99.1 y
los lockfiles de v0.14.0 intactos (no son las pruebas de la fase de interfaz):

| Comprobación | Resultado |
| --- | --- |
| `cargo check --locked -p mcla --features sdl3-sys/sdl-unix-console-build` | Compila. La opción de SDL es solo para la comprobación Linux, sin cambiar el manifiesto. |
| `cargo test --locked --workspace --exclude mcla --no-fail-fast` | 220 aprobadas por el runner, 0 fallidas, 12 ignoradas. |
| `cargo test --locked -p mcla --features sdl3-sys/sdl-unix-console-build` | 805 aprobadas por el runner, 3 fallidas, 122 ignoradas. Las cuatro pruebas nuevas del juego pasan. |
| En `launcher/`, `cargo test --locked -p mcla-support -p mcla-updater --no-fail-fast` | 133 aprobadas, 0 fallidas. |
| Compilación Windows x64 | Completada posteriormente en Actions para `b68c4d0`, con Rust 1.99.0, WinLibs UCRT y `--locked`; ejecución directa sin `--data` validada por el usuario: Henry aparece y la primera carrera se completa. No valida visualmente esta fase nueva. |

Las tres fallidas son `lane_into_junction_matches_the_original`,
`race_clearing_matches_the_original` y `road_density_matches_the_original`, de
`vehicles::traffic::tests`. Las tres intentan abrir
`resources/city/sc/0x9960fcd8.r66`, que no está en el entorno. El archivo de
pruebas `traffic.rs` es idéntico al de la etiqueta oficial v0.14.0: no se han
cambiado ni desactivado sus comprobaciones. Otras pruebas existentes retornan
antes de comprobar datos cuando faltan sus fixtures; su estado `ok` no valida
los archivos reales ausentes.

El binario de prueba de la corrección de navegación se compiló en el
[run 42058245472](https://github.com/desterrado017-rgb/rusty-mcla-espa-ol/actions/runs/42058245472).
La fase de interfaz no publica otro ejecutable. Se conservan Rust 1.99.0 en los
workflows, WinLibs UCRT, `static-dxc`, los manifiestos y los lockfiles. Las pruebas
Linux disponibles usan Rust 1.99.1; no sustituyen la futura validación de esta
fase en Windows con el toolchain fijado.
