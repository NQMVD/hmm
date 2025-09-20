use crate::app::{App, Mode};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_events(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(16))? {
        if let Event::Key(key) = event::read()? {
            handle_key_event(app, key)?;
        }
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    // Clear message on most key presses
    if !matches!(app.mode, Mode::Edit | Mode::Search) {
        app.clear_message();
    }

    match app.mode {
        Mode::Normal => handle_normal_mode(app, key)?,
        Mode::Edit => handle_edit_mode(app, key)?,
        Mode::Search => handle_search_mode(app, key)?,
    }
    
    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        // Navigation
        KeyCode::Char('h') | KeyCode::Left => navigate_to_parent(app),
        KeyCode::Char('j') | KeyCode::Down => navigate_down(app),
        KeyCode::Char('k') | KeyCode::Up => navigate_up(app),
        KeyCode::Char('l') | KeyCode::Right => navigate_to_child(app),
        
        // Node movement
        KeyCode::Char('H') => move_node_left(app),
        KeyCode::Char('J') => move_node_down(app),
        KeyCode::Char('K') => move_node_up(app),
        KeyCode::Char('L') => move_node_right(app),
        
        // Editing
        KeyCode::Char('i') | KeyCode::Char('a') | KeyCode::Char('e') | KeyCode::Enter => {
            app.enter_edit_mode();
        },
        KeyCode::Char('I') | KeyCode::Char('A') | KeyCode::Char('E') => {
            app.enter_edit_mode();
            app.edit_buffer.clear();
            app.edit_cursor = 0;
        },
        KeyCode::Char('o') => app.add_sibling(),
        KeyCode::Char('O') | KeyCode::Tab => app.add_child(),
        KeyCode::Char('d') => app.delete_node(),
        KeyCode::Char('y') => app.copy_node(),
        KeyCode::Char('p') => app.paste_as_child(),
        KeyCode::Char('P') => app.paste_as_sibling(),
        KeyCode::Char('u') => app.undo(),
        
        // Collapsing/Expanding
        KeyCode::Char(' ') | KeyCode::Char('z') => app.toggle_collapse(),
        KeyCode::Char('Z') => app.collapse_all(),
        KeyCode::Char('b') => app.expand_all(),
        KeyCode::Char('1') => app.collapse_to_level(1),
        KeyCode::Char('2') => app.collapse_to_level(2),
        KeyCode::Char('3') => app.collapse_to_level(3),
        KeyCode::Char('4') => app.collapse_to_level(4),
        KeyCode::Char('5') => app.collapse_to_level(5),
        
        // Search
        KeyCode::Char('/') => app.enter_search_mode(),
        KeyCode::Char('n') => app.next_search_result(),
        KeyCode::Char('N') => app.previous_search_result(),
        
        // Ranking
        KeyCode::Char('=') => app.increase_positive_rank(),
        KeyCode::Char('-') => app.increase_negative_rank(),
        KeyCode::Char('*') => app.add_star(),
        
        // Symbols
        KeyCode::Char('t') => app.toggle_symbol(),
        
        // File operations
        KeyCode::Char('s') => {
            if let Err(e) = app.save() {
                app.set_message(format!("Save error: {}", e));
            }
        },
        KeyCode::Char('S') => {
            // In a real implementation, we'd show a save dialog
            app.set_message("Save As not implemented yet".to_string());
        },
        KeyCode::Char('x') => {
            let _html = app.export_html();
            app.set_message("HTML export created".to_string());
            // TODO: Save to file or clipboard
        },
        KeyCode::Char('X') => {
            // Export to clipboard
            app.set_message("Text export not implemented yet".to_string());
        },
        
        // Navigation shortcuts
        KeyCode::Char('g') => navigate_to_top(app),
        KeyCode::Char('G') => navigate_to_bottom(app),
        KeyCode::Char('m') | KeyCode::Char('~') => navigate_to_root(app),
        
        // Help
        KeyCode::Char('?') => show_help(app),
        
        // Quit
        KeyCode::Char('q') => {
            if app.modified {
                app.set_message("Unsaved changes! Use Q to quit without saving.".to_string());
            } else {
                app.should_quit = true;
            }
        },
        KeyCode::Char('Q') => app.should_quit = true,
        
        // Ctrl combinations
        _ if key.modifiers.contains(KeyModifiers::CONTROL) => match key.code {
            KeyCode::Char('c') => app.should_quit = true,
            KeyCode::Char('f') => app.enter_search_mode(),
            _ => {}
        },
        
        _ => {}
    }
    
    Ok(())
}

