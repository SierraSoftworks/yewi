"""Outlines the yewi wordmark and mark from the bundled fonts into assets/.

Needs `pip install fonttools brotli uharfbuzz`. Usage: python3 tools/gen-logo.py
"""
import io
from pathlib import Path
from fontTools.ttLib import TTFont
import uharfbuzz as hb

ROOT = Path(__file__).resolve().parent.parent
FONTS = ROOT / "yewi-fonts" / "fonts"
FONT = "montserrat-latin.woff2"
TRACKING = -0.02  # em
# (text, weight, colour role): a light "yew" and a bold accent "i", read as "UI".
WORDMARK = [("yew", 300, "fg"), ("i", 600, "accent")]
# One step heavier so the mark survives at favicon size.
MARK = [("y", 400, "fg"), ("i", 700, "accent")]
# --yi-fg and the accent as text, resolved for the default accent in each theme.
LIGHT = {"fg": "#15171a", "accent": "#0e6abc"}
DARK = {"fg": "#d0d0d1", "accent": "#69b4ff"}
TILE = {"fg": "#ffffff", "accent": "#69b4ff", "bg": "#15171a"}

def load(name):
    f = TTFont(FONTS / name); f.flavor = None
    buf = io.BytesIO(); f.save(buf)
    return hb.Face(buf.getvalue())

class Pen:
    def __init__(s, dx): s.d = []; s.dx = dx
    def p(s, x, y): return f"{round(x + s.dx, 1):g} {round(-y, 1):g}"
    def move_to(s, x, y): s.d.append("M" + s.p(x, y))
    def line_to(s, x, y): s.d.append("L" + s.p(x, y))
    def quad_to(s, a, b, x, y): s.d.append("Q" + s.p(a, b) + " " + s.p(x, y))
    def cubic_to(s, a, b, c, d, x, y): s.d.append("C" + s.p(a, b) + " " + s.p(c, d) + " " + s.p(x, y))
    def close_path(s): s.d.append("Z")

def outline(face, tracking, runs):
    """Returns ([(role, path)], width, (ymin, ymax)) in font units, y down, baseline 0."""
    upem = face.upem; x = 0; out = []; ys = []; xs = []
    for text, wght, role in runs:
        font = hb.Font(face); font.set_variations({"wght": wght})
        buf = hb.Buffer(); buf.add_str(text); buf.guess_segment_properties()
        hb.shape(font, buf, {"liga": False, "calt": False})
        pen = Pen(0)
        funcs = hb.DrawFuncs()
        funcs.set_move_to_func(lambda x_, y_, p: p.move_to(x_, y_))
        funcs.set_line_to_func(lambda x_, y_, p: p.line_to(x_, y_))
        funcs.set_quadratic_to_func(lambda a, b, x_, y_, p: p.quad_to(a, b, x_, y_))
        funcs.set_cubic_to_func(lambda a, b, c, d, x_, y_, p: p.cubic_to(a, b, c, d, x_, y_))
        funcs.set_close_path_func(lambda p: p.close_path())
        for info, pos in zip(buf.glyph_infos, buf.glyph_positions):
            pen.dx = x + pos.x_offset
            font.draw_glyph(info.codepoint, funcs, pen)
            ext = font.get_glyph_extents(info.codepoint)
            xs += [x + ext.x_bearing, x + ext.x_bearing + ext.width]
            ys += [-ext.y_bearing, -(ext.y_bearing + ext.height)]
            x += pos.x_advance + tracking * upem
        out.append((role, "".join(pen.d)))
    return out, min(xs), max(xs), min(ys), max(ys)

def wordmark(pal, pad=40):
    paths, x0, x1, y0, y1 = outline(load(FONT), TRACKING, WORDMARK)
    w, h = x1 - x0 + 2 * pad, y1 - y0 + 2 * pad
    body = "".join(f'<path fill="{pal[r]}" d="{d}"/>' for r, d in paths)
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{x0 - pad:g} {y0 - pad:g} {w:g} {h:g}" '
            f'width="{w * 64 / h:.0f}" height="64" role="img" aria-label="yewi"><title>yewi</title>{body}</svg>\n')

def mark():
    paths, x0, x1, y0, y1 = outline(load(FONT), TRACKING, MARK)
    size = max(x1 - x0, y1 - y0) / 0.56
    ox, oy = (x0 + x1) / 2 - size / 2, (y0 + y1) / 2 - size / 2
    body = "".join(f'<path fill="{TILE[r]}" d="{d}"/>' for r, d in paths)
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{ox:g} {oy:g} {size:g} {size:g}" width="64" height="64" '
            f'role="img" aria-label="yewi"><title>yewi</title>'
            f'<rect x="{ox:g}" y="{oy:g}" width="{size:g}" height="{size:g}" rx="{size * 0.22:g}" fill="{TILE["bg"]}"/>{body}</svg>\n')

out = ROOT / "assets"
out.mkdir(exist_ok=True)
(out / "logo-light.svg").write_text(wordmark(LIGHT))
(out / "logo-dark.svg").write_text(wordmark(DARK))
(out / "mark.svg").write_text(mark())
