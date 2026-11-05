# Verificación del protocolo original de coleccionables

Base de las pruebas: `df1671b`; marcador auténtico publicado en `05c25f6`.
Las comprobaciones usan el informe original aportado por el usuario. La
[actualización de autocierre](form94-autoclose-duration.md) registra la observación
posterior: se cierra automáticamente sin pulsar un botón y la conducción continúa.
Solo queda por acreditar la duración precisa; estas pruebas no la validan ni son
el origen de la observación de conducción continua.

## Resultado y límite funcional

**68 pruebas focalizadas correctas, 0 fallidas y 0 ignoradas.** Son las 51
regresiones existentes, una prueba adicional con las frases auténticas de progreso
y 16 casos nuevos del backend/protocolo/persistencia (7 + 9).

Esto prueba el marcador activo y las APIs preparadas. **No significa que la
primera recogida ya muestre el formulario 94:** el pickup no se ha conectado al
backend. No hay duración o política de pausa inferida, ni dispatcher activo de
`CAREER_AUTOSAVE`; ese adaptador de App sigue pendiente.

## Marcador: RED y GREEN

Se compiló el módulo real `collectable_text.rs` como harness independiente con
las bibliotecas locales existentes, usando Rust 1.99.1. En una copia temporal
se restauró únicamente el reemplazo antiguo con espacios. El nuevo test de texto
auténtico falló con el recurso original aún sin sustituir:

```text
left:  "COLLECTABLE <D>NUM_COLLECTABLE_COLLECTED</D> OF 60 FOUND"
right: "COLLECTABLE 0 OF 60 FOUND"
test result: FAILED. 0 passed; 1 failed
```

Con el identificador exacto con guiones bajos pasan las cinco pruebas del módulo.
Los fixtures reales son transcripciones reducidas del informe, no binarios
originales. Conservan el doble espacio español y el denominador aunque el total
del llamador difiera. Se cubren primeras cantidades, posteriores y múltiplos de
diez hasta 60, más conservación de tablas y respaldos por clave.

La variante antigua con espacios se mantiene como control negativo: no se
normalizan ni traducen identificadores desconocidos.

## Harness completo y selección reproducible

En `mcla-rs/`, con las bibliotecas de desarrollo Linux del entorno:

```sh
cargo +1.99.1 test --locked -p mcla --features sdl3-sys/sdl-unix-console-build --no-run
```

Desde la raíz, pasando el ejecutable de **pruebas** indicado por Cargo:

```sh
python3 - mcla-rs/target/debug/deps/mcla-e7d298343089f2e5 <<'PY'
import json, subprocess, sys
from pathlib import Path
tests = json.loads(Path('localization-audit/form94-focused-tests.json').read_text())
for row in tests:
    result = subprocess.run([sys.argv[1], '--exact', row['test'], '--test-threads=1'],
                            text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    print(result.stdout, end='')
    assert result.returncode == 0
    assert 'test result: ok. 1 passed; 0 failed; 0 ignored;' in result.stdout
PY
```

Se comprobaron por nombre los 68 resultados: ninguno quedó sin ejecutar por un
filtro vacío. La lista exacta está en
[form94-focused-tests.json](form94-focused-tests.json). No se ejecutó toda la
suite del juego ni se validaron fixtures originales ausentes.

## Cobertura nueva

- Texto real EN/ES de progreso y payload original de título/descripción,
  StringTable seleccionada, tildes, espacios y respaldo inglés por clave.
- `xTCO == 0` con cualquier número recogido; `xTCO` no cero y las cuatro
  exclusiones; tutoriales suprimidos que pueden ser elegibles después.
- Ninguna marca al solicitar o mientras el formulario esté abierto; confirmación
  real de apertura, consulta inicial inmediata y reconsultas a 100 ms de tiempo
  no escalado. Visibilidad desconocida, formulario ajeno o resultado fuera de
  orden no autorizan efectos.
- El resultado del formulario no se usa como código de botón. El flujo sigue
  ocupado hasta terminar sus efectos; un token ajeno no autoriza estadísticas,
  evento ni mensaje, aunque tenga el mismo contador.
- Orden probado en Game: asignar `xTCO/0` con operación 3, callback
  `CAREER_AUTOSAVE`, progreso auténtico y liberación del flujo. El callback ve
  ambas tablas actualizadas antes de que se sustituya el mensaje previo.
- JSON de Career/SaveData, estados 0/1/no cero, precedencia existente entre
  estadísticas, progreso y `gCOL` independientes, MC4 tipado con campos ajenos
  y MC4 incompatible/raw conservados. No se crea un flag nuevo ni se añade
  `xTCO` a `tutorials_shown`.
- Se respetan límites del gestor de estadísticas: no se finge persistencia si
  una inserción se rechaza ni se cambia por ello el resto de efectos originales.
- Las 51 regresiones anteriores incluyen catálogo de 120 entradas, Henry sin
  `--data`, máscara de apertura, teclas persistidas y preferencias de subtítulos.

Los guardados de prueba permanecen en memoria. Tres pruebas crean solo XML de
misión sintético temporal para ejercitar `Career::restore` con una carrera válida;
no usan `load_save` sobre directorios del usuario ni acceden a sus partidas.

## Integridad y alcance

Los 146 archivos de [form94-protected-files.json](form94-protected-files.json)
son idénticos byte a byte a `df1671b`: incluyen catálogo, Game, vídeo, Henry,
renderer, estadísticas existentes, formatos, configuración, manifiestos,
lockfiles y workflows. `main.rs` solo añade declaraciones de módulos preparados;
su pickup no se modifica. La descripción de `xTCO` en `tools/mc4save/small.py`
se corrige y el archivo pasa el parser de Python.

La revisión independiente [form94-protocol-review.md](form94-protocol-review.md)
detectó y ayudó a corregir dos problemas antes de compilar: retrasar la primera
consulta y permitir reentrada antes de aplicar los efectos. No identificó otros
defectos en el código final revisado. `git diff --check` pasa.

Las pruebas locales usan Rust 1.99.1 disponible en Linux. Los workflows conservan
Rust 1.99.0, WinLibs y dependencias bloqueadas. No hay compilación o publicación
Windows ni validación visual nueva. No se incorporan los binarios originales.

Queda pendiente la duración precisa, mediante evidencia dirigida o una
observación instrumentada del original; la conexión real a recogida, formulario y autoguardado de App;
y la comprobación visual y de avisos de trucos al completar esa integración.
