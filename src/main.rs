use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tabled::{
    Table, Tabled,
    settings::{
        Modify, Panel, Remove, Style, location::ByColumnName, object::Rows,
        themes::BorderCorrection, width::Wrap,
    },
};

#[derive(Parser, Debug)]
#[command(
    name = "bk",
    version,
    about = "A CLI for referencing Bash keyboard shortcuts.",
    long_about = "A CLI for referencing Bash keyboard shortcuts.

Flags can be chained Unix-style: bk -me shows movement and edit shortcuts.
Run without flags to show all shortcuts organized by category.",
    after_help = "EXAMPLES:
    bk             Show all shortcuts
    bk -m          Show movement shortcuts only
    bk -me         Show movement and edit shortcuts (chained)
    bk -e -r       Show edit and recall shortcuts (separate)
    bk --uninstall Uninstall the bk CLI
    bk --version   Show version information"
)]

struct Args {
    /// Show movement related shortcuts
    #[arg(short, long)]
    movement: bool,

    /// Show edit related shortcuts
    #[arg(short, long)]
    edit: bool,

    /// Show command recall (history) related shortcuts
    #[arg(short, long)]
    recall: bool,

    /// Show process related shortcuts
    #[arg(short, long)]
    process: bool,

    /// Uninstall the bk CLI
    #[arg(long)]
    uninstall: bool,
}

/// Structure to hold a keyboard shortcut with its key combination and description
#[derive(Clone, Tabled)]
struct Shortcut {
    #[tabled(rename = "Shortcut", order = 1)]
    key: &'static str,
    #[tabled(rename = "Description", order = 0)]
    description: &'static str,
}

impl Shortcut {
    fn new(key: &'static str, description: &'static str) -> Self {
        Self { key, description }
    }
}

/// Initialize all keyboard shortcuts organized by category
fn init_shortcuts() -> HashMap<&'static str, Vec<Shortcut>> {
    let mut shortcuts = HashMap::new();

    // Movement shortcuts - cursor navigation and positioning
    shortcuts.insert(
        "Movement",
        vec![
            Shortcut::new("Ctrl+a", "Go to the beginning of the line (Home)"),
            Shortcut::new("Ctrl+e", "Go to the End of the line (End)"),
            Shortcut::new("Ctrl+f", "Forward one character (Right arrow)"),
            Shortcut::new("Ctrl+b", "Backward one character (Left arrow)"),
            Shortcut::new("Alt+f", "Forward (right) one word (Alt-Right arrow)"),
            Shortcut::new("Alt+b", "Back (left) one word (Alt-Left arrow)"),
            Shortcut::new(
                "Ctrl+xx",
                "Toggle between the start of line and current cursor position",
            ),
        ],
    );

    // Edit shortcuts - text editing and manipulation
    shortcuts.insert(
        "Edit",
        vec![
            Shortcut::new("Ctrl+l", "Clear the Screen, similar to the clear command"),
            Shortcut::new("Alt+Del", "Delete the Word before the cursor"),
            Shortcut::new("Alt+d", "Delete the Word after the cursor"),
            Shortcut::new("Ctrl+d", "Delete character under the cursor"),
            Shortcut::new("Ctrl+h", "Delete character before the cursor (Backspace)"),
            Shortcut::new("Ctrl+w", "Cut the Word before the cursor to the clipboard"),
            Shortcut::new("Ctrl+k", "Cut the Line after the cursor to the clipboard"),
            Shortcut::new(
                "Ctrl+u",
                "Cut/delete the Line before the cursor to the clipboard",
            ),
            Shortcut::new("Alt+t", "Swap current word with previous"),
            Shortcut::new(
                "Ctrl+t",
                "Swap the last two characters before the cursor (typo)",
            ),
            Shortcut::new("Esc+t", "Swap the last two words before the cursor"),
            Shortcut::new("Ctrl+y", "Paste the last thing to be cut (yank)"),
            Shortcut::new(
                "Alt+u",
                "UPPER capitalize every character from the cursor to the end of the current word",
            ),
            Shortcut::new(
                "Alt+l",
                "Lower the case of every character from the cursor to the end of the current word",
            ),
            Shortcut::new(
                "Alt+c",
                "Capitalize the character under the cursor and move to the end of the word",
            ),
            Shortcut::new(
                "Alt+r",
                "Cancel the changes and put back the line as it was in the history (revert)",
            ),
            Shortcut::new("Ctrl+_", "Undo"),
            Shortcut::new("Tab", "Tab completion for file/directory names"),
        ],
    );

    // History shortcuts - command history navigation and search
    shortcuts.insert("History", vec![
        Shortcut::new("Ctrl+r", "Recall the last command including the specified character(s). Search the command history as you type"),
        Shortcut::new("Ctrl+p", "Previous command in history (walk back)"),
        Shortcut::new("Ctrl+n", "Next command in history (walk forward)"),
        Shortcut::new("Ctrl+s", "Go back to the next most recent command"),
        Shortcut::new("Ctrl+o", "Execute the command found via Ctrl+r or Ctrl+s"),
        Shortcut::new("Ctrl+g", "Escape from history searching mode"),
        Shortcut::new("!!", "Repeat last command"),
        Shortcut::new("!n", "Repeat from the last command: args n e.g. !:2 for the second argument"),
        Shortcut::new("!n:m", "Repeat from the last command: args from n to m. e.g. !:2-3 for the second and third"),
        Shortcut::new("!n:$", "Repeat from the last command: args n to the last argument"),
        Shortcut::new("!n:p", "Print last command starting with n"),
        Shortcut::new("!string", "Print the last command beginning with string"),
        Shortcut::new("!:q", "Quote the last command with proper Bash escaping applied"),
        Shortcut::new("!$", "Last argument of previous command"),
        Shortcut::new("Alt+.", "Last argument of previous command"),
        Shortcut::new("!*", "All arguments of previous command"),
        Shortcut::new("^abc^def", "Run previous command, replacing abc with def"),
    ]);

    // Process shortcuts - process control and management
    shortcuts.insert("Process", vec![
        Shortcut::new("Ctrl+c", "Interrupt/Kill whatever you are running (SIGINT)"),
        Shortcut::new("Ctrl+l", "Clear the screen"),
        Shortcut::new("Ctrl+s", "Stop output to the screen (for long running verbose commands). Then use PgUp/PgDn for navigation"),
        Shortcut::new("Ctrl+q", "Allow output to the screen (if previously stopped using command above)"),
        Shortcut::new("Ctrl+d", "Send an EOF marker, unless disabled by an option, this will close the current shell (EXIT)"),
        Shortcut::new("Ctrl+z", "Send the signal SIGTSTP to the current task, which suspends it. To return to it later enter 'fg process name' (foreground)"),
    ]);

    shortcuts
}

