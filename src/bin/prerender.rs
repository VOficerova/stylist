//! Печатает готовую HTML-страницу в stdout. `build.sh` сохраняет её как `dist/index.html`.

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let html = vasilisa_site::view::page().into_string();
    let mut out = io::stdout().lock();
    out.write_all(html.as_bytes())?;
    out.write_all(b"\n")
}
