#!/usr/bin/env python3
"""Render a topology-locked Apartment 106 entry concept panorama.

This is deliberately a deterministic fixed shell, not a generated room collage.
The raw panorama uses a standard longitude layout: u=0.50 faces inward,
u=0.25 faces viewer-left, and u=0/1 faces the entry door. Publication can
rotate the seam onto the uninterrupted viewer-right wall.
"""

from __future__ import annotations

import argparse
from pathlib import Path

import cv2
import numpy as np


CAMERA = np.array([0.0, 0.95, 1.60], dtype=np.float32)


def args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    parser.add_argument("--width", type=int, default=2048)
    parser.add_argument("--front-overlay", type=Path)
    parser.add_argument("--left-overlay", type=Path)
    parser.add_argument("--right-overlay", type=Path)
    parser.add_argument("--back-overlay", type=Path)
    return parser.parse_args()


def texture(name: str, a: np.ndarray, b: np.ndarray) -> np.ndarray:
    shape = a.shape + (3,)
    colour = np.empty(shape, dtype=np.float32)

    if name == "wall":
        grain = 2.0 * np.sin(a * 5.1 + b * 2.7) + 1.0 * np.sin(a * 19.0)
        colour[:] = np.array([224, 219, 211], np.float32)
        colour += grain[..., None]
    elif name == "ceiling":
        colour[:] = np.array([232, 227, 218], np.float32)
        colour += (1.5 * np.sin(a * 4.0 + b * 3.0))[..., None]
    elif name == "floor":
        plank = np.floor((a + 4.0) / 0.19)
        grain = 8 * np.sin(b * 16 + plank * 1.7) + 3 * np.sin(b * 49 + a * 7)
        colour[:] = np.array([184, 144, 101], np.float32)
        colour += grain[..., None] * np.array([1.0, .78, .52], np.float32)
        seam = np.minimum(np.mod(a + 4.0, .19), .19 - np.mod(a + 4.0, .19)) < .006
        end_seam = (np.mod(b + np.mod(plank, 3) * .43, 1.25) < .008)
        colour[seam | end_seam] *= .68
    elif name == "oak":
        grain = 9 * np.sin(a * 40.0) + 4 * np.sin(a * 93.0 + b * 2.0)
        colour[:] = np.array([167, 124, 83], np.float32)
        colour += grain[..., None] * np.array([1.0, .74, .48], np.float32)
    elif name == "storage":
        colour[:] = np.array([211, 205, 195], np.float32)
        divisions = np.minimum(np.mod(a - .05, .35), .35 - np.mod(a - .05, .35)) < .008
        colour[divisions] *= .66
        pull = (np.mod(a - .10, .70) < .035) & (b > .35) & (b < 1.45)
        colour[pull] = np.array([139, 95, 59], np.float32)
    elif name == "tile":
        colour[:] = np.array([188, 183, 175], np.float32)
        joints = (np.minimum(np.mod(a + 4, .42), .42 - np.mod(a + 4, .42)) < .008) | (
            np.minimum(np.mod(b, .42), .42 - np.mod(b, .42)) < .008
        )
        colour[joints] *= .72
    elif name == "laundry":
        colour[:] = np.array([202, 198, 191], np.float32)
        washer = ((a - 2.08) / .28) ** 2 + ((b - .70) / .30) ** 2 < 1
        washer_rim = ((a - 2.08) / .34) ** 2 + ((b - .70) / .36) ** 2 < 1
        colour[washer_rim] = np.array([176, 177, 176], np.float32)
        colour[washer] = np.array([35, 39, 41], np.float32)
        bench = (b > .94) & (b < 1.03)
        colour[bench] = np.array([177, 160, 142], np.float32)
        cabinet = (b > 1.03) & (b < 1.50)
        colour[cabinet] = np.array([160, 119, 82], np.float32)
    elif name == "bath":
        colour[:] = np.array([208, 204, 197], np.float32)
        vanity = (b > .55) & (b < 1.02)
        colour[vanity] = np.array([162, 121, 83], np.float32)
        top = (b > 1.02) & (b < 1.09)
        colour[top] = np.array([205, 197, 185], np.float32)
        mirror = (b > 1.18) & (b < 2.05) & (a > 3.00) & (a < 3.50)
        colour[mirror] = np.array([157, 166, 166], np.float32)
    elif name == "door":
        grain = 8 * np.sin(a * 34.0) + 3 * np.sin(a * 88.0 + b)
        colour[:] = np.array([158, 116, 77], np.float32)
        colour += grain[..., None] * np.array([1.0, .72, .45], np.float32)
        # Inside-facing view: handle is viewer-left, geometrically +x.
        handle = (a > .27) & (a < .48) & (b > .96) & (b < 1.02)
        rose = ((a - .29) / .045) ** 2 + ((b - .99) / .06) ** 2 < 1
        colour[handle | rose] = np.array([34, 32, 30], np.float32)
        hinge = (a < -.46) & ((np.mod(b - .28, .72) < .15))
        colour[hinge] = np.array([45, 43, 40], np.float32)
    elif name == "far":
        colour[:] = np.array([226, 220, 210], np.float32)
        cabinet = (a < -.18) & (b < 1.78)
        colour[cabinet] = np.array([160, 117, 79], np.float32)
    else:
        colour[:] = np.array([255, 0, 255], np.float32)

    return colour


