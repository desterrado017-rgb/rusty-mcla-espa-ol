# Revisión final de presentación y Getvar/lang

Revisión estática contra `b68c4d02d384f611d176170bb893053a697503af`. No se han ejecutado compilaciones ni cargado recursos o partidas. El texto original de Rockstar no se ha sustituido por traducciones manuales.

## Fallo demostrado y prueba focalizada

La nueva cadena del port `SecondKey` utilizaba ` (2.ª TECLA)`. El carácter `ª` (U+00AA) no existe en el inventario de la fuente bitmap de reserva, `hud.rs::FONT_ROWS`. `TextFont::glyph` solo consulta ese inventario, convierte mayúsculas ASCII y contempla tres sustituciones ASCII; `TextFont::draw` avanza el cursor sin dibujar cuando falta un glifo. Por tanto, esa ruta omitía la `ª`.

`port_ui_phase1` ha corregido únicamente esta cadena C a ` (TECLA 2)`. El nuevo `game/src/port_text_tests.rs` comprueba EN/ES de todo el catálogo destinado al juego usando el inventario real incluido desde `hud.rs`, sin duplicar sus letras ni imponer límites arbitrarios de caracteres. Se excluyen los doce textos de los diálogos nativos GPU. Esta prueba falla si se restaura ` (2.ª TECLA)`.

Las cadenas `BuildStreetRaceFailed`, `VehicleReady` y `VehicleLoadFailed` conservan su caja original en el catálogo: los consumidores reales de `main.rs` las envían a `Game::show_message`. Antes del dibujo, `game.rs` convierte `self.message.to_uppercase()` tanto al publicar el splash Flash (línea5182) como al pintar la reserva bitmap (línea5195). La prueba reproduce esa transformación únicamente para estos tres casos; no atribuye conversión Unicode a `TextFont::glyph`.

Esta prueba certifica compatibilidad con el inventario de glifos declarado por el lector actual de la fuente bitmap; no certifica medidas de las fuentes originales `.r27` ni que las texturas ausentes contengan todos los glifos esperados.

## Anchuras y campos que necesitan validación con recursos

No es posible afirmar que los nuevos textos quepan en los menús Flash con los archivos disponibles. `mcla_flash::player::measure_string` usa los avances reales de `Character::Font` del mismo `Movie`. Sin fuentes devuelve0, como reconoce la prueba `measure_string_without_fonts_is_zero`; emplear esa respuesta como comprobación de ajuste produciría un falso positivo. El renderer `draw::edit_text` usa los mismos avances, `EditText.rect[0]`, altura y espaciado para envolver líneas; sin el `.r27` no conocemos los límites reales de los campos ni sus máscaras/transformaciones.

Prioridad visual:

- DISPLAY/PANTALLA: parejas de etiquetas y valores como `CONTADOR DE FPS` / `FPS + TIEMPO POR FOTOGRAMA`, `SINCRONIZACIÓN VERTICAL` / `DESACTIVADO`, `MODO DE VENTANA` / `PANTALLA COMPLETA`, más el valor dinámico del monitor. La nueva longitud puede cambiar la anchura calculada por el propio SWF o cruzar columnas.
- PERFORMANCE/RENDIMIENTO y controles: `RESOLUCIÓN DE SOMBRAS`, `DISTANCIA DE DIBUJADO`, `ZONA MUERTA DEL STICK`, `CONTROLES DEL TECLADO`, la captura `PULSA UNA TECLA - ESC: CANCELAR` y los nombres de acciones con el sufijo ` (TECLA 2)`. Comprobar columnas, filas vecinas y barra inferior.
- HUD: `ERROR AL GUARDAR LA CAPTURA`, `CAPTURA GUARDADA`, avisos de vehículo y cadenas con nombres dinámicos. `show_message` usa el splash original o una línea centrada sin envolver; aún falta demostrar que caben con fuente y máscara reales.
- Ayudas PC de mapa, editor, vinilos, rendimiento y foto: especialmente `MapKeyboardHint`, `RaceEditorKeyboardHint` y `PhotoKeyboardHint`. Se dibujan en una sola línea, con posiciones o alineaciones fijas. No se ha introducido una adaptación de tamaño o una abreviatura sin ver el resultado.

