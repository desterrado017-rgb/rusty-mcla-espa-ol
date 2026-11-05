# Resultado actualizado: rutas distintas para la navegación de la IA

**Defecto confirmado:** al arrancar sin `--data`, el mundo y los vehículos usan
la carpeta detectada correctamente, pero `offroad::mesh()` busca la malla de IA
en otra carpeta. No es un cambio introducido por la localización española.

## Ruta utilizada en la sesión fallida

El log `game-20261105-155909.log` registra argumentos vacíos y la raíz
`<instalación>/data/cache` con origen `InstallRoot`. El resolver común usa:

`--data → MCLA_DATA → paths.data_dir → raíz portable → junto al exe → fallback`.

Sin embargo, el lector antiguo de `offroad.rs` usa de forma independiente:

`primer --data literal → MCLA_DATA_DIR → D:/MCLA/extracted/cache`.

Eso ignora la detección portable, ajustes y la variable oficial `MCLA_DATA`.
También puede discrepar ante un argumento posicional o varios `--data`.
El primer resultado, incluido `None`, queda almacenado en un `OnceLock`.
El launcher siempre pasa `--data`, por lo que evita este defecto.

## Prueba en el registro y en el código

- Línea29: `off-road: no navigation mesh`.
- Línea513: `peds: nav mesh 52987 polygons, 15112 sidewalk in 351 islands`.
- `peds.rs` carga el mismo `resources/city/sc/0x69d17d57.r26`, pero con la raíz
  correcta. No falta ese recurso ni hace falta reconstruir los datos.
- `brain::tick` y `brain::run` retornan antes de navegar cuando la malla falta.
  Por tanto no ejecutan la navegación ni recuperación del cerebro del rival.
- El registro muestra que Henry se crea y recibe IA. El defecto no explica por
  sí solo un descarte de geometría: no se afirma que se haya demostrado todo
  el mecanismo de la ausencia visual.
- El código defectuoso de `offroad.rs` es idéntico en el oficial `30d8b1f` y
  nuestra integración `62cc08c`. La comparación launcher frente a exe directo
  mezclaba modos de arranque, no solo idiomas o binarios.

## Corrección mínima

`offroad::mesh()` consulta `hooks::get()?.data` y carga el recurso desde la
misma raíz inmutable que usa `main`, sin volver a interpretar argumentos ni
variables antiguas. La consulta precede al `OnceLock`: una llamada prematura
no fija un fallo antes de que exista la configuración de arranque.

Los mensajes identifican ahora la ruta completa, el origen de la selección y
si el fallo es de lectura o de parseo. No se cambia el orden de precedencia
del resolver común, la configuración de idioma, el formato de las partidas,
las dependencias, la colocación del rival ni los criterios de dibujo.

## Regresión reproducida sin datos del juego

La prueba `offroad::tests::navigation_mesh_uses_runtime_root_without_raw_data_argument`
llama al lector público real en cuatro procesos aislados. Usa un recurso MCLR
sintético con un triángulo transitable, guardado en una ruta temporal con espacios
y Unicode. Cubre raíz de ajustes, raíz de entorno, raíz explícita ya resuelta y
una llamada anterior a inicializar `hooks`. Verifica la malla parseada, su caché
y que los bytes originales del fixture permanecen intactos. El proceso padre
no cambia sus variables de entorno ni abre ajustes/partidas del usuario.

**Antes de la corrección:** los cuatro casos fallan con «navigation mesh must
use the same resolved data root as the game». No fallan por un fixture ausente.
La prueba existente `config::tests::data_dir_resolution_order` cubre además la
selección portable `InstallRoot`, `BesideExe` y las prioridades de las entradas.

**Después de la corrección:** los cuatro procesos aislados pasan. La misma
prueba del lector público cambia de FAILED a OK sin modificar el fixture.
Se han repetido y aprobado también las seis pruebas focalizadas de misión,
trazas, localización e importación de opciones, y
`mcla-config::tests::data_dir_resolution_order` (1 prueba real ejecutada).

Comando de regresión en Linux, manteniendo Cargo.lock:

```sh
cargo +1.99.1 test --locked -p mcla --features sdl3-sys/sdl-unix-console-build offroad::tests::navigation_mesh_uses_runtime_root_without_raw_data_argument -- --exact --nocapture
```

