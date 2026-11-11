//! Finds every interactive widget the views build and reads its padding and
//! text size out of the source, so a size the check reports cannot drift from
//! the code that draws it.
//!
//! This is deliberately a small, regular scanner and not a Rust parser. It
//! relies on one fact about this code base: widgets are built as a
//! constructor call followed by a method chain (`button(..).on_press(..)
//! .padding(..)`), and padding and text size are written as token paths
//! (`tokens.spacing.sm`, `tokens.typography.label.size`) or numeric literals.
//! Anything it cannot read is an **error**, never a silent skip: a site whose
//! padding or text size it cannot evaluate fails the build and names itself.

use std::path::{Path, PathBuf};

use snora::design::Tokens;

/// iced 0.14 defaults, cited where each is defined.
/// `iced_widget::button::DEFAULT_PADDING` (pick_list reuses it).
pub(super) const BUTTON_PAD: (f32, f32) = (5.0, 10.0); // (vertical, horizontal) per side
/// `iced_widget::text_input::DEFAULT_PADDING` and the `text_editor` default.
pub(super) const INPUT_PAD: f32 = 5.0;
/// `iced_core::Settings::default().default_text_size`.
pub(super) const DEFAULT_FONT: f32 = 16.0;
/// `iced_core::text::LineHeight::default()`, `Relative(1.3)`.
pub(super) const DEFAULT_LINE_HEIGHT: f32 = 1.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Kind {
    Button,
    TextInput,
    TextEditor,
    PickList,
    MouseArea,
}

/// One method call in a chain: `.name(args)`.
#[derive(Debug, Clone)]
pub(super) struct Call {
    pub name: String,
    pub args: String,
}

/// An interactive widget found in the source.
#[derive(Debug, Clone)]
pub(super) struct Site {
    /// `relative/path.rs::enclosing_fn#n` — the key the registry uses.
    pub id: String,
    pub kind: Kind,
    pub line: usize,
    /// Calls chained directly onto the constructor.
    pub chain: Vec<Call>,
    /// Every `.padding(..)` written inside the constructor's own arguments
    /// (a container nested in a button, for instance).
    pub nested_padding: Vec<String>,
    /// `(size expr, line-height expr)` for each text sized inside the arguments.
    pub texts: Vec<(String, Option<String>)>,
}

/// Size a site resolves to under one set of tokens. `height` is exact (token
/// arithmetic); the horizontal fields feed a width estimate made elsewhere.
#[derive(Debug, Clone, Copy)]
pub(super) struct Measure {
    pub height: f32,
    /// Total horizontal padding, both sides, every layer.
    pub pad_h: f32,
    /// Font size of the widget's text, for the width estimate.
    pub font: f32,
    /// Width is `Length::Fill` — decided by the layout, not by content.
    pub fill_width: bool,
}

// ── Source masking and matching ───────────────────────────────────────────

