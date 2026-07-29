# catwalk
Catwalk is a command-line tool for exporting a repository into a structured, portable format while preserving directory layout and source file context.

It collected project structure, filters unnecessary generated files, detects file types, and produces a single organized output that can be used for code review, documentation, archival, or analysis workflows.

## Features

- Generate a complete repository tree
- Export text files with explicit file boundaries
- Preserve repository-relative file paths
- Detect file types from extensions and filenames
- Ignore common generated files and directories
- Write output to stdout or a file
- Support custom exclusion rules
- Handle unreadable or binary files safely

## Installation

Clone the repository:

```bash
git clone https://github.com/Ashintosh/catwalk.git
cd catwalk
```

Build with Cargo:

```bash
cargo build --release
```

The compiled binary will be available at:

```target/release/catwalk```

## Usage

```bash
# Export the current directory
catwalk.

# Export a specific directory
catwalk ./my-project

# Write output to a file
catwalk ./my-project --output export.txt

# Skip the directory tree
catwalk ./my-project --exclude vendor --exclude generated
```

## Output Format

Catwalk produces a structured export containing:

- Repository directory tree
- File path and type
- File contents

Example:

```xml
<PROJECT_TREE>
├── app
│   ├── config.rs
│   ├── mod.rs
│   └── run.rs
└── cli
    ├── args.rs
    ├── mod.rs
    └── validate.rs
</PROJECT_TREE>
<FILE path="app/config.rs" type="rust">
<CONTENT>
use crate::{cli::Args, model::Config};

pub fn from_args(args: Args) -> Config {
    Config {
        root: args.path,
        exclude: args.exclude,
        follow_symlinks: false,
        print_tree: !args.no_tree,
        output: args.output,
    }
}

</CONTENT>
</FILE>

...
```

## Ignored Files

Catwalk avoids exporting common generated or dependency directories by default.

Default ignored include:

- `.git`
- `.svn`
- `.hd`
- `node_modules`
- `target`
- `__pycache__`
- `.venv`
- `venv`

Additional exclusions can be supplied with:

```bash
--exclude <directory>
```

## File Handling

Text files are exported with their contents preserved.

Binary files and files that cannot be decoded as UTF-8 are skipped automatically.

## Project Structure
```
src/
├── app/
│   └── Application execution flow
├── cli/
│   └── Command-line parsing and validation
├── error/
│   └── Application error types
├── export/
│   └── Output formatting and writers
├── filetype/
│   └── File type detection
├── fs/
│   └── Filesystem traversal and filtering
├── model/
│   └── Shared application data structures
├── lib.rs
└── main.rs
```

## TODO

Planned improvements:

- Add ID number to `<FILE>` attributes for easy searching/referencing
- Add support for configurable ignore rules
- Improve file type detection for files without extensions
- Add support for additional output formats
- Add configurable export sections (tree, files, metadata)
- Add tests covering filesystem traversal and export formatting

## Known Issues / Quirks

- Files that cannot be decoded as UTF-8 are skipped during export
- Default ignore rules are intentionally conservative and may require additional exclusions for some projects
- The export format is currently versioned buy may change before a stable format is finalized
- Symbolic link handling is limited and may change in future versions
- Large repositories may produce very large output files

## License

Catwalk is licensed under MIT.
See the [LICENSE](LICENSE) file for details.