#!/usr/bin/env python3
"""
Generate DATA_REQUIREMENTS.md - what we have and what is still needed for accurate
grain simulation, per film and per emulsion class (B&W / colour negative / slide).

Reads tools/grain_data.json (built by build_grain_data.py).
"""
import json, os, collections

HERE = os.path.dirname(os.path.abspath(__file__))
data = json.load(open(os.path.join(HERE, "grain_data.json"), encoding="utf-8"))
films = data["films"]

CLASS = {"bw_neg": "Black & white", "color_neg": "Colour negative", "color_rev": "Colour slide"}

# Fields the grain renderer actually consumes, and where each comes from.
def graininess_conf(f):
    return f["graininess"]["confidence"]

def push_ok(f):
    return bool(f.get("push") or f.get("ei_range"))

def has_color(f):
    return f["type"] != "bw_neg"

rows = []
for name in sorted(films):
    f = films[name]
    rows.append({
        "name": name,
        "class": CLASS.get(f["type"], f["type"]),
        "graininess": (f["graininess"]["value"], f["graininess"]["metric"], graininess_conf(f)),
        "size": f["grain_size_um"]["confidence"],
        "clustering": f["clustering"]["confidence"],
        "color": "n/a" if not has_color(f) else "unverified",
        "halation": "yes" if f["halation"]["enabled"] else "-",
        "push": "yes" if push_ok(f) else "-",
    })

# ---- coverage summaries -----------------------------------------------------
gi = collections.Counter(r["graininess"][2] for r in rows)
by_class = collections.defaultdict(collections.Counter)
for r in rows:
    by_class[r["class"]][r["graininess"][2]] += 1

lines = []
lines.append("# Data requirements for accurate grain simulation\n")
lines.append("Generated from `grain_data.json`. This lists, per film, which grain-relevant "
             "fields are sourced (manufacturer datasheet) versus modelled/estimated, and what is "
             "still missing to simulate grain accurately.\n")

lines.append("## Fields the renderer needs\n")
lines.append("| Field | Used for | Current source | Status |")
lines.append("|---|---|---|---|")
lines.append("| Graininess (RMS/PGI at D=1.0) | grain amplitude | datasheet / class-derived / estimated | 17/35 datasheet |")
lines.append("| Grain size distribution (mean, spread) | grain size + count | literature/ISO model | **all estimated** |")
lines.append("| Morphology (tabular/cubic/core-shell/sigma) | grain shape | datasheets + general docs | good |")
lines.append("| Clustering probability / correlation length | cluster pass | app_baseline | **all unverified** |")
lines.append("| Grain colour (per-channel for colour stocks) | grain tint | color.json | **all unverified** |")
lines.append("| Halation (radius, intensity) | highlight glow | derived; CineStill only | 2 films only |")
lines.append("| Push response (stops, EI, grain increase) | exposure slider | datasheet (partial) | ~11/35 |")
lines.append("")

lines.append("## Graininess coverage\n")
lines.append(f"- datasheet: {gi['datasheet']}  |  derived: {gi['derived']}  |  "
             f"estimated from curve: {gi['estimated_from_curve']}  |  estimated: {gi['estimated']}\n")
for cls in ["Black & white", "Colour negative", "Colour slide"]:
    c = by_class[cls]
    total = sum(c.values())
    lines.append(f"- **{cls}** ({total}): " + ", ".join(f"{k}={v}" for k, v in sorted(c.items())))
lines.append("")

lines.append("## Per-film status\n")
lines.append("| Film | Class | Graininess | Confidence | Size | Clustering | Colour | Halation | Push |")
lines.append("|---|---|---|---|---|---|---|---|---|")
for r in rows:
    v, metric, conf = r["graininess"]
    g = f"{v:g} {metric}" if v is not None else "not published"
    lines.append(f"| {r['name']} | {r['class']} | {g} | {conf} | {r['size']} | "
                 f"{r['clustering']} | {r['color']} | {r['halation']} | {r['push']} |")