class Renderer:
    def __init__(self, directions: np.ndarray):
        self.d = directions
        self.best = np.full(directions.shape[:2], np.inf, dtype=np.float32)
        self.rgb = np.zeros(directions.shape, dtype=np.float32)

    def plane(self, axis: int, value: float, lo_a: float, hi_a: float, lo_b: float, hi_b: float, material: str):
        other = [index for index in range(3) if index != axis]
        denominator = self.d[..., axis]
        valid_denominator = np.abs(denominator) > 1e-6
        t = np.where(valid_denominator, (value - CAMERA[axis]) / denominator, np.inf)
        point_a = CAMERA[other[0]] + t * self.d[..., other[0]]
        point_b = CAMERA[other[1]] + t * self.d[..., other[1]]
        valid = (t > .002) & (t < self.best) & (point_a >= lo_a) & (point_a <= hi_a) & (point_b >= lo_b) & (point_b <= hi_b)
        if not np.any(valid):
            return
        sampled = texture(material, point_a, point_b)
        # Mild depth falloff gives the narrow hall readable scale.
        shade = np.clip(1.05 - .018 * t, .78, 1.04)
        sampled *= shade[..., None]
        self.rgb[valid] = sampled[valid]
        self.best[valid] = t[valid]


def render(width: int) -> np.ndarray:
    height = width // 2
    result = np.zeros((height, width, 3), dtype=np.uint8)

    for row0 in range(0, height, 64):
        row1 = min(height, row0 + 64)
        ys = (np.arange(row0, row1, dtype=np.float32) + .5) / height
        xs = (np.arange(width, dtype=np.float32) + .5) / width
        longitude = (xs - .50) * (2 * np.pi)
        latitude = (.5 - ys) * np.pi
        lon, lat = np.meshgrid(longitude, latitude)
        cos_lat = np.cos(lat)
        directions = np.stack([np.sin(lon) * cos_lat, np.cos(lon) * cos_lat, np.sin(lat)], axis=-1).astype(np.float32)
        renderer = Renderer(directions)

        # Main hall shell: inward is +y, viewer-left is -x.
        renderer.plane(2, 0.0, -.75, .75, -.35, 8.0, "floor")
        renderer.plane(2, 2.62, -.75, .75, -.35, 8.0, "ceiling")
        renderer.plane(0, .75, -.35, 8.0, 0.0, 2.62, "wall")

        # Left wall segments with distinct storage, Laundry and Bath openings.
        renderer.plane(0, -.75, -.35, .02, 0.0, 2.62, "wall")
        renderer.plane(0, -.745, .02, 1.48, 0.0, 2.42, "storage")
        renderer.plane(0, -.75, 1.48, 1.70, 0.0, 2.62, "wall")
        renderer.plane(0, -.75, 2.50, 2.76, 0.0, 2.62, "wall")
        renderer.plane(0, -.75, 3.72, 8.0, 0.0, 2.62, "wall")

        # Laundry recess: opening y=1.70..2.50, separate from Bath.
        renderer.plane(0, -1.45, 1.70, 2.50, 0.0, 2.42, "laundry")
        renderer.plane(1, 1.70, -1.45, -.75, 0.0, 2.42, "wall")
        renderer.plane(1, 2.50, -1.45, -.75, 0.0, 2.42, "wall")
        renderer.plane(2, 0.0, -1.45, -.75, 1.70, 2.50, "tile")
        renderer.plane(2, 2.42, -1.45, -.75, 1.70, 2.50, "ceiling")

        # Main Bath recess: opening y=2.76..3.72, with its own jambs.
        renderer.plane(0, -1.62, 2.76, 3.72, 0.0, 2.42, "bath")
        renderer.plane(1, 2.76, -1.62, -.75, 0.0, 2.42, "wall")
        renderer.plane(1, 3.72, -1.62, -.75, 0.0, 2.42, "wall")
        renderer.plane(2, 0.0, -1.62, -.75, 2.76, 3.72, "tile")
        renderer.plane(2, 2.42, -1.62, -.75, 2.76, 3.72, "ceiling")

        # Back wall and the single entry door.
        renderer.plane(1, -.35, -.75, -.52, 0.0, 2.62, "wall")
        renderer.plane(1, -.35, .52, .75, 0.0, 2.62, "wall")
        renderer.plane(1, -.355, -.52, .52, 0.0, 2.28, "door")
        renderer.plane(1, -.35, -.52, .52, 2.28, 2.62, "wall")

        # Restrained distant public-axis glimpse; no island at the entry.
        renderer.plane(1, 8.0, -.75, .75, 0.0, 2.62, "far")

        missing = ~np.isfinite(renderer.best)
        renderer.rgb[missing] = np.array([220, 215, 207], np.float32)
        result[row0:row1] = np.clip(renderer.rgb, 0, 255).astype(np.uint8)

    return cv2.cvtColor(result, cv2.COLOR_RGB2BGR)


