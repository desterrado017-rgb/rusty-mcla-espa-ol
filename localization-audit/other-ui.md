# Auditoría estática de módulos adicionales — Fase 1

Base: `b68c4d02d384f611d176170bb893053a697503af`. Inspección de código sin ejecutar el juego, sin acceso a los recursos originales, sin alterar fuentes, dependencias, guardados ni datos del juego.

## Entregables y alcance

- `phase1-other-ui-audit.tsv`: 385 registros revisados y agrupados por origen (66 A, 93 B, 49 C, 177 D).
- `phase1-other-ui-literals.tsv`: inventario léxico de **15.992 literales** de los **183 archivos Rust** incluidos. Conserva ruta, línea, token exacto, contenido y contexto; los comentarios no se consideran código ejecutable. Incluye 5.202 literales situados en secciones/tests/fixtures para no confundirlos con UI. Las pistas automáticas de este fichero sirven para revisar: **no son una clasificación semántica definitiva**.
- `phase1-other-ui-coverage.tsv`: cobertura por archivo, incluidos los que no contienen literales.
- `phase1-career-hud-manual.tsv`: auditoría manual complementaria de Career, HUD, mapas, álbum y adaptador de teclado, integrada en el TSV principal. Incluye los orígenes relevantes de `config/src/keybinds.rs`, dado que el adaptador `game/src/keybinds.rs` no declara las etiquetas.

Excluidos de esta entrega por auditorías paralelas: `game.rs`, `main.rs`, `mc4.rs`, `hooks.rs`, `flash_ui.rs`, `boot.rs`, `localization.rs`, `cutscene.rs`, `video.rs`, y el nuevo catálogo `port_text.rs`. Se usan referencias a sus consumidores únicamente para demostrar que una cadena termina en pantalla.

El inventario es exhaustivo respecto a literales Rust de este ámbito. No permite afirmar cobertura exhaustiva de los textos que proceden de recursos, contenido Flash, tablas originales, nombres de mods o valores de la partida. Los textos dinámicos se auditan por ruta de selección; su contenido español sigue pendiente de comprobación con recursos originales.

## Significado y límites de las categorías

- **A**: existe una ruta comprobada a una clave original mediante la StringTable activa. Se documenta el fallback inglés cuando existe. No se afirma que la clave española esté presente ni que su contenido sea correcto: eso requiere leer los recursos originales. Ninguna traducción oficial se ha inventado.
- **B**: ruta de presentación con texto inglés/ID ensamblado que evita StringTable. El bypass se demuestra estáticamente y se describe su alcance; no se ha afirmado que la ruta se reproduzca siempre, ni que haya fallado el cargador español. Algunas frases pertenecen a funciones originales aunque la reserva/implementación offline sea del port; se dejan sin propuesta manual hasta demostrar las claves originales. Las frases exactas cuyo origen editorial no puede demostrarse lo indican expresamente.
- **C**: interfaz o ayudas propias del port comprobadas en cabeceras/decisiones del código. Propuestas de castellano etiquetadas como manuales, nunca atribuidas a Rockstar. Parte de ellas son sugerencias para una etapa posterior, no implementación realizada por este auditor.
- **D**: claves, identificadores, logs, errores técnicos, perfiles, nombres de recursos y tokens que deben conservarse. El TSV contiene ejemplos/familias revisados; el inventario bruto conserva cada literal para búsquedas posteriores.

## Hallazgos con implicación práctica