/// `src` with comment and string/char-literal *contents* blanked (same length,
/// newlines kept), so parentheses inside them never confuse matching.
fn mask(src: &[char]) -> Vec<char> {
    let n = src.len();
    let mut out = src.to_vec();
    let mut i = 0;
    while i < n {
        match src[i] {
            '/' if i + 1 < n && src[i + 1] == '/' => {
                while i < n && src[i] != '\n' {
                    out[i] = ' ';
                    i += 1;
                }
            }
            '/' if i + 1 < n && src[i + 1] == '*' => {
                let mut depth = 0;
                while i < n {
                    if i + 1 < n && src[i] == '/' && src[i + 1] == '*' {
                        depth += 1;
                        out[i] = ' ';
                        out[i + 1] = ' ';
                        i += 2;
                    } else if i + 1 < n && src[i] == '*' && src[i + 1] == '/' {
                        depth -= 1;
                        out[i] = ' ';
                        out[i + 1] = ' ';
                        i += 2;
                        if depth == 0 {
                            break;
                        }
                    } else {
                        if src[i] != '\n' {
                            out[i] = ' ';
                        }
                        i += 1;
                    }
                }
            }
            '"' => {
                i += 1;
                while i < n && src[i] != '"' {
                    if src[i] == '\\' && i + 1 < n {
                        out[i] = ' ';
                        i += 1;
                    }
                    if src[i] != '\n' {
                        out[i] = ' ';
                    }
                    i += 1;
                }
                i += 1;
            }
            '\'' => {
                // A char literal is `'x'` or `'\..'`; anything else is a lifetime.
                if i + 1 < n && src[i + 1] == '\\' {
                    if let Some(close) = (i + 2..(i + 8).min(n)).find(|&j| src[j] == '\'') {
                        for slot in out.iter_mut().take(close).skip(i + 1) {
                            *slot = ' ';
                        }
                        i = close;
                    }
                } else if i + 2 < n && src[i + 2] == '\'' {
                    out[i + 1] = ' ';
                    i += 2;
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    out
}

/// Index of the `)` matching the `(` at `open`, over masked text.
fn matching_paren(mask: &[char], open: usize) -> Option<usize> {
    let mut depth = 0;
    for (j, &c) in mask.iter().enumerate().skip(open) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(j);
                }
            }
            _ => {}
        }
    }
    None
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// The `.name(args)` calls that follow position `pos`, back to back.
fn chain_after(src: &[char], mask: &[char], mut pos: usize) -> Vec<Call> {
    let mut calls = Vec::new();
    loop {
        let mut j = pos;
        while j < mask.len() && mask[j].is_whitespace() {
            j += 1;
        }
        if j >= mask.len() || mask[j] != '.' {
            break;
        }
        j += 1;
        let name_start = j;
        while j < mask.len() && is_ident(mask[j]) {
            j += 1;
        }
        if j == name_start || j >= mask.len() || mask[j] != '(' {
            break; // field access or `.await`-like; the chain ends here
        }
        let Some(close) = matching_paren(mask, j) else {
            break;
        };
        calls.push(Call {
            name: src[name_start..j].iter().collect(),
            args: src[j + 1..close].iter().collect(),
        });
        pos = close + 1;
    }
    calls
}

/// Every `.name(..)` call anywhere in `start..end`, as `(name, args)`.
fn calls_within(src: &[char], mask: &[char], start: usize, end: usize) -> Vec<Call> {
    let mut found = Vec::new();
    let mut i = start;
    while i < end {
        if mask[i] == '.' {
            let name_start = i + 1;
            let mut j = name_start;
            while j < end && is_ident(mask[j]) {
                j += 1;
            }
            if j > name_start
                && j < end
                && mask[j] == '('
                && let Some(close) = matching_paren(mask, j)
            {
                found.push(Call {
                    name: src[name_start..j].iter().collect(),
                    args: src[j + 1..close].iter().collect(),
                });
            }
        }
        i += 1;
    }
    found
}

const CONSTRUCTORS: [(&str, Kind); 5] = [
    ("button", Kind::Button),
    ("text_input", Kind::TextInput),
    ("text_editor", Kind::TextEditor),
    ("pick_list", Kind::PickList),
    ("mouse_area", Kind::MouseArea),
];

/// Every interactive widget constructor in one file's text.
pub(super) fn scan_source(rel_path: &str, text: &str) -> Vec<Site> {
    let src: Vec<char> = text.chars().collect();
    let mk = mask(&src);
    let n = mk.len();

    // Byte-for-byte aligned list of `fn name` positions.
    let mut fns: Vec<(usize, String)> = Vec::new();
    let mut i = 0;
    while i + 3 < n {
        if mk[i] == 'f'
            && mk[i + 1] == 'n'
            && mk[i + 2].is_whitespace()
            && (i == 0 || !is_ident(mk[i - 1]))
        {
            let mut j = i + 3;
            while j < n && mk[j].is_whitespace() {
                j += 1;
            }
            let s = j;
            while j < n && is_ident(mk[j]) {
                j += 1;
            }
            fns.push((i, src[s..j].iter().collect()));
        }
        i += 1;
    }

    let mut sites = Vec::new();
    let mut per_fn: std::collections::HashMap<String, usize> = Default::default();
    let mut i = 0;
    while i < n {
        let at_word_start = i == 0 || !is_ident(mk[i - 1]);
        if at_word_start {
            for (name, kind) in CONSTRUCTORS {
                let len = name.len();
                if i + len < n
                    && mk[i..i + len].iter().copied().eq(name.chars())
                    && !is_ident(mk[i + len])
                {
                    let mut j = i + len;
                    while j < n && mk[j].is_whitespace() {
                        j += 1;
                    }
                    // Constructor call: `name(`, not a method call and not a fn item.
                    let prev: String = mk[i.saturating_sub(4)..i].iter().collect();
                    let is_method = i > 0 && mk[i - 1] == '.';
                    let is_item = prev.trim_end().ends_with("fn");
                    if j < n
                        && mk[j] == '('
                        && !is_method
                        && !is_item
                        && let Some(close) = matching_paren(&mk, j)
                    {
                        let func = fns
                            .iter()
                            .rev()
                            .find(|(pos, _)| *pos < i)
                            .map(|(_, f)| f.clone())
                            .unwrap_or_else(|| "<module>".into());
                        let k = per_fn.entry(func.clone()).or_insert(0);
                        *k += 1;
                        let args = calls_within(&src, &mk, j + 1, close);
                        let nested_padding = args
                            .iter()
                            .filter(|c| c.name == "padding")
                            .map(|c| c.args.clone())
                            .collect();
                        // Pair each `.size(..)` with a `.line_height(..)`
                        // chained straight after it.
                        let mut texts = Vec::new();
                        for pos in j + 1..close {
                            if mk[pos] == '.' {
                                let after = chain_after(&src, &mk, pos);
                                if let Some(first) = after.first()
                                    && first.name == "size"
                                {
                                    let lh = after
                                        .iter()
                                        .skip(1)
                                        .find(|c| c.name == "line_height")
                                        .map(|c| c.args.clone());
                                    texts.push((first.args.clone(), lh));
                                }
                            }
                        }
                        sites.push(Site {
                            id: format!("{rel_path}::{func}#{k}"),
                            kind,
                            line: src[..i].iter().filter(|&&c| c == '\n').count() + 1,
                            chain: chain_after(&src, &mk, close + 1),
                            nested_padding,
                            texts,
                        });
                    }
                    break;
                }
            }
        }
        i += 1;
    }
    sites
}

/// Every `.rs` file under `src_dir` that is not test code, scanned.
pub(super) fn scan_tree(src_dir: &Path) -> Vec<Site> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
            .map(|e| e.unwrap().path())
            .collect();
        entries.sort();
        for p in entries {
            let name = p.file_name().unwrap().to_string_lossy().to_string();
            if p.is_dir() {
                if name != "tests" {
                    walk(&p, out);
                }
            } else if name.ends_with(".rs") && name != "tests.rs" {
                out.push(p);
            }
        }
    }
    let mut files = Vec::new();
    walk(src_dir, &mut files);
    let mut all = Vec::new();
    for f in files {
        let rel = f
            .strip_prefix(src_dir)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let text = std::fs::read_to_string(&f).unwrap();
        all.extend(scan_source(&rel, &text));
    }
    all
}