La opción de SDL solo permite comprobar Linux; no cambia manifiestos ni
lockfiles. El fixture y los subprocesos tampoco arrancan el juego, usan GPU
ni cargan/escriben partidas. La prueba de configuración cubre la raíz portable,
y las pruebas del lector cubren sus consumidores sin una segunda resolución.

La validación visual en Windows del modo sin argumentos sigue pendiente.
No se ha compilado ni publicado un nuevo `mcla.exe` durante esta revisión.

---

# Auditoría histórica y diagnósticos de la primera revisión

Las secciones siguientes conservan la investigación previa al hallazgo de la
doble resolución. Sus hipótesis de temporización no son la conclusión actual.

# Investigación: rival de la primera carrera

Estado histórico de la primera revisión: causa aún no demostrada.
El resultado actualizado figura en la sección siguiente.
Rama independiente: `fix/first-race-rival-v0.14.0`. No se ha publicado un
nuevo ejecutable. La localización y las dependencias se conservan intactas.


## Actualización: comportamiento intermitente confirmado por el usuario

El mismo ejecutable modificado, sin recompilar ni corregir, ha mostrado a Henry
correctamente con español. Esto descarta un fallo determinista causado solo
por seleccionar `es`; no demuestra todavía la causa ni excluye un problema
de temporización. Se suspenden más cambios de código y cualquier publicación
hasta comparar el log correcto y el fallido. Los diagnósticos anteriores
siguen locales y sin commit/publicación. No se ha aplicado un fix.

El log fallido registra salto de la escena inicial de 1.93s a57.63s y elige
`gen_prerace_taunt_hp_03`. Para comparar: estado nuevo/restaurado, forma de
saltar la escena, escena de salida, transición/streaming y posición del rival.
En el código el inicio con warp espera el handover de cámara; la ruta sin warp
espera streaming. La colocación inicial del rival usa un sondeo de suelo distinto
al del jugador. Son condiciones concretas que deben observarse, no causas
demostradas ni motivos para cambiar el código sin reproducción.

## Evidencia nueva del usuario

Registro leído: `game-20261105-155909.log`, con `game.language=en`.
El archivo del usuario se ha leído solamente y no se incorpora al repositorio.
Las horas de las líneas del registro son UTC.

| Hora / líneas | Hecho observado |
| --- | --- |
| 14:59:11 / 461 | Se activa RACE_GH_06. |
| 14:59:53 / 794–803 | La escena inicial carga el 280Z de Henry y su configuración GH_06_0_vp_dsn_280z_75. |
| 15:00:02 / 834 | La misión entra en el registro después de la escena. |
| 15:00:03 / 838–844 | GH_06 obtiene slot y se crea circulando a 39 m del jugador. |
| 15:00:11 / 871–872 | Se acepta el desafío y se pasa a hw_ordered_jp_11. |
| 15:00:14 / 904 | La selección de parrilla incluye HENRY / vp_dsn_280z_75. |
| 15:00:14 / 915–919 | La carrera es de hookman; el corredor 1 tiene el modelo esperado y una IA de 17 objetivos. |
| 15:00:16 / 935–939 | Empieza gen_prerace_taunt_hp_03; el slot 1 todavía resuelve al UID 375. |
| 15:00:19 / 953 | La escena dispara la cuenta atrás. |

Esto descarta que en esta ejecución falte la creación inicial del rival.
El log general no permite determinar por qué no se ve después: no registra
su posición, pose, actividad física ni motivo de descarte al dibujarlo.
La lista larga de checkpoints impresa después contiene también índices de
tráfico; no es el número de participantes de la carrera.

Los eliminadores normales de tráfico conservan los coches `pinned`, como este
rival. El borrado de coches de escena `cine:` no afecta al conductor GH_06.
La escena registrada no tiene movers de coches, por lo que ese bucle no puede
teletransportar a Henry. La voz/configuración de conductor sin preparar no
prueba que falte el vehículo. `placed=false` tampoco lo oculta por sí solo.

## Diagnósticos preparados

Se habilitan solo con `MCLA_DIAG_FIRST_RIVAL=1`, usan el log existente y no
reparan, fuerzan, mueven, eliminan ni reviven nada:

