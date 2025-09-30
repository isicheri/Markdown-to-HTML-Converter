# Markdown to HTML Converter (Rust)

🚀 A fast, lightweight Markdown → HTML converter written in Rust.  
Unlike simple regex-based solutions, this project uses **tokenization + AST (Abstract Syntax Tree)** for production-level parsing.

---

## ✨ Features (MVP)
- [x] Parse headings (#, ##, ###)
- [x] Convert paragraphs
- [x] Bold (**text**) and italics (*text*)
- [x] Inline code (`code`)
- [ ] Lists (unordered/ordered)
- [ ] Code blocks (```rust ... ```)

---

## 🏗 Architecture
1. **Tokenizer** → Breaks raw Markdown into tokens.
2. **Parser** → Converts tokens into an AST.
3. **Renderer** → Walks the AST and generates clean HTML.
4. **Error Handling** → Graceful recovery from malformed Markdown.

---

## 🛠 Usage
```bash
# Clone the repo
git clone https://github.com/isicheri/Markdown-to-HTML-Converter.git
cd Markdown-to-HTML-Converter

# Run with Cargo
cargo run input.md output.html