#!/usr/bin/env python3
"""
Build a sourced grain-characterisation dataset for the 35 film stocks in the app.

Outputs (written to this tools/ directory):
  - grain_data.json : per-film grain model data with a source reference per field
  - SOURCES.md      : catalogue of the primary documents used
  - data_audit.md   : comparison of the collected values against the current app data

Only manufacturer-published values are treated as authoritative. Everything that
is not published by the manufacturer is explicitly marked as derived/estimated in
the `confidence` field and documented in `basis`.
"""
import json, os, io, datetime

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)

# ---------------------------------------------------------------------------
# 1. Source catalogue
# ---------------------------------------------------------------------------
SOURCES = {
    "kodak_f4017": {
        "title": "KODAK PROFESSIONAL TRI-X 400 Film - Technical Data F-4017 (2007)",
        "url": "https://125px.com/docs/film/kodak/f4017-400TX-2007.pdf",
        "provides": "Diffuse RMS granularity (48 um aperture, net density 1.0, 12x, HC-110 B)",
    },
    "kodak_f4016": {
        "title": "KODAK PROFESSIONAL T-MAX 100 Film - Technical Data F-4016 (2018)",
        "url": "https://125px.com/docs/film/kodak/f4016_tmax_100-2018.pdf",
        "provides": "Diffuse RMS granularity + resolving power (D-76)",
    },
    "kodak_f4043": {
        "title": "KODAK PROFESSIONAL T-MAX 400 Film - Technical Data F-4043 (2016)",
        "url": "https://125px.com/docs/film/kodak/f4043_TMax_400-2016.pdf",
        "provides": "Diffuse RMS granularity + resolving power (D-76)",
    },
    "kodak_e4024": {
        "title": "KODAK EKTACHROME E100G Film - E-4024",
        "url": "https://125px.com/docs/film/kodak/e4024-Ektachrome_E100G.pdf",
        "provides": "Diffuse RMS granularity (rms 8)",
    },
    "kodak_e4046": {
        "title": "KODAK PROFESSIONAL EKTAR 100 Film - E-4046 (2016)",
        "url": "https://125px.com/docs/film/kodak/e4046_ektar_100-2016.pdf",
        "provides": "Print Grain Index table (35 mm, 4x6 in / 4.4x)",
    },
    "kodak_e7022": {
        "title": "KODAK GOLD 200 Film - E-7022 (2016)",
        "url": "https://125px.com/docs/film/kodak/E7022_Gold_200-2016.pdf",
        "provides": "Print Grain Index table (35 mm, 4x6 in / 4.4x)",
    },
    "kodak_e7019": {
        "title": "KODAK ULTRA MAX 400 Film - E-7019",
        "url": "https://125px.com/docs/film/kodak/E7019_en-Ultra_Max_400.pdf",
        "provides": "Print Grain Index table (35 mm, 4x6 in / 4.4x)",
    },
    "kodak_e4051": {
        "title": "KODAK PROFESSIONAL PORTRA 160 Film - E-4051 (2016)",
        "url": "https://125px.com/docs/film/kodak/e4051_Portra_160-2016.pdf",
        "provides": "Print Grain Index table (35 mm, 4x6 in / 4.4x)",
    },
    "kodak_e4050": {
        "title": "KODAK PROFESSIONAL PORTRA 400 Film - E-4050 (2016)",
        "url": "https://125px.com/docs/film/kodak/e4050_portra_400-2016.pdf",
        "provides": "Print Grain Index table (35 mm, 4x6 in / 4.4x)",
    },
    "kodak_e4040": {
        "title": "KODAK PROFESSIONAL PORTRA 800 Film - E-4040 (2016)",
        "url": "https://125px.com/docs/film/kodak/e4040_portra_800-2016.pdf",
        "provides": "Print Grain Index table (35 mm, 4x6 in / 4.4x)",
    },
    "kodak_e58": {
        "title": "KODAK Publication E-58 - Print Grain Index (2000)",
        "url": "https://125px.com/docs/techpubs/kodak/e58-2000_07.pdf",
        "provides": "PGI method/scale: 2 units = 1 jnd, PGI 25 = visual graininess threshold",
    },
    "kodak_vision3_500t": {
        "title": "KODAK VISION3 500T 5219 - Technical Information TI2647",
        "url": "https://125px.com/docs/motionpicture/kodak/5219-Vision3-500T-tech.pdf",
        "provides": "Diffuse RMS granularity curves (R/G/B, 48 um aperture) - curve only, no single value",
    },
    "kodak_vision3_50d": {
        "title": "KODAK VISION3 50D 5203 - Technical Information TI2657",
        "url": "https://125px.com/docs/motionpicture/kodak_2018/5203_ti2657.pdf",
        "provides": "Diffuse RMS granularity curves (R/G/B, 48 um aperture) - curve only, no single value",
    },
    "fuji_velvia50": {
        "title": "FUJICHROME Velvia 50 Professional [RVP50] - Product Information Bulletin",
        "url": "https://125px.com/docs/film/fuji/velvia_50_datasheet.pdf",
        "provides": "Diffuse RMS granularity 9 + resolving power",
    },
    "fuji_velvia100": {
        "title": "FUJICHROME Velvia 100 Professional [RVP100] - Product Information Bulletin",
        "url": "https://125px.com/docs/film/fuji/velvia_100_datasheet.pdf",
        "provides": "Diffuse RMS granularity 8 + resolving power",
    },
    "fuji_provia100f": {
        "title": "FUJICHROME PROVIA 100F Professional [RDP III] - Data Sheet",
        "url": "https://125px.com/docs/film/fuji/provia_100f_datasheet.pdf",
        "provides": "Diffuse RMS granularity 8 + resolving power",
    },
    "fuji_provia400x": {
        "title": "FUJICHROME PROVIA 400X Professional [RXP] - Product Information Bulletin",
        "url": "https://125px.com/docs/film/fuji/Provia_400X_PIB_1007.pdf",
        "provides": "Diffuse RMS granularity 11 + resolving power",
    },
    "fuji_pro160s": {
        "title": "FUJICOLOR PRO 160S - Data Sheet",
        "url": "https://125px.com/docs/film/fuji/pro_160s_datasheet.pdf",
        "provides": "Diffuse RMS granularity 3 (Fujifilm colour-negative scale) + resolving power",
    },
    "fuji_pro400h": {
        "title": "FUJICOLOR PRO 400H - Data Sheet",
        "url": "https://125px.com/docs/film/fuji/pro_400h_datasheet.pdf",
        "provides": "Diffuse RMS granularity 4 (Fujifilm colour-negative scale) + resolving power",
    },
    "fuji_superia400": {
        "title": "FUJICOLOR SUPERIA X-TRA 400 [CH] - Product Information Bulletin",
        "url": "https://125px.com/docs/film/fuji/superia_xtra400_datasheet.pdf",
        "provides": "Diffuse RMS granularity 4 (Fujifilm colour-negative scale) + resolving power",
    },
    "fuji_acros": {
        "title": "FUJIFILM NEOPAN 100 ACROS - Data Sheet AF3-095E",
        "url": "https://125px.com/docs/film/fuji/NeopanAcros100.pdf",
        "provides": "Diffuse RMS granularity 7 + resolving power (read via OCR, page 4)",
    },
    "agfa_films": {
        "title": "AGFA Professional Film range brochure (Optima II, Portrait XPS, Ultra, RSX II, APX, Scala)",
        "url": "https://125px.com/docs/film/agfa/agfa_films.pdf",
        "provides": "RMS granularity (x1000) + resolving power for Agfa emulsions; used as class data for the AgfaPhoto-era stocks",
    },
    "iso10505": {
        "title": "ISO 10505:2009 - Photography: Root mean square granularity of photographic films",
        "url": "https://www.iso.org/standard/41634.html",
        "provides": "Standard measurement definition for RMS granularity (48 um aperture, D=1.0)",
    },
    "wikipedia_filmgrain": {
        "title": "Wikipedia - Film grain (RMS granularity, Selwyn granularity)",
        "url": "https://en.wikipedia.org/wiki/Film_grain",
        "provides": "Physical definitions and Selwyn's law",
    },
    "wikipedia_tabular": {
        "title": "Wikipedia - Tabular-grain film",
        "url": "https://en.wikipedia.org/wiki/Tabular-grain_film",
        "provides": "Emulsion morphology: tabular vs cubic, T-GRAIN, core-shell",
    },
    "wikipedia_halation": {
        "title": "Wikipedia - Anti-halation backing / halation",
        "url": "https://en.wikipedia.org/wiki/Anti-halation_backing",
        "provides": "Halation mechanism (red blow-out around highlights when anti-halation/rem-jet is absent)",
    },
    "kodak_tgrain_paper": {
        "title": "Kofron & Booms, 'KODAK T-Grain Emulsions in Color Films', J. Soc. Photogr. Sci. Tech. Japan 49(6), 1986",
        "url": "https://www.jstage.jst.go.jp/article/photogrst1964/49/6/49_6_499/_pdf",
        "provides": "Tabular grain dimensions: equivalent circular diameter 2.33 um, thickness 0.12 um "
                    "(aspect 22.6:1) for an experimental emulsion; 3:1-5:1 aspect for production tabular "
                    "colour emulsions; cubic reference emulsions ~1-1.4 um. Class-level size ranges only.",
    },
}

