#!/usr/bin/env python3
from pathlib import Path

import cv2
import numpy as np


ROOT = Path(__file__).resolve().parents[1]
JOB = ROOT / "output/imagegen/living-front-2x-tiles-v1"
NAMES = ("top-left", "top-right", "bottom-left", "bottom-right")


def align_detail(reference: np.ndarray, generated: np.ndarray):
    target = cv2.resize(reference, (1254, 1254), interpolation=cv2.INTER_LANCZOS4)
    gray_target = cv2.cvtColor(target, cv2.COLOR_BGR2GRAY)
    gray_generated = cv2.cvtColor(generated, cv2.COLOR_BGR2GRAY)
    sift = cv2.SIFT_create(nfeatures=5000)
    kp_generated, des_generated = sift.detectAndCompute(gray_generated, None)
    kp_target, des_target = sift.detectAndCompute(gray_target, None)
    matches = cv2.BFMatcher().knnMatch(des_generated, des_target, k=2)
    good = [a for a, b in matches if a.distance < 0.72 * b.distance]
    if len(good) < 12:
        return target, 0, False

    source_points = np.float32([kp_generated[m.queryIdx].pt for m in good]).reshape(-1, 1, 2)
    target_points = np.float32([kp_target[m.trainIdx].pt for m in good]).reshape(-1, 1, 2)
    homography, inliers = cv2.findHomography(source_points, target_points, cv2.RANSAC, 4.0)
    if homography is None:
        return target, len(good), False

    aligned = cv2.warpPerspective(
        generated,
        homography,
        (1254, 1254),
        flags=cv2.INTER_LANCZOS4,
        borderMode=cv2.BORDER_REFLECT,
    )
    smooth = cv2.GaussianBlur(aligned, (0, 0), 1.5)
    detail = aligned.astype(np.float32) - smooth.astype(np.float32)
    target_edges = cv2.Canny(gray_target, 70, 150)
    target_edges = cv2.dilate(target_edges, np.ones((9, 9), np.uint8), iterations=1) > 0
    detail_strength = np.max(np.abs(detail), axis=2)
    safe_detail = np.logical_or(detail_strength < 16.0, target_edges).astype(np.float32)[..., None]
    enhanced = np.clip(target.astype(np.float32) + 0.48 * detail * safe_detail, 0, 255).astype(np.uint8)
    return enhanced, int(inliers.sum()) if inliers is not None else len(good), True


def seam_mask(name: str):
    mask = np.ones((1254, 1254), np.float32)
    feather = 72
    ramp = np.linspace(0.0, 1.0, feather, dtype=np.float32)
    if name.endswith("left"):
        mask[:, -feather:] *= ramp[::-1][None, :]
    else:
        mask[:, :feather] *= ramp[None, :]
    if name.startswith("top"):
        mask[-feather:, :] *= ramp[::-1][:, None]
    else:
        mask[:feather, :] *= ramp[:, None]
    return mask[..., None]


def main():
    enhanced = {}
    reports = []
    for name in NAMES:
        reference = cv2.imread(str(JOB / "reference" / f"{name}.png"), cv2.IMREAD_COLOR)
        generated = cv2.imread(str(JOB / "generated" / f"{name}.png"), cv2.IMREAD_COLOR)
        base = cv2.resize(reference, (1254, 1254), interpolation=cv2.INTER_LANCZOS4)
        tile, matches, aligned = align_detail(reference, generated)
        mask = seam_mask(name)
        tile = np.clip(base * (1.0 - mask) + tile * mask, 0, 255).astype(np.uint8)
        enhanced[name] = tile
        cv2.imwrite(str(JOB / "generated" / f"{name}-aligned.png"), tile)
        reports.append(f"{name}: aligned={aligned}, inlier_matches={matches}")

    top = np.hstack((enhanced["top-left"], enhanced["top-right"]))
    bottom = np.hstack((enhanced["bottom-left"], enhanced["bottom-right"]))
    result = np.vstack((top, bottom))
    cv2.imwrite(str(JOB / "living-front-detail-2508.png"), result)
    print("\n".join(reports))
    print(f"output={JOB / 'living-front-detail-2508.png'}")


if __name__ == "__main__":
    main()
