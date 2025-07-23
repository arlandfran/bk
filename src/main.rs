use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use tabled::{
    Table, Tabled,
    settings::{Modify, Panel, Remove, Style, Width, object::Columns},
};

#[derive(Parser, Debug)]
#[command(
    name = "bk",
    version,
    about = "A simple CLI for referencing bash keyboard shortcuts.",
    long_about = "A simple CLI for referencing bash keyboard shortcuts.

Flags can be chained Unix-style:
bk -me shows movement and edit related shortcuts.
Run without flags to show all shortcuts organized by category.",
    after_help = "EXAMPLES:
    bk             Show all shortcuts
    bk -m          Show movement shortcuts only
    bk -me         Show movement and edit shortcuts (chained)
    bk -e -r       Show edit and recall shortcuts (separate)"
)]
struct Args {
    /// Show movement related shortcuts
    #[arg(short, long)]
    movement: bool,

    /// Show edit related shortcuts  
    #[arg(short, long)]
    edit: bool,

    /// Show command recall related shortcuts
    #[arg(short, long)]
    recall: bool,

    /// Show process related shortcuts
    #[arg(short, long)]
    process: bool,

    /// Remove the bk binary from your system
    #[arg(long)]
    uninstall: bool,
}

#[derive(Clone, Tabled)]
struct Shortcut {
    description: &'static str,
    key: &'static str,
}

impl Shortcut {
    fn new(key: &'static str, description: &'static str) -> Self {
        Self { key, description }
    }
}

fn init_shortcuts() -> HashMap<&'static str, Vec<Shortcut>> {
    HashMap::from([
        (
            "Movement",
            vec![
                Shortcut::new("ctrl+a", "Go to line start (home)"),
                Shortcut::new("ctrl+e", "Go to line end (end)"),
                Shortcut::new("ctrl+p", "Previous command (up)"),
                Shortcut::new("ctrl+n", "Next command (down)"),
                Shortcut::new("ctrl+f", "Move forward one char (right)"),
                Shortcut::new("ctrl+b", "Move back one char (left)"),
                Shortcut::new("alt+f", "Move forward one word (alt+right)"),
                Shortcut::new("alt+b", "Move back one word (alt+left)"),
                Shortcut::new("ctrl+xx", "Toggle between line start and cursor"),
            ],
        ),
        (
            "Edit",
            vec![
                Shortcut::new("ctrl+l", "Clear screen"),
                Shortcut::new("alt+del", "Delete word before cursor"),
                Shortcut::new("alt+d", "Delete word after cursor"),
                Shortcut::new("ctrl+d", "Delete char under cursor"),
                Shortcut::new("ctrl+h", "Delete char before cursor (backspace)"),
                Shortcut::new("ctrl+w", "Cut word before cursor to clipboard"),
                Shortcut::new("ctrl+k", "Cut line after cursor to clipboard"),
                Shortcut::new("ctrl+u", "Cut line before cursor to clipboard"),
                Shortcut::new("alt+t", "Swap current word with previous"),
                Shortcut::new("ctrl+t", "Swap last two chars before cursor"),
                Shortcut::new("esc+t", "Swap last two words before cursor"),
                Shortcut::new("ctrl+y", "Paste from clipboard (yank)"),
                Shortcut::new("alt+u", "UPPERCASE word from cursor"),
                Shortcut::new("alt+l", "lowercase word from cursor"),
                Shortcut::new("alt+c", "Capitalize char and move to word end"),
                Shortcut::new("alt+r", "Revert line to history version"),
                Shortcut::new("ctrl+_", "Undo"),
                Shortcut::new("tab", "Auto-complete file/directory names"),
            ],
        ),
        (
            "Recall",
            vec![
                Shortcut::new("ctrl+r", "Search command history as you type"),
                Shortcut::new("ctrl+p", "Previous command in history (walk back)"),
                Shortcut::new("ctrl+n", "Next command in history (walk forward)"),
                Shortcut::new("ctrl+s", "Go back to the next most recent command"),
                Shortcut::new("ctrl+o", "Execute the command found via ctrl+r or ctrl+s"),
                Shortcut::new("ctrl+g", "Escape from history searching mode"),
                Shortcut::new("!!", "Repeat last command"),
                Shortcut::new(
                    "!n",
                    "Repeat nth arg from last command (!:2 for second arg)",
                ),
                Shortcut::new("!n:m", "Repeat args n to m from last command (!:2-3)"),
                Shortcut::new(
                    "!n:$",
                    "Repeat from the last command: args n to the last argument",
                ),
                Shortcut::new("!n:p", "Print last command starting with n"),
                Shortcut::new("!string", "Print the last command beginning with string"),
                Shortcut::new(
                    "!:q",
                    "Quote the last command with proper Bash escaping applied",
                ),
                Shortcut::new("!$", "Last argument of previous command"),
                Shortcut::new("alt+.", "Last argument of previous command"),
                Shortcut::new("!*", "All arguments of previous command"),
                Shortcut::new("^abc^def", "Run previous command, replacing abc with def"),
            ],
        ),
        (
            "Process",
            vec![
                Shortcut::new("ctrl+c", "Kill/Interrupt current process (SIGINT)"),
                Shortcut::new("ctrl+s", "Stop screen output (scroll with PgUp/PgDn)"),
                Shortcut::new("ctrl+q", "Resume screen output (after ctrl+s)"),
                Shortcut::new("ctrl+d", "Send EOF - closes shell if empty (EXIT)"),
                Shortcut::new("ctrl+z", "Suspend process (SIGTSTP) - resume: fg"),
            ],
        ),
    ])
}

