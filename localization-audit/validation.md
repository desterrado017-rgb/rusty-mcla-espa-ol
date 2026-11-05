# Verificación de la fase de interfaz española

Base: `b68c4d02d384f611d176170bb893053a697503af`. Linux x64, Rust 1.99.1
disponible en la instancia; las versiones de Actions siguen en Rust 1.99.0 y
WinLibs UCRT, sin modificar workflows, manifiestos ni lockfiles. No se crea un
artefacto Windows ni se ejecuta el juego contra datos o partidas del usuario.

## Comandos y resultados

Desde `mcla-rs/`, con las bibliotecas de desarrollo Linux/SDL del entorno:

```sh
cargo +1.99.1 test --locked -p mcla-flash
cargo +1.99.1 test --locked -p mcla-config -p mcla-formats
cargo +1.99.1 test --locked -p mcla --features sdl3-sys/sdl-unix-console-build --no-run
```

La opción SDL se usa solamente para compilar en este entorno Linux. No cambia
el manifiesto ni la compilación Windows. Desde la raíz del repositorio:

```sh
python3 tools/test_spanish_ui.py mcla-rs/target/debug/deps/mcla-e7d298343089f2e5
python3 tools/audit_spanish_strings.py --output /tmp/mcla-spanish-audit-recheck
cmp localization-audit/spanish-literals.tsv.gz /tmp/mcla-spanish-audit-recheck/spanish-literals.tsv.gz
cmp localization-audit/spanish-literal-coverage.tsv /tmp/mcla-spanish-audit-recheck/spanish-literal-coverage.tsv
git diff --check b68c4d02d384f611d176170bb893053a697503af
```

El nombre del ejecutable de **pruebas** depende del compilador; utilizar el que
indique `cargo ... --no-run`. No pasar el ejecutable normal del juego al script.
Los 41 tests se ejecutaron exactamente por nombre, cada uno en un proceso de
pruebas, con `--test-threads=1`, y se comprobó que cada resultado fuera realmente
`1 passed; 0 failed; 0 ignored`. Su lista y resultados están en
[`focused-tests.json`](focused-tests.json).

| Conjunto | Correctas | Fallidas | Ignoradas |
|---|---:|---:|---:|
| Juego: selección focalizada | 41 | 0 | 0 |
| Flash | 28 | 0 | 2 |
| Configuración | 24 | 0 | 0 |
| Formatos | 59 | 0 | 8 |
| Total de esta verificación | 152 | 0 | 10 |

La compilación final del harness se completó correctamente. Emite avisos de
código sin usar; no se han aplicado correcciones ajenas a la localización.
Esta selección no equivale a ejecutar toda la suite de `mcla` ni a validar los
fixtures originales ausentes. Las diez pruebas ignoradas siguen ignoradas y
no certifican sus datos.

## Qué demuestran las pruebas

- Catálogo: todas las 120 entradas tienen EN/ES, con marcadores dinámicos
  conservados y sin traducir nombres de APIs/unidades.
- Flash: una película AVM1 sintética ejecuta `Getvar/lang` y `Getstring`.
  La regresión del handler común falló antes de corregirlo: devolvía `Some(0)`
  donde se esperaba `Some(1)`. Después pasa para selección 0, 1 y ausencia de
  selección. El host del juego además corrige un global previo equivocado.
- Dinero: selección de `G_Money`, override diagnóstico y respaldo inglés exacto.
- Presentación de controles: páginas reales sintéticas en ambos idiomas,
  títulos/columnas/prompts, mismas filas habilitadas, iconos, propiedades,
  bindings y estado de partida. Nueve acciones originales consultan recursos;
  seis acciones propias usan catálogo. Se comprueban respaldos por clave.
- Teclas: las 98 definiciones conservan código/nombre; las 81 asignables
  recorren serialización/deserialización real de bindings. El adaptador solo
  cambia la etiqueta presentada; inglés y tokens desconocidos son idénticos.
- Subtítulos: respeta `false` y `true`, mantiene texto/intervalos de la pista
  sintética y conserva opciones y metadatos de idioma al importar/exportar.
  Los predeterminados siguen en `false`.
- Glifos: se consulta el inventario bitmap real declarado en `hud.rs`, en vez
  de suponer soporte Unicode. La etiqueta propia evita `ª`, ausente, usando
  `(TECLA 2)`. Esto no prueba anchos ni texturas originales ausentes.
- Henry: vuelven a pasar la regresión de la raíz de navegación resuelta sin
  argumento de datos, la solicitud del mismo rival tras la escena en ambos
  idiomas y las comprobaciones de diagnóstico. Usan fixtures sintéticos.

## Preservación y límites

[`protected-files.json`](protected-files.json) registra 95 archivos cuyo contenido
es idéntico byte a byte a la base validada, incluidos todos los manifiestos,
lockfiles y workflows, configuración, formatos, navegación, Career, Boot,
lector de localización, lector CSV y módulos de juego esenciales. No hay un
cambio de versiones o dependencias para hacer pasar la compilación.

La auditoría reproducida conserva idénticos el gzip y el TSV de cobertura.
Las revisiones del diff de `game.rs` y `main.rs` comprueban cambios de texto,
sin alterar condiciones de carrera, físicas, IA ni ajustes gráficos.

No hay datos originales del juego ni partidas en estas evidencias. Las tablas,
fuentes, escenas Flash y CSV ausentes impiden certificar su contenido español o
ausencia de recortes. Los centinelas de las pruebas no son diálogos de Rockstar.
La validación Windows de Henry corresponde a la base `b68c4d0`; deberá repetirse
en un futuro binario de esta fase junto con la revisión visual. Esta entrega
no publica ese binario.
