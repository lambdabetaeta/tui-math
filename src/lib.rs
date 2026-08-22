//! # tui-math
//!
//! Render LaTeX math beautifully in terminal UIs with ratatui.
//!
//! ## Example
//!
//! ```rust
//! use tui_math::render_latex;
//!
//! // Render LaTeX to Unicode string
//! let rendered = render_latex(r"\frac{x^2 + 1}{y}").unwrap();
//! println!("{}", rendered);
//! ```
//!
//! ## Features
//!
//! The renderer itself — [`render_latex`], [`render_mathml`], [`MathRenderer`]
//! and [`MathBox`] — depends on no terminal library at all.  The ratatui
//! widgets live behind the default `widgets` feature, and the demo binary
//! behind `bin`, so a caller already holding its own ratatui version can take
//! the renderer alone:
//!
//! ```toml
//! tui-math = { version = "0.1", default-features = false }
//! ```
//!
//! With `widgets` on (the default), the same expression renders as a widget:
//!
//! ```rust,no_run
//! # #[cfg(feature = "widgets")] {
//! use tui_math::MathWidget;
//!
//! let widget = MathWidget::new(r"\int_0^\infty e^{-x^2} dx");
//! # }
//! ```

#[cfg(feature = "widgets")]
mod canvas_widget;
mod mathbox;
mod renderer;
mod unicode_maps;
#[cfg(feature = "widgets")]
mod widget;

#[cfg(feature = "widgets")]
pub use canvas_widget::CanvasMathWidget;
pub use mathbox::MathBox;
pub use renderer::{MathRenderer, RenderError};
#[cfg(feature = "widgets")]
pub use widget::{MathWidget, MathWidgetState, StatefulMathWidget};

/// Render LaTeX math to a Unicode string for terminal display
pub fn render_latex(latex: &str) -> Result<String, RenderError> {
    let renderer = MathRenderer::new();
    renderer.render_latex(latex)
}

/// Render MathML to a Unicode string for terminal display
pub fn render_mathml(mathml: &str) -> Result<String, RenderError> {
    let renderer = MathRenderer::new();
    renderer.render_mathml(mathml)
}
