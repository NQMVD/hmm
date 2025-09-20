use crate::app::{App, Mode};
use crate::node::Node;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use uuid::Uuid;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3), // Main area
            Constraint::Length(1), // Status bar
        ])
        .split(f.size());

    // Main tree view
    draw_tree(f, chunks[0], app);
    
    // Status bar
    draw_status_bar(f, chunks[1], app);
    
    // Overlays
    if app.mode == Mode::Edit {
        draw_edit_popup(f, app);
    } else if app.mode == Mode::Search {
        draw_search_popup(f, app);
    }
}

fn draw_tree(f: &mut Frame, area: Rect, app: &mut App) {
    let tree_items = build_tree_items(&app.root, app.active_node_id, &app.search_results, 0);
    
    // Find the index of the active node
    let mut list_state = ListState::default();
    if let Some(index) = find_active_node_index(&tree_items, app.active_node_id) {
        list_state.select(Some(index));
    }
    
    let list = List::new(tree_items)
        .block(Block::default().borders(Borders::ALL).title("Mind Map"))
        .highlight_style(
            Style::default()
                .bg(Color::LightYellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut list_state);
}

fn build_tree_items<'a>(
    node: &'a Node,
    active_id: Uuid,
    search_results: &'a [Uuid],
    level: usize,
) -> Vec<ListItem<'a>> {
    let mut items = Vec::new();
    
    // Skip root if it only has one child and the text is generic
    if level == 0 && node.children.len() == 1 && (node.text == "root" || node.text.is_empty()) {
        return build_tree_items(&node.children[0], active_id, search_results, 0);
    }
    
    let indent = "  ".repeat(level);
    let mut text_parts = vec![Span::raw(indent)];
    
    // Add collapse indicator
    if !node.children.is_empty() {
        if node.collapsed {
            text_parts.push(Span::raw("▶ "));
        } else {
            text_parts.push(Span::raw("▼ "));
        }
    } else {
        text_parts.push(Span::raw("  "));
    }
    
    // Add symbol if present
    if let Some(symbol) = node.symbol {
        text_parts.push(Span::styled(
            format!("{} ", symbol),
            Style::default().fg(Color::Green),
        ));
    }
    
    // Add node text
    let mut text_style = Style::default();
    if node.id == active_id {
        text_style = text_style.add_modifier(Modifier::BOLD);
    }
    if search_results.contains(&node.id) {
        text_style = text_style.bg(Color::Yellow).fg(Color::Black);
    }
    text_parts.push(Span::styled(&node.text, text_style));
    
    // Add metadata
    let mut metadata_parts = Vec::new();
    
    if node.rank_positive != 0 || node.rank_negative != 0 {
        let rank_text = format!(" [+{}/{}]", node.rank_positive, -node.rank_negative);
        let rank_color = if node.rank_positive > -node.rank_negative {
            Color::Green
        } else if node.rank_positive < -node.rank_negative {
            Color::Red
        } else {
            Color::Yellow
        };
        metadata_parts.push(Span::styled(rank_text, Style::default().fg(rank_color)));
    }
    
    if node.stars > 0 {
        let stars = "★".repeat(node.stars as usize);
        metadata_parts.push(Span::styled(
            format!(" {}", stars),
            Style::default().fg(Color::Yellow),
        ));
    }
    
    if node.collapsed && !node.children.is_empty() {
        metadata_parts.push(Span::styled(
            format!(" ({})", node.children.len()),
            Style::default().fg(Color::Cyan),
        ));
    }
    
    text_parts.extend(metadata_parts);
    
    let item = ListItem::new(Line::from(text_parts));
    items.push(item);
    
    // Add children if not collapsed
    if !node.collapsed {
        for child in &node.children {
            items.extend(build_tree_items(child, active_id, search_results, level + 1));
        }
    }
    
    items
}

fn find_active_node_index(items: &[ListItem], _active_id: Uuid) -> Option<usize> {
    // This is a simplified approach - in a real implementation,
    // we'd need to track node IDs while building the list
    // For now, just return the first item
    if !items.is_empty() {
        Some(0)
    } else {
        None
    }
}

fn draw_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let mode_text = match app.mode {
        Mode::Normal => "NORMAL",
        Mode::Edit => "EDIT",
        Mode::Search => "SEARCH",
    };
    
    let filename = app.file_manager.filename
        .as_ref()
        .map(|f| std::path::Path::new(f).file_name().unwrap_or_default().to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string());
    
    let modified = if app.modified { "*" } else { "" };
    
    let status_text = match &app.message {
        Some(msg) => msg.clone(),
        None => format!("{}  {}{}  Press ? for help", mode_text, filename, modified),
    };
    
    let status = Paragraph::new(status_text)
        .style(Style::default().bg(Color::Blue).fg(Color::White));
    
    f.render_widget(status, area);
}

fn draw_edit_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    
    f.render_widget(Clear, area);
    
    let edit_text = format!("{}{}", app.edit_buffer, "_");
    let paragraph = Paragraph::new(edit_text)
        .block(Block::default().borders(Borders::ALL).title("Edit Node"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

fn draw_search_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, f.size());
    
    f.render_widget(Clear, area);
    
    let search_text = format!("{}{}", app.search_query, "_");
    let paragraph = Paragraph::new(search_text)
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}