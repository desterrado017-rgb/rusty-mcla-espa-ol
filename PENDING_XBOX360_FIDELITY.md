# Fidelidad Xbox 360: trabajo pendiente

## COLLECTABLE-FIRST-TUTORIAL — aviso del primer coleccionable

**Prioridad:** alta. **Base del backend:** `df1671b`; actualización desde `a8512f0`.
**Estado:** marcador auténtico corregido; backend de disparador, visibilidad y
persistencia implementado para pruebas. **Panel todavía no activado en la
recogida:** faltan la duración precisa y el adaptador de App.

**Cierre confirmado por observación del usuario:** automático tras unos segundos,
sin necesitar pulsar un botón. El botón de cierre deja de ser un requisito pendiente.
**Conducción confirmada por el usuario:** continúa mientras se muestra el panel;
no se introducirá una pausa por este tutorial.

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

El único dato de fidelidad pendiente es la **duración precisa**. El resultado numérico se mantiene
opaco y no constituye un bloqueo independiente: el script lo consulta y lo ignora.
La integración y aceptación pendientes son:

1. Acreditar el tiempo de autocierre (origen, base de tiempo y transición)
   mediante análisis dirigido o una observación instrumentada del original.
2. Conectar la recogida al backend y al formulario real con esa política verificada,
   conservando la actualización de `gCOL` y el estado local previos al panel,
   enlazando `CAREER_AUTOSAVE` al guardado existente y conservando sus restricciones.
3. Reproducir y validar visualmente primera recogida elegible, posteriores, carga,
   supresiones y avisos de trucos en los hitos correspondientes.

[Contrato, implementación, pruebas y límites](localization-audit/first-collectable-tutorial.md).
La [investigación de fidelidad desde `188dc57`](localization-audit/form94-fidelity-investigation.md)
separa script, clase nativa y Flash; documenta los recursos y funciones exactos
que faltan. La [actualización de autocierre y duración](localization-audit/form94-autoclose-duration.md)
incorpora el nuevo análisis R27 y sustituye la incertidumbre anterior sobre el
cierre. `is_shown()` del port no sustituye `Form_GetFormVisible`.
La tarea sigue abierta. Se conservan las 120 entradas, Henry, la transición inicial,
recursos y partidas. No se publica un ejecutable.
