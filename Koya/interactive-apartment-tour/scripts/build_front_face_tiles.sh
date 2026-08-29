#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "$0")/.." && pwd)"
source_image="$project_dir/outputs/panorama-production/living-cubemap-v1/final-faces/front.png"
output_dir="$project_dir/outputs/panorama-production/front-face-4k-v1/source-tiles"

mkdir -p "$output_dir"
positions=(0 279 557 836)

for row in 0 1 2 3; do
  for col in 0 1 2 3; do
    x="${positions[$col]}"
    y="${positions[$row]}"
    magick "$source_image" -crop "418x418+${x}+${y}" +repage \
      "$output_dir/r${row}-c${col}.png"
  done
done

for row in 0 1 2 3; do
  magick "$output_dir/r${row}-c0.png" "$output_dir/r${row}-c1.png" \
    "$output_dir/r${row}-c2.png" "$output_dir/r${row}-c3.png" \
    +append "$output_dir/row-${row}.png"
done
magick "$output_dir/row-0.png" "$output_dir/row-1.png" \
  "$output_dir/row-2.png" "$output_dir/row-3.png" -append \
  "$project_dir/outputs/panorama-production/front-face-4k-v1/source-contact-sheet.jpg"
