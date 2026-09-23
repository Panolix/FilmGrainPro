#!/usr/bin/env python3
"""
datasheet curve digitizer

Extracts numeric curves from manufacturer datasheet PDFs (graininess, characteristic,
spectral-sensitivity, reciprocity, MTF ...). Uses ghostscript to rasterise and
ImageMagick to convert pixels, so it needs no Python imaging libraries.

Pipeline
--------
  1. render  : PDF page -> PNG at a chosen DPI
  2. detect  : find the plot frame (long dark horizontal/vertical lines)
  3. digitize: trace the curve(s) inside a plot box and convert pixel -> data units
  4. overlay : draw the digitised points back on the image for visual verification

Axis value ranges (e.g. x = log exposure 0..4) are not printed as data the tool can
reliably read, so they are supplied on the command line after inspecting the chart.

Examples
--------
  python3 digitize_curve.py render datasheet.pdf 5 600 page5.png
  python3 digitize_curve.py detect page5.png
  python3 digitize_curve.py digitize page5.png --box 380 260 1180 900 \
      --xdata 0 4 --ydata 0 3 --channel dark --out curve.csv
  python3 digitize_curve.py overlay page5.png --box 380 260 1180 900 \
      --xdata 0 4 --ydata 0 3 --out overlay.png
"""
import argparse
import os
import subprocess
import sys

def run(cmd, **kw):
    return subprocess.run(cmd, check=True, capture_output=True, **kw)

# ---------------------------------------------------------------- pixel loading

def _read_pnm_header(data, pos=0):
    """Parse a PNM header; return (magic, width, height, maxval, pos)."""
    tokens = []
    while len(tokens) < 4:
        # skip whitespace and comments
        while pos < len(data) and data[pos:pos+1].isspace():
            pos += 1
        if data[pos:pos+1] == b'#':
            while pos < len(data) and data[pos:pos+1] != b'\n':
                pos += 1
            continue
        start = pos
        while pos < len(data) and not data[pos:pos+1].isspace():
            pos += 1
        tokens.append(data[start:pos])
    magic = tokens[0]
    width = int(tokens[1]); height = int(tokens[2]); maxval = int(tokens[3])
    pos += 1  # single whitespace after maxval
    return magic, width, height, maxval, pos

def load_ppm(path):
    with open(path, 'rb') as fh:
        data = fh.read()
    magic, w, h, maxval, pos = _read_pnm_header(data)
    if magic != b'P6':
        raise ValueError(f"{path}: expected P6 PPM, got {magic!r}")
    if maxval != 255:
        raise ValueError(f"{path}: only 8-bit PPM supported (maxval={maxval})")
    return w, h, data[pos:pos + w * h * 3]

def load_pgm(path):
    with open(path, 'rb') as fh:
        data = fh.read()
    magic, w, h, maxval, pos = _read_pnm_header(data)
    if magic != b'P5':
        raise ValueError(f"{path}: expected P5 PGM, got {magic!r}")
    return w, h, data[pos:pos + w * h]

def to_ppm(png):
    out = run(['magick', png, '-depth', '8', 'ppm:-']).stdout
    tmp = os.path.join(os.path.dirname(png) or '.', '.dg_tmp.ppm')
    with open(tmp, 'wb') as fh:
        fh.write(out)
    return tmp

def to_pgm(png):
    out = run(['magick', png, '-colorspace', 'Gray', '-depth', '8', 'pgm:-']).stdout
    tmp = os.path.join(os.path.dirname(png) or '.', '.dg_tmp.pgm')
    with open(tmp, 'wb') as fh:
        fh.write(out)
    return tmp

# ---------------------------------------------------------------- commands

def cmd_render(args):
    run(['gs', '-q', '-dNOPAUSE', '-dBATCH', '-dSAFER', '-sDEVICE=png16m',
         f'-r{args.dpi}', f'-dFirstPage={args.page}', f'-dLastPage={args.page}',
         f'-sOutputFile={args.out}', args.pdf])
    print(f"rendered {args.pdf} page {args.page} @ {args.dpi}dpi -> {args.out}")

