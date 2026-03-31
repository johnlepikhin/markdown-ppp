/// Configuration for plaintext rendering output.
///
/// # Examples
///
/// ```rust
/// use markdown_ppp::plaintext_printer::config::Config;
///
/// let config = Config::default().with_width(120);
/// ```
pub struct Config {
    pub(crate) width: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { width: 80 }
    }
}

impl Config {
    /// Set the line width for pretty-printing plaintext output.
    pub fn with_width(self, width: usize) -> Self {
        Self { width }
    }
}
