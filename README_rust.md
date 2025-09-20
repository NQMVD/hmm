# Rust Mind Map

A modern Rust rewrite of the h-m-m (Hackers Mind Map) tool - a simple, fast, keyboard-centric terminal-based application for creating, editing, and managing mind maps.

![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-blue.svg)

## Features

- **Terminal-based UI**: Built with Ratatui for a responsive terminal interface
- **Keyboard-centric**: Vim-like key bindings for efficient navigation and editing
- **Cross-platform**: Works on Linux, macOS, and Windows
- **File format**: Simple plain text format with tab-based indentation
- **Tree operations**: Add, remove, move, copy/paste nodes
- **Search**: Fast text search across the entire mind map
- **Ranking**: Positive/negative ranking and star rating system
- **Symbols**: Toggle symbols like ✓ and ✗ on nodes
- **Export**: HTML export functionality
- **Undo/Redo**: Full undo support

## Installation

### Building from source

```bash
git clone <repository-url>
cd hmm
cargo build --release
./target/release/rust-mind-map
```

### Dependencies

The application uses the following main dependencies:
- `ratatui` - Terminal user interface framework
- `crossterm` - Cross-platform terminal manipulation
- `clap` - Command line argument parsing
- `serde` + `toml` - Configuration management
- `uuid` - Node identification
- `regex` - Search functionality

## Usage

### Basic Commands

```bash
# Start with empty mind map
rust-mind-map

# Open existing file
rust-mind-map my-map.rmm

# Show help
rust-mind-map --help

# Set configuration options
rust-mind-map --auto-save true --max-leaf-node-width 60 my-map.rmm
```

### Key Bindings

#### Navigation
- `h/←` - Go to parent node
- `j/↓` - Go down to next sibling
- `k/↑` - Go up to previous sibling  
- `l/→` - Go to first child
- `g` - Go to top
- `G` - Go to bottom
- `m/~` - Go to root

#### Editing
- `i/a/e/Enter` - Edit node text
- `I/A/E` - Replace node text
- `o` - Add new sibling
- `O/Tab` - Add new child
- `d` - Delete node (with clipboard)
- `y` - Copy node
- `p` - Paste as child
- `P` - Paste as sibling
- `u` - Undo

#### Tree Operations
- `Space/z` - Toggle collapse/expand
- `Z` - Collapse all
- `b` - Expand all
- `1-5` - Collapse to specific level

#### Search
- `/` - Enter search mode
- `n` - Next search result
- `N` - Previous search result

#### Ranking & Symbols
- `=` - Increase positive rank
- `-` - Increase negative rank  
- `*` - Add star
- `t` - Toggle symbol (✓/✗)

#### File Operations
- `s` - Save
- `S` - Save as
- `x` - Export to HTML
- `X` - Export to text clipboard
- `q` - Quit (with confirmation if modified)
- `Q` - Quit without saving

#### Help
- `?` - Show help screen

### File Format

Mind maps are stored in plain text files with `.rmm` extension. The structure uses tab indentation:

```
My Project
	Planning Phase
		Research
		Requirements [rank: +2/-1, stars: 3]
	Development Phase
		Backend
			Database Design
			API Development
		Frontend
			UI Design
			Implementation
	Testing Phase
		Unit Tests
		Integration Tests
```

Metadata can be included in square brackets:
- `[rank: +2/-1]` - Positive/negative rankings
- `[stars: 3]` - Star ratings (0-5)

## Configuration

Configuration can be set via:

1. Command line arguments
2. Environment variables (prefix with `rust_mind_map_`)
3. Configuration file

Configuration file locations:
- Linux: `~/.config/rust-mind-map/config.toml`
- macOS: `~/Library/Preferences/rust-mind-map/config.toml`
- Windows: `./config.toml` (same directory as executable)

Example configuration:
```toml
max_parent_node_width = 25
max_leaf_node_width = 55
line_spacing = 1
initial_depth = 1
center_lock = false
focus_lock = false
max_undo_steps = 24
auto_save = false
symbol1 = '✓'
symbol2 = '✗'
```

## Comparison with Original h-m-m

This Rust implementation provides:
- **Better Performance**: Compiled Rust vs interpreted PHP
- **Memory Safety**: Rust's ownership system prevents memory issues  
- **Cross-platform**: Single binary works across platforms
- **Modern Architecture**: Clean separation of concerns with modules
- **Type Safety**: Compile-time error catching
- **Better Error Handling**: Comprehensive error messages

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

### Code Structure

- `src/main.rs` - Application entry point and CLI handling
- `src/app.rs` - Core application state and logic
- `src/node.rs` - Tree node structure and operations
- `src/ui.rs` - Terminal user interface rendering
- `src/input.rs` - Event handling and key bindings
- `src/file.rs` - File parsing and serialization
- `src/config.rs` - Configuration management

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- Original h-m-m tool by Nader K. Rad
- Ratatui library for the excellent TUI framework
- Crossterm for cross-platform terminal support