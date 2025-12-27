// src/parser.rs
use crate::languages::SupportedLanguage;
use crate::models::{CodeSymbol, SourceLocation};
use tree_sitter::{Parser, Query, QueryCursor};
use streaming_iterator::StreamingIterator; 

pub struct CodeParser;

impl CodeParser {
    pub fn parse(content: &str, lang: SupportedLanguage) -> Vec<CodeSymbol> {
        let mut parser = Parser::new();
        // Use a safer grammar loading approach
        if let Err(_) = parser.set_language(&lang.get_grammar()) {
            return vec![];
        }

        let tree = match parser.parse(content, None) {
            Some(t) => t,
            None => return vec![],
        };

        let query_str = lang.get_query();
        if query_str.is_empty() { return vec![]; }

        // SAFELY create the query. If it fails, print error but DO NOT CRASH.
        let query = match Query::new(&lang.get_grammar(), query_str) {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Query Error for {:?}: {:?}", lang, e);
                return vec![];
            }
        };

        let mut cursor = QueryCursor::new();
        let content_bytes = content.as_bytes();
        let mut matches = cursor.matches(&query, tree.root_node(), content_bytes);

        let mut symbols = Vec::new();
        
        while let Some(m) = matches.next() {
            for capture in m.captures {
                let node = capture.node;
                // Safely extract text
                let name = match node.utf8_text(content_bytes) {
                    Ok(n) => n.to_string(),
                    Err(_) => "invalid_utf8".to_string(),
                };
                
                let start = node.start_position();
                let end = node.end_position();

                symbols.push(CodeSymbol {
                    name,
                    kind: "definition".to_string(),
                    start: SourceLocation { row: start.row as u32, column: start.column as u32 },
                    end: SourceLocation { row: end.row as u32, column: end.column as u32 },
                    signature: "".to_string(),
                });
            }
        }
        
        symbols
    }
}