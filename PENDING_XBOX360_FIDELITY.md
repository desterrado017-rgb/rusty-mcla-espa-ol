# Fidelidad Xbox 360: trabajo pendiente

## COLLECTABLE-FIRST-TUTORIAL — aviso del primer coleccionable

**Prioridad:** alta. **Base:** `48d2792ab84904dbfe3e5071d53f2bb0ad4afadb`.
**Estado:** registrado e investigado; implementación y pruebas nuevas pendientes
de verificar el script original y sus recursos. No está resuelto por la
corrección del marcador dinámico.

Comportamiento solicitado para Midnight Club: Los Angeles Complete Edition:

1. En la primera recogida (1/60), presentar el tutorial original «COLLECTABLES»
   sobre el desbloqueo de trucos, con su diseño y controles originales.
2. En las posteriores (2/60, 3/60, etc.), presentar solo el progreso normal.
3. Obtener todo texto oficial de los recursos originales españoles, conservando
   el inglés y el respaldo por clave. No redactar una traducción del tutorial.
4. Reproducir las condiciones y persistencia de `Game/MiscTasks/Collectables`.
   No repetir un tutorial ya mostrado ni migrar partidas por suposición.
5. Mantener `collectable_text::found_message`, las 120 entradas, Henry y las
   correcciones anteriores. No modificar recursos ni publicar un ejecutable.

Ya están identificados el formulario **94** y las claves
`tutorial_collectables_title` / `tutorial_collectables_desc`. Falta acreditar
su disparador, estado persistente, cierre, pausa y secuencia con el progreso.
Asignar directamente `tutorial_box` no reproduce automáticamente sus controles.

El [informe de investigación y matriz de pruebas](localization-audit/first-collectable-tutorial.md)
detalla los hallazgos y archivos necesarios. La tarea no se cerrará hasta probar
primera recogida, recogidas posteriores y carga de partida conforme al contrato
original, y validar visualmente el panel y sus controles en Windows.
