#! env python3
import os
import sys

def main():
    readme = """\
# AOC 2021 (Rust)

## How to run
```bash
git clone https://github.com/AP2008/aoc2021
cd aoc2021
make run
```

## Benchmarks
```bash
make bench
```
{}

## Solutions
{}
"""
    output_format = "```\n{}```"
    output = output_format.format(os.popen("target/release/aoc2021").read())
    print(readme.format(sys.stdin.read(), output))

if __name__ == '__main__':
    main()