# ---------------------------------------------------------------------------
# 2. Per-film collected data
#    grain: (metric, value, scale, source)  metric in {rms_diffuse, pgi, None}
#    morph: tabular | core_shell | cubic | sigma
# ---------------------------------------------------------------------------
# type: bw_neg | color_neg | color_rev
FILMS = [
    # ---- Kodak B&W -------------------------------------------------------
    dict(name="Kodak Tri-X 400", type="bw_neg", iso=400, morph="cubic",
         grain=("rms_diffuse", 17, "kodak_bw_rms_48um", "kodak_f4017"), conf="datasheet",
         cluster="moderate", contrast="high", hal=False,
         note="Classic cubic-silver emulsion; coarse grain is a defining feature."),
    dict(name="Kodak T-Max 100", type="bw_neg", iso=100, morph="tabular",
         grain=("rms_diffuse", 8, "kodak_bw_rms_48um", "kodak_f4016"), conf="datasheet",
         cluster="none", contrast="medium", hal=False, rp=(63, 200),
         note="Kodak T-GRAIN tabular emulsion."),
    dict(name="Kodak T-Max 400", type="bw_neg", iso=400, morph="tabular",
         grain=("rms_diffuse", 10, "kodak_bw_rms_48um", "kodak_f4043"), conf="datasheet",
         cluster="none", contrast="medium", hal=False, rp=(50, 200),
         note="Kodak T-GRAIN tabular emulsion."),
    # ---- Kodak colour negative ------------------------------------------
    dict(name="Kodak Ektar 100", type="color_neg", iso=100, morph="tabular",
         grain=("pgi", 24, "kodak_pgi_35mm_4x6", "kodak_e4046"), conf="datasheet",
         cluster="none", contrast="high", hal=False,
         note="Datasheet states PGI is 'less than 25' (at/below the visual grain threshold)."),
    dict(name="Kodak Gold 200", type="color_neg", iso=200, morph="tabular",
         grain=("pgi", 44, "kodak_pgi_35mm_4x6", "kodak_e7022"), conf="datasheet",
         cluster="light", contrast="medium", hal=False, note=""),
    dict(name="Kodak UltraMax 400", type="color_neg", iso=400, morph="tabular",
         grain=("pgi", 46, "kodak_pgi_35mm_4x6", "kodak_e7019"), conf="datasheet",
         cluster="moderate", contrast="medium-high", hal=False, note=""),
    dict(name="Kodak Portra 160", type="color_neg", iso=160, morph="tabular",
         grain=("pgi", 28, "kodak_pgi_35mm_4x6", "kodak_e4051"), conf="datasheet",
         cluster="light", contrast="low", hal=False, note=""),
    dict(name="Kodak Portra 400", type="color_neg", iso=400, morph="tabular",
         grain=("pgi", 37, "kodak_pgi_35mm_4x6", "kodak_e4050"), conf="datasheet",
         cluster="light", contrast="medium", hal=False, note=""),
    dict(name="Kodak Portra 800", type="color_neg", iso=800, morph="tabular",
         grain=("pgi", 48, "kodak_pgi_35mm_4x6", "kodak_e4040"), conf="datasheet",
         cluster="moderate", contrast="medium", hal=False, note=""),
    # ---- Kodak colour reversal ------------------------------------------
    dict(name="Kodak Ektachrome E100", type="color_rev", iso=100, morph="tabular",
         grain=("rms_diffuse", 8, "reverse_rms_48um", "kodak_e4024"), conf="derived",
         cluster="none", contrast="high", hal=False,
         note="rms 8 taken from the E100G datasheet (E-4024); the revived E100 uses the same T-GRAIN technology."),
    # ---- Kodak cinema ----------------------------------------------------
    dict(name="Kodak Vision3 500T", type="color_neg", iso=500, morph="tabular",
         grain=("rms_diffuse", 7, "reverse_rms_48um", "kodak_vision3_500t"), conf="estimated_from_curve",
         cluster="moderate", contrast="medium-high", hal=False,
         note="Datasheet gives R/G/B granularity curves only; ~7 sigma*1000 at D=1.0 read from curve."),
    dict(name="Kodak Vision3 50D", type="color_neg", iso=50, morph="tabular",
         grain=("rms_diffuse", 6, "reverse_rms_48um", "kodak_vision3_50d"), conf="estimated_from_curve",
         cluster="light", contrast="medium", hal=False,
         note="Datasheet gives R/G/B granularity curves only; ~6 sigma*1000 at D=1.0 read from curve."),
    # ---- Fujifilm B&W ----------------------------------------------------
    dict(name="Fuji Acros 100", type="bw_neg", iso=100, morph="sigma",
         grain=("rms_diffuse", 7, "fuji_bw_rms_48um", "fuji_acros"), conf="datasheet",
         cluster="none", contrast="medium", hal=False, rp=(60, 200),
         note="Neopan 100 Acros; Super Fine Sigma grain."),
    dict(name="Fuji Neopan 400", type="bw_neg", iso=400, morph="cubic",
         grain=("rms_diffuse", 12, "fuji_bw_rms_48um", None), conf="estimated",
         cluster="light", contrast="medium", hal=False,
         note="No numeric granularity in available AF datasheet; estimated from class (cubic ISO 400)."),
    dict(name="Fuji Neopan 1600", type="bw_neg", iso=1600, morph="cubic",
         grain=("rms_diffuse", 18, "fuji_bw_rms_48um", None), conf="estimated",
         cluster="moderate", contrast="medium-high", hal=False,
         note="No numeric granularity in available AF datasheet; estimated from class (fast cubic B&W)."),
    # ---- Fujifilm colour negative ---------------------------------------
    dict(name="Fuji Pro 160S", type="color_neg", iso=160, morph="sigma",
         grain=("rms_diffuse", 3, "fuji_colneg_rms_48um", "fuji_pro160s"), conf="datasheet",
         cluster="light", contrast="low", hal=False, rp=(63, 125), note=""),
    dict(name="Fuji Pro 400H", type="color_neg", iso=400, morph="sigma",
         grain=("rms_diffuse", 4, "fuji_colneg_rms_48um", "fuji_pro400h"), conf="datasheet",
         cluster="light", contrast="low-medium", hal=False, rp=(50, 125),
         note="Known for a pastel/green-cyan palette, not neutral colour."),
    dict(name="Fuji Superia 400", type="color_neg", iso=400, morph="sigma",
         grain=("rms_diffuse", 4, "fuji_colneg_rms_48um", "fuji_superia400"), conf="datasheet",
         cluster="moderate", contrast="medium", hal=False, rp=(50, 125),
         note="Superia X-TRA 400; 4th cyan-sensitive layer."),
    dict(name="Fuji C200", type="color_neg", iso=200, morph="sigma",
         grain=("rms_diffuse", 4, "fuji_colneg_rms_48um", None), conf="estimated",
         cluster="light", contrast="medium", hal=False,
         note="Consumer C-41 film; no PIB in archive. Estimated on the Fujifilm colour-negative scale."),
    dict(name="Fuji Natura 1600", type="color_neg", iso=1600, morph="sigma",
         grain=("rms_diffuse", 6, "fuji_colneg_rms_48um", None), conf="estimated",
         cluster="heavy", contrast="medium-high", hal=False,
         note="Japan-market high-speed film; no PIB in archive. Estimated on the Fujifilm colour-negative scale for ISO 1600."),
    # ---- Fujifilm colour reversal ---------------------------------------
    dict(name="Fuji Velvia 50", type="color_rev", iso=50, morph="tabular",
         grain=("rms_diffuse", 9, "reverse_rms_48um", "fuji_velvia50"), conf="datasheet",
         cluster="none", contrast="very high", hal=False, rp=(80, 160), note=""),
    dict(name="Fuji Velvia 100", type="color_rev", iso=100, morph="tabular",
         grain=("rms_diffuse", 8, "reverse_rms_48um", "fuji_velvia100"), conf="datasheet",
         cluster="none", contrast="very high", hal=False, rp=(80, 160), note=""),
    dict(name="Fuji Provia 100F", type="color_rev", iso=100, morph="tabular",
         grain=("rms_diffuse", 8, "reverse_rms_48um", "fuji_provia100f"), conf="datasheet",
         cluster="none", contrast="medium", hal=False, rp=(60, 140), note=""),
    dict(name="Fuji Provia 400X", type="color_rev", iso=400, morph="tabular",
         grain=("rms_diffuse", 11, "reverse_rms_48um", "fuji_provia400x"), conf="datasheet",
         cluster="none", contrast="medium", hal=False, rp=(55, 135), note=""),
    # ---- Ilford (no numeric granularity published) -----------------------
    dict(name="Ilford Delta 100", type="bw_neg", iso=100, morph="core_shell",
         grain=("rms_diffuse", 7, "third_party_estimate", None), conf="estimated",
         cluster="none", contrast="medium-high", hal=False,
         note="Ilford does not publish numeric RMS; estimated from class (core-shell tabular ISO 100)."),
    dict(name="Ilford Delta 400", type="bw_neg", iso=400, morph="core_shell",
         grain=("rms_diffuse", 11, "third_party_estimate", None), conf="estimated",
         cluster="light", contrast="low-medium", hal=False,
         note="Ilford does not publish numeric RMS; estimated from class."),
    dict(name="Ilford Delta 3200", type="bw_neg", iso=3200, morph="core_shell",
         grain=("rms_diffuse", 22, "third_party_estimate", None), conf="estimated",
         cluster="moderate", contrast="medium-high", hal=False,
         note="Ilford does not publish numeric RMS; estimated from class (nominal ISO 1000, EI 3200)."),
    dict(name="Ilford HP5 Plus", type="bw_neg", iso=400, morph="cubic",
         grain=("rms_diffuse", 17, "third_party_estimate", None), conf="estimated",
         cluster="light", contrast="medium", hal=False,
         note="Ilford does not publish numeric RMS; estimated from class (cubic ISO 400)."),
    dict(name="Ilford FP4 Plus", type="bw_neg", iso=125, morph="cubic",
         grain=("rms_diffuse", 11, "third_party_estimate", None), conf="estimated",
         cluster="none", contrast="medium", hal=False,
         note="Ilford does not publish numeric RMS; estimated from class."),
    dict(name="Ilford Pan F Plus 50", type="bw_neg", iso=50, morph="cubic",
         grain=("rms_diffuse", 7, "third_party_estimate", None), conf="estimated",
         cluster="light", contrast="high", hal=False,
         note="Ilford does not publish numeric RMS; estimated from class (slow cubic)."),
    # ---- Agfa (rebadged / no datasheet) ---------------------------------
    dict(name="Agfa CT Precisa 100", type="color_rev", iso=100, morph="tabular",
         grain=("rms_diffuse", 10, "reverse_rms_48um", "agfa_films"), conf="derived",
         cluster="none", contrast="very high", hal=False,
         note="AgfaPhoto-era reversal film. Value taken from Agfa's own RSX II 100 reversal class (RMS 10)."),
    dict(name="Agfa Vista 200", type="color_neg", iso=200, morph="cubic",
         grain=("rms_diffuse", 4.5, "fuji_colneg_rms_48um", "agfa_films"), conf="derived",
         cluster="light", contrast="medium", hal=False,
         note="AgfaPhoto-branded consumer film. Value from Agfa's Optima II 200 colour-negative class (RMS 4.5)."),
    dict(name="Agfa Vista 400", type="color_neg", iso=400, morph="cubic",
         grain=("rms_diffuse", 4.5, "fuji_colneg_rms_48um", "agfa_films"), conf="derived",
         cluster="moderate", contrast="medium", hal=False,
         note="AgfaPhoto-branded consumer film. Value from Agfa's Optima II 400 colour-negative class (RMS 4.5)."),
    # ---- CineStill (derived from Kodak Vision3) -------------------------
    dict(name="CineStill 50D", type="color_neg", iso=50, morph="tabular",
         grain=("rms_diffuse", 6, "reverse_rms_48um", "kodak_vision3_50d"), conf="derived",
         cluster="light", contrast="medium", hal=True,
         note="Rebadged Kodak Vision3 50D (ECN-2) with rem-jet removed; granularity derived from Vision3 50D."),
    dict(name="CineStill 800T", type="color_neg", iso=800, morph="tabular",
         grain=("rms_diffuse", 7, "reverse_rms_48um", "kodak_vision3_500t"), conf="derived",
         cluster="heavy", contrast="medium-high", hal=True,
         note="Rebadged Kodak Vision3 500T (ECN-2), rated ISO 800, rem-jet removed -> strong halation."),
]

