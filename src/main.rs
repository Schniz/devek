use clap::{CommandFactory, Parser};

#[derive(clap::ValueEnum, Clone, Debug, Default)]
enum Type {
    /// Store the content as plain text
    /// This is the most widely supported format and should work in most apps.
    Text,
    #[default]
    /// Store the content as HTML, which allows for rich text formatting
    /// in many apps. Devek will also generate a plain text alternative
    /// using the `html2text` crate.
    Html,
}

/// Simple program to set HTML clipboard content
///
/// This should be a cross-platform way to set clipboard content from the command line,
/// allowing you to set both text and HTML content.
/// For some reason, `pbcopy` on MacOS doesn't copy RTF text correctly, so this is a workaround for
/// this.
///
/// # Examples
///
/// Set plain text content
///
/// ```
/// devek --type text "Hello, World!"
/// ```
///
/// Set HTML content
///
/// ```
/// devek --type html "<b>Hello, World!</b>"
/// ```
///
/// Pipe content from stdin
///
/// ```
/// node -p '"Hello world".bold()' | devek --type html -
/// ```
#[derive(Parser, Debug)]
struct Cli {
    /// Text content to set in the clipboard
    ///
    /// By default, the content is set as HTML.
    /// Use `--type text` to set as plain text.
    #[clap(long, value_enum, default_value_t)]
    r#type: Type,
    /// HTML content to set in the clipboard.
    /// Use `-` or omit to read from stdin.
    #[clap(default_value = "-")]
    content: String,
    /// Fallback plaintext content for HTML clipboard entries.
    #[clap(long)]
    fallback: Option<String>,
}

fn main() {
    if std::env::args().any(|x| &x == "--kdl-usage") {
        clap_usage::generate(&mut Cli::command(), "devek", &mut std::io::stdout());
        return;
    }

    let cli = Cli::parse();
    let mut clipboard = arboard::Clipboard::new().unwrap();

    let content = if cli.content != "-" {
        cli.content
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer
    };

    match cli.r#type {
        Type::Text => clipboard.set_text(content).unwrap(),
        Type::Html => {
            let fallback = cli
                .fallback
                .or_else(|| html2text::from_read(content.as_bytes(), usize::MAX).ok());
            clipboard.set_html(content, fallback).unwrap()
        }
    }
}