def cmd_detect(args):
    pgm = to_pgm(args.png)
    w, h, px = load_pgm(pgm)
    dark = args.dark
    # fraction of dark pixels per row / per column
    rows = [sum(1 for x in range(w) if px[y * w + x] < dark) / w for y in range(h)]
    cols = [sum(1 for y in range(h) if px[y * w + x] < dark) / h for x in range(w)]
    def lines(frac, thr):
        hits = [i for i, f in enumerate(frac) if f >= thr]
        # group consecutive indices
        groups = []
        for i in hits:
            if groups and i - groups[-1][-1] <= 2:
                groups[-1].append(i)
            else:
                groups.append([i])
        return [int(sum(g) / len(g)) for g in groups]
    hl = lines(rows, args.line_thr)
    vl = lines(cols, args.line_thr)
    print(f"image {w}x{h}")
    print(f"horizontal frame lines (y): {hl}")
    print(f"vertical frame lines (x):   {vl}")
    if hl and vl:
        print(f"suggested box: {vl[0]} {hl[0]} {vl[-1]} {hl[-1]}")

def _classify(r, g, b, channel, dark):
    lum = (r + g + b) / 3
    if channel == 'dark':
        return lum < dark
    if channel == 'r':
        return r - max(g, b) > 25 and lum < 250
    if channel == 'g':
        return g - max(r, b) > 25 and lum < 250
    if channel == 'b':
        return b - max(r, g) > 25 and lum < 250
    return lum < dark