El menú de reserva dibujado por Rust tampoco garantiza ausencia de solapamiento: `game.rs` asigna anchura760*sc a páginas de opciones, coloca la etiqueta a x0+34*sc y el valor alineado a x0+726*sc, ambos a15*sc. El espacio combinado verificable para ambas columnas es692*sc, sin separación explícita. Se debe comparar la suma de medidas reales de cada pareja con ese intervalo; no con número de caracteres. La ruta con `UiArt.font` requiere la textura `pause-font100-height500-000` del recurso original; la ruta sin esa fuente dibuja con `egui::FontId::proportional(size*1.35)`. La estimación de reserva `Game::text_width` usa bytes*size*0.7 y no coincide necesariamente con egui: no sirve para certificar ajuste de texto español.

Un test adicional sin recursos podría medir precisamente solo la ruta egui con `Context::run` + `fonts.layout_no_wrap` usando la misma `FontId` y comparar los rectángulos de etiqueta/valor con las posiciones anteriores. No probaría la ruta habitual con fuentes del juego ni los menús Flash. No se ha añadido una prueba que declare verificada esa presentación general a partir de la fuente de reserva.

Prueba de ajuste propuesta al disponer de recursos: cargar la película real de pausa/prompt/HUD, construir páginas con las cadenas C EN/ES y combinaciones reales de valores, ejecutar el mismo flujo de presentación y recoger los `EditText` visibles. Validar que existen los glifos, que las líneas resultantes y sus rectángulos transformados permanecen dentro del campo/máscara, y que las cajas de etiqueta/valor o filas adyacentes no se cruzan. Añadir captura de esas páginas en Windows a1280x720 y1920x1080. Para las ayudas de una línea, medir con la fuente efectiva de `Game::text` y comparar el rectángulo de dibujo con el borde de pantalla/panel que realmente se usa. No asumir fuentes ni anchuras de fixtures artificiales.

Recursos mínimos para esta comprobación de Flash/font: `resources/ui/pause/0xaed9b6c9.r27`, `resources/ui/prompt/0x3ae388d8.r27`, `resources/ui/hud/0x1f0e1cda.r27`, `resources/ui/shared_latin.r9` y las tablas `ui/*` que realmente carga la instalación. Añadir `resources/ui/navsys/0x733d5b6e.r27` y `resources/ui/garage/0xfcc35320.r27` para comprobar también columnas del editor de teclado y garaje. No se requieren ISO, guardados ni reconstrucción de datos.

## Alcance del cambio Getvar/lang e inicialización

La diferencia de producción solo afecta la respuesta a `FSCommand:Getvar` con argumento exacto `lang`: el host devuelve `strings.lang`0/1 antes del handler común; el handler genérico conserva una selección existente y usa0 si no existe. No se han cambiado `Player::new`, `tick`, orden de fotogramas, los cuatro ticks de carga, acumuladores, `aspect`, `platform`, `zone`, `setpolarcam` ni la selección/carga de tablas.

El valor de `_global.lang` puede cambiar qué rama de texto o formato numérico ejecuta la propia película. Ese es el efecto buscado; no se puede certificar la ejecución interna de una película ausente. El cambio no llama navegación, creación de vehículos, IA, carrera, rutas de datos ni código de guardados, ni cambia sus condiciones. Mantiene la corrección Windows de Henry de la base validada. Las pruebas sintéticas demuestran la devolución del idioma en AVM1 y lectura seleccionada de cadenas; no sustituyen una prueba visual y de la primera carrera en Windows.

## Presentación de nombres de teclas del port

`glyphs.rs` separa ahora los tokens históricos `key_name_on(icon, screen)` de su presentación. `key_name` conserva su firma y valores; no se modifican los códigos de teclas ni los bindings. Las tres salidas `prompt`, `key_caps` y `button_name` aplican `port_text::key_label` al último paso de la ruta Keyboard. Los helpers privados reciben idioma, dispositivo y pantalla explícitos; las nuevas pruebas no cambian los globals existentes. Las rutas Xbox/PlayStation conservan texto, iconos, nombres de botones y ausencia de key-caps. Se comprueban MOUSE, ARROWS, LEFT/RIGHT, PGUP/PGDN, BKSP, overrides MARK y tokens desconocidos. Los rótulos españoles RATÓN, FLECHAS, IZQ./DER., REPÁG/AVPÁG y RETROCESO son texto C del port, no cadenas de Rockstar. Sus anchuras en las películas originales siguen sujetas a la comprobación de recursos indicada arriba.