// ── Evaluation against tokens ─────────────────────────────────────────────

/// Evaluate a length written as a spacing token path or a numeric literal.
fn eval_len(expr: &str, t: &Tokens) -> Result<f32, String> {
    let e: String = expr.split_whitespace().collect();
    if let Some((_, name)) = e.rsplit_once(".spacing.") {
        return match name {
            "xs" => Ok(t.spacing.xs),
            "sm" => Ok(t.spacing.sm),
            "md" => Ok(t.spacing.md),
            "lg" => Ok(t.spacing.lg),
            "xl" => Ok(t.spacing.xl),
            "xxl" => Ok(t.spacing.xxl),
            other => Err(format!("unknown spacing token `{other}`")),
        };
    }
    e.trim_end_matches("f32")
        .parse::<f32>()
        .map_err(|_| format!("cannot evaluate length `{e}`"))
}

/// `(font size, line height)` of a typography role, by name.
pub(super) fn role(t: &Tokens, name: &str) -> Result<(f32, f32), String> {
    let r = match name {
        "body" => t.typography.body,
        "body_small" => t.typography.body_small,
        "label" => t.typography.label,
        "title" => t.typography.title,
        "heading" => t.typography.heading,
        "display" => t.typography.display,
        other => return Err(format!("unknown typography role `{other}`")),
    };
    Ok((r.size, r.line_height))
}

/// A line height written as a typography role's `.line_height` or a literal.
fn eval_line_height(expr: &str, t: &Tokens) -> Result<f32, String> {
    let l: String = expr.split_whitespace().collect();
    match l.split(".typography.").nth(1) {
        Some(rest) => Ok(role(t, rest.split('.').next().unwrap_or(""))?.1),
        None => eval_len(&l, t),
    }
}

/// `(font size, text height)` for a `.size(..)` and its optional line height.
/// A size with no line height is rendered at iced's default, not at "none".
fn eval_text(size: &str, lh: Option<&str>, t: &Tokens) -> Result<(f32, f32), String> {
    let s: String = size.split_whitespace().collect();
    let font = match s.split(".typography.").nth(1) {
        Some(rest) => role(t, rest.split('.').next().unwrap_or(""))?.0,
        None => eval_len(&s, t)?,
    };
    let factor = match lh {
        Some(l) => eval_line_height(l, t)?,
        None => DEFAULT_LINE_HEIGHT,
    };
    Ok((font, font * factor))
}

