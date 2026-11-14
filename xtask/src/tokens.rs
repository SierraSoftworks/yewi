//! Generates `yewi-tokens/src/generated.rs` from `yewi/scss/tokens.scss`.
//!
//! The stylesheet is compiled with `grass`, as the `yewi` build script does,
//! and each `--yi-*` declaration is then evaluated the way a browser would:
//! `var()`, `light-dark()`, `color-mix()` and relative `oklch()` are worked
//! out for both schemes. A value this cannot evaluate is an error, so a new
//! kind of token is noticed here rather than silently left out.

use std::{collections::BTreeMap, f64::consts::PI, fmt::Write as _, path::Path};

const BASE: &str = ":where(:root, [data-yi-scope])";
const COMPACT: &str = ":where([data-yi-density=compact])";
const REM: f64 = 16.0;

type Result<T> = std::result::Result<T, String>;

/// The source of `generated.rs` for the stylesheet under `root`.
pub fn generate(root: &Path) -> String {
    let scss = root.join("yewi/scss");
    let options = grass::Options::default().load_path(&scss);
    let css = grass::from_path(scss.join("tokens.scss"), &options).unwrap_or_else(|e| panic!("tokens.scss: {e}"));
    render(&css).unwrap_or_else(|e| panic!("tokens.scss: {e}"))
}

fn render(css: &str) -> Result<String> {
    let base = declarations(css, BASE)?;
    let compact = declarations(css, COMPACT)?;
    let env: Env = base.iter().cloned().collect();

    let mut out = String::from("// Generated from yewi/scss/tokens.scss by `cargo xtask tokens`. Do not edit.\n\n");
    out.push_str("use crate::{Color, Rgba, Shadow};\n");
    for (name, value) in &base {
        write!(out, "\n{}", constant(name, value, &env, "")?).unwrap();
    }

    // The compact density, evaluated with its own values in place.
    let mut env = env;
    env.extend(compact.iter().cloned());
    out.push_str("\n/// The tokens that `data-yi-density=\"compact\"` changes.\npub mod compact {\n");
    for (at, (name, value)) in compact.iter().enumerate() {
        let separator = if at == 0 { "" } else { "\n" };
        write!(out, "{separator}{}", constant(name, value, &env, "    ")?).unwrap();
    }
    out.push_str("}\n");
    Ok(out)
}

/// The `--yi-*` declarations of the rule with this selector, in order.
fn declarations(css: &str, selector: &str) -> Result<Vec<(String, String)>> {
    let normalise = |text: &str| text.replace('"', "").split_whitespace().collect::<Vec<_>>().join(" ");
    let (mut found, mut rest) = (Vec::new(), css);
    while let Some((head, tail)) = rest.split_once('{') {
        let (body, tail) = tail.split_once('}').ok_or("unclosed rule")?;
        rest = tail;
        if normalise(head) != selector {
            continue;
        }
        for declaration in body.split(';') {
            if let Some((name, value)) = declaration.split_once(':')
                && let Some(name) = name.trim().strip_prefix("--yi-")
            {
                found.push((name.to_owned(), value.split_whitespace().collect::<Vec<_>>().join(" ")));
            }
        }
    }
    if found.is_empty() { Err(format!("no tokens under `{selector}`")) } else { Ok(found) }
}

/// One `pub const`, with the property it comes from as its documentation.
fn constant(name: &str, value: &str, env: &Env, indent: &str) -> Result<String> {
    let (ty, expr) = rust(name, value, env).map_err(|e| format!("--yi-{name}: {e}"))?;
    let ident = name.replace('-', "_").to_uppercase();
    Ok(format!("{indent}/// `--yi-{name}`\n{indent}pub const {ident}: {ty} = {expr};\n"))
}

