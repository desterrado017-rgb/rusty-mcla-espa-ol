# Fidelidad Xbox 360: trabajo pendiente

## COLLECTABLE-FIRST-TUTORIAL — aviso del primer coleccionable

**Prioridad:** alta. **Base de esta continuación:** `df1671b`.
**Estado:** marcador auténtico corregido; backend de disparador, visibilidad y
persistencia implementado para pruebas. **Panel todavía no activado en la
recogida:** faltan los controles y la pausa nativos y el adaptador de App.

El informe original aportado por el usuario acredita el formulario **94**, las
claves `tutorial_collectables_title` / `tutorial_collectables_desc`, la condición
`xTCO == 0` y las exclusiones de tutoriales/red/conexión/editor. Tras cerrarse el
formulario: `xTCO = 1` → `CAREER_AUTOSAVE` → progreso normal.

`xTCO` no es `gCOL` ni depende de `collected.len() == 1`. La interpretación anterior
como indicador de todos los coleccionables encontrados queda corregida. El token
real de progreso es `<D>NUM_COLLECTABLE_COLLECTED</D>`; la variante con espacios
que usaba el port era incorrecta y se corrigió en `05c25f6`.

La parte preparada comprende estado transitorio, consulta inmediata de visibilidad
y reconsultas cada 100 ms, resultado de cierre, asignación de la estadística real,
petición de autoguardado y progreso posterior. El payload Flash toma sus textos
originales de StringTable. No se introduce un bool guardado ni una migración.

El usuario confirmó que no dispone aún de evidencia del botón de cierre ni de la
pausa. Por eso quedan pendientes:

1. Verificar entradas, cierre y pausa del formulario nativo 94 mediante código
   original o una observación Xbox 360/Xenia con pulsaciones y simulación visibles.
2. Conectar la recogida al backend y al formulario real con esa política verificada,
   conservando la actualización de `gCOL` y el estado local previos al panel,
   enlazando `CAREER_AUTOSAVE` al guardado existente y conservando sus restricciones.
3. Reproducir y validar visualmente primera recogida elegible, posteriores, carga,
   supresiones y avisos de trucos en los hitos correspondientes.

[Contrato, implementación, pruebas y límites](localization-audit/first-collectable-tutorial.md).
La tarea sigue abierta. Se conservan las 120 entradas, Henry, la transición inicial,
recursos y partidas. No se publica un ejecutable.
