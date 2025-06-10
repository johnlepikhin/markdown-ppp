use nom::IResult;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Function type for mapping elements.
type ElementMapFn<ELT> = Rc<RefCell<Box<dyn FnMut(ELT) -> ELT>>>;

/// Function type for mapping elements.
type ElementFlatMapFn<ELT> = Rc<RefCell<Box<dyn FnMut(ELT) -> Vec<ELT>>>>;

/// Function type for custom block parsers.
type CustomBlockParserFn =
    Rc<RefCell<Box<dyn for<'a> FnMut(&'a str) -> IResult<&'a str, Vec<crate::ast::Block>>>>>;

/// Function type for custom inline parsers.
type CustomInlineParserFn =
    Rc<RefCell<Box<dyn for<'a> FnMut(&'a str) -> IResult<&'a str, Vec<crate::ast::Inline>>>>>;

/// Behavior of the parser when encountering certain elements.
#[derive(Clone)]
pub enum ElementBehavior<ELT> {
    /// The parser will parse the element normally.
    Parse,

    /// The parser will ignore the element and not parse it. In this case, alternative
    /// parsers will be tried.
    Ignore,

    /// Parse element but do not include it in the output.
    Skip,

    /// Parse the element and apply a custom function to it.
    Map(ElementMapFn<ELT>),

    /// Parse the element and apply a custom function to it which returns an array of elements.
    FlatMap(ElementFlatMapFn<ELT>),
}

/// A configuration for the Markdown parser.
#[derive(Clone)]
pub struct MarkdownParserConfig {
    /// If true, the parser will allow headings without a space after the hash marks.
    pub(crate) allow_no_space_in_headings: bool,

    /// A map of HTML entities to their corresponding `Entity` structs.
    pub(crate) html_entities_map: HashMap<String, &'static entities::Entity>,

    /// The behavior of the parser when encountering blockquotes.
    pub(crate) block_blockquote_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering headings in style 1 (e.g., `# Heading`).
    pub(crate) block_heading_v1_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering headings in style 2 (e.g., `Heading\n===`).
    pub(crate) block_heading_v2_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering thematic breaks (e.g., `---`).
    pub(crate) block_thematic_break_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering lists.
    pub(crate) block_list_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering code blocks.
    pub(crate) block_code_block_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering HTML blocks.
    pub(crate) block_html_block_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering footnote definitions.
    pub(crate) block_footnote_definition_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering link definitions.
    pub(crate) block_link_definition_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering tables.
    pub(crate) block_table_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering block paragraphs.
    pub(crate) block_paragraph_behavior: ElementBehavior<crate::ast::Block>,

    /// The behavior of the parser when encountering inline autolinks.
    pub(crate) inline_autolink_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline links.
    pub(crate) inline_link_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline footnote references.
    pub(crate) inline_footnote_reference_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline reference links.
    pub(crate) inline_reference_link_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline hard newlines.
    pub(crate) inline_hard_newline_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline images.
    pub(crate) inline_image_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline code spans.
    pub(crate) inline_code_span_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline emphasis.
    pub(crate) inline_emphasis_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline strikethrough.
    pub(crate) inline_strikethrough_behavior: ElementBehavior<crate::ast::Inline>,

    /// The behavior of the parser when encountering inline text.
    pub(crate) inline_text_behavior: ElementBehavior<crate::ast::Inline>,

    /// A custom parser for blocks. This is a function that takes a string and returns a `Block`.
    pub(crate) custom_block_parser: Option<CustomBlockParserFn>,

    /// A custom parser for inlines. This is a function that takes a string and returns a `Inline`.
    pub(crate) custom_inline_parser: Option<CustomInlineParserFn>,
}

impl Default for MarkdownParserConfig {
    fn default() -> Self {
        Self {
            allow_no_space_in_headings: false,
            html_entities_map: Self::make_html_entities_map(),
            block_blockquote_behavior: ElementBehavior::Parse,
            block_heading_v1_behavior: ElementBehavior::Parse,
            block_heading_v2_behavior: ElementBehavior::Parse,
            block_thematic_break_behavior: ElementBehavior::Parse,
            block_list_behavior: ElementBehavior::Parse,
            block_code_block_behavior: ElementBehavior::Parse,
            block_html_block_behavior: ElementBehavior::Parse,
            block_footnote_definition_behavior: ElementBehavior::Parse,
            block_link_definition_behavior: ElementBehavior::Parse,
            block_table_behavior: ElementBehavior::Parse,
            block_paragraph_behavior: ElementBehavior::Parse,
            inline_autolink_behavior: ElementBehavior::Parse,
            inline_link_behavior: ElementBehavior::Parse,
            inline_footnote_reference_behavior: ElementBehavior::Parse,
            inline_reference_link_behavior: ElementBehavior::Parse,
            inline_hard_newline_behavior: ElementBehavior::Parse,
            inline_image_behavior: ElementBehavior::Parse,
            inline_code_span_behavior: ElementBehavior::Parse,
            inline_emphasis_behavior: ElementBehavior::Parse,
            inline_strikethrough_behavior: ElementBehavior::Parse,
            inline_text_behavior: ElementBehavior::Parse,
            custom_block_parser: None,
            custom_inline_parser: None,
        }
    }
}