/// The Rust type and expression for a token's value.
fn rust(name: &str, value: &str, env: &Env) -> Result<(&'static str, String)> {
    let groups = parse(value)?;
    let single = if let [group] = groups.as_slice() { Some(group.as_slice()) } else { None };

    if let Some(group) = single
        && let Some(color) = themed(group, env)
    {
        return Ok(("Color", color));
    }
    if let Some([Node::Number(number, unit)]) = single {
        let weight = name.contains("weight") || name.starts_with("z-");
        return match unit.as_str() {
            "" if weight && number.fract() == 0.0 => Ok(("u16", format!("{number}"))),
            "" | "px" => Ok(("f32", float(*number))),
            "rem" => Ok(("f32", float(number * REM))),
            "ms" => Ok(("f32", float(*number))),
            "s" => Ok(("f32", float(number * 1000.0))),
            _ => Err(format!("unknown unit `{unit}`")),
        };
    }
    if let Some([Node::Function(function, args)]) = single
        && function == "cubic-bezier"
    {
        let points: Vec<String> = args.iter().map(|arg| scalar(arg, env).map(float)).collect::<Result<_>>()?;
        return Ok(("[f32; 4]", format!("[{}]", points.join(", "))));
    }
    if let Some([Node::Ident(keyword)]) = single {
        return Ok(("&str", format!("{keyword:?}")));
    }
    if let Ok(layers) = groups.iter().map(|group| shadow(group, env)).collect::<Result<Vec<_>>>() {
        return Ok(("&[Shadow]", format!("&[\n    {},\n]", layers.join(",\n    "))));
    }
    let families = families(&groups, env)?;
    Ok(("&[&str]", format!("&{families:?}")))
}

fn float(number: f64) -> String {
    format!("{:?}", number as f32)
}

fn themed(group: &[Node], env: &Env) -> Option<String> {
    let rgba = |scheme| {
        let [r, g, b, a] = color(group, env, scheme).ok()?.map(|channel| (channel.clamp(0.0, 1.0) * 255.0).round());
        Some(format!("Rgba::new({r}, {g}, {b}, {a})"))
    };
    Some(format!("Color {{ light: {}, dark: {} }}", rgba(Scheme::Light)?, rgba(Scheme::Dark)?))
}

/// `<x> <y> <blur> [<spread>] <colour>`
fn shadow(group: &[Node], env: &Env) -> Result<String> {
    let lengths: Vec<f64> = group.iter().map_while(|node| length(node).ok()).collect();
    let [x, y, blur, spread] = match lengths.as_slice() {
        [x, y, blur] => [*x, *y, *blur, 0.0],
        [x, y, blur, spread] => [*x, *y, *blur, *spread],
        _ => return Err("not a shadow".into()),
    };
    let color = themed(&group[lengths.len()..], env).ok_or("a shadow needs a colour")?;
    let [x, y, blur, spread] = [x, y, blur, spread].map(float);
    Ok(format!("Shadow {{ x: {x}, y: {y}, blur: {blur}, spread: {spread}, color: {color} }}"))
}

fn length(node: &Node) -> Result<f64> {
    match node {
        Node::Number(number, unit) if unit == "px" || (unit.is_empty() && *number == 0.0) => Ok(*number),
        Node::Number(number, unit) if unit == "rem" => Ok(number * REM),
        _ => Err("not a length".into()),
    }
}

/// A font stack, with any `var()` to another stack spliced in.
fn families(groups: &[Vec<Node>], env: &Env) -> Result<Vec<String>> {
    let mut found = Vec::new();
    for group in groups {
        match group.as_slice() {
            [Node::String(name)] => found.push(name.clone()),
            [Node::Function(function, args)] if function == "var" => {
                found.extend(families(&parse(lookup(args, env)?)?, env)?);
            }
            names if !names.is_empty() && names.iter().all(|node| matches!(node, Node::Ident(_))) => {
                let words: Vec<&str> =
                    names.iter().map(|node| if let Node::Ident(word) = node { word.as_str() } else { "" }).collect();
                found.push(words.join(" "));
            }
            _ => return Err("not a value this generator understands".into()),
        }
    }
    Ok(found)
}

// ---- Values -------------------------------------------------------------

type Env = BTreeMap<String, String>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Scheme {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Number(f64, String),
    Ident(String),
    Hash(String),
    String(String),
    /// A name and its comma-separated arguments.
    Function(String, Vec<Vec<Node>>),
    /// `/` or `*`.
    Operator(char),
}

/// Parses a value into its comma-separated groups of components.
fn parse(value: &str) -> Result<Vec<Vec<Node>>> {
    let chars: Vec<char> = value.chars().collect();
    let mut at = 0;
    let groups = parse_groups(&chars, &mut at)?;
    if at < chars.len() { Err(format!("unexpected `{}`", chars[at])) } else { Ok(groups) }
}

