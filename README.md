# CC Session Manager

A CLI and GUI tool to manage Claude Code sessions, allowing you to view statistics, delete old sessions, and clean up projects efficiently.

## Version
- **Current**: 0.2.0
- **Release Date**: 2025

## Modes

### CLI Mode (Default)
Terminal-based interface with keyboard navigation.

### GUI Mode
Modern desktop application built with Tauri.

## Features

- **Session Statistics**: View detailed statistics about your Claude Code sessions including total count, storage usage, and age distribution
- **Project Management**: Browse projects and selectively delete individual sessions or entire projects
- **Age-based Cleanup**: Automatically find and delete sessions older than a specified number of days
- **Interactive UI**: User-friendly terminal interface with intuitive menu navigation and selection (CLI) / Modern desktop app (GUI)
- **Fast Performance**: Efficient scanning and management of large session collections
- **Safe Operations**: Confirmation prompts for destructive operations to prevent accidental data loss

## Installation

### Prerequisites

- Rust 1.70+ installed
- Claude Code installed on your system
- For GUI mode: Node.js (for frontend build)

### Building CLI Mode (Default)

```bash
cd cc-session-manager
cargo build --release
```

Binary available at: `target/release/cc-session-manager`

### For GUI Mode

Frontend build is required first (React + Vite + TailwindCSS):

```bash
cd cc-session-manager/ui
npm install
npm run build
cd ..

# Then build the Tauri app
cargo build --release --features gui
```

### Using Cargo (Recommended)

```bash
# CLI mode
cargo install --path . --features cli

# GUI mode
cargo install --path . --features gui
```

This will install the binary globally in your Cargo bin directory.

## Usage

### CLI Mode

```bash
# Run with default CLI interface
cargo run --features cli

# Or if installed
cc-session-manager
```

### GUI Mode

```bash
# Build and run GUI
cargo run --features gui
```

On macOS, the GUI app will be created at:
`target/release/bundle/dmg/CC Session Manager_0.2.0_x64.dmg`

## Menu Options (CLI)

1. **Show Statistics** - View comprehensive session statistics
2. **Manage by Project** - Browse and manage sessions by project
3. **Delete by Age** - Find and delete sessions older than N days
4. **Delete Project** - Remove entire project with all sessions
5. **Exit** - Clean exit from the application

### Keyboard Navigation (CLI)

- Use arrow keys or numbers to navigate menus
- Press Enter to select options
- Use Space to toggle selections in multi-select mode
- Press Esc or 0 to cancel/go back

## Requirements

- **Rust 1.70+** - Modern Rust toolchain with 2021 edition
- **Claude Code** - Must be installed (the tool automatically detects the Claude directory)
- **Terminal** - Compatible terminal application (macOS Terminal, iTerm2, Windows Terminal, etc.)
- For GUI: Node.js 18+ (for frontend dependency management)

## Dependencies

### Core
- `serde` - JSON serialization/deserialization for session metadata
- `tokio` - Async runtime for efficient file operations
- `chrono` - Date/time handling with serialization support
- `anyhow` - Ergonomic error handling and propagation
- `dirs` - Cross-platform directory path resolution

### CLI Mode
- `crossterm` - Cross-platform terminal manipulation and UI

### GUI Mode
- `tauri` v2 - Desktop application framework (Rust backend)
- **React 18** - UI framework (Frontend)
- **Vite** - Build tool
- **TailwindCSS** - Styling
- **Radix UI** - Accessible UI components
- **Lucide React** - Icons

## Project Structure

```
cc-session-manager/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── tauri_main.rs     # GUI entry point
│   ├── lib.rs            # Core library
│   ├── models.rs         # Data models
│   ├── session_manager.rs # Session management logic
│   ├── ui.rs             # CLI UI components
│   ├── utils.rs          # Utility functions
│   └── commands.rs       # Tauri commands
├── src-tauri/            # Tauri configuration
│   ├── src/
│   ├── tauri.conf.json
│   └── icons/
└── ui/                   # Frontend (for GUI mode)
```

## How It Works

The tool automatically locates your Claude Code installation directory and scans for:

- Project folders containing session data
- Session metadata files with timestamps and usage information
- Storage usage calculations for cleanup decisions

All operations are performed safely with confirmation prompts for destructive actions.

## Platform Support

| Platform | CLI | GUI |
|----------|-----|-----|
| macOS | ✅ | ✅ |
| Linux | ✅ | ✅ |
| Windows | ✅ | ✅ |

## Troubleshooting

### "Claude directory not found"
- Ensure Claude Code is properly installed
- Check that Claude has been run at least once to create the session directory
- Verify the tool has permission to access your home directory

### GUI won't start
- Ensure Node.js is installed for frontend builds
- Check that the `ui/dist` directory exists (run `npm install && npm run build` in the ui directory)
- Check logs for any errors

### Performance Issues
- For large numbers of sessions, the initial scan may take a few seconds
- Consider using the "Delete by Age" feature for bulk cleanup operations

## Development

```bash
# Clone and setup
git clone https://github.com/petch-x/cc-session-manager.git
cd cc-session-manager/cc-session-manager

# Build CLI
cargo build

# Build GUI
cargo build --features gui

# Run tests
cargo test

# Run CLI
cargo run

# Run GUI
cargo run --features gui

# Lint
cargo clippy
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to:

- Submit Pull Requests for bug fixes and features
- Open Issues for bugs or enhancement requests
- Improve documentation and examples
