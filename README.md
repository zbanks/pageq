`pageq`: a newfangled pager
===========================

`pageq` is a pager, more or less like `more` or `less`.

The goal is to make a pager that is _slightly_ more interactive than `less`, so it can be leveraged to make low-effort TUIs.

However, the core functionality should remain very similar to `less` so that `pageq` experts can still get by when they work on systems with only `less`. And conversely, so `less` experts have an easy time picking up `pageq`. This is similar how a `ripgrep` user can fall back on `grep -r`, and a `grep` user can easily pick up `ripgrep` (see also: `vim`/`vi`; `less`/`more`)

### Features in `less` that zbanks uses wants to preserve

- Quitting
- Help (`h`)
- All general motion commands (`hjkl`, `gG`, `np`, etc.)
- Searching (`/`, `nN`; although maybe the `/` vs. `?` distinction isn't necessary?)
- Filtering (`&`, including inverse filters)
- Following (`F`)
- Escape-sequence passthrough (color, etc.)
- Option to chop long lines
- Option to quit if the output fits in 1 screen

### Features in `less` that zbanks thinks are important but doesn't personally use

Some of these have bad UI and are just hard to use, so `pageq` should make them more discoverable.

- Multiple files (UI needs to be easier to use! I don't actually know how this works)
- Shell commands (`!`)
- Saving to a file (`s`) 
- Opening an editor (`v`)
- Input preprocessor (`LESSOPEN`; these should be "first class")
- Configuration file
- Keybinding file
- "Secure" mode (this can be conservative)

### Features in `less` that zbanks doesn't think are important

- Marks (feels like they are only needed for _other_ commands, like `|`)
- `ctags` support (potentially a cool way to make TUIs? probably should use vim instead!)
- Character sets (is only supporting utf-8 & "binary" acceptable in 2020?)
- Super-customizable prompt

### New features zbanks wants to add to `pageq`

Be careful not to put too many features into `pageq` that should stay in the shell. A benefit of a dumb pager is that you end up with a oneliner to reproduce your work at the end.

- Mouse support!
    - On exit, print contents that were highlighted (maybe optional?)
- Emphasize saving to a file
    - Goal is to make it easier to build up pipelines
- "Nonblocking" behavior, particularly when reading from pipes
    - Be able to toggle follow on/off
    - Searching/filtering a large file should show progress
- Folds and/or splits
    - Pin first row
    - Hide column(s)
    - Makes for an okay interface for reading tabular data
- Preprocessors (uncompress, hexdump, grid, colorize)
    - Should be able to apply _after_ launching
- Progressive filtering (multiple regexes)
    - Solving puzzles, add successive filters
