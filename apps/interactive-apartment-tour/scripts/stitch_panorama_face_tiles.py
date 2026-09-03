#!/usr/bin/env python3
from pathlib import Path

import cv2
import numpy as np


ROOT = Path(__file__).resolve().parents[1]
JOB = ROOT / "output/imagegen/panorama-cubeface-enhance-v1/front"
TILE = 1254
SOURCE_TILE = 1015
SOURCE_STEP = 759
SCALE = TILE / SOURCE_TILE
STEP = round(SOURCE_STEP * SCALE)
FACE = STEP + TILE
NAMES = {
    "top-left": (0, 0),
    "top-right": (STEP, 0),
    "bottom-left": (0, STEP),
    "bottom-right": (STEP, STEP),
}


def align_to_reference(reference: np.ndarray, generated: np.ndarray):
    target = cv2.resize(reference, (TILE, TILE), interpolation=cv2.INTER_LANCZOS4)
    sift = cv2.SIFT_create(nfeatures=7000)
    kp_g, des_g = sift.detectAndCompute(cv2.cvtColor(generated, cv2.COLOR_BGR2GRAY), None)
    kp_t, des_t = sift.detectAndCompute(cv2.cvtColor(target, cv2.COLOR_BGR2GRAY), None)
    matches = cv2.BFMatcher().knnMatch(des_g, des_t, k=2)
    good = [a for a, b in matches if a.distance < 0.76 * b.distance]
    if len(good) < 16:
        return target, 0, False
    src = np.float32([kp_g[m.queryIdx].pt for m in good]).reshape(-1, 1, 2)
    dst = np.float32([kp_t[m.trainIdx].pt for m in good]).reshape(-1, 1, 2)
    matrix, inliers = cv2.findHomography(src, dst, cv2.RANSAC, 4.0)
    if matrix is None:
        return target, 0, False
    aligned = cv2.warpPerspective(generated, matrix, (TILE, TILE), flags=cv2.INTER_LANCZOS4, borderMode=cv2.BORDER_REFLECT)

    # Correct local model drift after the global projective alignment.
    target_gray = cv2.cvtColor(target, cv2.COLOR_BGR2GRAY)
    aligned_gray = cv2.cvtColor(aligned, cv2.COLOR_BGR2GRAY)
    flow_engine = cv2.DISOpticalFlow_create(cv2.DISOPTICAL_FLOW_PRESET_MEDIUM)
    flow_engine.setUseSpatialPropagation(True)
    flow = flow_engine.calc(target_gray, aligned_gray, None)
    flow = cv2.GaussianBlur(flow, (0, 0), 2.2)
    grid_x, grid_y = np.meshgrid(np.arange(TILE, dtype=np.float32), np.arange(TILE, dtype=np.float32))
    aligned = cv2.remap(
        aligned,
        grid_x + flow[..., 0],
        grid_y + flow[..., 1],
        interpolation=cv2.INTER_LANCZOS4,
        borderMode=cv2.BORDER_REFLECT,
    )

    # Match broad colour/brightness to the common panorama while retaining AI detail.
    aligned_f = aligned.astype(np.float32)
    target_f = target.astype(np.float32)
    smooth_a = cv2.GaussianBlur(aligned_f, (0, 0), 24)
    smooth_t = cv2.GaussianBlur(target_f, (0, 0), 24)
    aligned = np.clip(aligned_f + 0.72 * (smooth_t - smooth_a), 0, 255).astype(np.uint8)
    return aligned, int(inliers.sum()) if inliers is not None else len(good), True


def tile_weight(name: str):
    overlap = TILE - STEP
    w = np.ones((TILE, TILE), np.float32)
    ramp = np.linspace(0.0, 1.0, overlap, dtype=np.float32)
    if name.endswith("left"):
        w[:, -overlap:] *= ramp[::-1][None, :]
    else:
        w[:, :overlap] *= ramp[None, :]
    if name.startswith("top"):
        w[-overlap:, :] *= ramp[::-1][:, None]
    else:
        w[:overlap, :] *= ramp[:, None]
    return w[..., None]


def main():
    accumulation = np.zeros((FACE, FACE, 3), np.float32)
    weights = np.zeros((FACE, FACE, 1), np.float32)
    reports = []
    for name, (x, y) in NAMES.items():
        ref = cv2.imread(str(JOB / "reference" / f"{name}.png"), cv2.IMREAD_COLOR)
        generated = cv2.imread(str(JOB / "generated" / f"{name}.png"), cv2.IMREAD_COLOR)
        aligned, matches, ok = align_to_reference(ref, generated)
        cv2.imwrite(str(JOB / "generated" / f"{name}-aligned-full.png"), aligned)
        weight = tile_weight(name)
        accumulation[y:y + TILE, x:x + TILE] += aligned.astype(np.float32) * weight
        weights[y:y + TILE, x:x + TILE] += weight
        reports.append(f"{name}: aligned={ok}, inlier_matches={matches}")

    stitched = accumulation / np.maximum(weights, 1e-6)
    base = cv2.imread(str(JOB / "reference/full.png"), cv2.IMREAD_COLOR)
    base = cv2.resize(base, (FACE, FACE), interpolation=cv2.INTER_LANCZOS4).astype(np.float32)

    # Preserve the cube-face perimeter so adjacent faces remain mathematically continuous.
    edge = 150
    axis = np.minimum(np.arange(FACE), np.arange(FACE)[::-1]).astype(np.float32)
    edge_weight = np.clip(axis / edge, 0.0, 1.0)
    perimeter = np.minimum(edge_weight[:, None], edge_weight[None, :])[..., None]
    stitched = base * (1.0 - perimeter) + stitched * perimeter
    stitched = np.clip(stitched, 0, 255).astype(np.uint8)
    output = JOB / f"front-enhanced-{FACE}.png"
    cv2.imwrite(str(output), stitched)
    print("\n".join(reports))
    print(f"face={FACE}x{FACE}")
    print(f"output={output}")


if __name__ == "__main__":
    main()