def composite_directional_overlay(base: np.ndarray, overlay_path: Path | None, yaw_degrees: float, horizontal_fov: float) -> np.ndarray:
    if overlay_path is None:
        return base
    overlay = cv2.imread(str(overlay_path), cv2.IMREAD_COLOR)
    if overlay is None:
        raise SystemExit(f"Could not read overlay {overlay_path}")

    height, width = base.shape[:2]
    overlay_height, overlay_width = overlay.shape[:2]
    aspect = overlay_width / overlay_height
    tan_h = np.tan(np.deg2rad(horizontal_fov) / 2)
    tan_v = tan_h / aspect
    yaw = np.deg2rad(yaw_degrees)
    forward = np.array([np.sin(yaw), np.cos(yaw), 0.0], np.float32)
    right = np.array([np.cos(yaw), -np.sin(yaw), 0.0], np.float32)

    output = base.copy()
    for row0 in range(0, height, 64):
        row1 = min(height, row0 + 64)
        ys = (np.arange(row0, row1, dtype=np.float32) + .5) / height
        xs = (np.arange(width, dtype=np.float32) + .5) / width
        lon, lat = np.meshgrid((xs - .5) * 2 * np.pi, (.5 - ys) * np.pi)
        cos_lat = np.cos(lat)
        directions = np.stack([np.sin(lon) * cos_lat, np.cos(lon) * cos_lat, np.sin(lat)], axis=-1)
        depth = directions[..., 0] * forward[0] + directions[..., 1] * forward[1]
        horizontal = directions[..., 0] * right[0] + directions[..., 1] * right[1]
        sx = horizontal / np.maximum(depth, 1e-6) / tan_h
        sy = directions[..., 2] / np.maximum(depth, 1e-6) / tan_v
        valid = (depth > 0) & (np.abs(sx) <= 1) & (np.abs(sy) <= 1)
        map_x = ((sx + 1) * .5 * (overlay_width - 1)).astype(np.float32)
        map_y = ((1 - sy) * .5 * (overlay_height - 1)).astype(np.float32)
        sampled = cv2.remap(overlay, map_x, map_y, cv2.INTER_LANCZOS4, borderMode=cv2.BORDER_REFLECT)

        # A broad horizontal overlap lets adjacent directions cross-fade over
        # the same fixed geometry instead of exposing a 90-degree colour band.
        alpha_h = np.clip((.98 - np.abs(sx)) / .28, 0, 1)
        alpha_v = np.clip((.99 - np.abs(sy)) / .04, 0, 1)
        alpha = np.minimum(alpha_h, alpha_v)
        alpha = alpha * alpha * (3 - 2 * alpha)
        alpha *= valid
        target = output[row0:row1].astype(np.float32)
        output[row0:row1] = np.clip(target * (1 - alpha[..., None]) + sampled.astype(np.float32) * alpha[..., None], 0, 255).astype(np.uint8)
    return output


def main() -> None:
    options = args()
    if options.width < 1024 or options.width % 2:
        raise SystemExit("width must be an even number >= 1024")
    image = render(options.width)
    image = composite_directional_overlay(image, options.front_overlay, 0, 110)
    image = composite_directional_overlay(image, options.left_overlay, -90, 110)
    image = composite_directional_overlay(image, options.right_overlay, 90, 110)
    image = composite_directional_overlay(image, options.back_overlay, 180, 110)
    options.output.parent.mkdir(parents=True, exist_ok=True)
    if not cv2.imwrite(str(options.output), image, [cv2.IMWRITE_PNG_COMPRESSION, 3]):
        raise SystemExit(f"Could not write {options.output}")
    print(f"saved {options.output} {options.width}x{options.width // 2}")


if __name__ == "__main__":
    main()