# ---------------------------------------------------------------------------
# 2b. Additional manufacturer-published data (extracted from the same PDFs)
#     push: rated EI + EI reached per push stop, from datasheet push tables
#     ei_range: recommended working exposure-index range
#     anti_halation: only where the datasheet states it
# ---------------------------------------------------------------------------
PUSH = {
    "Kodak Tri-X 400":        dict(rated=400, ei=[400, 800, 1600, 3200], stops=3, src="kodak_f4017"),
    "Kodak T-Max 100":        dict(rated=100, ei=[100, 200, 400, 800], stops=3, src="kodak_f4016"),
    "Kodak T-Max 400":        dict(rated=400, ei=[400, 800, 1600, 3200], stops=3, src="kodak_f4043"),
    "Kodak Portra 800":       dict(rated=800, ei=[800, 1600, 3200], stops=2, src="kodak_e4040"),
    "Kodak Ektachrome E100":  dict(rated=100, ei=[100, 200], stops=1, src="kodak_e4024"),
    "Fuji Provia 400X":       dict(rated=400, ei=[280, 1600], stops=2, src="fuji_provia400x"),
    "Fuji Provia 100F":       dict(rated=100, ei=[100, 400], stops=2, src="fuji_provia100f"),
    "Fuji Velvia 50":         dict(rated=50, ei=[50, 100], stops=1, src="fuji_velvia50"),
    "Fuji Velvia 100":        dict(rated=100, ei=[100, 200], stops=1, src="fuji_velvia100"),
}
EI_RANGE = {
    "Ilford Delta 100":      [50, 200],
    "Ilford Delta 400":      [200, 3200],
    "Ilford Delta 3200":     [1000, 6400],
    "Ilford HP5 Plus":       [400, 3200],
    "Ilford FP4 Plus":       [50, 200],
    "Ilford Pan F Plus 50":  [25, 50],
    "Kodak Tri-X 400":       [400, 3200],
    "Kodak T-Max 100":       [100, 800],
    "Kodak T-Max 400":       [400, 3200],
    "Kodak Portra 800":      [800, 3200],
    "Kodak Ektachrome E100": [100, 200],
    "Fuji Provia 400X":      [280, 1600],
}
ANTI_HALATION = {
    "Ilford HP5 Plus": "Anti-halation backing that clears during development (sheet film).",
    "Ilford FP4 Plus": "Anti-halation backing that clears during development (sheet film).",
    "CineStill 50D": "Rem-jet anti-halation layer removed, producing halation.",
    "CineStill 800T": "Rem-jet anti-halation layer removed, producing strong halation.",
}
RESOLVING = {
    "Kodak T-Max 100":    (63, 200),
    "Kodak T-Max 400":    (50, 200),
    "Fuji Acros 100":     (60, 200),
    "Fuji Velvia 50":     (80, 160),
    "Fuji Velvia 100":    (80, 160),
    "Fuji Provia 100F":   (60, 140),
    "Fuji Provia 400X":   (55, 135),
    "Fuji Pro 160S":      (63, 125),
    "Fuji Pro 400H":      (50, 125),
    "Fuji Superia 400":   (50, 125),
}

