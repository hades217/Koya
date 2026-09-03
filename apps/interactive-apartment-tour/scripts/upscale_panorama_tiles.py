#!/usr/bin/env python3
"""Upscale an equirectangular panorama in overlap-safe tiles.

Horizontal padding wraps around the 360 seam. Vertical padding reflects the
ceiling/floor poles. Only the centre of each inferred tile is retained, so the
final image has no duplicated or blended geometry at tile boundaries.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import cv2
import numpy as np


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    parser.add_argument("model", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--tile", type=int, default=256)
    parser.add_argument("--margin", type=int, default=32)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    source = cv2.imread(str(args.input), cv2.IMREAD_COLOR)
    if source is None:
        raise SystemExit(f"Could not read {args.input}")

    scale = 4
    height, width = source.shape[:2]
    margin = args.margin

    horizontally_wrapped = np.concatenate(
        [source[:, -margin:], source, source[:, :margin]], axis=1
    )
    padded = cv2.copyMakeBorder(
        horizontally_wrapped, margin, margin, 0, 0, cv2.BORDER_REFLECT_101
    )

    super_res = cv2.dnn_superres.DnnSuperResImpl_create()
    super_res.readModel(str(args.model))
    super_res.setModel("edsr", scale)

    output = np.empty((height * scale, width * scale, 3), dtype=np.uint8)
    tile_count = ((height + args.tile - 1) // args.tile) * (
        (width + args.tile - 1) // args.tile
    )
    completed = 0

    for y0 in range(0, height, args.tile):
        y1 = min(height, y0 + args.tile)
        for x0 in range(0, width, args.tile):
            x1 = min(width, x0 + args.tile)
            expanded = padded[y0 : y1 + 2 * margin, x0 : x1 + 2 * margin]
            enhanced = super_res.upsample(expanded)
            centre = enhanced[
                margin * scale : (margin + y1 - y0) * scale,
                margin * scale : (margin + x1 - x0) * scale,
            ]
            output[y0 * scale : y1 * scale, x0 * scale : x1 * scale] = centre
            completed += 1
            print(f"tile {completed}/{tile_count}", flush=True)

    args.output.parent.mkdir(parents=True, exist_ok=True)
    if not cv2.imwrite(str(args.output), output, [cv2.IMWRITE_PNG_COMPRESSION, 3]):
        raise SystemExit(f"Could not write {args.output}")
    print(f"saved {args.output} {width * scale}x{height * scale}", flush=True)


if __name__ == "__main__":
    main()
