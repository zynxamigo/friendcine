use serde_json::{json, Value};
use std::collections::HashMap;

fn attr(l: &str, k: &str) -> Option<String> {
    let p = format!("{k}=\"");
    let s = l.find(&p)? + p.len();
    let e = l[s..].find('"')?;
    Some(l[s..s + e].to_string())
}

// "Nome da Série S01E02" -> ("Nome da Série", "S01E02")
fn split_ep(t: &str) -> Option<(String, String)> {
    let b = t.as_bytes();
    for i in 0..b.len() {
        if b[i].to_ascii_lowercase() != b's' || (i > 0 && b[i - 1].is_ascii_alphanumeric()) {
            continue;
        }
        let mut j = i + 1;
        let d = j;
        while j < b.len() && b[j].is_ascii_digit() { j += 1; }
        if j == d || j >= b.len() || b[j].to_ascii_lowercase() != b'e' { continue; }
        let mut k = j + 1;
        let e = k;
        while k < b.len() && b[k].is_ascii_digit() { k += 1; }
        if k == e { continue; }
        let name = t[..i].trim_matches(|c: char| " -_.".contains(c)).to_string();
        if name.is_empty() { continue; }
        return Some((name, t[i..k].to_uppercase()));
    }
    None
}

pub fn parse(text: &str) -> Value {
    let mut cats: Vec<(String, Vec<Value>)> = vec![];
    let mut ci: HashMap<String, usize> = HashMap::new();
    let mut si: HashMap<(usize, String), usize> = HashMap::new();
    let mut pend: Option<(String, String, String)> = None;
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with("#EXTINF") {
            let title = l.rsplit_once(',').map(|x| x.1.trim()).unwrap_or("").to_string();
            let g = attr(l, "group-title").filter(|g| !g.is_empty()).unwrap_or_else(|| "Outros".into());
            pend = Some((title, g, attr(l, "tvg-logo").unwrap_or_default()));
        } else if !l.is_empty() && !l.starts_with('#') {
            let Some((title, g, logo)) = pend.take() else { continue };
            let c = *ci.entry(g.clone()).or_insert_with(|| {
                cats.push((g.clone(), vec![]));
                cats.len() - 1
            });
            match split_ep(&title) {
                Some((name, ep)) => {
                    let idx = *si.entry((c, name.clone())).or_insert_with(|| {
                        cats[c].1.push(json!({"title": name, "logo": logo, "episodes": []}));
                        cats[c].1.len() - 1
                    });
                    cats[c].1[idx]["episodes"].as_array_mut().unwrap().push(json!({"title": ep, "url": l}));
                }
                None => cats[c].1.push(json!({"title": title, "logo": logo, "url": l})),
            }
        }
    }
    json!({"categories": cats.into_iter().map(|(n, i)| json!({"name": n, "items": i})).collect::<Vec<_>>()})
}