fn parse_groups(chars: &[char], at: &mut usize) -> Result<Vec<Vec<Node>>> {
    let mut groups = vec![Vec::new()];
    let take = |at: &mut usize, keep: fn(char) -> bool| -> String {
        let start = *at;
        while *at < chars.len() && keep(chars[*at]) {
            *at += 1;
        }
        chars[start..*at].iter().collect()
    };
    while let Some(&c) = chars.get(*at) {
        let next = chars.get(*at + 1).copied().unwrap_or(' ');
        let group = groups.last_mut().unwrap();
        match c {
            ')' => break,
            ',' => {
                *at += 1;
                groups.push(Vec::new());
            }
            c if c.is_whitespace() => *at += 1,
            '/' | '*' => {
                *at += 1;
                group.push(Node::Operator(c));
            }
            '#' => {
                *at += 1;
                group.push(Node::Hash(take(at, |c| c.is_ascii_hexdigit())));
            }
            '"' => {
                *at += 1;
                group.push(Node::String(take(at, |c| c != '"')));
                *at += 1;
            }
            c if c.is_ascii_digit() || (matches!(c, '.' | '-') && (next.is_ascii_digit() || next == '.')) => {
                *at += 1;
                let digits = format!("{c}{}", take(at, |c| c.is_ascii_digit() || c == '.'));
                let number = digits.parse().map_err(|_| format!("bad number `{digits}`"))?;
                group.push(Node::Number(number, take(at, |c| c.is_ascii_alphabetic() || c == '%')));
            }
            c if c.is_ascii_alphabetic() || c == '-' => {
                let name = take(at, |c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
                if chars.get(*at) == Some(&'(') {
                    *at += 1;
                    let args = parse_groups(chars, at)?;
                    if chars.get(*at) != Some(&')') {
                        return Err(format!("unclosed `{name}(`"));
                    }
                    *at += 1;
                    group.push(Node::Function(name, args));
                } else {
                    group.push(Node::Ident(name));
                }
            }
            _ => return Err(format!("unexpected `{c}`")),
        }
    }
    Ok(groups)
}

/// The value of the property a `var()` names.
fn lookup<'a>(args: &[Vec<Node>], env: &'a Env) -> Result<&'a str> {
    let [Node::Ident(name)] = args.first().map(Vec::as_slice).unwrap_or_default() else {
        return Err("var() needs a name".into());
    };
    let token = name.strip_prefix("--yi-").unwrap_or(name);
    env.get(token).map(String::as_str).ok_or_else(|| format!("{name} is not a token"))
}

/// A plain number: a literal, a `var()` of one, or a `calc()` product.
/// Percentages come back as written (`12%` is 12).
fn scalar(nodes: &[Node], env: &Env) -> Result<f64> {
    match nodes {
        [Node::Number(number, _)] => Ok(*number),
        [Node::Function(function, args)] if function == "var" => {
            let groups = parse(lookup(args, env)?)?;
            scalar(groups.first().map(Vec::as_slice).unwrap_or_default(), env)
        }
        [Node::Function(function, args)] if function == "calc" => {
            let [terms] = args.as_slice() else { return Err("calc() takes one expression".into()) };
            terms.split(|node| *node == Node::Operator('*')).map(|term| scalar(term, env)).product()
        }
        _ => Err("not a number".into()),
    }
}

// ---- Colours ------------------------------------------------------------

/// Straight-alpha sRGB, each channel from 0 to 1 (before clamping to the gamut).
type Rgba = [f64; 4];

