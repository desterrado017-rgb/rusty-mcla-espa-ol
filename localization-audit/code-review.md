# Revisión independiente de fase 1

Revisor: `phase1_final_review`. Base: `b68c4d02d384f611d176170bb893053a697503af`.
Revisión de lectura de fuentes y diff, sin compilación ni ejecución del juego. Revisión actualizada tras la congelación de las fuentes de fase 1. La ejecución de las pruebas
la coordina root y no forma parte de esta revisión independiente.

## Resultado de código

No se han identificado cambios en físicas, IA, navegación, resolución de datos, escritura
ni formato de partidas, recursos originales, manifiestos, lockfiles o versiones de workflows.
`offroad.rs` y la corrección validada de Henry permanecen intactos en el diff observado.
No se genera ni publica un ejecutable.

- `flash/src/player.rs::common_fscommand`: ya no sobrescribe `lang` existente con cero;
  consumidores sin selección mantienen el respaldo inglés. `flash_ui.rs::fscommand` responde
  con `strings.lang` antes del handler común; la consulta no modifica el orden ni número de ticks.
- `FlashMovie::load` fija la selección antes de ticks y el callback comparte la misma tabla.
  Boot sigue reutilizando ese callback. El cambio no altera globals `aspect`/`platform`, tiempo,
  creación de vehículos ni estado de carrera.
- `hangout_bank_label`: reutiliza `G_Money`, clave ya utilizada por garage/GPS para el mismo
  campo monetario. Conserva prioridad de `MCLA_BANKTXT` y el respaldo literal `MONEY  `.
  El contenido real y ajuste visual requieren recursos originales; no se certifica la frase española.
- `Video::subtitle(enabled)`: conserva los intervalos y devuelve los mismos datos sin mutarlos.
  El gate usa `game.opts.subtitles`; la carga normal `main.rs::load` llama a `game.load_save`
  antes de `opening_tick`, que crea la película. `load_save` lee `sd.opts`, luego importa opciones
  MC4. Defaults y escrituras existentes no se modifican. La nueva prueba debe cubrir ambos bool.
- `hooks`: los helpers solo formatean presentación. `display`, `game_live`, `monitors` clonan
  estado y liberan sus mutex antes de formatear. No se ha encontrado nueva adquisición anidada
  ni cambio de setters de gráficos o palabras persistidas.
- `mc4::keyboard_action_label`: separa seis adiciones de teclado de acciones originales,
  sin asumir que `shared=false` significa texto del port. Se conservan constantes, nombres
  técnicos y teclas serializadas. GPS/garaje reutilizan claves visibles existentes; las otras
  acciones originales consultan sus identificadores. Solo las seis adiciones inequívocas usan
  traducción manual. La página inglesa conserva sus etiquetas.
- Los textos de ayuda PC, error procedural, captura F12 y modo ventana son adiciones del port.
  No se sustituyen los nombres de habilidades, vehículos ni acciones originales por traducciones
  inventadas. La palabra de habilidad sigue procediendo de `Kind::name`, pendiente de clave oficial.

## Inglés

Se compararon los valores EN del catálogo con fuentes de la base de game/config. Los únicos
que no aparecen literalmente completos en la base son valores ensamblados equivalentes:
`BORDERLESS` (enum `.word().to_uppercase()`), plantillas `{count}`, `{action}`, `{vehicle}`,
`{keys}`, `{ability}`, `{path}`. Corresponden a los formatos anteriores; no se detectó cambio
editorial en el inglés del catálogo. La nueva elección de G_Money sí sustituye una reserva
por recurso oficial, de forma intencionada.

## Límites y comprobaciones visuales pendientes

- No se han leído los recursos españoles reales; una ruta correcta A no certifica contenido.
- No se han medido anchos usando atlas Flash/bitmap originales. Títulos y valores largos como
  SINCRONIZACIÓN VERTICAL, DESENFOQUE DE MOVIMIENTO, FPS + TIEMPO POR FOTOGRAMA y ayudas de
  mapa/editor requieren comprobarse en resoluciones y escalas de UI admitidas.
