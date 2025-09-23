//! Demonstration of AST transformation capabilities
//!
//! This example shows how to use the ast-transform feature to manipulate Markdown AST.

#[cfg(feature = "ast-transform")]
fn main() {
    use markdown_ppp::ast_transform::*;
    use markdown_ppp::parser::{parse_markdown, MarkdownParserState};

    let markdown_input = r#"
# Hello World

This is a paragraph with *emphasis* and **strong** text.

Here's an image: ![alt text](/old/image.jpg)

And a link: [Click here](http://example.com)

Some code: `console.log("hello")`

More text with ENV_VAR and OTHER_VAR variables.
"#;

    println!("=== Original Markdown ===");
    println!("{markdown_input}");

    // Parse markdown into AST
    let doc = parse_markdown(MarkdownParserState::default(), markdown_input).unwrap();

    println!("\n=== AST Query Examples ===");

    // Find all text elements
    let texts = doc.find_all_inlines(|inline| matches!(inline, markdown_ppp::ast::Inline::Text(_)));
    println!("Found {} text elements", texts.len());

    // Find all images
    let images =
        doc.find_all_inlines(|inline| matches!(inline, markdown_ppp::ast::Inline::Image(_)));
    println!("Found {} images", images.len());

    // Count autolinks
    let autolink_count =
        doc.count_inlines(|inline| matches!(inline, markdown_ppp::ast::Inline::Autolink(_)));
    println!("Found {autolink_count} autolinks");

    println!("\n=== Transformation Examples ===");

    // Example 1: Transform all text to uppercase
    let _uppercase_doc = doc.clone().transform_text(|text| text.to_uppercase());
    println!("✓ Transformed all text to uppercase");

    // Example 2: Make all image URLs absolute
    let _absolute_images_doc = doc.clone().transform_image_urls(|url| {
        if url.starts_with('/') {
            format!("https://cdn.example.com{url}")
        } else {
            url
        }
    });
    println!("✓ Made all image URLs absolute");

    // Example 3: Convert HTTP links to HTTPS
    let _https_doc = doc
        .clone()
        .transform_link_urls(|url| url.replace("http://", "https://"));
    println!("✓ Converted HTTP links to HTTPS");

    // Example 4: Pipeline transformation
    let processed_doc = TransformPipeline::new()
        .transform_text(|s| s.trim().to_string())
        .transform_image_urls(|url| {
            if url.starts_with('/') {
                format!("https://cdn.example.com{url}")
            } else {
                url
            }
        })
        .transform_link_urls(|url| url.replace("http://", "https://"))
        .transform_code(|code| format!(">>> {code} <<<"))
        .normalize_whitespace()
        .apply(doc.clone());

    println!("✓ Applied transformation pipeline");

    // Example 5: Conditional transformation
    let is_production = true;
    let _conditional_doc = doc
        .clone()
        .transform_if_doc(
            |_| is_production,
            |d| d.transform_image_urls(|url| format!("https://production-cdn.com{url}")),
        )
        .transform_if_doc(
            |_| !is_production,
            |d| d.transform_image_urls(|url| format!("https://dev-cdn.com{url}")),
        );

    println!("✓ Applied conditional transformations");

    // Example 6: Custom visitor
    struct TextCollector {
        texts: Vec<String>,
    }

    impl Visitor for TextCollector {
        fn visit_inline(&mut self, inline: &markdown_ppp::ast::Inline) {
            if let markdown_ppp::ast::Inline::Text(text) = inline {
                self.texts.push(text.clone());
            }
            self.walk_inline(inline);
        }
    }

    let mut collector = TextCollector { texts: Vec::new() };
    doc.visit_with(&mut collector);
    println!(
        "✓ Collected {} text elements using custom visitor",
        collector.texts.len()
    );

    // Example 7: Custom transformer
    struct PrefixTransformer {
        prefix: String,
    }

    impl Transformer for PrefixTransformer {
        fn transform_inline(
            &mut self,
            inline: markdown_ppp::ast::Inline,
        ) -> markdown_ppp::ast::Inline {
            match inline {
                markdown_ppp::ast::Inline::Text(text) => {
                    markdown_ppp::ast::Inline::Text(format!("{}{}", self.prefix, text))
                }
                other => self.walk_transform_inline(other),
            }
        }
    }

    let _prefixed_doc = markdown_ppp::ast_transform::Transform::transform_with(
        doc.clone(),
        PrefixTransformer {
            prefix: "[PREFIX] ".to_string(),
        },
    );
    println!("✓ Applied custom transformer with prefix");

    println!("\n=== Filter Examples ===");

    // Filter and cleanup operations
    let _cleaned_doc = processed_doc
        .remove_empty_paragraphs()
        .remove_empty_text()
        .filter_blocks(|block| !matches!(block, markdown_ppp::ast::Block::ThematicBreak));

    println!("✓ Cleaned up document: removed empty elements and thematic breaks");

    println!("\n=== Functional Pipeline Example ===");

    // Functional style with pipe operator
    let _functional_doc = doc
        .clone()
        .pipe(|d| d.transform_text(|s| s.to_lowercase()))
        .pipe(|d| d.transform_image_urls(|url| format!("optimized/{url}")))
        .pipe(|d| d.normalize_whitespace())
        .pipe(|d| d.remove_empty_text());

    println!("✓ Applied functional pipeline transformations");

    println!("\n=== Alternative Pipeline Example ===");

    // Alternative pipeline using method chaining
    let _chained_doc = doc
        .clone()
        .transform_text(|s| s.replace("Hello", "Hi"))
        .transform_image_urls(|url| format!("/assets{url}"))
        .remove_empty_paragraphs();

    println!("✓ Applied transformations using method chaining");

    println!("\n=== Demo Complete ===");
    println!("The ast-transform feature provides powerful tools for:");
    println!("• Querying AST elements with predicates");
    println!("• Transforming content with type-safe operations");
    println!("• Building complex transformation pipelines");
    println!("• Custom visitors and transformers");
    println!("• Functional composition patterns");
    println!("• Method chaining and pipeline builders");
}

#[cfg(not(feature = "ast-transform"))]
fn main() {
    println!("This example requires the 'ast-transform' feature.");
    println!("Run with: cargo run --example ast_transform_demo --features ast-transform");
}