# Long/short-exposure (reciprocity) compensation, where the datasheet is explicit.
RECIPROCITY = {
    "Fuji Pro 400H":   {"4s": "+1/2 stop", "16s": "+1 stop", "source": "fuji_pro400h"},
    "Fuji Superia 400": {"4s": "+1/3 stop", "16s": "+2/3 stop", "64s": "+1 stop", "source": "fuji_superia400"},
    "Fuji Pro 160S":   {"4s": "+1/2 stop", "16s": "+1 stop", "source": "fuji_pro160s"},
    "Ilford Delta 100": {"formula": "Ta = Tm^1.26 (no adjustment 1/10000-1s)", "source": "ilford_delta100"},
    "Ilford Delta 400": {"formula": "Ta = Tm^1.26 (no adjustment 1/10000-1s)", "source": "ilford_delta400"},
    "Ilford Delta 3200": {"formula": "Ta = Tm^1.26 (no adjustment 1/10000-1s)", "source": "ilford_delta3200"},
    "Ilford HP5 Plus": {"formula": "Ta = Tm^1.26 (no adjustment 1/10000-1s)", "source": "ilford_hp5"},
    "Ilford FP4 Plus": {"formula": "Ta = Tm^1.26 (no adjustment 1/10000-1s)", "source": "ilford_fp4"},
    "Ilford Pan F Plus 50": {"formula": "Ta = Tm^1.26 (no adjustment 1/10000-1s)", "source": "ilford_panf"},
    "Kodak T-Max 100": {"1/10000s": "+1/3 stop", "10s": "+1/3 stop", "100s": "+1 stop", "source": "kodak_f4016"},
    "Kodak Tri-X 400": {"1/100000s": "+1 stop", "1/10000s": "+1/2 stop", "source": "kodak_f4017"},
    "Kodak T-Max 400": {"1/10000s": "+1/3 stop", "10s": "+1/3 stop", "100s": "+1 stop", "source": "kodak_f4043"},
    "Kodak Vision3 500T": {"10s": "+1 stop", "source": "kodak_vision3_500t"},
    "Kodak Vision3 50D": {"note": "no adjustment 1/1000-1s", "source": "kodak_vision3_50d"},
}