- Sesión, rutas efectivas, idioma y commit/run si el compilador recibió las
  variables GITHUB_SHA/GITHUB_RUN_ID; si no, informa `unrecorded`.
- Estados de misión, briefing, entrada al registro, slot, loop, modelo,
  suspensión y condiciones de colocación. Las esperas se deduplican por estado.
- Tipo de carrera, número de rivales y creación de cada vehículo de parrilla.
- Seguimiento por UID original frente al índice actual cada 60 frames:
  posición física/de dibujo, IA, activación, streaming, geometría y LOD.
- En el paso opaco principal, causa real de descarte por stealth, LOD,
  distancia o frustum, o número real de draws e índices del cuerpo enviados.

No se ha ejecutado el juego contra datos ni partidas reales. Las pruebas usan
solo datos sintéticos temporales. No se ha reconstruido ni descargado el juego.

## Pruebas: alcance y pendiente

La prueba `first_mission_requests_the_same_rival_after_its_scene_in_both_languages`
ejecuta las funciones reales `Career::load → new_game → call_delivered →
Roamers::sync` con XML/textos sintéticos. Comprueba que no se solicita al rival
antes de la escena y que después se solicita el mismo GH_06, 280Z, configuración
y loop tanto en inglés como en español. Otra prueba verifica la deduplicación
y reentrada de las trazas.

Estas pruebas **no reproducen la desaparición visual ni demuestran una solución**.
La prueba de regresión del fallo concreto queda pendiente de identificar su
condición causal. Los resultados de ejecución se anotan al final de este archivo.

## Siguiente comparación necesaria

El selector de escena usa reloj real XOR tick como semilla. Dos ejecuciones
pueden elegir escenas distintas aunque compartan partida y datos. Falta el log
del oficial que funciona para comparar la escena y la transición de salida.
El gancho existente `MCLA_PRERACE_TAUNT` permite fijar el índice en ambos
binarios sobre los mismos datos. No se ha impuesto una escena como supuesto fix.
El gancho ya existente `MCLA_GRID_TRACE=1` puede aportar información física con
los ejecutables actuales sin publicar otro.

En futuras pruebas aisladas hay que especificar `--save-dir` y/o
`MCLA_SAVE_FILE` con una carpeta de prueba. En este código,
`RUSTY_MCLA_USER_DIR` por sí solo no redirige `game::savedir::default_dir`.
No usar `MCLA_NEW_GAME` sobre los guardados habituales. No se ha ejecutado
ninguna de esas pruebas sobre la instalación del usuario en esta revisión.

## Auditoría completa de los cambios de localización

Scope: read-only comparison of upstream v0.14.0 `30d8b1f3b54557664d81bb7cfc9cc33caea53647` with fork `62cc08c30a4450f37a4d1d5c09a9c3267ee6079a`. Upstream tag resolves to that base SHA. No game resources or saves are available, and no game execution was attempted. These notes describe static evidence, not a demonstrated cause of the user's observed missing rival.

## Every production hunk

