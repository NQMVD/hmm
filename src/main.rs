mod app;
mod config;
mod file;
mod input;
mod node;
mod ui;

use app::App;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

#[derive(Parser)]
#[command(name = "rust-mind-map")]
#[command(about = "A terminal-based mind mapping tool")]
#[command(version = "0.1.0")]
struct Args {
    /// Mind map file to open
    filename: Option<String>,
    
    /// Configuration file path
    #[arg(long)]
    config: Option<String>,
    
    /// Maximum width for parent nodes
    #[arg(long)]
    max_parent_node_width: Option<usize>,
    
    /// Maximum width for leaf nodes
    #[arg(long)]
    max_leaf_node_width: Option<usize>,
    
    /// Enable auto save
    #[arg(long)]
    auto_save: Option<bool>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the application
    let result = run_app(&mut terminal, args);
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
    
    Ok(())
}

fn run_app<B>(terminal: &mut Terminal<B>, args: Args) -> Result<(), Box<dyn std::error::Error>>
where
    B: ratatui::backend::Backend,
{
    // Create app instance
    let mut app = App::new(args.filename)?;
    
    // Apply command line overrides
    if let Some(width) = args.max_parent_node_width {
        app.config.max_parent_node_width = width;
    }
    if let Some(width) = args.max_leaf_node_width {
        app.config.max_leaf_node_width = width;
    }
    if let Some(auto_save) = args.auto_save {
        app.config.auto_save = auto_save;
    }
    
    // Main loop
    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        input::handle_events(&mut app)?;
    }
    
    Ok(())
}
