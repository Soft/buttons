#!/usr/bin/env bash

main() {
    local input="$1" output dim
    [[ -z "$input" ]] && return 1
    output=$(basename "$input")
    output=${output%%.*}
    for dim in 72 96 128 144 152 192; do
        inkscape --export-png "${output}-${dim}.png" \
            --export-width "$dim" \
            --export-height "$dim" \
            "$input" &&
        pngcrush -ow -brute "${output}-${dim}.png"
    done
}

main "$@"