/// `(vertical total, horizontal total)` of one `.padding(..)` argument.
fn eval_padding(arg: &str, t: &Tokens) -> Result<(f32, f32), String> {
    let e: String = arg.split_whitespace().collect();
    if let (Some(a), Some(b)) = (e.find('['), e.rfind(']')) {
        let parts: Result<Vec<f32>, String> = e[a + 1..b]
            .split(',')
            .filter(|p| !p.is_empty()) // rustfmt's trailing comma
            .map(|p| eval_len(p, t))
            .collect();
        return match parts?.as_slice() {
            [v, h] => Ok((2.0 * v, 2.0 * h)),
            [top, right, bottom, left] => Ok((top + bottom, right + left)),
            other => Err(format!("padding array of {} values", other.len())),
        };
    }
    let inner = e
        .strip_prefix("iced::Padding::new(")
        .or_else(|| e.strip_prefix("Padding::new("))
        .map(|r| r.trim_end_matches(')'))
        .unwrap_or(&e);
    let x = eval_len(inner, t)?;
    Ok((2.0 * x, 2.0 * x))
}

fn chain_arg<'a>(site: &'a Site, name: &str) -> Option<&'a str> {
    site.chain
        .iter()
        .rev()
        .find(|c| c.name == name)
        .map(|c| c.args.as_str())
}

