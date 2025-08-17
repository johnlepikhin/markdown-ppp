use crate::ast::{GitHubAlert, GitHubAlertType};
use crate::printer::{config::Config, ToDoc};
use pretty::{Arena, DocAllocator, DocBuilder};
use std::rc::Rc;

impl GitHubAlertType {
    /// Get the lowercase name of the alert type for markdown output
    pub(crate) fn as_markdown_str(&self) -> &'static str {
        match self {
            GitHubAlertType::Note => "note",
            GitHubAlertType::Tip => "tip",
            GitHubAlertType::Important => "important",
            GitHubAlertType::Warning => "warning",
            GitHubAlertType::Caution => "caution",
        }
    }
}

/// Convert GitHub alert to markdown blockquote with alert marker
pub(crate) fn github_alert_to_doc<'a>(
    alert: &GitHubAlert,
    config: Rc<Config>,
    arena: &'a Arena<'a>,
) -> DocBuilder<'a, Arena<'a>, ()> {
    // Create the alert marker line
    let marker = format!("> [!{}]", alert.alert_type.as_markdown_str().to_uppercase());
    let mut lines = vec![marker];

    // Convert alert blocks to blockquote format
    if !alert.blocks.is_empty() {
        let content_doc = alert.blocks.to_doc(config.clone(), arena);
        let content_string = content_doc.pretty(80).to_string();

        for line in content_string.lines() {
            lines.push(format!("> {line}"));
        }
    }

    arena.intersperse(
        lines.into_iter().map(|line| arena.text(line)),
        arena.hardline(),
    )
}