lines.append("")

lines.append("## What is still needed, by priority\n")
lines.append("1. **Grain size distribution per film (mean + spread, µm).** Not published by any "
             "manufacturer; currently a single literature/ISO model. Needed for grain size AND "
             "count to differ correctly between films. Obtain by **scan measurement** of each "
             "stock (autocorrelation / power-spectral density) - see `MEASUREMENT.md`.")
lines.append("2. **Clustering (probability + correlation length).** Currently the unverified "
             "`fixed.json`/`more.json` baseline. Needed for authentic grain clumping. Obtain by "
             "measuring spatial statistics from scans.")
lines.append("3. **Per-channel grain colour for colour negative and slide films.** Currently the "
             "unverified `color.json` values. Derive from datasheet spectral-sensitivity curves "
             "(the spectral pages we can digitise) or measure per-channel variance from scans.")
lines.append("4. **Graininess for the 10 estimated + 6 derived films.** Ilford (6), Fuji Neopan "
             "400/1600, Fuji C200/Natura are the estimated ones; Agfa/Vision3/CineStill are "
             "derived from published parents. Datasheets for Ilford publish no numeric RMS, so "
             "these need measurement or a documented third-party value.")
lines.append("5. **Halation radius/intensity for CineStill** (and any other halating stock). "
             "Currently derived; measure from a highlight on real footage/scans.")
lines.append("6. **Push-response curves (grain vs push) for more films.** Datasheet push tables "
             "exist for Kodak B&W, Portra 800, E100, Provia/Velvia; extend to the rest, and to the "
             "exposure-slider mapping.")
lines.append("")

lines.append("## Not needed for this version\n")
lines.append("- Density-dependent granularity curves and characteristic curves: these modulate "
             "grain against image tone, but the app renders grain on a **black/transparent "
             "background** with no underlying image density, so they do not apply. The graininess "
             "value we use is the curve's value at D=1.0.")
lines.append("")

lines.append("## External sources: what exists and what does not\n")
lines.append("A dedicated search was done for the missing fields. The findings:")
lines.append("- **Manufacturer datasheets** publish graininess (RMS/PGI), push tables, resolving "
             "power, morphology and spectral-sensitivity curves - the only genuinely per-film "
             "published data. Already extracted into `grain_data.json`.")
lines.append("- **Literature (class level, not per film):** Kodak's T-Grain paper "
             "(Kofron & Booms, *J. Soc. Photogr. Sci. Tech. Japan* 49(6), 1986) gives tabular "
             "grain dimensions - ~2.33 um equivalent diameter x 0.12 um thickness (aspect 22.6:1), "
             "3:1-5:1 aspect for production tabular colour emulsions - and cubic reference "
             "emulsions ~1-1.4 um. This is used to justify the grain-size model, but it is not "
             "per film.")
lines.append("- **No public database, datasheet or paper publishes, per film:** grain-size "
             "distribution, clustering (spatial correlation), per-channel grain colour, or halation "
             "radius. These are internal emulsion/coating properties.")
lines.append("- Datasheet **spectral-sensitivity / MTF / granularity curves** can be digitised for "
             "colour and granularity *trends*, but they do not yield grain size or clustering.")
lines.append("- Search engines returned no third-party compiled dataset (RMS/PGI for Ilford and "
             "Neopan included); the only route is measurement.")
lines.append("")
lines.append("## Practical conclusion\n")
lines.append("- For a *mostly realistic* render, the only genuinely missing per-film data is "
             "**grain size**, **clustering**, and (for colour stocks) **per-channel colour** - all "
             "of which require measurement from real film, because no manufacturer publishes them.")
lines.append("- Everything else (amplitude, shape, class, push, halation flag) is already sourced "
             "or class-derived and wired into the renderer.")

open(os.path.join(HERE, "DATA_REQUIREMENTS.md"), "w", encoding="utf-8").write("\n".join(lines) + "\n")
print("Wrote DATA_REQUIREMENTS.md")
print("graininess:", dict(gi))
