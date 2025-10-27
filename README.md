# CC Session Manager

A CLI tool to manage Claude Code sessions, allowing you to view statistics, delete old sessions, and clean up projects efficiently.

## Features

- **📊 Session Statistics**: View detailed statistics about your Claude Code sessions including total count, storage usage, and age distribution
- **🗂️ Project Management**: Browse projects and selectively delete individual sessions or entire projects
- **🕐 Age-based Cleanup**: Automatically find and delete sessions older than a specified number of days
- **🎯 Interactive UI**: User-friendly terminal interface with intuitive menu navigation and selection
- **⚡ Fast Performance**: Efficient scanning and management of large session collections
- **🔒 Safe Operations**: Confirmation prompts for destructive operations to prevent accidental data loss

## Installation

### Prerequisites

- Rust 1.70+ installed
- Claude Code installed on your system

### From Source

```bash
git clone https://github.com/petch-x/cc-session-manager.git
cd cc-session-manager
cargo build --release
```

The binary will be available at `target/release/cc-session-manager`.

### Using Cargo (Recommended)

```bash
cargo install --path .
```

This will install the binary globally in your Cargo bin directory.

## Usage

Run the application:

```bash
cargo run
```

Or if installed:

```bash
cc-session-manager
```

### Menu Options

1. **📊 View Statistics** - Shows comprehensive statistics about your Claude Code sessions including:
   - Total number of projects and sessions
   - Storage usage breakdown
   - Session age distribution
   - Largest projects by session count

2. **🗂️ Manage Projects** - Browse through all projects and:
   - View session details with timestamps
   - Selectively delete individual sessions
   - Sort sessions by date or name
   - Preview session metadata before deletion

3. **🕐 Delete by Age** - Smart cleanup feature:
   - Specify age threshold in days
   - Preview all sessions matching criteria
   - Selectively delete from the filtered list
   - Batch delete with confirmation

4. **🗑️ Delete Project** - Complete project removal:
   - Remove entire project directories
   - Delete all associated sessions
   - Confirmation prompt to prevent accidents
   - Immediate feedback on deletion status

5. **🚪 Exit** - Clean exit from the application

### Keyboard Navigation

- Use arrow keys to navigate menus
- Press Enter to select options
- Use Space to toggle selections in multi-select mode
- Press Esc to cancel operations or go back

## Requirements

- **Rust 1.70+** - Modern Rust toolchain with 2021 edition
- **Claude Code** - Must be installed (the tool automatically detects the Claude directory)
- **Terminal** - Compatible terminal application (macOS Terminal, iTerm2, Windows Terminal, etc.)

## Dependencies

- `serde` - JSON serialization/deserialization for session metadata
- `tokio` - Async runtime for efficient file operations
- `clap` - Command line argument parsing and help generation
- `crossterm` - Cross-platform terminal manipulation and UI
- `chrono` - Date/time handling with serialization support
- `anyhow` - Ergonomic error handling and propagation
- `dirs` - Cross-platform directory path resolution

## How It Works

The tool automatically locates your Claude Code installation directory and scans for:

- Project folders containing session data
- Session metadata files with timestamps and usage information
- Storage usage calculations for cleanup decisions

All operations are performed safely with confirmation prompts for destructive actions.

## Troubleshooting

### "Claude directory not found"
- Ensure Claude Code is properly installed
- Check that Claude has been run at least once to create the session directory
- Verify the tool has permission to access your home directory

### Performance Issues
- For large numbers of sessions, the initial scan may take a few seconds
- Consider using the "Delete by Age" feature for bulk cleanup operations

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to:

- Submit Pull Requests for bug fixes and features
- Open Issues for bugs or enhancement requests
- Improve documentation and examples

### Development Setup

```bash
git clone https://github.com/petch-x/cc-session-manager.git
cd cc-session-manager
cargo test
cargo run
```