# ---------------------------------------------------------------------------
# 3. Derived grain size model (NOT manufacturer data) -----------------------
# ---------------------------------------------------------------------------
ISO_SIZE = {50: 0.45, 100: 0.50, 125: 0.55, 160: 0.55, 200: 0.60,
            400: 0.90, 500: 0.95, 800: 1.10, 1600: 1.40, 3200: 1.80}
FAMILY_FACTOR = {"tabular": 0.9, "core_shell": 0.9, "sigma": 0.95, "cubic": 1.0}


def grain_size(iso, morph):
    base = ISO_SIZE.get(iso, 0.9)
    mean = round(base * FAMILY_FACTOR.get(morph, 1.0), 2)
    if morph in ("tabular", "core_shell"):
        note = ("Tabular emulsion: class data from Kodak's T-Grain paper (diameter up to ~2.3 um, "
                "thickness ~0.12 um, product aspect 3:1-5:1) scaled by ISO; not a per-film measurement.")
    else:
        note = ("Cubic/sigma emulsion: class range from emulsion literature "
                "(cubic reference grains ~1-1.4 um) scaled by ISO; not a per-film measurement.")
    return {
        "min": round(mean * 0.4, 2),
        "mean": mean,
        "max": round(mean * 2.2, 2),
        "distribution": "lognormal",
        "sigma": 0.35,
        "confidence": "estimated",
        "basis": note,
        "source": "kodak_tgrain_paper",
    }