/// Resolve `site` under `t`. `content_role` supplies the text role when the
/// constructor's own arguments carry no sized text (a row built elsewhere).
pub(super) fn measure(
    site: &Site,
    t: &Tokens,
    content_role: Option<&str>,
) -> Result<Measure, String> {
    let err = |m: String| format!("{} (line {}): {m}", site.id, site.line);
    let fill_width = match chain_arg(site, "width") {
        Some(w) => w.contains("Fill"),
        None => matches!(site.kind, Kind::TextInput | Kind::TextEditor),
    };

    let own_pad = match chain_arg(site, "padding") {
        Some(p) => eval_padding(p, t).map_err(&err)?,
        None => match site.kind {
            Kind::Button | Kind::PickList => (2.0 * BUTTON_PAD.0, 2.0 * BUTTON_PAD.1),
            _ => (2.0 * INPUT_PAD, 2.0 * INPUT_PAD),
        },
    };
    let mut nested = (0.0, 0.0);
    for p in &site.nested_padding {
        let (v, h) = eval_padding(p, t).map_err(&err)?;
        nested = (nested.0 + v, nested.1 + h);
    }
    let pad_v = own_pad.0 + nested.0;
    let pad_h = own_pad.1 + nested.1;

    // Text: for a button, the tallest sized text inside; for an input or a
    // pick list, the widget's own size and line height (or iced's defaults).
    let (font, text_h) = match site.kind {
        Kind::Button => {
            let mut best: Option<(f32, f32)> = None;
            for (s, l) in &site.texts {
                let m = eval_text(s, l.as_deref(), t).map_err(&err)?;
                if best.is_none_or(|b| m.1 > b.1) {
                    best = Some(m);
                }
            }
            match (best, content_role) {
                (Some(b), _) => b,
                (None, Some(r)) => {
                    let (f, l) = role(t, r).map_err(&err)?;
                    (f, f * l)
                }
                (None, None) => {
                    return Err(err(
                        "no sized text inside the constructor; give the registry a content role"
                            .into(),
                    ));
                }
            }
        }
        Kind::TextInput | Kind::TextEditor | Kind::PickList => {
            let size_call = if site.kind == Kind::PickList {
                "text_size"
            } else {
                "size"
            };
            let lh_call = if site.kind == Kind::PickList {
                "text_line_height"
            } else {
                "line_height"
            };
            let font = match chain_arg(site, size_call) {
                Some(s) => eval_text(s, None, t).map_err(&err)?.0,
                None => DEFAULT_FONT,
            };
            let factor = match chain_arg(site, lh_call) {
                Some(l) => eval_line_height(l, t).map_err(&err)?,
                None => DEFAULT_LINE_HEIGHT,
            };
            (font, font * factor)
        }
        Kind::MouseArea => {
            return Err(err(
                "a mouse_area is not measured; register an exemption".into()
            ));
        }
    };

    // A text_editor with a fixed height is exactly that tall.
    let fixed = chain_arg(site, "height")
        .and_then(|h| h.split("Fixed(").nth(1))
        .and_then(|h| h.trim_end_matches(')').trim().parse::<f32>().ok());
    let height = fixed.unwrap_or(text_h + pad_v);

    Ok(Measure {
        height,
        pad_h,
        font,
        fill_width,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design_tokens::tokens_for;
    use aaai::profile::prefs::Theme;

    fn tokens() -> Tokens {
        tokens_for(&Theme::Light)
    }

    fn one(src: &str) -> Site {
        let sites = scan_source("t.rs", src);
        assert_eq!(sites.len(), 1, "expected one site in {src}");
        sites.into_iter().next().unwrap()
    }

    #[test]
    fn button_height_is_text_plus_chained_padding() {
        let t = tokens();
        let s = one(
            "fn v(tokens: &T) { let b = button(text(\"x\").size(tokens.typography.label.size)\
             .line_height(tokens.typography.label.line_height)).padding(Padding::from([tokens.spacing.sm, tokens.spacing.lg])); }",
        );
        let m = measure(&s, &t, None).unwrap();
        assert!(
            (m.height - (14.0 * 1.2 + 16.0)).abs() < 1e-4,
            "{}",
            m.height
        );
        assert!((m.pad_h - 32.0).abs() < 1e-4);
    }

    #[test]
    fn button_without_padding_uses_iced_default_and_nested_container_adds() {
        let t = tokens();
        let s = one(
            "fn v() { button(container(text(\"x\").size(tokens.typography.label.size)\
             .line_height(tokens.typography.label.line_height))\
             .padding(Padding::from([tokens.spacing.sm, tokens.spacing.md]))).on_press(M); }",
        );
        let m = measure(&s, &t, None).unwrap();
        // 16.8 text + 10 (iced default 5+5) + 16 (container 8+8)
        assert!((m.height - 42.8).abs() < 1e-4, "{}", m.height);
    }

    #[test]
    fn text_input_defaults_and_overrides() {
        let t = tokens();
        let d = one("fn v() { text_input(\"p\", v).padding(tokens.spacing.sm); }");
        let m = measure(&d, &t, None).unwrap();
        assert!((m.height - (16.0 * 1.3 + 16.0)).abs() < 1e-4);
        assert!(m.fill_width);
        let s = one(
            "fn v() { text_input(\"p\", v).padding(Padding::from([tokens.spacing.xs, tokens.spacing.sm]))\
             .size(tokens.typography.body_small.size).line_height(tokens.typography.body_small.line_height); }",
        );
        let m = measure(&s, &t, None).unwrap();
        assert!((m.height - (14.0 * 1.35 + 8.0)).abs() < 1e-4);
    }

    #[test]
    fn pick_list_uses_text_size_and_button_default_padding() {
        let t = tokens();
        let s = one("fn v() { pick_list(a, b, f).text_size(tokens.typography.label.size); }");
        let m = measure(&s, &t, None).unwrap();
        assert!((m.height - (14.0 * 1.3 + 10.0)).abs() < 1e-4);
        assert!(!m.fill_width);
    }

    #[test]
    fn fixed_height_wins_and_fill_is_read() {
        let t = tokens();
        let s = one("fn v() { text_editor(&c).height(Length::Fixed(72.0)).padding(8); }");
        assert!((measure(&s, &t, None).unwrap().height - 72.0).abs() < 1e-4);
        let b = one(
            "fn v() { button(text(\"x\").size(tokens.typography.label.size).line_height(1.0)).width(Length::Fill).padding(0); }",
        );
        let m = measure(&b, &t, None).unwrap();
        assert!(m.fill_width);
        assert!((m.height - 14.0).abs() < 1e-4);
    }

    #[test]
    fn strings_comments_and_method_calls_are_not_constructors() {
        let src = "fn v() {\n // button(1)\n let s = \"text_input(x)\"; foo.button(2); /* pick_list( */ }\nfn button(x: u8) {}";
        assert!(scan_source("t.rs", src).is_empty());
    }

    #[test]
    fn unreadable_padding_is_an_error_not_a_skip() {
        let t = tokens();
        let s = one(
            "fn v() { button(text(\"x\").size(tokens.typography.label.size).line_height(1.0)).padding(some_runtime_value()); }",
        );
        assert!(measure(&s, &t, None).is_err());
    }

    #[test]
    fn site_ids_are_per_function_ordinals() {
        let sites = scan_source(
            "a/b.rs",
            "fn one() { button(x); button(y); }\nfn two() { button(z); }",
        );
        let ids: Vec<_> = sites.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, ["a/b.rs::one#1", "a/b.rs::one#2", "a/b.rs::two#1"]);
    }
}