/// Format and display shortcuts for a given category
fn display_shortcuts(shortcuts: &[Shortcut], category: &str) {
    let mut binding = Table::new(shortcuts);
    let table = binding
        .with(Style::blank())
        .with(Modify::new(ByColumnName::new("Description")).with(Wrap::new(60).keep_words(true)))
        .with(Panel::header(format!("{} related shortcuts", category)))
        .with(BorderCorrection::span())
        .with(Remove::row(Rows::one(1)));

    println!("{}\n", table);
}

/// Display all shortcuts organized by category
fn display_all_shortcuts(shortcuts_map: &HashMap<&str, Vec<Shortcut>>) {
    // Define the order we want to display categories
    let categories = ["Movement", "Edit", "History", "Process"];

    for &category in &categories {
        if let Some(shortcuts) = shortcuts_map.get(category) {
            display_shortcuts(shortcuts, category);
        }
    }
}

/// Get the path to the current executable
fn get_current_exe_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e).into())
}

/// Handle the uninstall command
fn handle_uninstall() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = get_current_exe_path()?;

    // Confirm with user before deletion
    println!("This will permanently delete the 'bk' binary from:");
    println!("  {}", exe_path.display());
    println!();
    print!("Are you sure you want to uninstall? (y/N): ");

    use std::io::{self, Write};
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    if input == "y" || input == "yes" {
        match fs::remove_file(&exe_path) {
            Ok(()) => {
                println!(
                    "✓ Successfully uninstalled 'bk' from {}",
                    exe_path.display()
                );
                println!("Thank you for using bk!");
            }
            Err(e) => {
                eprintln!("✗ Failed to remove binary: {}", e);
                eprintln!("You may need to run with elevated privileges or remove it manually.");
                return Err(e.into());
            }
        }
    } else {
        println!("Uninstall cancelled.");
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    // Handle uninstall first, as it's a special action
    if args.uninstall {
        if let Err(e) = handle_uninstall() {
            eprintln!("Error during uninstall: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Initialize the shortcuts data structure
    let shortcuts_map = init_shortcuts();

    // If no specific flags are provided, show all shortcuts
    if !args.movement && !args.edit && !args.recall && !args.process {
        display_all_shortcuts(&shortcuts_map);
        return;
    }

    // Display shortcuts based on the flags provided
    if args.movement {
        if let Some(shortcuts) = shortcuts_map.get("Movement") {
            display_shortcuts(shortcuts, "Movement");
        }
    }

    if args.edit {
        if let Some(shortcuts) = shortcuts_map.get("Edit") {
            display_shortcuts(shortcuts, "Edit");
        }
    }

    if args.recall {
        if let Some(shortcuts) = shortcuts_map.get("History") {
            display_shortcuts(shortcuts, "Recall");
        }
    }

    if args.process {
        if let Some(shortcuts) = shortcuts_map.get("Process") {
            display_shortcuts(shortcuts, "Process");
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_creation() {
        let shortcut = Shortcut::new("Ctrl+a", "Go to beginning of line");
        assert_eq!(shortcut.key, "Ctrl+a");
        assert_eq!(shortcut.description, "Go to beginning of line");
    }

    #[test]
    fn test_shortcuts_initialization() {
        let shortcuts = init_shortcuts();

        // Verify all categories are present
        assert!(shortcuts.contains_key("movement"));
        assert!(shortcuts.contains_key("edit"));
        assert!(shortcuts.contains_key("history"));
        assert!(shortcuts.contains_key("process"));

        // Verify each category has shortcuts
        assert!(!shortcuts.get("movement").unwrap().is_empty());
        assert!(!shortcuts.get("edit").unwrap().is_empty());
        assert!(!shortcuts.get("history").unwrap().is_empty());
        assert!(!shortcuts.get("process").unwrap().is_empty());
    }

    #[test]
    fn test_args_parsing() {
        use clap::Parser;

        // Test parsing with no arguments
        let args = Args::try_parse_from(&["bk"]).unwrap();
        assert!(!args.movement && !args.edit && !args.recall && !args.process);

        // Test parsing with single flag
        let args = Args::try_parse_from(&["bk", "-m"]).unwrap();
        assert!(args.movement && !args.edit && !args.recall && !args.process);

        // Test parsing with multiple flags
        let args = Args::try_parse_from(&["bk", "-me"]).unwrap();
        assert!(args.movement && args.edit && !args.recall && !args.process);

        // Test parsing uninstall flag
        let args = Args::try_parse_from(&["bk", "--uninstall"]).unwrap();
        assert!(!args.movement && !args.edit && !args.recall && !args.process && args.uninstall);
    }

    #[test]
    fn test_invalid_args() {
        use clap::Parser;

        // Test unknown short flag
        let result = Args::try_parse_from(&["bk", "-x"]);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::UnknownArgument);

        // Test unknown long flag
        let result = Args::try_parse_from(&["bk", "--unknown"]);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::UnknownArgument);

        // Test invalid flag combination (not really invalid in this case, but test malformed flag)
        let result = Args::try_parse_from(&["bk", "--"]);
        assert!(result.is_ok()); // -- is valid (end of options marker)

        // Test invalid argument with value (our flags don't take values)
        let result = Args::try_parse_from(&["bk", "--movement=true"]);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::TooManyValues);

        // Test typo in long flag
        let result = Args::try_parse_from(&["bk", "--movment"]); // missing 'e'
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::UnknownArgument);

        // Test positional arguments (which we don't accept)
        let result = Args::try_parse_from(&["bk", "extra_arg"]);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::UnknownArgument);

        // Test uninstall with value
        let result = Args::try_parse_from(&["bk", "--uninstall=true"]);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind(), clap::error::ErrorKind::TooManyValues);
    }

    #[test]
    fn test_get_current_exe_path() {
        // This should not panic and should return a valid path during testing
        let result = get_current_exe_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists() || path.to_string_lossy().contains("test")); // During tests, the path might be a test runner
    }

    #[test]
    fn test_uninstall_flag_combination() {
        use clap::Parser;

        // Test that uninstall can be combined with other flags (though uninstall takes precedence)
        let args = Args::try_parse_from(&["bk", "--uninstall", "-m"]).unwrap();
        assert!(args.uninstall && args.movement);
    }
}