fn handle_edit_mode(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => app.cancel_edit_mode(),
        KeyCode::Enter => app.exit_edit_mode(),
        KeyCode::Backspace => {
            if app.edit_cursor > 0 {
                app.edit_buffer.remove(app.edit_cursor - 1);
                app.edit_cursor -= 1;
            }
        },
        KeyCode::Delete => {
            if app.edit_cursor < app.edit_buffer.len() {
                app.edit_buffer.remove(app.edit_cursor);
            }
        },
        KeyCode::Left => {
            if app.edit_cursor > 0 {
                app.edit_cursor -= 1;
            }
        },
        KeyCode::Right => {
            if app.edit_cursor < app.edit_buffer.len() {
                app.edit_cursor += 1;
            }
        },
        KeyCode::Home => app.edit_cursor = 0,
        KeyCode::End => app.edit_cursor = app.edit_buffer.len(),
        KeyCode::Char(c) => {
            app.edit_buffer.insert(app.edit_cursor, c);
            app.edit_cursor += 1;
        },
        _ => {}
    }
    
    Ok(())
}

fn handle_search_mode(app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc => app.exit_search_mode(),
        KeyCode::Enter => app.perform_search(),
        KeyCode::Backspace => {
            app.search_query.pop();
        },
        KeyCode::Char(c) => {
            app.search_query.push(c);
        },
        _ => {}
    }
    
    Ok(())
}

// Navigation functions
fn navigate_to_parent(app: &mut App) {
    let all_nodes = app.root.get_all_nodes();
    for node_id in all_nodes {
        if let Some(parent) = app.root.find_parent_mut(node_id) {
            if node_id == app.active_node_id {
                app.active_node_id = parent.id;
                return;
            }
        }
    }
}

fn navigate_down(app: &mut App) {
    // Simplified navigation - in a real implementation, this would be more sophisticated
    let all_nodes = app.root.get_all_nodes();
    if let Some(current_index) = all_nodes.iter().position(|&id| id == app.active_node_id) {
        if current_index + 1 < all_nodes.len() {
            app.active_node_id = all_nodes[current_index + 1];
        }
    }
}

fn navigate_up(app: &mut App) {
    let all_nodes = app.root.get_all_nodes();
    if let Some(current_index) = all_nodes.iter().position(|&id| id == app.active_node_id) {
        if current_index > 0 {
            app.active_node_id = all_nodes[current_index - 1];
        }
    }
}

fn navigate_to_child(app: &mut App) {
    if let Some(node) = app.root.find_node(app.active_node_id) {
        if !node.children.is_empty() {
            app.active_node_id = node.children[0].id;
            
            // Expand if collapsed
            if let Some(node_mut) = app.root.find_node_mut(app.active_node_id) {
                if node_mut.collapsed {
                    node_mut.collapsed = false;
                }
            }
        }
    }
}

fn navigate_to_root(app: &mut App) {
    app.active_node_id = app.root.id;
}

fn navigate_to_top(app: &mut App) {
    let all_nodes = app.root.get_all_nodes();
    if !all_nodes.is_empty() {
        app.active_node_id = all_nodes[0];
    }
}

fn navigate_to_bottom(app: &mut App) {
    let all_nodes = app.root.get_all_nodes();
    if !all_nodes.is_empty() {
        app.active_node_id = all_nodes[all_nodes.len() - 1];
    }
}

// Placeholder functions for node movement
fn move_node_left(app: &mut App) {
    app.set_message("Move node left not implemented yet".to_string());
}

fn move_node_right(app: &mut App) {
    app.set_message("Move node right not implemented yet".to_string());
}

fn move_node_up(app: &mut App) {
    app.set_message("Move node up not implemented yet".to_string());
}

fn move_node_down(app: &mut App) {
    app.set_message("Move node down not implemented yet".to_string());
}

fn show_help(app: &mut App) {
    let help_text = "
KEYBOARD SHORTCUTS:

Navigation:
  h/←  - Go to parent
  j/↓  - Go down
  k/↑  - Go up  
  l/→  - Go to first child
  g    - Go to top
  G    - Go to bottom
  m/~  - Go to root

Editing:
  i/a/e/Enter - Edit node
  I/A/E       - Replace node text
  o           - Add sibling
  O/Tab       - Add child
  d           - Delete node
  y           - Copy node
  p           - Paste as child
  P           - Paste as sibling
  u           - Undo

Tree:
  Space/z     - Toggle collapse
  Z           - Collapse all
  b           - Expand all
  1-5         - Collapse to level

Search:
  /           - Search
  n           - Next result
  N           - Previous result

Ranking:
  =           - Increase positive rank
  -           - Increase negative rank
  *           - Add star
  t           - Toggle symbol

Files:
  s           - Save
  S           - Save as
  x           - Export HTML
  X           - Export text
  q           - Quit
  Q           - Quit without saving

Press any key to close help.
";
    app.set_message(help_text.to_string());
}