# ---------------------------------------------------------------------------
# 4. Assemble
# ---------------------------------------------------------------------------
cur = json.load(open(os.path.join(ROOT, "fixed.json"), encoding="utf-8"))
col = json.load(open(os.path.join(ROOT, "color.json"), encoding="utf-8"))
var = json.load(open(os.path.join(ROOT, "variation.json"), encoding="utf-8"))
more = json.load(open(os.path.join(ROOT, "more.json"), encoding="utf-8"))
img = json.load(open(os.path.join(ROOT, "imagecolors.json"), encoding="utf-8"))

films = {}
for f in FILMS:
    metric, value, scale, src = f["grain"]
    c = col.get(f["name"], {}).get("base_grain_color", {})
    _fx = cur.get(f["name"], {})
    _gc = _fx.get("grain_characteristics", {})
    _dd = _fx.get("density_distribution", {})
    _ts = _fx.get("technical_specs", {})
    _fi = _fx.get("film_info", {})
    _mo = more.get(f["name"], {}).get("clustering_data", {})
    _va = var.get(f["name"], {})
    _img = img.get(f["name"], {})

    # What the app already stores for the five hard-to-source fields. These
    # values ship in the repo today but have no cited origin - treat as a
    # baseline to be measured/validated, never as a source of truth.
    app_baseline = {
        "grain_size_um": _gc.get("size_um"),
        "clustering": {
            "level": _dd.get("clustering"),
            "cluster_size": _dd.get("cluster_size"),
            "fractal_dimension": _mo.get("fractal_dimension"),
            "spatial_correlation": _mo.get("spatial_correlation"),
            "cluster_probability": _mo.get("cluster_probability"),
        },
        "grain_color": {
            "base_rgb": [c.get("r"), c.get("g"), c.get("b")] if c else None,
            "variation_rgb": col.get(f["name"], {}).get("color_variation"),
            "size_variation_coeff": _va.get("size_variation_coeff"),
            "opacity_variation": _va.get("opacity_variation"),
        },
        "halation_text": _ts.get("halation"),
        "push": {
            "processing": _fi.get("processing"),
            "iso_grain_relationship": _ts.get("iso_grain_relationship"),
            "exposure_response": _img.get("exposure_response"),
        },
        "source": "app repo baseline (fixed.json/color.json/variation.json/more.json/imagecolors.json)",
        "confidence": "unverified",
    }
    films[f["name"]] = {
        "type": f["type"],
        "iso": f["iso"],
        "graininess": {
            "metric": metric,
            "value": value,
            "scale": scale,
            "at_density": 1.0 if metric else None,
            "aperture_um": 48 if metric else None,
            "source": src,
            "confidence": f["conf"],
        },
        "resolving_power_lines_mm": (
            {"toc_1_6_to_1": rp[0], "toc_1000_to_1": rp[1],
             "source": "manufacturer datasheet"}
            if (rp := (RESOLVING.get(f["name"]) or f.get("rp"))) else None
        ),
        "morphology": {
            "family": f["morph"],
            "crystal": {"tabular": "tabular_silver_halide_T-grain",
                        "core_shell": "core-shell_tabular",
                        "sigma": "sigma/super-fine",
                        "cubic": "cubic_silver_halide"}[f["morph"]],
            "source": "wikipedia_tabular",
            "confidence": "datasheet" if f["conf"] == "datasheet" else "documented_general",
        },
        "grain_size_um": grain_size(f["iso"], f["morph"]),
        "clustering": {"level": f["cluster"], "source": "app_baseline (unverified)", "confidence": "estimated"},
        "contrast_level": f["contrast"],
        "color_cast": cur.get(f["name"], {}).get("film_info") and cur[f["name"]]["visual_properties"].get("color_cast"),
        "color_rgb": [c.get("r"), c.get("g"), c.get("b")] if c else None,
        "halation": {
            "enabled": f["hal"],
            "color": "#ff3b1f" if f["hal"] else None,
            "radius_um": (120 if f["hal"] else 0),
            "source": "wikipedia_halation",
            "confidence": "derived" if f["hal"] else "datasheet",
        },
        "push": (
            {**PUSH[f["name"]], "confidence": "datasheet"} if f["name"] in PUSH else None
        ),
        "push_stops": PUSH[f["name"]]["stops"] if f["name"] in PUSH else None,
        "ei_range": (
            {"min": EI_RANGE[f["name"]][0], "max": EI_RANGE[f["name"]][1],
             "source": "manufacturer datasheet"} if f["name"] in EI_RANGE else None
        ),
        "anti_halation": ANTI_HALATION.get(f["name"]),
        "reciprocity": RECIPROCITY.get(f["name"]),
        "notes": f.get("note", ""),
        "app_baseline": app_baseline,
    }

