// src/languages.rs
use tree_sitter::Language;

#[derive(Debug, Clone, Copy)]
pub enum SupportedLanguage {
    TypeScript,
    TSX,
    JavaScript,
    Python,
    Rust,
    Go,
    Unknown,
}

impl SupportedLanguage {
    pub fn from_path(path: &str) -> Self {
        if path.ends_with(".ts") { return Self::TypeScript; }
        if path.ends_with(".tsx") { return Self::TSX; }
        if path.ends_with(".js") || path.ends_with(".mjs") { return Self::JavaScript; }
        if path.ends_with(".py") { return Self::Python; }
        if path.ends_with(".rs") { return Self::Rust; }
        if path.ends_with(".go") { return Self::Go; }
        Self::Unknown
    }

    pub fn get_grammar(&self) -> Language {
        match self {
            Self::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Self::TSX => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Self::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            Self::Python => tree_sitter_python::LANGUAGE.into(),
            Self::Rust => tree_sitter_rust::LANGUAGE.into(),
            Self::Go => tree_sitter_go::LANGUAGE.into(),
            Self::Unknown => tree_sitter_javascript::LANGUAGE.into(),
        }
    }

    pub fn get_query(&self) -> &str {
        match self {
            Self::Python => r#"
                (function_definition name: (identifier) @name)
                (class_definition name: (identifier) @name)
            "#,
            Self::Rust => r#"
                (function_item name: (identifier) @name)
                (struct_item name: (type_identifier) @name)
                (trait_item name: (type_identifier) @name)
            "#,
            // TypeScript/TSX: Uses 'type_identifier' for classes/interfaces
            Self::TypeScript | Self::TSX => r#"
                (function_declaration name: (identifier) @name)
                (class_declaration name: (type_identifier) @name)
                (interface_declaration name: (type_identifier) @name)
                (method_definition name: (property_identifier) @name)
            "#,
            // JavaScript: Uses 'identifier' for classes (No types!)
            Self::JavaScript => r#"
                (function_declaration name: (identifier) @name)
                (class_declaration name: (identifier) @name)
                (method_definition name: (property_identifier) @name)
            "#,
            _ => "",
        }
    }
}

// src/languages.rs (Append this to the end)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        // Assert that extensions map to the correct Enum
        assert!(matches!(SupportedLanguage::from_path("index.ts"), SupportedLanguage::TypeScript));
        assert!(matches!(SupportedLanguage::from_path("App.tsx"), SupportedLanguage::TSX));
        assert!(matches!(SupportedLanguage::from_path("script.py"), SupportedLanguage::Python));
        assert!(matches!(SupportedLanguage::from_path("unknown.xyz"), SupportedLanguage::Unknown));
    }

    #[test]
    fn test_grammar_loading() {
        // Verify that we can actually load the grammar without crashing
        let lang = SupportedLanguage::Python;
        let grammar = lang.get_grammar();
        // If grammar was null, this would panic or segfault in C-land
        assert!(grammar.node_kind_count() > 0);
    }
}