def _trace(png, box, xdata, ydata, channel, dark, invert_y):
    ppm = to_ppm(png)
    w, h, px = load_ppm(ppm)
    x0, y0o, x1, y1o = box
    y0, y1 = y0o + 3, y1o - 3          # inset to ignore the plot border
    span = y1 - y0 + 1
    pts = []
    for x in range(x0 + 2, x1 - 1):
        rows = []
        for y in range(y0, y1 + 1):
            i = (y * w + x) * 3
            if _classify(px[i], px[i + 1], px[i + 2], channel, dark):
                rows.append(y)
        if not rows:
            continue
        # A curve is a thin line; a column that is mostly dark is a frame/grid line.
        if len(rows) > 0.5 * span:
            continue
        y = sorted(rows)[len(rows) // 2]
        fx = (x - x0) / max(1, (x1 - x0))
        fy = (y - y0o) / max(1, (y1o - y0o))
        dx = xdata[0] + fx * (xdata[1] - xdata[0])
        if invert_y:
            dy = ydata[1] - fy * (ydata[1] - ydata[0])
        else:
            dy = ydata[0] + fy * (ydata[1] - ydata[0])
        pts.append((dx, dy, x, y))
    return pts, (w, h)

def _runs(px, w, x, y0, y1, channel, dark):
    """Return [(start,end)] pixel rows that are dark in column x."""
    runs = []
    start = None
    for y in range(y0, y1 + 1):
        i = (y * w + x) * 3
        on = _classify(px[i], px[i + 1], px[i + 2], channel, dark)
        if on and start is None:
            start = y
        elif not on and start is not None:
            runs.append((start, y - 1)); start = None
    if start is not None:
        runs.append((start, y1))
    return runs

def _trace_multi(png, box, xdata, ydata, channel, dark, invert_y, tol=15, min_points=30, min_range=0.0):
    """Trace several curves at once by tracking dark segments across columns."""
    ppm = to_ppm(png)
    w, h, px = load_ppm(ppm)
    x0, y0o, x1, y1o = box
    y0, y1 = y0o + 3, y1o - 3          # inset to ignore the plot border
    span = y1 - y0 + 1
    # minimum vertical extent (pixels) a trace must have to count as a curve, used to
    # reject horizontal text lines inside the plot
    min_px = min_range / max(1e-6, (ydata[1] - ydata[0])) * (y1o - y0o)
    traces = []  # list of [(x, midy)]
    for x in range(x0 + 2, x1 - 1):
        runs = [r for r in _runs(px, w, x, y0, y1, channel, dark) if (r[1] - r[0] + 1) < 0.5 * span]
        mids = [(a + b) / 2.0 for a, b in runs]
        used = set()
        for tr in traces:
            last = tr[-1][1]
            best, bestd = None, tol
            for i, m in enumerate(mids):
                if i in used:
                    continue
                d = abs(m - last)
                if d < bestd:
                    bestd, best = d, i
            if best is not None:
                used.add(best)
                tr.append((x, mids[best]))
        for i, m in enumerate(mids):
            if i not in used:
                traces.append([(x, m)])
    # keep traces that are long enough and vary enough vertically
    traces = [t for t in traces if len(t) >= min_points
              and (max(p[1] for p in t) - min(p[1] for p in t)) >= min_px]
    # merge traces that follow the same line (a thick/anti-aliased stroke can split)
    traces.sort(key=len, reverse=True)
    merged = []
    for tr in traces:
        ymap = {x: y for x, y in tr}
        attached = False
        for m in merged:
            shared = [x for x, _ in tr if x in {mx for mx, _ in m}]
            if len(shared) < 0.3 * len(tr):
                continue
            my = {x: y for x, y in m}
            close = sum(1 for x in shared if abs(ymap[x] - my[x]) <= 6)
            if close >= 0.7 * len(shared):
                m.extend((x, y) for x, y in tr if x not in my)
                m.sort()
                attached = True
                break
        if not attached:
            merged.append(list(tr))
    traces = merged
    traces.sort(key=lambda t: t[0][1] + t[-1][1])
    out = []
    for ci, tr in enumerate(traces):
        pts = []
        for x, y in tr:
            fx = (x - x0) / max(1, (x1 - x0))
            fy = (y - y0o) / max(1, (y1o - y0o))
            dx = xdata[0] + fx * (xdata[1] - xdata[0])
            dy = (ydata[1] - fy * (ydata[1] - ydata[0])) if invert_y else \
                 (ydata[0] + fy * (ydata[1] - ydata[0]))
            pts.append((ci, dx, dy, x, y))
        out.append(pts)
    return out, (w, h)

def apply_masks(png, rects):
    """Whiten annotation regions (text blocks / legends) before tracing."""
    if not rects:
        return png
    out = png + '.masked.png'
    cmd = ['magick', png, '-fill', 'white']
    for x0, y0, x1, y1 in rects:
        cmd += ['-draw', f'rectangle {x0},{y0} {x1},{y1}']
    cmd += [out]
    run(cmd)
    return out

def cmd_digitize(args):
    png = apply_masks(args.png, args.mask)
    if args.multi:
        traces, _ = _trace_multi(png, args.box, args.xdata, args.ydata,
                                 args.channel, args.dark, args.invert_y, args.tol, args.min_points, args.min_range)
        if not traces:
            print("no curves found - check --box/--channel/--dark", file=sys.stderr); sys.exit(2)
        with open(args.out, 'w') as fh:
            fh.write("curve,x,y\n")
            for tr in traces:
                for ci, dx, dy, _, _ in tr:
                    fh.write(f"{ci},{dx:.5f},{dy:.5f}\n")
        print(f"digitised {len(traces)} curves ({[len(t) for t in traces]} points) -> {args.out}")
        return
    pts, _ = _trace(png, args.box, args.xdata, args.ydata, args.channel, args.dark, args.invert_y)
    if not pts:
        print("no curve pixels found - check --box, --channel, --dark", file=sys.stderr)
        sys.exit(2)
    with open(args.out, 'w') as fh:
        fh.write("x,y\n")
        for dx, dy, _, _ in pts:
            fh.write(f"{dx:.5f},{dy:.5f}\n")
    xs = [p[0] for p in pts]
    print(f"digitised {len(pts)} points over x=[{min(xs):.3f},{max(xs):.3f}] -> {args.out}")

def cmd_overlay(args):
    colors = ['magenta', 'blue', 'red', 'green', 'orange', 'purple']
    png = apply_masks(args.png, args.mask)
    if args.multi:
        traces, _ = _trace_multi(png, args.box, args.xdata, args.ydata,
                                 args.channel, args.dark, args.invert_y, args.tol, args.min_points, args.min_range)
        draw = []
        for tr in traces:
            draw.append(f"stroke {colors[tr[0][0] % len(colors)]}")
            for ci, dx, dy, x, y in tr:
                draw.append(f"circle {x},{y} {x+1.5},{y}")
        run(['magick', png, '-fill', 'none', '-strokewidth', '1', '-draw', ' '.join(draw), args.out])
        print(f"wrote overlay with {len(traces)} curves -> {args.out}")
        return
    pts, (w, h) = _trace(png, args.box, args.xdata, args.ydata, args.channel, args.dark, args.invert_y)
    draws = []
    for dx, dy, x, y in pts:
        draws.append(f"circle {x},{y} {x+1.5},{y}")
    run(['magick', args.png, '-fill', 'none', '-stroke', 'magenta', '-strokewidth', '1',
         '-draw', ' '.join(draws), args.out])
    print(f"wrote overlay with {len(pts)} points -> {args.out}")

# ---------------------------------------------------------------- CLI

def main():
    ap = argparse.ArgumentParser(description="Digitise curves from datasheet PDFs")
    sub = ap.add_subparsers(dest='cmd', required=True)

    r = sub.add_parser('render'); r.set_defaults(func=cmd_render)
    r.add_argument('pdf'); r.add_argument('page', type=int); r.add_argument('dpi', type=int); r.add_argument('out')

    d = sub.add_parser('detect'); d.set_defaults(func=cmd_detect)
    d.add_argument('png')
    d.add_argument('--dark', type=int, default=128)
    d.add_argument('--line-thr', dest='line_thr', type=float, default=0.6)

    g = sub.add_parser('digitize'); g.set_defaults(func=cmd_digitize)
    g.add_argument('png')
    g.add_argument('--box', type=int, nargs=4, required=True, metavar=('X0', 'Y0', 'X1', 'Y1'))
    g.add_argument('--xdata', type=float, nargs=2, required=True, metavar=('XMIN', 'XMAX'))
    g.add_argument('--ydata', type=float, nargs=2, required=True, metavar=('YMIN', 'YMAX'))
    g.add_argument('--channel', choices=['dark', 'r', 'g', 'b'], default='dark')
    g.add_argument('--dark', type=int, default=128)
    g.add_argument('--invert-y', dest='invert_y', action='store_true')
    g.add_argument('--multi', action='store_true', help='trace several curves at once')
    g.add_argument('--tol', type=float, default=15.0, help='max y-jump to follow a curve (px)')
    g.add_argument('--min-points', dest='min_points', type=int, default=30)
    g.add_argument('--min-range', dest='min_range', type=float, default=0.0, help='min y-extent (data units) to count as a curve')
    g.add_argument('--mask', type=int, nargs=4, action='append', default=[], metavar=('X0','Y0','X1','Y1'),
                   help='whiten an annotation region before tracing (repeatable)')
    g.add_argument('--out', required=True)

    o = sub.add_parser('overlay'); o.set_defaults(func=cmd_overlay)
    o.add_argument('png')
    o.add_argument('--box', type=int, nargs=4, required=True, metavar=('X0', 'Y0', 'X1', 'Y1'))
    o.add_argument('--xdata', type=float, nargs=2, required=True, metavar=('XMIN', 'XMAX'))
    o.add_argument('--ydata', type=float, nargs=2, required=True, metavar=('YMIN', 'YMAX'))
    o.add_argument('--channel', choices=['dark', 'r', 'g', 'b'], default='dark')
    o.add_argument('--dark', type=int, default=128)
    o.add_argument('--invert-y', dest='invert_y', action='store_true')
    o.add_argument('--multi', action='store_true')
    o.add_argument('--tol', type=float, default=15.0)
    o.add_argument('--min-points', dest='min_points', type=int, default=30)
    o.add_argument('--min-range', dest='min_range', type=float, default=0.0)
    o.add_argument('--mask', type=int, nargs=4, action='append', default=[], metavar=('X0','Y0','X1','Y1'))
    o.add_argument('--out', required=True)

    args = ap.parse_args()
    args.func(args)

if __name__ == '__main__':
    main()
