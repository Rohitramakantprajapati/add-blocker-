#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CosmeticRule {
    pub selector: String,
    pub styles: String,
}

#[derive(Debug, Clone, Default)]
pub struct CosmeticRuleStore {
    rules: Vec<CosmeticRule>,
}

impl CosmeticRuleStore {
    pub fn push(&mut self, selector: impl Into<String>, styles: impl Into<String>) {
        self.rules.push(CosmeticRule {
            selector: selector.into(),
            styles: styles.into(),
        });
    }

    pub fn build_payload(&self) -> String {
        let mut payload = String::from("(() => {\n  const styles = document.createElement('style');\n  styles.textContent = `\n");
        for rule in &self.rules {
            payload.push_str(&rule.selector);
            payload.push_str(" { ");
            payload.push_str(&rule.styles);
            payload.push_str(" }\n");
        }
        payload.push_str("`;\n  document.documentElement.appendChild(styles);\n})();\n");
        payload
    }
}
