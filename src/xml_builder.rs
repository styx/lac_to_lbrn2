pub struct XmlBuilder {
    lines: Vec<String>,
    depth: usize,
}

impl XmlBuilder {
    #[must_use]
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
            .push(format!("{}<{}>{}</{}>", self.pad(), name, escape(text), name));
    }

    #[must_use]
    pub fn to_xml(&self) -> String {
        let mut s = self.lines.join("\n");
        s.push('\n');
        s
    }

    fn pad(&self) -> String {
        "    ".repeat(self.depth)
    }
}

impl Default for XmlBuilder {
    fn default() -> Self {
        Self::new()
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- escape() ---

    #[test]
    fn escape_empty_string() {
        assert_eq!(escape(""), "");
    }

    #[test]
    fn escape_no_special_chars_unchanged() {
        assert_eq!(escape("hello world"), "hello world");
    }

    #[test]
    fn escape_ampersand() {
        assert_eq!(escape("a&b"), "a&amp;b");
    }

    #[test]
    fn escape_double_quote() {
        assert_eq!(escape(r#"a"b"#), "a&quot;b");
    }

    #[test]
    fn escape_less_than() {
        assert_eq!(escape("a<b"), "a&lt;b");
    }

    #[test]
    fn escape_greater_than() {
        assert_eq!(escape("a>b"), "a&gt;b");
    }

    #[test]
    fn escape_all_special_chars_combined() {
        assert_eq!(escape(r#"&<>""#), "&amp;&lt;&gt;&quot;");
    }

    #[test]
    fn escape_already_escaped_ampersand_is_double_escaped() {
        // The function always escapes & — it does not detect prior escaping.
        assert_eq!(escape("&amp;"), "&amp;amp;");
    }

    #[test]
    fn escape_injection_attempt_neutralised() {
        // Input: "><script>alert(1)</script>
        assert_eq!(
            escape(r#""><script>alert(1)</script>"#),
            "&quot;&gt;&lt;script&gt;alert(1)&lt;/script&gt;"
        );
    }

    // --- XmlBuilder ---

    #[test]
    fn leaf_attribute_values_are_escaped() {
        let mut xml = XmlBuilder::new();
        xml.leaf("Foo", &[("val", "a<b&c")]);
        assert!(xml.to_xml().contains("a&lt;b&amp;c"));
    }

    #[test]
    fn inline_text_content_is_escaped() {
        let mut xml = XmlBuilder::new();
        xml.open("Root", &[]);
        xml.inline("T", "x & y");
        xml.close("Root");
        assert!(xml.to_xml().contains("x &amp; y"));
    }

    #[test]
    fn open_close_produces_matching_tags() {
        let mut xml = XmlBuilder::new();
        xml.open("Outer", &[]);
        xml.close("Outer");
        let out = xml.to_xml();
        assert!(out.contains("<Outer>"));
        assert!(out.contains("</Outer>"));
    }

    #[test]
    fn nesting_increases_indentation() {
        let mut xml = XmlBuilder::new();
        xml.open("A", &[]);
        xml.open("B", &[]);
        xml.leaf("C", &[]);
        xml.close("B");
        xml.close("A");
        let out = xml.to_xml();
        // C is at depth 2 → 8 spaces
        assert!(out.contains("        <C"));
    }

    #[test]
    fn default_equals_new() {
        let a = XmlBuilder::new();
        let b = XmlBuilder::default();
        assert_eq!(a.to_xml(), b.to_xml());
    }

    #[test]
    fn to_xml_ends_with_newline() {
        let xml = XmlBuilder::new();
        assert!(xml.to_xml().ends_with('\n'));
    }
}