out = {
    "schema_version": 1,
    "generated": datetime.date.today().isoformat(),
    "description": "Sourced grain characterisation for the app's 35 film stocks. "
                   "'datasheet' = manufacturer-published value; 'derived' = computed from a "
                   "published parent film; 'estimated' = not published, literature/class based.",
    "sources": SOURCES,
    "films": films,
}

# Reference curves read from datasheet charts (see extracted/manual_curves.json).
# These are visual readings, flagged accordingly, not machine-traced.
_manual_path = os.path.join(HERE, "extracted", "manual_curves.json")
if os.path.exists(_manual_path):
    with open(_manual_path, encoding="utf-8") as _fh:
        _manual = json.load(_fh)
    for _name, _sets in _manual.items():
        if _name.startswith("_") or _name not in out["films"]:
            continue
        out["films"][_name]["curves"] = _sets


with io.open(os.path.join(HERE, "grain_data.json"), "w", encoding="utf-8") as fh:
    json.dump(out, fh, indent=2, ensure_ascii=False)
    fh.write("\n")

# Slim, app-facing copy consumed by the Rust backend (src-tauri/src/main.rs).
slim = {"schema_version": 1, "films": {}}
for name, f in films.items():
    slim["films"][name] = {
        "iso": f["iso"],
        "type": f["type"],
        "graininess": f["graininess"],
        "morphology": {"family": f["morphology"]["family"]},
        "grain_size_um": f["grain_size_um"],
        "push": f["push"],
        "ei_range": f["ei_range"],
        "resolving_power_lines_mm": f["resolving_power_lines_mm"],
        "anti_halation": f["anti_halation"],
        "halation": f["halation"],
    }