fn color(nodes: &[Node], env: &Env, scheme: Scheme) -> Result<Rgba> {
    let [node] = nodes else { return Err("not a colour".into()) };
    match node {
        Node::Hash(hex) => {
            let digits: Vec<f64> = hex.chars().filter_map(|c| c.to_digit(16)).map(f64::from).collect();
            match digits.as_slice() {
                [r, g, b] => Ok([r * 17.0 / 255.0, g * 17.0 / 255.0, b * 17.0 / 255.0, 1.0]),
                [r1, r0, g1, g0, b1, b0] => {
                    Ok([(r1 * 16.0 + r0) / 255.0, (g1 * 16.0 + g0) / 255.0, (b1 * 16.0 + b0) / 255.0, 1.0])
                }
                _ => Err(format!("unsupported colour `#{hex}`")),
            }
        }
        Node::Ident(name) if name == "transparent" => Ok([0.0; 4]),
        Node::Function(function, args) => match (function.as_str(), args.as_slice()) {
            ("var", _) => {
                let groups = parse(lookup(args, env)?)?;
                color(groups.first().map(Vec::as_slice).unwrap_or_default(), env, scheme)
            }
            ("light-dark", [light, dark]) => color(if scheme == Scheme::Light { light } else { dark }, env, scheme),
            ("rgb", [parts]) => {
                let (channels, alpha) = split_alpha(parts, env)?;
                let [r, g, b] = channels else { return Err("rgb() needs three channels".into()) };
                let channel = |node: &Node| scalar(std::slice::from_ref(node), env).map(|value| value / 255.0);
                Ok([channel(r)?, channel(g)?, channel(b)?, alpha])
            }
            ("oklch", [parts]) => oklch(parts, env, scheme),
            ("color-mix", [space, first, second]) => mix(space, first, second, env, scheme),
            _ => Err(format!("unsupported `{function}()`")),
        },
        _ => Err("not a colour".into()),
    }
}

/// Whether a colour is written as hex, `rgb()` or a keyword, once `var()` and
/// `light-dark()` are seen through.
fn legacy(node: &Node, env: &Env, scheme: Scheme) -> bool {
    let first = |groups: &[Vec<Node>]| matches!(groups, [group, ..] if matches!(group.as_slice(), [node] if legacy(node, env, scheme)));
    match node {
        Node::Hash(_) | Node::Ident(_) => true,
        Node::Function(function, args) => match (function.as_str(), args.as_slice()) {
            ("rgb", _) => true,
            ("var", _) => lookup(args, env).and_then(parse).is_ok_and(|groups| first(&groups)),
            ("light-dark", [light, dark]) => {
                first(std::slice::from_ref(if scheme == Scheme::Light { light } else { dark }))
            }
            _ => false,
        },
        _ => false,
    }
}

/// Splits `a b c / alpha` into its components and the alpha (1 when absent).
fn split_alpha<'a>(parts: &'a [Node], env: &Env) -> Result<(&'a [Node], f64)> {
    match parts.iter().position(|node| *node == Node::Operator('/')) {
        Some(at) => Ok((&parts[..at], scalar(&parts[at + 1..], env)?)),
        None => Ok((parts, 1.0)),
    }
}

/// `oklch(L C H)` or `oklch(from <colour> L C H)`, where a component of the
/// relative form may use `l`, `c`, `h`, `min()` and `max()`.
fn oklch(parts: &[Node], env: &Env, scheme: Scheme) -> Result<Rgba> {
    let (parts, alpha) = split_alpha(parts, env)?;
    let (origin, components) = match parts {
        [Node::Ident(from), origin, rest @ ..] if from == "from" => {
            let [r, g, b, a] = color(std::slice::from_ref(origin), env, scheme)?;
            (Some((to_oklch([r, g, b]), a)), rest)
        }
        _ => (None, parts),
    };
    let [l, c, h] = components else { return Err("oklch() needs three components".into()) };
    let from = origin.map(|(lch, _)| lch);
    let lch = [component(l, from, env)?, component(c, from, env)?, component(h, from, env)?];
    let [r, g, b] = from_oklch(lch);
    Ok([r, g, b, alpha * origin.map_or(1.0, |(_, a)| a)])
}

fn component(node: &Node, origin: Option<[f64; 3]>, env: &Env) -> Result<f64> {
    match node {
        Node::Ident(channel) => {
            let at = ["l", "c", "h"].iter().position(|name| name == channel).ok_or(format!("unknown `{channel}`"))?;
            origin.map(|lch| lch[at]).ok_or_else(|| format!("`{channel}` needs `from`"))
        }
        Node::Function(function, args) if function == "min" || function == "max" => {
            let values = args
                .iter()
                .map(|arg| match arg.as_slice() {
                    [node] => component(node, origin, env),
                    _ => Err(format!("unsupported {function}() argument")),
                })
                .collect::<Result<Vec<_>>>()?;
            let pick = if function == "min" { f64::min } else { f64::max };
            values.into_iter().reduce(pick).ok_or_else(|| format!("empty {function}()"))
        }
        node => scalar(std::slice::from_ref(node), env),
    }
}