- `config/src/lib.rs`: new `Language` enum maps `en` to table 0 and `es` to 1; adds `GameSettings.language`, defaults it to Spanish, reads and writes only `game.language`, and adds its value to the log summary. All nonlanguage defaults and settings readers are unchanged. `GameSettings` is a normal Rust struct, not a serialized game-save layout; no pointer arithmetic or binary ABI consumer of this struct was found. `hooks::game_live` clones it, then accesses other settings by field name.
- `config/src/schema.rs`: adds the settings schema row for `game.language`, with default `es` and options `en | es`. No gameplay selector or game-save entry is changed.
- `formats/src/strtbl.rs`: the parser, table key/hash construction, merge algorithm, and language selection algorithm are unchanged. `get` now rejects an empty selected-language entry before trying English. For language 0 this is extensionally identical to the official implementation: if English is nonempty both return it; if empty the new fallback reads the same empty English entry; if absent both return None. This cannot by itself explain a failure also observed with selected language 0.
- `game/src/localization.rs`: factors main-table load and the sorted supplementary-table merge from Boot/Career into one function, then selects the requested language and logs the selected block. Main filename, excluded filename, file filtering, parse error tolerance, and merge order match upstream. The comment mentioning v0.13.0 is stale documentation; it does not affect execution. Boot does not add catalog fallback names either before or after this change.
- `game/src/career.rs`: replaces the duplicated string-table load with `localization::load`. **Real semantic delta:** official Career inserts `catalog::CUT_STRINGS` and mod-car fallback names after main-table parsing but before supplementary table merging; fork inserts them after every table was merged. Existing main-table strings win in both. A supplementary entry for a fallback-name key wins only in the fork. `CUT_STRINGS` contains only `vp_nsn_gtr_08` and `vp_nsn_gtr_08_shrt`. `modcars::strings` returns an already initialized registry slice and has no initialization side effect. No path from these display-name keys to selection or spawning of Henry was found. Do not change this order on the assumption it causes the reported bug.
- `game/src/boot.rs`: calls the common loader; replaces the post-four-tick `lang=0` assignment with the selected block. Boot's `load` still explicitly does four ticks before setting aspect, language, zone, and platform. **However, generic FlashMovie::load now pre-seeds lang, so those first four ticks see 0/1 instead of an absent lang unless the movie sets it itself.** This is a genuine initialization-order difference even with English, but its effect on the actual boot movie requires the user's resource to establish.
- `game/src/flash_ui.rs`: generic `FlashMovie::load` writes `lang` immediately after `Player::new`, before the first tick; `FlashMenu::show` writes it on update; the bank and prompt hard-coded English assignments are removed/replaced. `Player::new` only allocates objects/registry and does not tick the movie. `set_global` only writes that player's object member and invokes no host callback. `fscommand` handles Getstring, movie-local common commands, and movie-local camera output; it has no rival/spawn API. Flash can still affect UI timing or values, which must be traced with the original movies before ruling it out.
- `game/src/game.rs`: three UI-only replacements of literal ON/OFF with `G_On`/`G_Off` lookups: cheat menu, custom music genre indicator, and neon row. No first-mission state, vehicle load, AI, or renderer function changes.
- `game/src/hooks.rs`: new `language()` getter reads startup settings (or the default if hooks were not initialized). No new settings write and no save access.
- `game/src/main.rs`: adds `mod localization;` only. Every existing initialization, first-story-scene, pending-log, slot, and spawn statement is unchanged.
- `game/src/mc4.rs`: the only production hunk removes the `MCLA_LANGUAGE`/save-language override and `career.strings.set_lang` call from `mc4_options_in`. The rest of options import/export and all career/save/garage flags are unchanged. No writes of `Options.language` or `language_name` are added. All other hunks are tests.
- `SPANISH_XBOX360.md` and added test hunks: documentation / synthetic verification only; not compiled into release gameplay.

No dependency, Cargo.lock, build.rs, C shim, vehicle, opponent, cutscene, race, roamer, or save-format production code differs in this comparison.

## First-mission invariants found

- `Career::new_game` resolves and activates raw ID `RACE_GH_06`; it does not identify Henry from translated labels.
- `event_of`, `hookman_race`, `hookman_mission`, and `rival_id` use mission IDs, parsed conditions, and script/race identifiers. `hookman_wanted` uses translated names only for presentation.
- `Roamers::update` asks `career.mission_in_log(row.0)`, requests its raw hookman row, and subsequently emits/loads the vehicle; it does not test the language or translated display name.
- The initial mission waits for its introductory scene and `pending_log`/`call_delivered` transition before its man becomes active. This is a useful diagnostic boundary. A missing rival may mean the transition did not happen, a slot was not granted, a model failed to load, or an already-created vehicle was not rendered. The current static diff does not select one of those causes.

## Separate verified issue, not a fix for this report

`mcla-flash/src/player.rs::common_fscommand` handles `Getvar` with argument `lang` by writing 0, and `flash_ui::fscommand` invokes it before its own switch. Thus an original movie can overwrite the selected Spanish global. This function is identical upstream/fork. It can affect language consistency but cannot yet explain disappearance in both en and es. Do not bundle a change here into a rival fix without evidence.

## Synthetic early-language test that exercises production initialization

