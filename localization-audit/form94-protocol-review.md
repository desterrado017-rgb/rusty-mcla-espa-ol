# Revisión independiente del protocolo de coleccionables

Base de la revisión: `05c25f63564a257a8f4fe518978d1fd8c8106e65`, corrección del marcador sobre `df1671b`. Fuentes nuevas: `collectable_tutorial.rs`, `collectable_tutorial_tests.rs` y `collectable_tutorial_stats.rs`. Evidencia: informe técnico aportado por el usuario, leído como datos, y aclaración posterior de que botones/pausa nativos siguen sin verificar. Revisión de solo lectura: este revisor no compila, modifica fuentes ni accede a recursos/partidas originales.

## Resultado de la revisión final de fuentes

El modelo distingue correctamente `xTCO` de `gCOL`, mantiene el estado de ejecución transitorio y no introduce una marca persistente alternativa. La ruta de recogida real no está conectada: las declaraciones de módulos no activan un panel sin política nativa de cierre/pausa. La entrega debe indicar ese límite explícitamente y no afirmar implementación visual completa.

El puente de estadísticas reutiliza `mc4_stat`/`mc4_stat_submit(..., 3, 1.0)`. No añade `xTCO` a `tutorials_shown`, no reemplaza bloques MC4 incompatibles y no cambia el importador. La llamada de autoguardado se sitúa después de la asignación y antes de presentar el progreso. Conserva las restricciones existentes del gestor y no fuerza una escritura rechazada. Los tests de persistencia usan modelos JSON y un grafo de misión sintético para ejecutar la restauración real; no usan partidas del usuario.

El payload Flash selecciona título/descripción por las claves del formulario94 y StringTable. Conserva los textos EN/ES del informe y permite el respaldo inglés por clave. Los prompts llegan del futuro adaptador; una lista vacía no se presenta como prueba de que el original no tenga botones. No se deduce una pausa ni se copia A de otro tutorial.

## Puntos detectados y corregidos durante la revisión

1. Se detectó que `Flow::result` volvía a `Idle` antes de aplicar las estadísticas, autoguardado y progreso. Las fuentes finales mantienen `AwaitingEffects` hasta terminar la secuencia. `collectable_tutorial_finish` valida la finalización pendiente, aplica la asignación, llama al callback de autoguardado, publica el progreso y solo entonces confirma los efectos. El estado continúa ocupado incluso si el gestor rechaza una asignación, hasta terminar esa misma secuencia; después conserva la posibilidad original de volver a ser elegible si xTCO sigue cero.
2. Se detectó una espera inicial de100ms no acreditada. Ahora la primera consulta de visibilidad queda lista inmediatamente al reconocer que el formulario se mostró; las siguientes consultas esperan100ms no escalados. Los tests separan explícitamente primera consulta y reconsultas.
3. La finalización es de un solo uso y tiene identidad transitoria. Un token de otro flujo se rechaza antes de cualquier cambio de estadística, petición de guardado o mensaje, incluso para la misma cantidad recogida. Esa identidad no se guarda ni sustituye xTCO.

Las dos observaciones iniciales están resueltas en las fuentes revisadas. No se identificaron otros defectos de producción en este alcance. La API sigue sin conectarse a la recogida ni a entradas/pausa nativas; no debe anunciarse como tutorial ya activado o autoguardado ya integrado al loop de App.

## Límites

La compilación y las pruebas son responsabilidad del agente principal. Esta revisión no valida Windows ni presentación original. Los pequeños textos de prueba se transcriben del informe; no se incluyen los binarios originales. No se ha detectado modificación de Henry, máscara de apertura, catálogo120, dependencias o versiones en los archivos revisados.
