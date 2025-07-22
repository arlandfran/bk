use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[command(
    name = "bk",
    version,
    about = "A CLI for referencing Bash keyboard shortcuts.",
    long_about = "A CLI for referencing Bash keyboard shortcuts.

Flags can be chained Unix-style: bk -me shows movement and edit shortcuts.
Run without flags to show all shortcuts organized by category.",
    after_help = "EXAMPLES:
    bk           Show all shortcuts
    bk -m        Show movement shortcuts only
    bk -me       Show movement and edit shortcuts (chained)
    bk -e -r     Show edit and recall shortcuts (separate)
    bk --version Show version information"
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
}

/// Structure to hold a keyboard shortcut with its key combination and description
#[derive(Clone)]
struct Shortcut {
    key: &'static str,
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
        "movement",
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
        "edit",
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
    shortcuts.insert("history", vec![
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
    shortcuts.insert("process", vec![
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
    println!("=== {} Shortcuts ===", category.to_uppercase());
    for shortcut in shortcuts {
        println!("  {:12} {}", shortcut.key, shortcut.description);
    }
    println!();
}

/// Display all shortcuts organized by category
fn display_all_shortcuts(shortcuts_map: &HashMap<&str, Vec<Shortcut>>) {
    // Define the order we want to display categories
    let categories = ["movement", "edit", "history", "process"];

    for &category in &categories {
        if let Some(shortcuts) = shortcuts_map.get(category) {
            display_shortcuts(shortcuts, category);
        }
    }
}

fn main() {
    let args = Args::parse();

    // Initialize the shortcuts data structure
    let shortcuts_map = init_shortcuts();

    // If no specific flags are provided, show all shortcuts
    if !args.movement && !args.edit && !args.recall && !args.process {
        display_all_shortcuts(&shortcuts_map);
        return;
    }

    // Display shortcuts based on the flags provided
    if args.movement {
        if let Some(shortcuts) = shortcuts_map.get("movement") {
            display_shortcuts(shortcuts, "movement");
        }
    }

    if args.edit {
        if let Some(shortcuts) = shortcuts_map.get("edit") {
            display_shortcuts(shortcuts, "edit");
        }
    }

    if args.recall {
        if let Some(shortcuts) = shortcuts_map.get("history") {
            display_shortcuts(shortcuts, "recall");
        }
    }

    if args.process {
        if let Some(shortcuts) = shortcuts_map.get("process") {
            display_shortcuts(shortcuts, "process");
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
}