/// `color-mix(in <space>, <colour> [p], <colour> [p])` for `srgb` and `oklch`.
fn mix(space: &[Node], first: &[Node], second: &[Node], env: &Env, scheme: Scheme) -> Result<Rgba> {
    let [Node::Ident(_), Node::Ident(space)] = space else { return Err("color-mix() needs `in <space>`".into()) };
    let side = |nodes: &[Node]| -> Result<(Rgba, bool, Option<f64>)> {
        let (colour, share) = nodes.split_first().ok_or("color-mix() needs two colours")?;
        let share = if share.is_empty() { None } else { Some(scalar(share, env)? / 100.0) };
        Ok((color(std::slice::from_ref(colour), env, scheme)?, legacy(colour, env, scheme), share))
    };
    let ((a, legacy_a, share_a), (b, legacy_b, share_b)) = (side(first)?, side(second)?);
    let (share_a, share_b) = match (share_a, share_b) {
        (Some(a), Some(b)) => (a, b),
        (Some(a), None) => (a, 1.0 - a),
        (None, Some(b)) => (1.0 - b, b),
        (None, None) => (0.5, 0.5),
    };
    let total = share_a + share_b;
    if total <= 0.0 {
        return Err("color-mix() shares add up to nothing".into());
    }
    // Shares that fall short of 100% make the result that much more transparent.
    let (t, fade) = (share_b / total, total.min(1.0));

    let polar = match space.as_str() {
        "oklch" => true,
        "srgb" => false,
        _ => return Err(format!("unsupported colour space `{space}`")),
    };
    let convert = |[r, g, b, _]: Rgba| if polar { to_oklch([r, g, b]) } else { [r, g, b] };
    let (mut x, mut y) = (convert(a), convert(b));
    if polar {
        // A grey has no hue of its own: it takes the other colour's. Chromium is
        // far more willing to call a hex or `rgb()` colour grey than one given in
        // OKLCH, which is what lets a near-grey neutral take the accent's hue.
        let grey = |lch: [f64; 3], legacy: bool| lch[1] < if legacy { 0.02 } else { 1e-6 };
        match (grey(x, legacy_a), grey(y, legacy_b)) {
            (true, false) => x[2] = y[2],
            (false, true) => y[2] = x[2],
            _ => {}
        }
        // The shorter way round.
        if y[2] - x[2] > 180.0 {
            x[2] += 360.0;
        } else if y[2] - x[2] < -180.0 {
            y[2] += 360.0;
        }
    }
    // Colour is mixed premultiplied, so a transparent side adds no colour. Hue is an angle and is not.
    let alpha = a[3] * (1.0 - t) + b[3] * t;
    let mut mixed = [0.0; 3];
    for (at, channel) in mixed.iter_mut().enumerate() {
        let (wa, wb) = if polar && at == 2 { (1.0, 1.0) } else { (a[3], b[3]) };
        let divisor = if polar && at == 2 || alpha == 0.0 { 1.0 } else { alpha };
        *channel = (x[at] * wa * (1.0 - t) + y[at] * wb * t) / divisor;
    }
    let [r, g, b] = if polar { from_oklch(mixed) } else { mixed };
    Ok([r, g, b, alpha * fade])
}

fn to_linear(channel: f64) -> f64 {
    let magnitude = channel.abs();
    channel.signum() * if magnitude <= 0.04045 { magnitude / 12.92 } else { ((magnitude + 0.055) / 1.055).powf(2.4) }
}

fn from_linear(channel: f64) -> f64 {
    let magnitude = channel.abs();
    channel.signum()
        * if magnitude <= 0.0031308 { magnitude * 12.92 } else { 1.055 * magnitude.powf(1.0 / 2.4) - 0.055 }
}

/// sRGB to OKLCH: lightness 0 to 1, chroma, hue in degrees.
fn to_oklch(rgb: [f64; 3]) -> [f64; 3] {
    let [r, g, b] = rgb.map(to_linear);
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    let lightness = 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s;
    let a = 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s;
    let b = 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s;
    [lightness, a.hypot(b), b.atan2(a).to_degrees().rem_euclid(360.0)]
}