with io.open(os.path.join(ROOT, "grain_model.json"), "w", encoding="utf-8") as fh:
    json.dump(slim, fh, indent=2, ensure_ascii=False)
    fh.write("\n")

# ---------------------------------------------------------------------------
# 5. SOURCES.md
# ---------------------------------------------------------------------------
lines = ["# Grain data sources", "",
         "Collected to characterise the 35 film stocks. Primary sources are manufacturer "
         "data sheets; anything not published by the manufacturer is marked `estimated`/`derived`.", ""]
for sid, s in SOURCES.items():
    lines += [f"## {sid}", f"- **{s['title']}**", f"- {s['url']}", f"- Provides: {s['provides']}", ""]
open(os.path.join(HERE, "SOURCES.md"), "w", encoding="utf-8").write("\n".join(lines) + "\n")

# ---------------------------------------------------------------------------
# 6. data_audit.md - compare against the app's current data
# ---------------------------------------------------------------------------
audit = ["# Data audit: collected values vs current app data", "",
         "Current app data in `fixed.json` / `color.json` never carried a graininess metric "
         "(RMS or PGI); it used an invented `grains_per_mm2` and a `shape` string. The columns "
         "below show what the app has today versus what the sources give.", "",
         "| Film | Current `shape` (fixed.json) | True morphology | Current `density_per_mm2` | Source graininess |",
         "|---|---|---|---|---|"]
shape_map = {"tabular": "tabular", "core_shell": "core-shell (tabular)",
             "sigma": "sigma", "cubic": "cubic"}
for f in FILMS:
    name = f["name"]
    cur_shape = cur.get(name, {}).get("grain_characteristics", {}).get("shape", "?")
    cur_dens = cur.get(name, {}).get("density_distribution", {}).get("grains_per_mm2", "?")
    metric, value, scale, _ = f["grain"]
    g = f"{value} {metric}" if value is not None else "not published (estimated)"
    audit.append(f"| {name} | {cur_shape} | {shape_map[f['morph']]} | {cur_dens} | {g} |")
audit += ["", "## Notes", "",
          "- `shape` mismatches (e.g. Kodak Ektar/Portra, Vision3, CineStill marked `irregular` but are tabular T-GRAIN).",
          "- No film had a published graininess value stored; the app's `density_per_mm2` is unrelated to the RMS/PGI measures.",
          "- Ilford, Agfa and some Fuji films publish no numeric granularity; these are flagged `estimated`.",
          ""]
open(os.path.join(HERE, "data_audit.md"), "w", encoding="utf-8").write("\n".join(audit) + "\n")

print(f"Wrote grain_data.json ({len(films)} films), SOURCES.md, data_audit.md")
published = sum(1 for f in FILMS if f["conf"] == "datasheet")
print(f"  datasheet-backed graininess: {published}/{len(FILMS)}")