1. **C seguro y pequeño**: `modgarage.rs:176` define `MOD MISSING`, nota para placeholders de vehículos cuyo mod no está instalado (cabecera D27/D28). Se muestra en `game.rs:8147`. Propuesta propia del port: `MOD NO INSTALADO`.
2. **Teclado propio del port**: las ayudas en `glyphs::key_name` contienen MOUSE y ARROWS como descripciones propias pendientes; las etiquetas de tecla/botón físico (ENTER, ESC, PGUP/PGDN, BKSP, CROSS, etc.) se conservan en esta fase. Los `KeyDef.name` de `config/src/keybinds.rs` son simultáneamente etiquetas **y valores serializados**: jamás renombrarlos directamente. `ACTS.shared` indica qué ruta de presentación usa el editor; **no prueba el origen del texto**. Seis acciones se han identificado como ayudas adicionales propias; nueve etiquetas antes candidatas C corresponden a funciones originales (CAM_DOWN, MUSIC_PAUSE, PAUSE, cuatro WHEEL, GPS y garaje) y se han reclasificado B para reutilizar claves originales.
3. **Career, siete reservas**: seis formatos ingleses de resultados de series/torneos llegan a `event_line` y solo se ven cuando no se pinta la película de resultados original; el séptimo es un nombre de archivo convertido en título para NEXT EVENT. No son prueba de defecto de carga del español habitual.
4. **Garage, reservas originales**: `Slot::label()` y `pretty_part`/`variant_labels` producen inglés (Paint, Hood, STOCK, STYLE, WIDE BODY, NO SPOILER) sin lookup, consumido en `game.rs:7842/7864/8077/8363`. Deben buscarse las claves originales antes de traducir etiquetas. Los nombres de marcas y piezas derivados de mallas no se deben traducir a ciegas.
5. **Races y vinilos**: `RaceDef::title()` construye inglés y IDs para el HUD de reserva; `vinyl::ZONE_NAMES` se pinta directamente en el editor (`game.rs:6697`). Debe localizarse la selección de recursos originales, conservando índices, nombres técnicos y capacidades.
6. **Banderas offline**: `flags.rs` acredita como adición del port las carreras offline contra IA, pero los modos y conceptos ya existían en el original multijugador. Su HUD completo usa literales ingleses. No se atribuye por ello todo el texto a una interfaz exclusiva del port ni se propone una traducción manual de conceptos originales sin comprobar recursos.
7. **Riesgos de traducir texto usado como estado**: `flags.rs` prioriza objetivos con `starts_with("GET")`/`starts_with("STOP")`; `sound.rs:707` escoge sonido de victoria con `starts_with("YOU")` en el nombre de resultados. Una traducción directa puede cambiar lógica. Hay que separar el estado de la presentación antes de localizar estas rutas y añadir pruebas que cubran ambos idiomas.
8. **Álbum**: Zoom In/Out y Pan son claves originales, igual que los cuadros y prompts: deben llegar de StringTable. No son cadenas propias del port para traducir manualmente. Las rutas PNG, slots y textura son D.
9. **Guardado y arranque**: `saveflow::SgBox::spec` devuelve claves de los cuadros originales; retry/nosave y eventos saveStart/saveSuccess son tokens D. `bootload.rs` renderiza Boot sin añadir texto de jugador propio; sus nombres de threads, mensajes de panic y etiquetas WGPU son D. No se proponen cambios en preferencias ni guardados.
10. **HUD/GPU y audio**: principalmente renderizan texto suministrado por otras rutas o manejan identificadores de recursos y diagnósticos. Los nombres de habilidades y weather pueden usarse tanto como IDs/debug como presentación por sus consumidores: conservar token interno, resolver etiqueta por recurso/adaptador al mostrar.

## Verificación realizada

Se ha comprobado que los 385 registros tienen archivos existentes, líneas válidas y propuesta castellana únicamente cuando la categoría es C. No se ha generado ejecutable ni se han ejecutado pruebas con datos del jugador. La validación visual en Windows y el contenido de los recursos originales quedan explícitamente pendientes.


## Actualización de integración de presentación

La entrega consolidada final usa `localization-audit/inventory.tsv`, `port-catalog.tsv` y el inventario bruto canónico `spanish-literals.tsv.gz` (195 archivos/27.445 literales), en lugar de publicar los intermedios de trabajo enumerados arriba. El consolidado incorpora las otras auditorías y conserva la procedencia de cada fila.

Las 40 etiquetas visibles de teclas propias se han integrado mediante `port_text::key_label`, sin cambiar `KeyDef.name`, código, clave de guardado o parser. Las seis acciones adicionales del teclado usan catálogo; las nueve funciones originales usan sus claves originales. Las ayudas MOUSE/ARROWS y los alias LEFT/RIGHT, PGUP/PGDN y BKSP se adaptan únicamente en la presentación Keyboard; los tokens de `key_name`, controles Xbox/PlayStation y símbolos permanecen intactos. También se adapta la tecla GPS mostrada en la película Sidekick. Los valores técnicos ENTER/ESC/CTRL/ALT/NUM y letras/números/F-keys se preservan donde corresponde.

La comprobación final del consolidado encuentra consumidores para los 136/136 registros C auditados; esto mide integración estática de registros de este inventario, no todas las cadenas originales ni ajuste visual. Las pruebas y sus resultados finales se documentan en `SPANISH_TRANSLATION_AUDIT.md`.
