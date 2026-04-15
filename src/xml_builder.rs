pub struct XmlBuilder {
    lines: Vec<String>,
    depth: usize,
}

impl XmlBuilder {
    pub fn new() -> Self {
        Self {
            lines: vec![r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string()],
            depth: 0,
        }
    }

    pub fn open(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.lines
            .push(format!("{}<{}{}>", self.pad(), name, attr_str(attrs)));
        self.depth += 1;
    }

    pub fn close(&mut self, name: &str) {
        self.depth -= 1;
        self.lines.push(format!("{}</{}>", self.pad(), name));
    }

    pub fn leaf(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.lines
            .push(format!("{}<{}{}/>", self.pad(), name, attr_str(attrs)));
    }

    pub fn inline(&mut self, name: &str, text: &str) {
        self.lines
            .push(format!("{}<{}>{}</{}>", self.pad(), name, text, name));
    }

    pub fn to_xml(&self) -> String {
        let mut s = self.lines.join("\n");
        s.push('\n');
        s
    }

    fn pad(&self) -> String {
        "    ".repeat(self.depth)
    }
}

fn attr_str(attrs: &[(&str, &str)]) -> String {
    attrs
        .iter()
        .map(|(k, v)| format!(" {}=\"{}\"", k, escape(v)))
        .collect()
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
