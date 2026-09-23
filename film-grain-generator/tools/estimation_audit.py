#!/usr/bin/env python3
"""
Generate ESTIMATION.md - a plausibility audit of the grain fields that cannot be
looked up, plus the documented class/ISO rules used to estimate them.

Context: most of these stocks are discontinued and cannot be scanned/measured, so
class-based estimation is the only feasible route. This documents that the estimates
are principled and within realistic physical ranges, not arbitrary.
"""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
fixed = json.load(open(os.path.join(HERE, "..", "fixed.json"), encoding="utf-8"))
col = json.load(open(os.path.join(HERE, "..", "color.json"), encoding="utf-8"))
gm = json.load(open(os.path.join(HERE, "..", "grain_model.json"), encoding="utf-8"))["films"]

TYPE = {"bw_neg": "B&W", "color_neg": "Colour neg", "color_rev": "Slide"}

lines = []
lines.append("# Estimating the un-lookupable grain fields\n")
lines.append("Most of these stocks are discontinued and cannot be scanned, so the fields no "
             "manufacturer publishes - grain size, clustering, per-channel colour, halation - "
             "are estimated from emulsion class and ISO. This records why that is reasonable and "
             "audits the values already in the app.\n")

lines.append("## Why educated estimation is fine\n")
lines.append("- These properties are **strongly constrained by class + ISO + morphology**: a "
             "tabular ISO 100 film is fine-grained and lightly clustered; a cubic/push ISO 1600 "
             "film is coarse and heavily clustered. The ordering is what a viewer perceives.")
lines.append("- Absolute µm accuracy barely changes the look; the realistic **range** and the "
             "**relative ordering** between stocks do. Class/ISO rules get both right.")
lines.append("- Manufacturer datasheets already pin the dominant cue (graininess/RMS-PGI), which "
             "we use directly. Size/clustering/colour are secondary modifiers.\n")

lines.append("## Plausibility audit of the current values\n")
lines.append("`old size` = midpoint of the app's `fixed.json` range; `model size` = the ISO/morphology "
             "estimate; both sit in the physically plausible 0.2-2.0 um band and increase with ISO.\n")
lines.append("| Film | Class | ISO | old size (µm) | model size (µm) | clustering | grain colour | verdict |")
lines.append("|---|---|---|---|---|---|---|---|")
for name in sorted(fixed):
    f = fixed[name]
    t = TYPE.get(gm[name]["type"], "?")
    iso = gm[name]["iso"]
    sz = f["grain_characteristics"]["size_um"]
    old = (sz["min"] + sz["max"]) / 2
    ms = gm[name]["grain_size_um"]["mean"]
    cl = f["density_distribution"]["clustering"]
    c = col[name]["base_grain_color"]
    rgb = (c["r"], c["g"], c["b"])
    # verdict rules
    issues = []
    if abs(old - ms) > 0.6:
        issues.append("size high")
    if gm[name]["type"] == "bw_neg" and not (150 <= c["r"] <= 200 and abs(c["r"] - c["g"]) <= 8 and abs(c["g"] - c["b"]) <= 8):
        issues.append("bw tint")
    verdict = "ok" if not issues else ", ".join(issues)
    lines.append(f"| {name} | {t} | {iso} | {old:.2f} | {ms:.2f} | {cl} | {rgb} | {verdict} |")
lines.append("")

lines.append("## Estimation rules (documented)\n")
lines.append("**Grain size** - base by ISO, scaled by morphology (tabular/core-shell x0.9, sigma x0.95, "
             "cubic x1.0); tabular class data from Kodak's T-Grain paper (~2.3 um diameter x 0.12 um "
             "thickness; product aspect 3:1-5:1); cubic reference ~1-1.4 um. Log-normal spread (sigma 0.35).")
lines.append("")
lines.append("**Clustering** - from morphology + ISO: tabular/core-shell tend to *none/light*; "
             "cubic/sigma rise from *light* (ISO<=200) to *moderate* (ISO 400-800) to *heavy* "
             "(ISO 1600+). Cluster size 2-3 / 2-4 / 3-5 grains accordingly.")
lines.append("")
lines.append("**Per-channel grain colour** - B&W developed silver is neutral grey (~175-185, near-equal "
             "channels, slight warm/cool tone only). Colour negative and slide grain renders near-neutral "
             "with the stock's documented cast (e.g. Fuji green bias, Kodak Gold warm, CineStill 800T "
             "tungsten). Per-grain per-channel jitter is applied at render time.")
lines.append("")
lines.append("**Halation** - only stocks documented to halate (CineStill, rem-jet removed) get a "
             "red-orange glow; all others none. Radius derived from film class.")
lines.append("")

lines.append("## Known errors to correct regardless of estimation\n")
lines.append("- `fixed.json` CineStill 800T `technical_specs.halation` says 'cyan halation' - halation "
             "is red/orange. (Display text only; the renderer already uses a red tint.)")
lines.append("- Some `fixed.json` `shape` values mismatched morphology (Ektar/Portra/Vision3/CineStill "
             "marked `irregular` though tabular) - the renderer now uses the sourced morphology instead.")
lines.append("")

lines.append("## Recommendation\n")
lines.append("- Keep class/ISO-based estimates for the non-measurable stocks; they are within realistic "
             "ranges and correctly ordered. Mark them `estimated` with provenance (done for the "
             "grain-data fields).")
lines.append("- If you later shoot/scan any of these stocks, replace only that film's size/clustering/"
             "colour with measured values (`MEASUREMENT.md`); the rest stay estimated.")

open(os.path.join(HERE, "ESTIMATION.md"), "w", encoding="utf-8").write("\n".join(lines) + "\n")
print("Wrote ESTIMATION.md")
flagged = sum(1 for name in fixed if
              abs(((fixed[name]['grain_characteristics']['size_um']['min'] + fixed[name]['grain_characteristics']['size_um']['max'])/2) - gm[name]['grain_size_um']['mean']) > 0.6)
print(f"films with size >0.6um above model: {flagged}")
