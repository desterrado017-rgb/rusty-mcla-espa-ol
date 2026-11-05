# Verificación del seguimiento de coleccionables y apertura

Base: `5c740ad2c0c29528193ecd16fc6dfa4ecb4e1348`. Pruebas Linux x64 con Rust
1.99.1 disponible en la instancia. Los workflows continúan fijados a Rust
1.99.0 y WinLibs; no se modifican dependencias ni lockfiles y no se compila ni
publica un ejecutable Windows de este seguimiento.

## Regresión demostrada antes y después

Tras extraer el bloque real de UI sin cambiar su comportamiento, el harness
produjo este resultado antes de aplicar la corrección de cobertura:

```text
opening_movie_eof_keeps_black_until_the_transition_tick ... FAILED
opening_movie_skip_keeps_black_until_the_transition_tick ... FAILED
test result: FAILED. 2 passed; 2 failed; 0 ignored
```

Ambos fallos detectaban que el fotograma de fin exponía el mundo antes del tick
de transición. Los dos controles de otros contextos pasaban. La salida completa
se conserva en [opening-red.log](opening-red.log). Después de conservar y pintar
la máscara retenida, los cinco tests de apertura pasan, incluidos el control
adicional de entradas y el contexto de atracción.

Las cuatro pruebas de coleccionables pasan usando StringTable y centinelas
sintéticos EN/ES: cantidades 0,1,9,10,49,50,100; plantillas con/sin denominador;
marcador repetido, ajeno o incompleto; textos sin marcador; respaldo inglés por
clave española vacía/ausente; clave inexistente; tablas inmutables. No incorporan
frases oficiales ni suponen un denominador español.

## Comandos y resultados finales

Desde `mcla-rs/`, con las bibliotecas de desarrollo Linux del entorno:

```sh
cargo +1.99.1 test --locked -p mcla --features sdl3-sys/sdl-unix-console-build --no-run
cargo +1.99.1 test --locked -p mcla-config -p mcla-formats
```

La opción SDL solo permite compilar en esta instancia Linux. No cambia la
configuración Windows. Se utilizó el ejecutable de pruebas indicado por Cargo
para ejecutar **exactamente por nombre**, con `--test-threads=1`, los 51 tests de
[followup-focused-tests.json](followup-focused-tests.json). Cada resultado fue
comprobado como `1 passed; 0 failed; 0 ignored`, no simplemente código de salida
cero con cero tests seleccionados.

La lista contiene las 41 regresiones focalizadas anteriores, las cuatro nuevas
de coleccionables, las cinco de UI de película y una de cadencia de diagnósticos.
Se puede reproducir desde la raíz, sustituyendo el nombre del harness por el
indicado en la compilación:

```sh
python3 - mcla-rs/target/debug/deps/mcla-e7d298343089f2e5 <<'PY'
import json, subprocess, sys
from pathlib import Path
tests = json.loads(Path('localization-audit/followup-focused-tests.json').read_text())
for row in tests:
    result = subprocess.run([sys.argv[1], '--exact', row['test'], '--test-threads=1'],
                            text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    print(result.stdout, end='')
    assert result.returncode == 0
    assert 'test result: ok. 1 passed; 0 failed; 0 ignored;' in result.stdout
PY
```

| Conjunto final | Correctas | Fallidas | Ignoradas |
|---|---:|---:|---:|
| Juego: selección focalizada | 51 | 0 | 0 |
| Configuración | 24 | 0 | 0 |
| Formatos | 59 | 0 | 8 |
| Total de este seguimiento | 134 | 0 | 8 |

Los dos fallos RED anteriores eran deliberados para comprobar la regresión y
ya no forman parte del resultado final. No se ejecutó la suite completa del
juego ni se cuentan las ocho pruebas ignoradas como validación de recursos.
Los avisos de compilación de código sin usar siguen fuera del alcance.

## Qué preservan las regresiones

- Las 120 entradas bilingües, marcadores y glifos del catálogo; inglés y
  español, controles, bindings y etiquetas físicas persistidas.
- La raíz de navegación de Henry sin `--data`, la solicitud del mismo rival
  después de su escena en ambos idiomas y sus diagnósticos previos.
- Preferencias de subtítulos `false`/`true`, valores predeterminados `false`,
  importación/exportación de opciones y metadatos de idioma.
- Fin natural y salto de película mediante `Video::show` y la misma UI usada
  por producción; renders sin tick intermedio y posterior máscara del tick.
- Partidas existentes, atracción, película activa/sin película, consumo de
  entradas y petición de desvanecimiento, sin adelantar el fin.
- Diagnósticos: repetición acotada por cambios o cada60frames, sin mutar estados
  del juego. Los tests de UI usan MCV sintético en memoria anónima y no esperan
  al reloj, abren películas originales ni necesitan GPU.

## Alcance preservado y límites

Los **144 archivos** de [followup-protected-files.json](followup-protected-files.json)
son idénticos byte a byte a `5c740ad`, incluidos catálogo, Henry, cámara, streaming,
renderizador, parser, configuración, manifiestos, lockfiles y workflows.
`git diff --check` pasa sobre la entrega completa.

Los nuevos diagnósticos se activan únicamente con `MCLA_DIAG_OPENING=1` y
reutilizan el log existente. No se ejecutó el juego contra datos o guardados del
usuario y no se modificaron recursos originales.

La cobertura negra queda demostrada en la UI sin GPU; el artefacto concreto
debe confirmarse visualmente en Windows con INTRO NORMAL, fin completo y salto
temprano, usando el perfil de prueba aislado. No se atribuye un sector/material
ni se declara resuelto otro fallo de profundidad a partir de esos tests.
No se afirma disponibilidad de subtítulos españoles sin los CSV originales.