- Se detectó el glifo faltante `ª` en SecondKey; se corrigió únicamente la adición del port a
  `(TECLA 2)`. El auditor Flash prepara prueba del inventario real de glifos.
- Los mensajes de `show_message` se convierten a mayúsculas en `game.rs` antes de HUD/bitmap;
  no se debe afirmar que las vocales acentuadas minúsculas de las plantillas se dibujan sin ese paso.
- Las pruebas sintéticas y esta revisión no validan traducción visual ni revalidan la primera carrera
  en este nuevo commit. La confirmación funcional Windows corresponde a la base validada.

## Archivos observados

- `mcla-rs/crates/flash/src/player.rs`
- `mcla-rs/crates/game/src/flash_ui.rs`
- `mcla-rs/crates/game/src/game.rs`
- `mcla-rs/crates/game/src/hooks.rs`
- `mcla-rs/crates/game/src/main.rs`
- `mcla-rs/crates/game/src/mc4.rs`
- `mcla-rs/crates/game/src/video.rs`
- `mcla-rs/crates/game/src/port_text.rs`
- `mcla-rs/crates/game/src/port_text_tests.rs`
- `tools/audit_spanish_strings.py`

## Segunda revisión de fuentes congeladas: adaptador de teclas

Se revisaron de nuevo `mc4.rs`, `port_text.rs`, `glyphs.rs` y el último cambio
GPS de `flash_ui.rs` después de congelar las fuentes (catálogo de 120 entradas).
No se identificaron defectos nuevos en esta revisión estática.

- El macro del catálogo genera `key_label(language, name)` como adaptador de presentación:
  inglés devuelve el mismo `&str` recibido y español solo sustituye los alias declarados.
  Los tokens desconocidos conservan el contenido y la vida del argumento. Las 40 etiquetas
  de teclas y cuatro etiquetas de prompt son propias del teclado/ratón del port, acreditado
  por la cabecera de `config/src/keybinds.rs`; no se modificó dicho módulo.
- `KEYS`, `ACTS`, `Key::name`, `Key::code`, índices, asignación y serialización de bindings
  están intactos. `mc4_ctrl_page_in` usa el adaptador únicamente para construir la columna
  de valores. La generación de filas, la obligación de asignar, la captura, los props, las
  acciones, los iconos y las escrituras de partida conservan el flujo previo.
- `glyphs::key_name` delega en `key_name_on(icon, screen)` sin idioma; todos sus literales y
  casos originales permanecen iguales. Solo `prompt`, `key_caps` y `button_name` formatean
  el texto que se dibuja. `Device::Xbox` y `Device::PlayStation` conservan las ramas, marcos
  y nombres previos. `MARK`, el token de tecla marcada y `GPS_KEY` no se traducen ni mutan.
- Inglés mantiene incluso las variantes abreviadas de prompts: `PGUP`, `PGDN`, `BKSP`
  permanecen tal cual porque `key_label(English, name)` devuelve el argumento, no la
  etiqueta canónica del catálogo.
- La última línea de `FlashPhone` transforma únicamente el texto de `btn_back` de teclado,
  conservando nombre de clip, color `0xff30_3030` y la condición por dispositivo. El GPS
  continúa usando su token original para controles y solo muestra su etiqueta española.
- Los tests añadidos comprueban una presentación sin mutar el modelo y una serialización
  sintética del esquema de bindings. Su éxito no sustituye verificación visual con película
  original, anchos reales ni recursos españoles. No se ha ejecutado ningún test por este revisor.

La prueba de glifos analiza el inventario real ya existente de `hud.rs`, excluye diálogos
nativos de GPU y respeta la mayusculización de mensajes del HUD. Es una comprobación de
cobertura de caracteres; no mide ancho ni certifica ausencia de desbordamientos.