impl MarkdownParserConfig {
    fn make_html_entities_map() -> HashMap<String, &'static entities::Entity> {
        let mut map = HashMap::new();
        for entity in entities::ENTITIES.iter() {
            map.insert(entity.entity.to_string(), entity);
        }
        map
    }

    /// Enable the parser to allow headings without a space after the hash marks.
    pub fn with_allow_no_space_in_headings(self) -> Self {
        Self {
            allow_no_space_in_headings: true,
            ..self
        }
    }

    /// Set a custom map of HTML entities.
    pub fn with_html_entities_map(
        self,
        html_entities_map: HashMap<String, &'static entities::Entity>,
    ) -> Self {
        Self {
            html_entities_map,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering blockquotes.
    pub fn with_block_blockquote_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_blockquote_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering headings in style 1 (e.g., `# Heading`).
    pub fn with_block_heading_v1_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_heading_v1_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering headings in style 2 (e.g., `Heading\n===`).
    pub fn with_block_heading_v2_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_heading_v2_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering thematic breaks (e.g., `---`).
    pub fn with_block_thematic_break_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_thematic_break_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering lists.
    pub fn with_block_list_behavior(self, behavior: ElementBehavior<crate::ast::Block>) -> Self {
        Self {
            block_list_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering code blocks.
    pub fn with_block_code_block_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_code_block_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering HTML blocks.
    pub fn with_block_html_block_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_html_block_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering footnote definitions.
    pub fn with_block_footnote_definition_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_footnote_definition_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering link definitions.
    pub fn with_block_link_definition_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_link_definition_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering tables.
    pub fn with_block_table_behavior(self, behavior: ElementBehavior<crate::ast::Block>) -> Self {
        Self {
            block_table_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering block paragraphs.
    pub fn with_block_paragraph_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Block>,
    ) -> Self {
        Self {
            block_paragraph_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline autolinks.
    pub fn with_inline_autolink_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Inline>,
    ) -> Self {
        Self {
            inline_autolink_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline links.
    pub fn with_inline_link_behavior(self, behavior: ElementBehavior<crate::ast::Inline>) -> Self {
        Self {
            inline_link_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline footnote references.
    pub fn with_inline_footnote_reference_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Inline>,
    ) -> Self {
        Self {
            inline_footnote_reference_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline reference links.
    pub fn with_inline_reference_link_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Inline>,
    ) -> Self {
        Self {
            inline_reference_link_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline hard newlines.
    pub fn with_inline_hard_newline_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Inline>,
    ) -> Self {
        Self {
            inline_hard_newline_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline images.
    pub fn with_inline_image_behavior(self, behavior: ElementBehavior<crate::ast::Inline>) -> Self {
        Self {
            inline_image_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline code spans.
    pub fn with_inline_code_span_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Inline>,
    ) -> Self {
        Self {
            inline_code_span_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline emphasis.
    pub fn with_inline_emphasis_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Inline>,
    ) -> Self {
        Self {
            inline_emphasis_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline strikethrough.
    pub fn with_inline_strikethrough_behavior(
        self,
        behavior: ElementBehavior<crate::ast::Inline>,
    ) -> Self {
        Self {
            inline_strikethrough_behavior: behavior,
            ..self
        }
    }

    /// Set the behavior of the parser when encountering inline text.
    pub fn with_inline_text_behavior(self, behavior: ElementBehavior<crate::ast::Inline>) -> Self {
        Self {
            inline_text_behavior: behavior,
            ..self
        }
    }

    /// Set a custom parser for blocks.
    pub fn with_custom_block_parser(self, parser: CustomBlockParserFn) -> Self {
        Self {
            custom_block_parser: Some(parser),
            ..self
        }
    }

    /// Set a custom parser for inlines.
    pub fn with_custom_inline_parser(self, parser: CustomInlineParserFn) -> Self {
        Self {
            custom_inline_parser: Some(parser),
            ..self
        }
    }
}
