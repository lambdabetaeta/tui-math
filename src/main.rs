//! Demo application for tui-math

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_math::MathWidget;

const EXAMPLES: &[(&str, &str)] = &[
    ("Quadratic Formula", r"x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}"),
    ("Euler's Identity", r"e^{i\pi} + 1 = 0"),
    ("Integral", r"\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}"),
    ("Sum", r"\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}"),
    ("Greek Letters", r"\alpha + \beta = \gamma"),
    ("Matrix-like", r"a_{11} + a_{22} + a_{33}"),
    ("Fraction", r"\frac{a + b}{c + d}"),
    ("Square Root", r"\sqrt{x^2 + y^2}"),
    ("Limits", r"\lim_{x \to \infty} \frac{1}{x} = 0"),
    ("Derivative", r"\frac{d}{dx} x^n = nx^{n-1}"),
    ("Product", r"\prod_{i=1}^{n} i = n!"),
    ("Binomial", r"\binom{n}{k} = \frac{n!}{k!(n-k)!}"),
];

struct App {
    current_example: usize,
    custom_latex: String,
    editing: bool,
}

impl App {
    fn new() -> Self {
        Self {
            current_example: 0,
            custom_latex: String::new(),
            editing: false,
        }
    }

    fn current_latex(&self) -> &str {
        if self.editing {
            &self.custom_latex
        } else {
            EXAMPLES[self.current_example].1
        }
    }

    fn current_title(&self) -> &str {
        if self.editing {
            "Custom Input"
        } else {
            EXAMPLES[self.current_example].0
        }
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') if !app.editing => return Ok(()),
                KeyCode::Esc => {
                    if app.editing {
                        app.editing = false;
                    } else {
                        return Ok(());
                    }
                }
                KeyCode::Right | KeyCode::Char('l') if !app.editing => {
                    app.current_example = (app.current_example + 1) % EXAMPLES.len();
                }
                KeyCode::Left | KeyCode::Char('h') if !app.editing => {
                    app.current_example = app.current_example.checked_sub(1).unwrap_or(EXAMPLES.len() - 1);
                }
                KeyCode::Char('e') if !app.editing => {
                    app.editing = true;
                    app.custom_latex = EXAMPLES[app.current_example].1.to_string();
                }
                KeyCode::Enter if app.editing => {
                    app.editing = false;
                }
                KeyCode::Char(c) if app.editing => {
                    app.custom_latex.push(c);
                }
                KeyCode::Backspace if app.editing => {
                    app.custom_latex.pop();
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(3),  // LaTeX source
            Constraint::Min(10),    // Rendered output
            Constraint::Length(3),  // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(Line::from(vec![
        Span::styled("tui-math ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("- "),
        Span::styled(app.current_title(), Style::default().fg(Color::Yellow)),
        Span::raw(format!(" ({}/{})", app.current_example + 1, EXAMPLES.len())),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Demo"));
    f.render_widget(title, chunks[0]);

    // LaTeX source
    let source_style = if app.editing {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Gray)
    };
    let source = Paragraph::new(app.current_latex())
        .style(source_style)
        .block(Block::default().borders(Borders::ALL).title(if app.editing { "LaTeX (editing)" } else { "LaTeX" }));
    f.render_widget(source, chunks[1]);

    // Rendered math
    let math_widget = MathWidget::new(app.current_latex())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Rendered"));
    f.render_widget(math_widget, chunks[2]);

    // Help
    let help_text = if app.editing {
        "Enter: finish editing | Esc: cancel | Type to edit"
    } else {
        "←/→ or h/l: navigate | e: edit | q/Esc: quit"
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[3]);
}