fn format_table(shortcuts: &[Shortcut], category: &str) -> String {
    Table::new(shortcuts)
        .with(Modify::new(Columns::first()).with(Width::increase(57)))
        .with(Style::blank())
        .with(Panel::header(format!("{} related shortcuts", category)))
        .with(Remove::row(tabled::settings::object::Rows::one(1)))
        .to_string()
}

fn build_output(args: &Args, shortcuts: &HashMap<&str, Vec<Shortcut>>) -> String {
    let categories = [
        ("Movement", args.movement),
        ("Edit", args.edit),
        ("Recall", args.recall),
        ("Process", args.process),
    ];

    let show_all = categories.iter().all(|(_, flag)| !*flag);
    let mut output = String::new();

    for (category, flag) in categories {
        if show_all || flag {
            if let Some(shortcuts) = shortcuts.get(category) {
                output.push_str(&format_table(shortcuts, category));
                output.push_str("\n\n");
            }
        }
    }

    output
}

fn handle_uninstall() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;

    print!("Remove 'bk' from {}? (y/N): ", exe_path.display());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        fs::remove_file(&exe_path)?;
        println!("✓ Successfully uninstalled bk. Thank you for using it!");
    } else {
        println!("Uninstall cancelled.");
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    if args.uninstall {
        if let Err(e) = handle_uninstall() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let shortcuts = init_shortcuts();
    print!("{}", build_output(&args, &shortcuts));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_creation() {
        let shortcut = Shortcut::new("Ctrl+a", "Go to beginning");
        assert_eq!(shortcut.key, "Ctrl+a");
        assert_eq!(shortcut.description, "Go to beginning");
    }

    #[test]
    fn test_all_categories_present() {
        let shortcuts = init_shortcuts();
        let expected = ["Movement", "Edit", "Recall", "Process"];

        for category in expected {
            assert!(shortcuts.contains_key(category));
            assert!(!shortcuts[category].is_empty());
        }
    }

    #[test]
    fn test_show_all_shortcuts() {
        let shortcuts = init_shortcuts();
        let args = Args {
            movement: false,
            edit: false,
            recall: false,
            process: false,
            uninstall: false,
        };

        let output = build_output(&args, &shortcuts);

        assert!(output.contains("Movement related shortcuts"));
        assert!(output.contains("Edit related shortcuts"));
        assert!(output.contains("Recall related shortcuts"));
        assert!(output.contains("Process related shortcuts"));
    }

    #[test]
    fn test_single_category() {
        let shortcuts = init_shortcuts();
        let args = Args {
            movement: true,
            edit: false,
            recall: false,
            process: false,
            uninstall: false,
        };

        let output = build_output(&args, &shortcuts);

        assert!(output.contains("Movement related shortcuts"));
        assert!(!output.contains("Edit related shortcuts"));
    }

    #[test]
    fn test_multiple_categories() {
        let shortcuts = init_shortcuts();
        let args = Args {
            movement: true,
            edit: true,
            recall: false,
            process: false,
            uninstall: false,
        };

        let output = build_output(&args, &shortcuts);

        assert!(output.contains("Movement related shortcuts"));
        assert!(output.contains("Edit related shortcuts"));
        assert!(!output.contains("Recall related shortcuts"));
    }
}