A test that manually calls `Player::new` then `set_global(lang)` only mirrors the implementation. It would prove nothing about whether `FlashMovie::load` actually uses it. To test the changed production path without original game files, extract the post-parse construction in `FlashMovie::load` into one private constructor, e.g. `from_movie(Arc<Movie>, rel, &UiShared)`, and have both `load` and its unit test invoke that constructor. This is an exact behavior-preserving seam, not a gameplay change.

Build an in-memory `Movie` with a root `Sprite` of one `Frame` containing `Cmd::DoAction(Code)` that performs `first_lang = _global.lang`, then Stop. AVM1 sequence: push "first_lang"; push "_global"; GetVariable (0x1c); push "lang"; GetMember (0x4e); SetVariable (0x1d); Stop (0x07); End (0x00). String push helper already exists in flash/player.rs tests: bytes `[0x96, (len+2) as u8, 0, 0]`, UTF-8 bytes, NUL. After constructing through the real constructor and one tick, read `player.get_member(Ref::Inst(player.root), &Value::str("first_lang"))`. It should be Int(0) / Int(1) for selected English / Spanish. A Player initialized like upstream sees Undef before Boot's later assignment.

This is an **initialization characterization test**, not a regression test proving or fixing the missing rival. It establishes that an actual consumer can see the ordering difference. To claim a rival regression, a test must instead fail on the observed bad mission/slot/spawn input and pass after a causally justified correction. The original boot movie's frame behavior or a captured first-rival trace is still needed if that hypothesis is pursued.

## Risk / next evidence

English failure narrows the issue: differing Spanish table text alone is insufficient. It does not prove the entire localization patch is uninvolved; generic Flash preinitialization, Career fallback precedence, or load timing changed in both modes. No specific one is currently proven responsible. A same-toolchain official source build is useful to separate source changes from build/compiler/runtime differences, but a successful build alone cannot establish runtime equivalence or a fix. Hold any new executable publication as requested until the cause and regression behavior are verified.


## Auditoría de compilaciones

Fecha: 2026-11-05. Auditoría de solo lectura del repositorio y de metadatos/registros de Actions. No se ha construido, ejecutado ni publicado ningún ejecutable durante esta auditoría.

## Referencias comparadas

- Base oficial v0.14.0: `30d8b1f3b54557664d81bb7cfc9cc33caea53647`.
- Integración española: `62cc08c30a4450f37a4d1d5c09a9c3267ee6079a`.
- Workflow independiente del fork: `b4ff5fc7e5fa7738dedacca2e0f3d288b1557a76`.
- Publicación oficial: https://github.com/holdmysocks/rusty-mcla/releases/tag/v0.14.0
- La nota oficial identifica https://github.com/holdmysocks/rusty-mcla/actions/runs/42025703362 (job `127239258971`).
- Fork compilado: https://github.com/desterrado017-rgb/rusty-mcla-espa-ol/actions/runs/42041870865 (job `127294468494`).

La publicación oficial señala el commit base anterior. Su `game-0.14.0.zip` fue creado a las 12:27:43 UTC y actualizado a las 12:27:46; la publicación salió a las 12:30:18. SHA-256 del ZIP publicado: `5bcc48134c652a9e40583f4733c9386d3ec1ca8643428c82f054ad6f94085c31`. Estos tiempos corresponden al run citado por las notas. Otro run posterior de release, `42027687390`, produjo un borrador `untagged-5deb06ea5fa18240e606`; no se toma como procedencia del paquete público.

## Evidencia de los registros reales

| Campo | Release oficial citado | Fork |
| --- | --- | --- |
| Runner | Windows Server 2025 | Windows Server 2025 |
| Imagen | windows-2025-vs2026 | windows-2025-vs2026 |
| Versión imagen | 20261030.294.1 | 20261030.294.1 |
| Versión runner | 2.338.2 | 2.338.2 |
| Rust | rustc 1.99.0 (b940084d7 2026-09-28) | Igual |
| Target/toolchain | 1.99.0-x86_64-pc-windows-gnu | Igual |
| C/C++ | gcc.exe (MinGW-W64 x86_64-ucrt-posix-seh, built by Brecht Sanders, r1) 16.2.0 | Igual |
| Generador CMake del paso de juego | MinGW Makefiles | Igual |
| CARGO_INCREMENTAL | 0 | 0 |
| CARGO_TERM_COLOR | always | always |
| Comando juego | cargo build --release -p mcla | cargo build --locked --release -p mcla |
| Perfil terminado | release [optimized + debuginfo] | Igual |
| Tiempo paso de juego | 6m 40s | 11m 13s |

