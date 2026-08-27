run year day:
    cargo run --bin aoc{{year}}_day$(printf '%02d' {{day}})

test year day:
    cargo test --bin aoc{{year}}_day$(printf '%02d' {{day}})

scaffold year day:
    #!/usr/bin/env bash
    set -e
    padded=$(printf '%02d' {{day}})
    name="aoc{{year}}_day${padded}"
    path="src/bin/aoc{{year}}/day${padded}.rs"
    mkdir -p "src/bin/aoc{{year}}"
    sed -e 's/__YEAR__/{{year}}/g' -e "s/__DAY__/$padded/g" \
        templates/day.rs > "$path"
    if ! grep -q "name = \"$name\"" Cargo.toml; then
        printf '\n[[bin]]\nname = "%s"\npath = "%s"\n' "$name" "$path" >> Cargo.toml
    fi
    echo "Scaffolded $name"

download year day:
    #!/usr/bin/env bash
    set -e
    mkdir -p data/input/{{year}}
    curl -s -o "data/input/{{year}}/$(printf '%02d' {{day}}).txt" \
        -H "Cookie: session=$(cat ~/.adventofcode.session)" \
        "https://adventofcode.com/{{year}}/day/{{day}}/input"
    echo "Downloaded input for {{year}} day {{day}}"

puzzle year day:
    #!/usr/bin/env bash
    set -e
    mkdir -p data/puzzles/{{year}}
    curl -s \
        -H "Cookie: session=$(cat ~/.adventofcode.session)" \
        "https://adventofcode.com/{{year}}/day/{{day}}" \
    | pandoc -f html -t markdown -o "data/puzzles/{{year}}/{{day}}.md"
    echo "Downloaded puzzle for {{year}} day {{day}}"

new year day:
    just scaffold {{year}} {{day}}
    just download {{year}} {{day}}
    just puzzle {{year}} {{day}}

lint:
    cargo clippy --all-targets -- -D warnings

format:
    cargo fmt
