#!/usr/bin/env python3
"""Inventory source literals at a fixed revision; this is not a translation classifier.

Only git and Python's standard library are required. Comments are excluded; tests,
identifiers and logs deliberately remain in the inventory for human review.
"""
import argparse
import csv
import gzip
import io
from pathlib import Path
import re
import subprocess


def literals(source):
    i = 0
    while i < len(source):
        if source.startswith('//', i):
            end = source.find('\n', i)
            i = len(source) if end < 0 else end
            continue
        if source.startswith('/*', i):
            depth = 1
            i += 2
            while i < len(source) and depth:
                if source.startswith('/*', i):
                    depth += 1
                    i += 2
                elif source.startswith('*/', i):
                    depth -= 1
                    i += 2
                else:
                    i += 1
            continue
        raw = re.match(r'(?:br|rb|r)(#*)"', source[i:i + 100])
        if raw and (i == 0 or not (source[i - 1].isalnum() or source[i - 1] == '_')):
            start = i
            end = source.find('"' + raw[1], i + raw.end())
            if end < 0:
                raise ValueError('Unclosed raw string')
            i = end + 1 + len(raw[1])
            yield start, source[start:i]
            continue
        start = i
        if source.startswith('b"', i):
            i += 1
        if source[i] == '"':
            i += 1
            while i < len(source):
                if source[i] == '\\':
                    i += 2
                elif source[i] == '"':
                    i += 1
                    yield start, source[start:i]
                    break
                else:
                    i += 1
            continue
        if source[i] == "'":
            char = re.match(r"'(?:[^'\\\n]|\\(?:[nrt0\\'\"]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}))'", source[i:])
            if char:
                i += char.end()
                continue
        i += 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--ref', default='b68c4d02d384f611d176170bb893053a697503af')
    parser.add_argument('--output', type=Path, default=Path('localization-audit'))
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]

    def git(*argv):
        return subprocess.check_output(['git', *argv], cwd=root).decode('utf-8')

    revision = git('rev-parse', args.ref).strip()
    extras = {
        'mcla-rs/crates/config/src/keybinds.rs',
        'mcla-rs/crates/formats/src/strtbl.rs',
        'mcla-rs/crates/flash/src/player.rs',
    }
    paths = [p for p in git('ls-tree', '-r', '--name-only', revision).splitlines()
             if p.endswith('.rs') and (p.startswith('mcla-rs/crates/game/src/') or p in extras)]
    args.output.mkdir(parents=True, exist_ok=True)
    buffer = io.StringIO(newline='')
    writer = csv.writer(buffer, delimiter='\t', lineterminator='\n')
    writer.writerow(['revision', 'file', 'line', 'literal_source', 'source_context'])
    coverage = []
    for path in paths:
        source = git('show', f'{revision}:{path}')
        lines = source.splitlines()
        count = 0
        for offset, literal in literals(source):
            line = source.count('\n', 0, offset) + 1
            writer.writerow([revision, path, line, literal, lines[line - 1].strip()])
            count += 1
        coverage.append((path, count))
    with (args.output / 'spanish-literals.tsv.gz').open('wb') as target:
        with gzip.GzipFile(filename='', mode='wb', fileobj=target, mtime=0) as compressed:
            compressed.write(buffer.getvalue().encode('utf-8'))
    with (args.output / 'spanish-literal-coverage.tsv').open('w', newline='') as target:
        writer = csv.writer(target, delimiter='\t', lineterminator='\n')
        writer.writerow(['revision', 'file', 'literal_count'])
        writer.writerows((revision, path, count) for path, count in coverage)
    print(f'{len(paths)} files; {sum(n for _, n in coverage)} literals; revision {revision}')


if __name__ == '__main__':
    main()
