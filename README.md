# `devek`

Simple program to set HTML clipboard content

This should be a cross-platform way to set clipboard content from the command line, allowing you to set both text and HTML content. For some reason, `pbcopy` on MacOS doesn't copy RTF text correctly, so this is a workaround for this.

# Examples

Set plain text content

``` devek --type text "Hello, World!" ```

Set HTML content

``` devek --type html "<b>Hello, World!</b>" ```

Pipe content from stdin

``` node -p '"Hello world".bold()' | devek --type html - ```


- **Usage**: `devek [--type <TYPE>] [--fallback <FALLBACK>] [CONTENT]`

## Arguments

### `[CONTENT]`

HTML content to set in the clipboard. Use `-` or omit to read from stdin

**Default:** `-`

## Flags

### `--type <TYPE>`

Text content to set in the clipboard

By default, the content is set as HTML. Use `--type text` to set as plain text.

**Choices:**

- `text`
- `html`

### `--fallback <FALLBACK>`

Fallback plaintext content for HTML clipboard entries
