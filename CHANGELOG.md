# Changelog

## 0.1.2

### Patch Changes

- [`e25a220`](https://github.com/lukehsiao/tool/commit/e25a2201f57e2dbe342ee0f9827749471343cce1) - **fix**: `wifi-qr` now escapes `"` in the SSID and password, as the WiFi QR format requires, so scanners read credentials containing quotes exactly as typed.

<pre>
$ git-stats v0.1.1..v0.1.2
Author           Commits  Changed Files  Insertions  Deletions  Net Δ
dependabot[bot]       12             12         +22        -22      0
Luke Hsiao             8             20        +483      -1053   -570
Total                 20             32        +505      -1075   -570
</pre>

## 0.1.1

### Patch Changes

- [`6216b6a`](https://github.com/lukehsiao/tool/commit/6216b6a31f21b5b452c2818595aced25fff3bd91) - [Internal] Switch release infrastructure from bespoke `git-cliff` scripts to [`changesets`](https://github.com/changesets/changesets).

<pre>
$ git-stats v0.1.0..v0.1.1
Author           Commits  Changed Files  Insertions  Deletions  Net Δ
Luke Hsiao            35            103       +4759      -3123  +1636
dependabot[bot]       21             38        +146       -152     -6
Total                 56            141       +4905      -3275  +1630
</pre>

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
