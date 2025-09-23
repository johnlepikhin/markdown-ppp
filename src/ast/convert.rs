//! Conversion utilities for AST nodes with user data
//!
//! This module provides traits and functions for converting between different
//! AST representations with and without user data.

use super::generic;
use super::*;

// ——————————————————————————————————————————————————————————————————————————
// Conversion traits
// ——————————————————————————————————————————————————————————————————————————

/// Add user data to an AST node
pub trait WithData<T>: Sized {
    /// The type with user data attached
    type WithDataType;

    /// Add user data to this AST node
    fn with_data(self, data: T) -> Self::WithDataType;

    /// Add default user data to this AST node
    fn with_default_data(self) -> Self::WithDataType
    where
        T: Default,
    {
        self.with_data(T::default())
    }
}

/// Remove user data from an AST node
pub trait StripData<T>: Sized {
    /// The type without user data
    type StrippedType;

    /// Remove user data from this AST node
    fn strip_data(self) -> Self::StrippedType;
}

/// Transform user data type in an AST node
/// NOTE: Replaced by visitor-based approach in `map_data_visitor` module
pub trait MapData<T, U>: Sized {
    /// The type with the new user data type
    type MappedType;

    /// Transform user data using the provided function
    fn map_data<F>(self, f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U;
}

// ——————————————————————————————————————————————————————————————————————————
// Conversion functions for regular AST -> generic AST
// ——————————————————————————————————————————————————————————————————————————

impl<T: Default> WithData<T> for Document {
    type WithDataType = generic::Document<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::Document {
            blocks: self
                .blocks
                .into_iter()
                .map(|b| b.with_data(T::default()))
                .collect(),
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for Block {
    type WithDataType = generic::Block<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        match self {
            Block::Paragraph(content) => generic::Block::Paragraph {
                content: content
                    .into_iter()
                    .map(|i| i.with_data(T::default()))
                    .collect(),
                user_data: data,
            },
            Block::Heading(heading) => generic::Block::Heading(heading.with_data(data)),
            Block::ThematicBreak => generic::Block::ThematicBreak { user_data: data },
            Block::BlockQuote(blocks) => generic::Block::BlockQuote {
                blocks: blocks
                    .into_iter()
                    .map(|b| b.with_data(T::default()))
                    .collect(),
                user_data: data,
            },
            Block::List(list) => generic::Block::List(list.with_data(data)),
            Block::CodeBlock(code_block) => generic::Block::CodeBlock(code_block.with_data(data)),
            Block::HtmlBlock(content) => generic::Block::HtmlBlock {
                content,
                user_data: data,
            },
            Block::Definition(def) => generic::Block::Definition(def.with_data(data)),
            Block::Table(table) => generic::Block::Table(table.with_data(data)),
            Block::FootnoteDefinition(footnote) => {
                generic::Block::FootnoteDefinition(footnote.with_data(data))
            }
            Block::GitHubAlert(alert) => generic::Block::GitHubAlert(alert.with_data(data)),
            Block::Empty => generic::Block::Empty { user_data: data },
        }
    }
}

impl<T: Default> WithData<T> for Inline {
    type WithDataType = generic::Inline<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        match self {
            Inline::Text(content) => generic::Inline::Text {
                content,
                user_data: data,
            },
            Inline::LineBreak => generic::Inline::LineBreak { user_data: data },
            Inline::Code(content) => generic::Inline::Code {
                content,
                user_data: data,
            },
            Inline::Html(content) => generic::Inline::Html {
                content,
                user_data: data,
            },
            Inline::Link(link) => generic::Inline::Link(link.with_data(data)),
            Inline::LinkReference(link_ref) => {
                generic::Inline::LinkReference(link_ref.with_data(data))
            }
            Inline::Image(image) => generic::Inline::Image(image.with_data(data)),
            Inline::Emphasis(content) => generic::Inline::Emphasis {
                content: content
                    .into_iter()
                    .map(|i| i.with_data(T::default()))
                    .collect(),
                user_data: data,
            },
            Inline::Strong(content) => generic::Inline::Strong {
                content: content
                    .into_iter()
                    .map(|i| i.with_data(T::default()))
                    .collect(),
                user_data: data,
            },
            Inline::Strikethrough(content) => generic::Inline::Strikethrough {
                content: content
                    .into_iter()
                    .map(|i| i.with_data(T::default()))
                    .collect(),
                user_data: data,
            },
            Inline::Autolink(url) => generic::Inline::Autolink {
                url,
                user_data: data,
            },
            Inline::FootnoteReference(label) => generic::Inline::FootnoteReference {
                label,
                user_data: data,
            },
            Inline::Empty => generic::Inline::Empty { user_data: data },
        }
    }
}

impl<T: Default> WithData<T> for Heading {
    type WithDataType = generic::Heading<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::Heading {
            kind: self.kind,
            content: self
                .content
                .into_iter()
                .map(|i| i.with_data(T::default()))
                .collect(),
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for List {
    type WithDataType = generic::List<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::List {
            kind: generic::ListKind::from(self.kind),
            items: self
                .items
                .into_iter()
                .map(|i| i.with_data(T::default()))
                .collect(),
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for ListItem {
    type WithDataType = generic::ListItem<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::ListItem {
            task: self.task,
            blocks: self
                .blocks
                .into_iter()
                .map(|b| b.with_data(T::default()))
                .collect(),
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for CodeBlock {
    type WithDataType = generic::CodeBlock<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::CodeBlock {
            kind: self.kind,
            literal: self.literal,
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for LinkDefinition {
    type WithDataType = generic::LinkDefinition<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::LinkDefinition {
            label: self
                .label
                .into_iter()
                .map(|i| i.with_data(T::default()))
                .collect(),
            destination: self.destination,
            title: self.title,
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for Table {
    type WithDataType = generic::Table<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::Table {
            rows: self
                .rows
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|cell| {
                            cell.into_iter()
                                .map(|i| i.with_data(T::default()))
                                .collect()
                        })
                        .collect()
                })
                .collect(),
            alignments: self.alignments,
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for FootnoteDefinition {
    type WithDataType = generic::FootnoteDefinition<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::FootnoteDefinition {
            label: self.label,
            blocks: self
                .blocks
                .into_iter()
                .map(|b| b.with_data(T::default()))
                .collect(),
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for GitHubAlert {
    type WithDataType = generic::GitHubAlertNode<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::GitHubAlertNode {
            alert_type: self.alert_type,
            blocks: self
                .blocks
                .into_iter()
                .map(|b| b.with_data(T::default()))
                .collect(),
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for Link {
    type WithDataType = generic::Link<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::Link {
            destination: self.destination,
            title: self.title,
            children: self
                .children
                .into_iter()
                .map(|i| i.with_data(T::default()))
                .collect(),
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for Image {
    type WithDataType = generic::Image<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::Image {
            destination: self.destination,
            title: self.title,
            alt: self.alt,
            user_data: data,
        }
    }
}

impl<T: Default> WithData<T> for LinkReference {
    type WithDataType = generic::LinkReference<T>;

    fn with_data(self, data: T) -> Self::WithDataType {
        generic::LinkReference {
            label: self
                .label
                .into_iter()
                .map(|i| i.with_data(T::default()))
                .collect(),
            text: self
                .text
                .into_iter()
                .map(|i| i.with_data(T::default()))
                .collect(),
            user_data: data,
        }
    }
}

// ——————————————————————————————————————————————————————————————————————————
// Conversion functions for generic AST -> regular AST
// ——————————————————————————————————————————————————————————————————————————

impl<T> StripData<T> for generic::Document<T> {
    type StrippedType = Document;

    fn strip_data(self) -> Self::StrippedType {
        Document {
            blocks: self.blocks.into_iter().map(|b| b.strip_data()).collect(),
        }
    }
}

impl<T> StripData<T> for generic::Block<T> {
    type StrippedType = Block;

    fn strip_data(self) -> Self::StrippedType {
        match self {
            generic::Block::Paragraph { content, .. } => {
                Block::Paragraph(content.into_iter().map(|i| i.strip_data()).collect())
            }
            generic::Block::Heading(heading) => Block::Heading(heading.strip_data()),
            generic::Block::ThematicBreak { .. } => Block::ThematicBreak,
            generic::Block::BlockQuote { blocks, .. } => {
                Block::BlockQuote(blocks.into_iter().map(|b| b.strip_data()).collect())
            }
            generic::Block::List(list) => Block::List(list.strip_data()),
            generic::Block::CodeBlock(code_block) => Block::CodeBlock(code_block.strip_data()),
            generic::Block::HtmlBlock { content, .. } => Block::HtmlBlock(content),
            generic::Block::Definition(def) => Block::Definition(def.strip_data()),
            generic::Block::Table(table) => Block::Table(table.strip_data()),
            generic::Block::FootnoteDefinition(footnote) => {
                Block::FootnoteDefinition(footnote.strip_data())
            }
            generic::Block::GitHubAlert(alert) => Block::GitHubAlert(alert.strip_data()),
            generic::Block::Empty { .. } => Block::Empty,
        }
    }
}

impl<T> StripData<T> for generic::Inline<T> {
    type StrippedType = Inline;

    fn strip_data(self) -> Self::StrippedType {
        match self {
            generic::Inline::Text { content, .. } => Inline::Text(content),
            generic::Inline::LineBreak { .. } => Inline::LineBreak,
            generic::Inline::Code { content, .. } => Inline::Code(content),
            generic::Inline::Html { content, .. } => Inline::Html(content),
            generic::Inline::Link(link) => Inline::Link(link.strip_data()),
            generic::Inline::LinkReference(link_ref) => {
                Inline::LinkReference(link_ref.strip_data())
            }
            generic::Inline::Image(image) => Inline::Image(image.strip_data()),
            generic::Inline::Emphasis { content, .. } => {
                Inline::Emphasis(content.into_iter().map(|i| i.strip_data()).collect())
            }
            generic::Inline::Strong { content, .. } => {
                Inline::Strong(content.into_iter().map(|i| i.strip_data()).collect())
            }
            generic::Inline::Strikethrough { content, .. } => {
                Inline::Strikethrough(content.into_iter().map(|i| i.strip_data()).collect())
            }
            generic::Inline::Autolink { url, .. } => Inline::Autolink(url),
            generic::Inline::FootnoteReference { label, .. } => Inline::FootnoteReference(label),
            generic::Inline::Empty { .. } => Inline::Empty,
        }
    }
}

impl<T> StripData<T> for generic::Heading<T> {
    type StrippedType = Heading;

    fn strip_data(self) -> Self::StrippedType {
        Heading {
            kind: self.kind,
            content: self.content.into_iter().map(|i| i.strip_data()).collect(),
        }
    }
}

impl<T> StripData<T> for generic::List<T> {
    type StrippedType = List;

    fn strip_data(self) -> Self::StrippedType {
        List {
            kind: self.kind.into(),
            items: self.items.into_iter().map(|i| i.strip_data()).collect(),
        }
    }
}

impl<T> StripData<T> for generic::ListItem<T> {
    type StrippedType = ListItem;

    fn strip_data(self) -> Self::StrippedType {
        ListItem {
            task: self.task,
            blocks: self.blocks.into_iter().map(|b| b.strip_data()).collect(),
        }
    }
}

impl<T> StripData<T> for generic::CodeBlock<T> {
    type StrippedType = CodeBlock;

    fn strip_data(self) -> Self::StrippedType {
        CodeBlock {
            kind: self.kind,
            literal: self.literal,
        }
    }
}

impl<T> StripData<T> for generic::LinkDefinition<T> {
    type StrippedType = LinkDefinition;

    fn strip_data(self) -> Self::StrippedType {
        LinkDefinition {
            label: self.label.into_iter().map(|i| i.strip_data()).collect(),
            destination: self.destination,
            title: self.title,
        }
    }
}

impl<T> StripData<T> for generic::Table<T> {
    type StrippedType = Table;

    fn strip_data(self) -> Self::StrippedType {
        Table {
            rows: self
                .rows
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|cell| cell.into_iter().map(|i| i.strip_data()).collect())
                        .collect()
                })
                .collect(),
            alignments: self.alignments,
        }
    }
}

impl<T> StripData<T> for generic::FootnoteDefinition<T> {
    type StrippedType = FootnoteDefinition;

    fn strip_data(self) -> Self::StrippedType {
        FootnoteDefinition {
            label: self.label,
            blocks: self.blocks.into_iter().map(|b| b.strip_data()).collect(),
        }
    }
}

impl<T> StripData<T> for generic::GitHubAlertNode<T> {
    type StrippedType = GitHubAlert;

    fn strip_data(self) -> Self::StrippedType {
        GitHubAlert {
            alert_type: self.alert_type,
            blocks: self.blocks.into_iter().map(|b| b.strip_data()).collect(),
        }
    }
}

impl<T> StripData<T> for generic::Link<T> {
    type StrippedType = Link;

    fn strip_data(self) -> Self::StrippedType {
        Link {
            destination: self.destination,
            title: self.title,
            children: self.children.into_iter().map(|i| i.strip_data()).collect(),
        }
    }
}

impl<T> StripData<T> for generic::Image<T> {
    type StrippedType = Image;

    fn strip_data(self) -> Self::StrippedType {
        Image {
            destination: self.destination,
            title: self.title,
            alt: self.alt,
        }
    }
}

impl<T> StripData<T> for generic::LinkReference<T> {
    type StrippedType = LinkReference;

    fn strip_data(self) -> Self::StrippedType {
        LinkReference {
            label: self.label.into_iter().map(|i| i.strip_data()).collect(),
            text: self.text.into_iter().map(|i| i.strip_data()).collect(),
        }
    }
}

// ——————————————————————————————————————————————————————————————————————————
// MapData implementations (transform user data type)
// NOTE: Disabled due to compiler recursion limits
// ——————————————————————————————————————————————————————————————————————————

/*
// Temporarily commented out due to recursion limit issues
impl<T, U> MapData<T, U> for generic::Document<T> {
    type MappedType = generic::Document<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::Document {
            blocks: self.blocks.into_iter().map(|b| b.map_data(&mut f)).collect(),
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::Block<T> {
    type MappedType = generic::Block<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        match self {
            generic::Block::Paragraph { content, user_data } => generic::Block::Paragraph {
                content: content.into_iter().map(|i| i.map_data(&mut f)).collect(),
                user_data: f(user_data),
            },
            generic::Block::Heading(heading) => generic::Block::Heading(heading.map_data(f)),
            generic::Block::ThematicBreak { user_data } => generic::Block::ThematicBreak { user_data: f(user_data) },
            generic::Block::BlockQuote { blocks, user_data } => generic::Block::BlockQuote {
                blocks: blocks.into_iter().map(|b| b.map_data(&mut f)).collect(),
                user_data: f(user_data),
            },
            generic::Block::List(list) => generic::Block::List(list.map_data(f)),
            generic::Block::CodeBlock(code_block) => generic::Block::CodeBlock(code_block.map_data(f)),
            generic::Block::HtmlBlock { content, user_data } => generic::Block::HtmlBlock {
                content,
                user_data: f(user_data),
            },
            generic::Block::Definition(def) => generic::Block::Definition(def.map_data(f)),
            generic::Block::Table(table) => generic::Block::Table(table.map_data(f)),
            generic::Block::FootnoteDefinition(footnote) => generic::Block::FootnoteDefinition(footnote.map_data(f)),
            generic::Block::GitHubAlert(alert) => generic::Block::GitHubAlert(alert.map_data(f)),
            generic::Block::Empty { user_data } => generic::Block::Empty { user_data: f(user_data) },
        }
    }
}

impl<T, U> MapData<T, U> for generic::Inline<T> {
    type MappedType = generic::Inline<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        match self {
            generic::Inline::Text { content, user_data } => generic::Inline::Text {
                content,
                user_data: f(user_data),
            },
            generic::Inline::LineBreak { user_data } => generic::Inline::LineBreak { user_data: f(user_data) },
            generic::Inline::Code { content, user_data } => generic::Inline::Code {
                content,
                user_data: f(user_data),
            },
            generic::Inline::Html { content, user_data } => generic::Inline::Html {
                content,
                user_data: f(user_data),
            },
            generic::Inline::Link(link) => generic::Inline::Link(link.map_data(f)),
            generic::Inline::LinkReference(link_ref) => generic::Inline::LinkReference(link_ref.map_data(f)),
            generic::Inline::Image(image) => generic::Inline::Image(image.map_data(f)),
            generic::Inline::Emphasis { content, user_data } => generic::Inline::Emphasis {
                content: content.into_iter().map(|i| i.map_data(&mut f)).collect(),
                user_data: f(user_data),
            },
            generic::Inline::Strong { content, user_data } => generic::Inline::Strong {
                content: content.into_iter().map(|i| i.map_data(&mut f)).collect(),
                user_data: f(user_data),
            },
            generic::Inline::Strikethrough { content, user_data } => generic::Inline::Strikethrough {
                content: content.into_iter().map(|i| i.map_data(&mut f)).collect(),
                user_data: f(user_data),
            },
            generic::Inline::Autolink { url, user_data } => generic::Inline::Autolink {
                url,
                user_data: f(user_data),
            },
            generic::Inline::FootnoteReference { label, user_data } => generic::Inline::FootnoteReference {
                label,
                user_data: f(user_data),
            },
            generic::Inline::Empty { user_data } => generic::Inline::Empty { user_data: f(user_data) },
        }
    }
}

// Implementation for other types would follow similar patterns...
// For brevity, I'll implement a few key ones

impl<T, U> MapData<T, U> for generic::Heading<T> {
    type MappedType = generic::Heading<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::Heading {
            kind: self.kind,
            content: self.content.into_iter().map(|i| i.map_data(&mut f)).collect(),
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::List<T> {
    type MappedType = generic::List<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::List {
            kind: self.kind,
            items: self.items.into_iter().map(|i| i.map_data(&mut f)).collect(),
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::ListItem<T> {
    type MappedType = generic::ListItem<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::ListItem {
            task: self.task,
            blocks: self.blocks.into_iter().map(|b| b.map_data(&mut f)).collect(),
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::Link<T> {
    type MappedType = generic::Link<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::Link {
            destination: self.destination,
            title: self.title,
            children: self.children.into_iter().map(|i| i.map_data(&mut f)).collect(),
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::Image<T> {
    type MappedType = generic::Image<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::Image {
            destination: self.destination,
            title: self.title,
            alt: self.alt,
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::LinkReference<T> {
    type MappedType = generic::LinkReference<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::LinkReference {
            label: self.label.into_iter().map(|i| i.map_data(&mut f)).collect(),
            text: self.text.into_iter().map(|i| i.map_data(&mut f)).collect(),
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::CodeBlock<T> {
    type MappedType = generic::CodeBlock<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::CodeBlock {
            kind: self.kind,
            literal: self.literal,
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::Table<T> {
    type MappedType = generic::Table<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::Table {
            rows: self.rows.into_iter().map(|row| {
                row.into_iter().map(|cell| {
                    cell.into_iter().map(|i| i.map_data(&mut f)).collect()
                }).collect()
            }).collect(),
            alignments: self.alignments,
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::FootnoteDefinition<T> {
    type MappedType = generic::FootnoteDefinition<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::FootnoteDefinition {
            label: self.label,
            blocks: self.blocks.into_iter().map(|b| b.map_data(&mut f)).collect(),
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::GitHubAlertNode<T> {
    type MappedType = generic::GitHubAlertNode<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::GitHubAlertNode {
            alert_type: self.alert_type,
            blocks: self.blocks.into_iter().map(|b| b.map_data(&mut f)).collect(),
            user_data: f(self.user_data),
        }
    }
}

impl<T, U> MapData<T, U> for generic::LinkDefinition<T> {
    type MappedType = generic::LinkDefinition<U>;

    fn map_data<F>(self, mut f: F) -> Self::MappedType
    where
        F: FnMut(T) -> U,
    {
        generic::LinkDefinition {
            label: self.label.into_iter().map(|i| i.map_data(&mut f)).collect(),
            destination: self.destination,
            title: self.title,
            user_data: f(self.user_data),
        }
    }
}
*/

// End of MapData implementations - commented out due to recursion limits

// ——————————————————————————————————————————————————————————————————————————
// Helper functions
// ——————————————————————————————————————————————————————————————————————————

/// Convert from regular AST to generic AST with unit type
pub fn to_generic(doc: Document) -> generic::Document<()> {
    doc.with_default_data()
}

/// Convert from generic AST with unit type to regular AST
pub fn from_generic(doc: generic::Document<()>) -> Document {
    doc.strip_data()
}

// Conversion between ListKind variants
impl From<ListKind> for generic::ListKind {
    fn from(kind: ListKind) -> Self {
        match kind {
            ListKind::Ordered(opts) => generic::ListKind::Ordered(opts),
            ListKind::Bullet(bullet) => generic::ListKind::Bullet(bullet),
        }
    }
}

impl From<generic::ListKind> for ListKind {
    fn from(kind: generic::ListKind) -> Self {
        match kind {
            generic::ListKind::Ordered(opts) => ListKind::Ordered(opts),
            generic::ListKind::Bullet(bullet) => ListKind::Bullet(bullet),
        }
    }
}
