# Coleccionables: marcador dinámico y contador duplicado

> Errata posterior: el informe original acredita el marcador
> `<D>NUM_COLLECTABLE_COLLECTED</D>`. La variante con espacios de esta revisión
> histórica era incorrecta; se corrigió en `05c25f6`. Véase
> [el contrato actualizado](first-collectable-tutorial.md).


Base examinada: `5c740ad2c0c29528193ecd16fc6dfa4ecb4e1348`, rama `feature/spanish-ui-completion-v0.14.0`. La plantilla española real no está disponible en el checkout. Esta revisión inicial usó el token supuesto con espacios y pruebas sintéticas. El informe original posterior lo desmintió; el token auténtico con guiones bajos y los fixtures originales reducidos están documentados en el contrato actualizado.

## Causa demostrada

En la base, `game/src/main.rs:4966` obtiene `COLLECTABLE_FOUND_MSG` mediante la StringTable seleccionada y lo copia sin expandir variables. La línea siguiente hace `format!("{}   {} / {}", msg.to_uppercase(), collected.len(), barrels.len())` sin distinguir plantilla oficial de fallback. Por ello cualquier `<D>…</D>` de la plantilla llega literalmente a la presentación, y se añade siempre otro contador del port aunque la plantilla ya tenga su formato de progreso.

El mismo bloque de dos líneas existe en el v0.14.0 oficial (`30d8b1f3b54557664d81bb7cfc9cc33caea53647:main.rs:4963–4964`). No es una modificación de las 120 entradas del catálogo español ni de la corrección de navegación de Henry. Activar español hace visible ese defecto de presentación preexistente; el contenido exacto inglés requiere sus recursos para contrastarlo visualmente.

## Mecanismo y evidencia disponible

Las variables dinámicas de texto se representan como `<D>nombre</D>`. El parser `formats/src/strtbl.rs` conserva los bytes de texto UTF-16 decodificados y `StringTable::get` selecciona el idioma con respaldo inglés por clave; no expande variables de estado del juego.

El port ya expande variables antes de mostrar otros textos: `game/src/mc4b.rs:643` (`fill_d`) sustituye el único bloque de las plantillas de tiempo GOAL_TIME/RACE_INFO_GOALATTACK; `game/src/game.rs:14963` sustituye por nombre `<D>Navmap_Bank_Amount</D>`. Hay otros consumidores nombrados de PayOffAmount y POST_BEAT_ME_THERE_REP_AMOUNT. La expansión pertenece al consumidor que conoce el valor de la variable, no a una traducción manual del recurso ni a la normalización del parser.

Esta evidencia y el marcador reportado demuestran que el número de coleccionables recogidos debe insertarse en la variable de conteo, conservando el formato que haya en la plantilla. No demuestran que la plantilla original contenga un denominador concreto ni un texto como «de 60». Esta corrección no presupone ninguno: no añade, traduce ni reescribe ese texto.

## Corrección aplicada

Nuevo `game/src/collectable_text.rs::found_message`:

- Lee la misma clave con `StringTable::get`, conservando selección de idioma y fallback inglés existente por clave.
- La versión inicial sustituía un token supuesto con espacios, lo que no resolvía la variable real. La versión corregida en `05c25f6` sustituye únicamente `<D>NUM_COLLECTABLE_COLLECTED</D>`, conservando prefijos, sufijos, denominadores, saltos, acentos y otras variables.
- No añade otro contador a una plantilla oficial. Tampoco intenta eliminar todos los tags como si fueran HTML.
- Si la clave no existe, conserva exactamente el fallback inglés anterior del port: `COLLECTABLE FOUND   n / total`.

`main.rs` registra el módulo y usa ese resultado en las dos líneas de presentación. Se mantiene la conversión a mayúsculas que ya hacía el llamador. No cambian la detección de recogida, inserción de índices, efectos, sonido, contador real ni llamada existente a guardar. No se ejecuta el juego ni se abre o escribe ninguna partida/dato original durante esta investigación.

## Pruebas realizadas

Las cuatro pruebas de `collectable_text::tests` no requieren recursos ni archivos:

1. `original_language_template_gets_one_count_without_an_appended_counter`: idioma inglés/español mediante `Language::table_index()` y StringTable real; cantidades 0,1,9,10,49,50,100; conserva plantilla completa, caracteres y marcadores; variar el total del llamador no reescribe un denominador del recurso; tabla inmutable.
2. `only_the_named_count_is_replaced_and_resource_layout_is_preserved`: plantilla sin denominador, variable repetida, otro bloque dinámico, contador literal, texto sin variable y marcador incompleto; ambos idiomas.
3. `missing_spanish_key_keeps_the_existing_english_resource_fallback`: clave española ausente o vacía usa recurso inglés y sustituye el contador sin añadir sufijo; selección permanece en español.
4. `absent_resources_preserve_the_previous_english_port_fallback`: clave ausente en ambas tablas y distintas cantidades/totales conservan la reserva previa exacta.

`git diff --check` pasa. El agente principal compiló y ejecutó las cuatro pruebas: 4 correctas, 0 fallidas y 0 ignoradas. Los comandos y la regresión completa están en [followup-validation.md](followup-validation.md). La frase original y su aspecto en Windows requieren comprobarse con los recursos del usuario, sin que los centinelas constituyan una traducción de Rockstar.