No hay `RUSTFLAGS` en el entorno del paso de juego de ninguno de los dos registros. La opción `-C target-feature=+crt-static` del release oficial está confinada al paso MSVC del lanzador; no se aplica al juego GNU. `--locked` evita que Cargo modifique la resolución; no es una opción de generación de código.

La versión ejecutable de CMake no se imprime en estos pasos. Que compartan imagen del runner y generador es evidencia de configuración comparable, no prueba de igualdad binaria del resultado ni de todas las herramientas ejecutadas.

## Cachés y dependencias resueltas

Hay diferencias observables de caché:
- Oficial: WinLibs encontrado en caché con clave `winlibs-9714f9e55905000ec2a066ae033abdfe8c93083a925e3fe015d3c7cc4bb1c918`.
- Fork: WinLibs no estaba en caché; se descargó y pasó la verificación del mismo SHA-256.
- Oficial: caché Rust restaurada parcialmente desde `v0-rust-release-1.99.0-draft-Windows_NT-x64-c9e5093f-d8fe67fd`, 938 MB; solo aparecen 12 compilaciones de paquetes en el paso de juego.
- Fork: clave de caché `gnu-1.99.0`, sin restauración equivalente; aparecen 238 compilaciones de paquetes en el paso de juego.
- Ninguno de los paquetes compilados que aparecen en ambos pasos tiene una versión diferente.
- Ambos registros descargan/compilan `mach-dxcompiler-rs 0.1.7+2026.10.26-141dd7f.1`.
- El fork compila `sdl3-sys 0.7.2+SDL-3.4.18` y `sdl3-src 3.4.18`; ambos lockfiles y las características declaradas son idénticos a los de la base oficial.

La ausencia de una línea `Compiling` para una dependencia reutilizada por el oficial no permite comprobar su versión efectiva por esa línea del registro. La identidad de los lockfiles y los manifiestos da evidencia adicional, pero no se utiliza para afirmar identidad de ejecutables. La caché no es por sí sola prueba de que exista una regresión ni de que no exista.

## Identidad de entradas de compilación

SHA-256 calculado sobre los bytes de cada archivo mediante `git show <ref>:<path>`. Todos estos archivos son idénticos entre oficial y `62cc08c`:

| Archivo | SHA-256 en ambas referencias | Idénticos |
| --- | --- | --- |
| `mcla-rs/Cargo.lock` | `fd98c6170159f2ec63e2623f78033ca0ca6a4cebf8fec6fa26f7577e6461f2bf` | Sí |
| `launcher/Cargo.lock` | `610dac76cd1628bfa7d9322f3acca35e499e3bda0af17b08c83e0e4e43a7719d` | Sí |
| `mcla-rs/Cargo.toml` | `39d840dba94dec489193d3727f6fe68de99db2771a46d4e3abed0f06e0c91ad3` | Sí |
| `mcla-rs/crates/game/Cargo.toml` | `46dffb7dfef76dffdc9c5d7ac71e6fb7b716b527d99ae4cb9084d774a2d4c428` | Sí |
| `mcla-rs/crates/game/build.rs` | `ffbb737f182c1c4676cb63c9a76a551b9a52481f2015a53ab3935530f2497f38` | Sí |
| `mcla-rs/crates/game/csrc/dxc_crt.c` | `8bed15d5035aa7005fa143479459ee909142cf496f32c5c4d7e77a4505851f7c` | Sí |
| `.github/workflows/ci.yml` | `e4978fc3224ae95182377ccc2d6e2d06cd16ee972a50b0d31c2a5c2a52f1d14c` | Sí |
| `.github/workflows/release.yml` | `de1c6673618ba35702d82493293b359296d939613e55cf148b78466fa9fbcf00` | Sí |

El perfil release también es idéntico: `opt-level = 3`, `debug = 1`, `codegen-units = 4`, `lto = "thin"`. El manifiesto conserva `wgpu/static-dxc`, SDL3 estático desde fuentes y el puente UCRT de DXC. La comparación completa solo modifica los 13 archivos de integración documentados por `git show --stat 62cc08c`; no modifica la selección/creación de vehículos en sus propios módulos.

