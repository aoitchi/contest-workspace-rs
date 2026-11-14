use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use proc_macro2::{Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (lib, input) = match args.as_slice() {
        [f, lib, input] if f == "--lib" => (PathBuf::from(lib), PathBuf::from(input)),
        _ => {
            eprintln!("usage: bundle-rs --lib <crate-root> <file.rs>");
            std::process::exit(2);
        }
    };
    match run(&lib, &input) {
        Ok(out) => print!("{out}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn run(lib: &Path, input: &Path) -> Result<String, String> {
    let name = pkg_name(lib)?;
    let src = lib.join("src");
    let sol_text = read(input)?;
    let sol = parse(&sol_text, input)?;
    let root = load_module(&src.join("lib.rs"), &src)?;

    let mut refs = Refs::new(&[&name]);
    refs.visit_file(&sol);
    let mut queue: VecDeque<Vec<String>> = refs
        .out
        .into_iter()
        .map(|(_, s)| root.resolve(&s))
        .collect();
    if queue.is_empty() {
        return Ok(sol_text);
    }

    let mut reached: BTreeSet<Vec<String>> = BTreeSet::new();
    loop {
        while let Some(mp) = queue.pop_front() {
            if !reached.insert(mp.clone()) {
                continue;
            }
            let m = root.get(&mp).expect("reached module exists");
            queue.extend(deps(&root, m, &mp));
        }
        let mut modules = Vec::new();
        root.walk(&mut Vec::new(), &mut modules);
        for (mp, m) in modules {
            if reached.contains(&mp) {
                continue;
            }
            if impl_targets(&root, m, &mp)
                .iter()
                .any(|d| reached.contains(d))
            {
                queue.push_back(mp);
            }
        }
        if queue.is_empty() {
            break;
        }
    }
    let mut rw = Rewrite { name: name.clone() };
    let items = emit(&root, &mut Vec::new(), &reached, &mut rw);
    let mut ts = TokenStream::new();
    for item in &items {
        item.to_tokens(&mut ts);
    }

    let mut out = sol_text;
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("\n#[allow(dead_code, unused_imports)]\n");
    out.push_str(&format!("mod {name} {{\n{ts}\n}}\n"));
    Ok(out)
}

struct Module {
    items: Vec<syn::Item>,
    children: BTreeMap<String, Module>,
}

impl Module {
    fn resolve(&self, segs: &[String]) -> Vec<String> {
        let mut m = self;
        let mut out = Vec::new();
        for s in segs {
            match m.children.get(s) {
                Some(c) => {
                    m = c;
                    out.push(s.clone());
                }
                None => break,
            }
        }
        out
    }

    fn walk<'a>(&'a self, mp: &mut Vec<String>, out: &mut Vec<(Vec<String>, &'a Module)>) {
        out.push((mp.clone(), self));
        for (name, child) in &self.children {
            mp.push(name.clone());
            child.walk(mp, out);
            mp.pop();
        }
    }

    fn get(&self, mp: &[String]) -> Option<&Module> {
        let mut m = self;
        for s in mp {
            m = m.children.get(s)?;
        }
        Some(m)
    }
}

fn deps(root: &Module, m: &Module, mp: &[String]) -> Vec<Vec<String>> {
    let mut refs = Refs::new(&["crate", "self", "super"]);
    for item in &m.items {
        refs.visit_item(item);
    }
    refs.out
        .into_iter()
        .map(|(head, segs)| resolve_from(root, &head, &segs, mp))
        .collect()
}

fn load_module(file: &Path, dir: &Path) -> Result<Module, String> {
    let text = strip_comments(&read(file)?);
    let parsed = parse(&text, file)?;
    let mut items = Vec::new();
    let mut children = BTreeMap::new();
    for item in parsed.items {
        if attrs(&item).iter().any(is_cfg_test) {
            continue;
        }
        match item {
            syn::Item::Mod(m) if m.content.is_none() => {
                let name = m.ident.to_string();
                let child = [
                    dir.join(format!("{name}.rs")),
                    dir.join(&name).join("mod.rs"),
                ]
                .into_iter()
                .find(|p| p.is_file())
                .ok_or_else(|| {
                    format!("cannot find module `{name}` declared in {}", file.display())
                })?;
                children.insert(name.clone(), load_module(&child, &dir.join(&name))?);
            }
            other => items.push(other),
        }
    }
    Ok(Module { items, children })
}

enum Origin {
    Local,
    Crate(Vec<String>),
    Foreign,
}

fn impl_targets(root: &Module, m: &Module, mp: &[String]) -> Vec<Vec<String>> {
    let mut imports: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut local: BTreeSet<String> = BTreeSet::new();
    for item in &m.items {
        match item {
            syn::Item::Use(u) => {
                let mut refs = Refs::new(&["crate", "self", "super"]);
                refs.visit_use_tree(&u.tree);
                for (head, segs) in refs.out {
                    if let Some(last) = segs.last() {
                        imports.insert(last.clone(), resolve_from(root, &head, &segs, mp));
                    }
                }
            }
            syn::Item::Struct(i) => {
                local.insert(i.ident.to_string());
            }
            syn::Item::Enum(i) => {
                local.insert(i.ident.to_string());
            }
            syn::Item::Union(i) => {
                local.insert(i.ident.to_string());
            }
            syn::Item::Type(i) => {
                local.insert(i.ident.to_string());
            }
            syn::Item::Trait(i) => {
                local.insert(i.ident.to_string());
            }
            _ => {}
        }
    }
    let origin = |path: &syn::Path| {
        let segs: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        let (head, rest) = segs.split_first().expect("path is nonempty");
        match head.as_str() {
            "crate" | "self" | "super" => Origin::Crate(resolve_from(root, head, rest, mp)),
            _ if local.contains(head) => Origin::Local,
            _ => match imports.get(head) {
                Some(full) => Origin::Crate(full.clone()),
                None => Origin::Foreign,
            },
        }
    };
    let mut out = Vec::new();
    for item in &m.items {
        let syn::Item::Impl(i) = item else {
            continue;
        };
        let mut self_ty = &*i.self_ty;
        while let syn::Type::Reference(r) = self_ty {
            self_ty = &r.elem;
        }
        let self_origin = match self_ty {
            syn::Type::Path(t) => origin(&t.path),
            _ => Origin::Foreign,
        };
        match self_origin {
            Origin::Crate(p) => out.push(p),
            Origin::Local => {}
            Origin::Foreign => {
                if let Some((_, path, _)) = &i.trait_
                    && let Origin::Crate(p) = origin(path)
                {
                    out.push(p);
                }
            }
        }
    }
    out
}

fn resolve_from(root: &Module, head: &str, segs: &[String], mp: &[String]) -> Vec<String> {
    let base = match head {
        "crate" => vec![],
        "self" => mp.to_vec(),
        _ => mp[..mp.len().saturating_sub(1)].to_vec(),
    };
    root.resolve(&[base, segs.to_vec()].concat())
}

fn attrs(item: &syn::Item) -> &[syn::Attribute] {
    use syn::Item::*;
    match item {
        Const(i) => &i.attrs,
        Enum(i) => &i.attrs,
        ExternCrate(i) => &i.attrs,
        Fn(i) => &i.attrs,
        ForeignMod(i) => &i.attrs,
        Impl(i) => &i.attrs,
        Macro(i) => &i.attrs,
        Mod(i) => &i.attrs,
        Static(i) => &i.attrs,
        Struct(i) => &i.attrs,
        Trait(i) => &i.attrs,
        TraitAlias(i) => &i.attrs,
        Type(i) => &i.attrs,
        Union(i) => &i.attrs,
        Use(i) => &i.attrs,
        _ => &[],
    }
}

fn is_cfg_test(a: &syn::Attribute) -> bool {
    a.path().is_ident("cfg") && a.parse_args::<Ident>().is_ok_and(|i| i == "test")
}

struct Refs {
    heads: Vec<String>,
    out: Vec<(String, Vec<String>)>,
}

impl Refs {
    fn new(heads: &[&str]) -> Self {
        Self {
            heads: heads.iter().map(|s| s.to_string()).collect(),
            out: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for Refs {
    fn visit_path(&mut self, p: &'ast syn::Path) {
        if let Some(first) = p.segments.first()
            && let Some(h) = self.heads.iter().find(|h| first.ident == h.as_str())
        {
            let rest = p
                .segments
                .iter()
                .skip(1)
                .map(|s| s.ident.to_string())
                .collect();
            self.out.push((h.clone(), rest));
        }
        syn::visit::visit_path(self, p);
    }

    fn visit_use_tree(&mut self, t: &'ast syn::UseTree) {
        match t {
            syn::UseTree::Group(g) => {
                for t in &g.items {
                    self.visit_use_tree(t);
                }
            }
            syn::UseTree::Path(p) => {
                if let Some(h) = self.heads.iter().find(|h| p.ident == h.as_str()) {
                    for rest in flatten(&p.tree) {
                        self.out.push((h.clone(), rest));
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        self.visit_path(&m.path);
        let heads: Vec<&str> = self.heads.iter().map(String::as_str).collect();
        scan_tokens(m.tokens.clone(), &heads, &mut self.out);
    }
}

fn flatten(t: &syn::UseTree) -> Vec<Vec<String>> {
    match t {
        syn::UseTree::Path(p) => flatten(&p.tree)
            .into_iter()
            .map(|mut v| {
                v.insert(0, p.ident.to_string());
                v
            })
            .collect(),
        syn::UseTree::Name(n) if n.ident == "self" => vec![vec![]],
        syn::UseTree::Name(n) => vec![vec![n.ident.to_string()]],
        syn::UseTree::Rename(r) => vec![vec![r.ident.to_string()]],
        syn::UseTree::Glob(_) => vec![vec![]],
        syn::UseTree::Group(g) => g.items.iter().flat_map(flatten).collect(),
    }
}

fn scan_tokens(ts: TokenStream, heads: &[&str], out: &mut Vec<(String, Vec<String>)>) {
    let tts: Vec<TokenTree> = ts.into_iter().collect();
    let mut i = 0;
    while i < tts.len() {
        match &tts[i] {
            TokenTree::Group(g) => scan_tokens(g.stream(), heads, out),
            TokenTree::Ident(id) if heads.iter().any(|h| id == h) && is_colon2(&tts, i + 1) => {
                let mut rest = Vec::new();
                let mut j = i + 1;
                while is_colon2(&tts, j) {
                    match tts.get(j + 2) {
                        Some(TokenTree::Ident(s)) => {
                            rest.push(s.to_string());
                            j += 3;
                        }
                        _ => break,
                    }
                }
                out.push((id.to_string(), rest));
                i = j;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
}

fn is_colon2(t: &[TokenTree], i: usize) -> bool {
    matches!(
        (t.get(i), t.get(i + 1)),
        (Some(TokenTree::Punct(a)), Some(TokenTree::Punct(b)))
            if a.as_char() == ':' && b.as_char() == ':'
    )
}

struct Rewrite {
    name: String,
}

impl VisitMut for Rewrite {
    fn visit_path_mut(&mut self, p: &mut syn::Path) {
        if p.segments.first().is_some_and(|s| s.ident == "crate") {
            p.segments.insert(
                1,
                syn::PathSegment::from(Ident::new(&self.name, Span::call_site())),
            );
        }
        syn::visit_mut::visit_path_mut(self, p);
    }

    fn visit_vis_restricted_mut(&mut self, _: &mut syn::VisRestricted) {}

    fn visit_macro_mut(&mut self, m: &mut syn::Macro) {
        self.visit_path_mut(&mut m.path);
        m.tokens = rewrite_tokens(std::mem::take(&mut m.tokens), &self.name);
    }

    fn visit_use_tree_mut(&mut self, t: &mut syn::UseTree) {
        match t {
            syn::UseTree::Group(g) => {
                for t in g.items.iter_mut() {
                    self.visit_use_tree_mut(t);
                }
            }
            syn::UseTree::Path(p) if p.ident == "crate" => {
                let placeholder = syn::UseTree::Glob(syn::UseGlob {
                    star_token: Default::default(),
                });
                let inner = std::mem::replace(&mut p.tree, Box::new(placeholder));
                *p.tree = syn::UseTree::Path(syn::UsePath {
                    ident: Ident::new(&self.name, Span::call_site()),
                    colon2_token: Default::default(),
                    tree: inner,
                });
            }
            _ => {}
        }
    }
}

fn rewrite_tokens(ts: TokenStream, name: &str) -> TokenStream {
    let tts: Vec<TokenTree> = ts.into_iter().collect();
    let mut out = Vec::with_capacity(tts.len());
    let mut i = 0;
    while i < tts.len() {
        match &tts[i] {
            TokenTree::Group(g) => {
                let mut ng = Group::new(g.delimiter(), rewrite_tokens(g.stream(), name));
                ng.set_span(g.span());
                out.push(TokenTree::Group(ng));
            }
            TokenTree::Ident(id) if id == "crate" && is_colon2(&tts, i + 1) => {
                out.extend(tts[i..i + 3].iter().cloned());
                out.push(TokenTree::Ident(Ident::new(name, id.span())));
                out.push(TokenTree::Punct(Punct::new(':', Spacing::Joint)));
                out.push(TokenTree::Punct(Punct::new(':', Spacing::Alone)));
                i += 3;
                continue;
            }
            t => out.push(t.clone()),
        }
        i += 1;
    }
    out.into_iter().collect()
}

fn emit(
    m: &Module,
    mp: &mut Vec<String>,
    reached: &BTreeSet<Vec<String>>,
    rw: &mut Rewrite,
) -> Vec<syn::Item> {
    let mut items = Vec::new();
    if reached.contains(mp) {
        for item in &m.items {
            let mut item = item.clone();
            rw.visit_item_mut(&mut item);
            items.push(item);
        }
    }
    for (name, child) in &m.children {
        mp.push(name.clone());
        if reached.iter().any(|p| p.starts_with(mp)) {
            let content = emit(child, mp, reached, rw);
            items.push(syn::Item::Mod(syn::ItemMod {
                attrs: Vec::new(),
                vis: syn::Visibility::Public(Default::default()),
                unsafety: None,
                mod_token: Default::default(),
                ident: Ident::new(name, Span::call_site()),
                content: Some((Default::default(), content)),
                semi: None,
            }));
        }
        mp.pop();
    }
    items
}

fn parse(text: &str, path: &Path) -> Result<syn::File, String> {
    syn::parse_file(text).map_err(|e| format!("{}: {e}", path.display()))
}

fn pkg_name(lib: &Path) -> Result<String, String> {
    let text = read(&lib.join("Cargo.toml"))?;
    let mut in_pkg = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            in_pkg = t == "[package]";
        } else if in_pkg
            && let Some(v) = t
                .strip_prefix("name")
                .map(str::trim_start)
                .and_then(|r| r.strip_prefix('='))
        {
            return Ok(v.trim().trim_matches('"').replace('-', "_"));
        }
    }
    Err("no [package] name in Cargo.toml".to_string())
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

fn strip_comments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for l in text.lines().filter(|l| !l.trim_start().starts_with("//")) {
        out.push_str(l);
        out.push('\n');
    }
    out
}
