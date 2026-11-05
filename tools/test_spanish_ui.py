#!/usr/bin/env python3
"""Run the audited, asset-independent tests using an already compiled test harness.

Build with `cargo test --locked -p mcla --no-run` in mcla-rs, then pass the
reported test executable (not the normal game executable) as the sole argument.
Linux SDL builds may also require --features sdl3-sys/sdl-unix-console-build.
"""
import json
from pathlib import Path
import subprocess
import sys


def main():
    if len(sys.argv) != 2:
        raise SystemExit(f'Usage: {sys.argv[0]} PATH_TO_TEST_HARNESS')
    binary = str(Path(sys.argv[1]).resolve())
    root = Path(__file__).resolve().parents[1]
    records = json.loads((root / 'localization-audit/focused-tests.json').read_text())
    for record in records:
        name = record['test']
        result = subprocess.run([binary, '--exact', name, '--test-threads=1'],
                                text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        print(result.stdout, end='', flush=True)
        if result.returncode or 'test result: ok. 1 passed; 0 failed; 0 ignored;' not in result.stdout:
            raise SystemExit(f'Test failed or did not actually run: {name}')
    print(f'{len(records)} focused tests passed; none skipped.')


if __name__ == '__main__':
    main()