## Conclusión y límites

No se ha encontrado una diferencia de versión de toolchain, CRT, características, perfil, lockfile o configuración del linker que explique el rival ausente. Esto NO demuestra equivalencia de binarios ni descarta un problema dependiente de compilación, de caché, de timing o de entorno de ejecución. Tampoco demuestra que la prueba del usuario utilizara el paquete oficial identificado o el mismo archivo de datos/configuración/partida: falta identificar los binarios y los registros de esas pruebas concretas.

Antes de generar más binarios, la comparación de ejecución debe registrar: ruta y hash de los dos ejecutables, argumentos, configuración efectiva, directorio de trabajo, ruta canónica de datos, ruta real del guardado, estado de nueva partida/INTRO y el paso exacto donde diverge la creación del rival. Las nuevas pruebas deben usar un perfil de prueba aislado; no se deben modificar guardados originales.

Si sigue siendo necesario aislar compilación, un job controlado puede hacer checkout del oficial `30d8b1f` y de la corrección en carpetas distintas y construirlos con el mismo runner, las mismas herramientas verificadas y targets limpios separados, usando `--locked`. Debe imprimir versiones completas de Rust/GCC/CMake, opciones de compilación, hashes de fuente/lockfile/binario y ejecutar la misma prueba sintética de flujo. Bajo la restricción actual, no debe subir ni publicar esos ejecutables. Un A/B visual necesitaría, además, los recursos de la copia del usuario y un medio de prueba acordado que conserve sus archivos.

## Diagnóstico de arranque existente y ampliaciones sin dependencias nuevas

`hooks::header` ya incluye: versión del paquete (siempre 0.14.0 para ambas variantes), SO/arquitectura, ruta del ejecutable, argumentos, ruta/estado del archivo settings, resumen de settings efectivos, ruta/origen de datos, carpeta de guardado y screenshots. El adaptador/backend se registra después.

No registra commit, workflow/run de origen, compilador ni hash del ejecutable. Por eso la línea `version: 0.14.0` no identifica qué variante se ejecuta.

Añadidos útiles sin nuevas dependencias:
- `option_env!` para un identificador de commit/workflow incorporado explícitamente en futuras compilaciones; mostrar “no disponible” si no está.
- Ruta canónica de ejecutable/datos, directorio de trabajo y tamaño del ejecutable.
- Perfil por `cfg!(debug_assertions)`, SO/arquitectura y campos de procedencia si existen.
- Ruta de guardado efectiva mediante `Game::save_path(data_dir)`, no solo la carpeta.
- Indicadores explícitos de los ganchos relevantes (`MCLA_NEW_GAME`, override de guardado/datos, opciones que omitan o desactiven oponentes), sin volcar todas las variables de entorno.
- Al entrar en la misión: identificador estable, estado de carrera/intro/fresh_game y conteos/causas de rechazo de rivales, según la auditoría de flujo.

El campo `save:` actual usa `savedir::get()`; puede no representar el archivo realmente leído cuando hay `MCLA_SAVE_FILE`, cuyo override se resuelve primero en `Game::save_path` (game.rs). La carga ya registra la ruta efectiva y si existe.



## Resultados de esta revisión

Linux x64, Rust1.99.1, Cargo.lock intacto, SDL con
`--features sdl3-sys/sdl-unix-console-build` solo para comprobar Linux:

| Comprobación | Resultado |
| --- | --- |
| Nueva prueba real misión → slot en/es | 1 aprobada |
| Deduplicación y reentrada de trazas | 1 aprobada |
| Localización común | 3 aprobadas |
| Conservación de idioma al importar opciones | 1 aprobada |
| mcla-config --lib | 24 aprobadas |
| mcla-formats --lib | 59 aprobadas, 8 ignoradas |

Cero fallos. Algunas pruebas antiguas de formatos retornan si faltan sus
fixtures; esos resultados no validan recursos originales ausentes. No se
ha ejecutado el juego, generado una nueva compilación Windows ni publicado
un binario durante esta investigación. No existe todavía prueba de que el
fallo visual quede resuelto.
