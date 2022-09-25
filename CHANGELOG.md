## 0.1.0 - 2022-09-25

### Documentation
- (README) Add basic readme
- (plain_photos) Add docstring to `run`

### Features
- (pdfembed) Add script for embedding PDF fonts
- (pdfcrop) Add `pdf-crop` subcommand
- (gitemail) Add basic setup for git-send-email workflows
- (semver) Add `semver` subcommand

### Miscellaneous Tasks
- Add helpers for releases

### Refactor
- Refine help message, move basename to option
- Split commands into separate files
- (plain_photos) Remove unnecessary fn call
- Move struct defs to submodules
- Change how paths are manipulated
- Do not require the arg to be a specific size
- (gitemail) Default prefix to repo name
