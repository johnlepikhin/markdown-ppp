/// GitHub markdown alerts types
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GitHubAlertType {
    /// Blue note alert
    Note,
    /// Green tip alert
    Tip,
    /// Blue important alert
    Important,
    /// Yellow warning alert
    Warning,
    /// Red caution alert
    Caution,
    /// Custom alert with a user-defined label
    Custom(String),
}

/// GitHub alert block
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ast-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GitHubAlert {
    /// Type of the alert
    pub alert_type: GitHubAlertType,
    /// Content blocks inside the alert
    pub blocks: Vec<crate::ast::Block>,
}