fn from_oklch([lightness, chroma, hue]: [f64; 3]) -> [f64; 3] {
    let (a, b) = (chroma * (hue * PI / 180.0).cos(), chroma * (hue * PI / 180.0).sin());
    let l = (lightness + 0.3963377774 * a + 0.2158037573 * b).powi(3);
    let m = (lightness - 0.1055613458 * a - 0.0638541728 * b).powi(3);
    let s = (lightness - 0.0894841775 * a - 1.2914855480 * b).powi(3);
    [
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    ]
    .map(from_linear)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb8(value: &str, env: &Env, scheme: Scheme) -> [i32; 4] {
        let groups = parse(value).unwrap();
        color(&groups[0], env, scheme).unwrap().map(|channel| (channel.clamp(0.0, 1.0) * 255.0).round() as i32)
    }

    // Expected values are what Chromium computes for the same declaration.
    #[test]
    fn colours_evaluate_as_a_browser_does() {
        use Scheme::*;
        let env: Env = [("accent", "light-dark(#409eff, #69b4ff)"), ("tint", "1"), ("ok", "#146f43")]
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .into();
        let cases: &[(&str, Scheme, [i32; 4])] = &[
            ("#fff", Light, [255, 255, 255, 255]),
            ("var(--yi-accent)", Dark, [105, 180, 255, 255]),
            ("light-dark(rgb(20 26 34 / 0.06), rgb(0 0 0 / 0.3))", Light, [20, 26, 34, 15]),
            ("color-mix(in srgb, var(--yi-ok) 12%, transparent)", Light, [20, 111, 67, 31]),
            ("color-mix(in srgb, #000, #fff)", Light, [128, 128, 128, 255]),
            ("color-mix(in oklch, #146f43, light-dark(#000, #fff) 12%)", Light, [15, 93, 55, 255]),
            ("oklch(from var(--yi-accent) min(l, 0.52) min(c, 0.15) h)", Light, [14, 106, 188, 255]),
            (
                "color-mix(in oklch, oklch(from var(--yi-accent) 0.971 c h) calc(3% * var(--yi-tint)), #f6f6f6)",
                Light,
                [244, 246, 250, 255],
            ),
            // A near-grey hex takes the other colour's hue; the same colour in OKLCH keeps its own.
            ("color-mix(in oklch, oklch(0.252 0.13 250) 6%, #222224)", Dark, [30, 35, 40, 255]),
            ("color-mix(in oklch, oklch(0.252 0.13 250) 6%, oklch(from #222224 l c h))", Dark, [33, 34, 40, 255]),
        ];
        for (value, scheme, expected) in cases {
            let found = rgb8(value, &env, *scheme);
            let close = found.iter().zip(expected).all(|(a, b)| (a - b).abs() <= 1);
            assert!(close, "{value} ({scheme:?}): {found:?}, expected {expected:?}");
        }
    }

    #[test]
    fn each_kind_of_token_becomes_a_typed_constant() {
        let env: Env = [("font-system", "system-ui, \"Segoe UI\", sans-serif")]
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .into();
        let cases = [
            ("space-sm", "0.5rem", "f32", "8.0"),
            ("radius-sm", "4px", "f32", "4.0"),
            ("duration-pulse", "1.6s", "f32", "1600.0"),
            ("heading-weight-md", "600", "u16", "600"),
            ("leading", "1.5", "f32", "1.5"),
            ("ease", "cubic-bezier(0.2, 0, 0, 1)", "[f32; 4]", "[0.2, 0.0, 0.0, 1.0]"),
            ("font-mono-ligatures", "none", "&str", "\"none\""),
            ("font-body", "\"Inter\", var(--yi-font-system)", "&[&str]", "Segoe UI"),
            ("shadow", "0 1px 2px #000, 0 4px 14px #fff", "&[Shadow]", "blur: 14.0"),
            ("accent", "light-dark(#fff, #000)", "Color", "dark: Rgba::new(0, 0, 0, 255)"),
        ];
        for (name, value, ty, expr) in cases {
            let (found_ty, found) = rust(name, value, &env).unwrap();
            assert_eq!(found_ty, ty, "{name}");
            assert!(found.contains(expr), "{name}: {found}");
        }
    }

    #[test]
    fn a_value_it_cannot_evaluate_is_an_error() {
        for value in ["var(--yi-missing)", "conic-gradient(red, blue)", "12vw", "color-mix(in lab, #000, #fff)"] {
            assert!(rust("thing", value, &Env::new()).is_err(), "{value}");
        }
    }
}
