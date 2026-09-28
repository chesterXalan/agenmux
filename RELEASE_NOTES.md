# agenmux v0.7.0

## What's changed

v0.7.0 adds collapsible sessions and windows to the sidebar and makes agenmux
a tmux manager first: it opens on every session, window, and pane, with agent
status inline and management actions enabled.

### Collapsible sessions and windows

- Session and split-window headers fold and unfold, so a long tmux tree shrinks to the sessions you care about ([#143](https://github.com/snirt/agenmux/pull/143)).
- `Space` or a click on the selected header toggles it. `h`/`←` folds a header, or steps from a pane to its parent header; `→` unfolds it. `z` folds every branch and `Z` unfolds them all.
- A folded header shows `▶` in the color of the most urgent agent hidden inside it (blocked, then done, then working), so nothing needing attention disappears.
- Fold state survives rescans until the sidebar daemon restarts, and search and the attention filter show every match without changing it.
- Headers use `▼`/`▶` markers instead of a Nerd Font icon, secondary text is easier to read, and prompt previews start with `↳`.

### Tmux manager by default

- The sidebar shows all sessions, windows, and panes by default, and create, rename, and delete actions are enabled; deletes still confirm inline ([#145](https://github.com/snirt/agenmux/pull/145)).
- Set `display.show_all_panes = false` for the agent-only list and `tmux_management.enabled = false` for a read-only sidebar; each works independently.
- The nvim quick launcher moved from `e` to `oe`, so every launcher shares the `o` prefix.

### Fixes

- Unnamed windows keep tmux's automatic name instead of becoming `agenmux` while the sidebar is focused ([#145](https://github.com/snirt/agenmux/pull/145)).
- Loading the plugin no longer stalls `tmux source-file` or tmux startup while the engine downloads or builds ([#145](https://github.com/snirt/agenmux/pull/145)).

### Installer

- The Nerd Font check now asks whether the sample renders and sets `display.agent_label` to `icon` or `text`, keeping an existing value ([#145](https://github.com/snirt/agenmux/pull/145)).
- The installer installs the native engine with a progress indicator before reloading tmux, so the first toggle opens at once ([#145](https://github.com/snirt/agenmux/pull/145)).

### Upgrade notes

- A configuration that binds `o` in `[keys.normal]` is now rejected, because launcher sequences are active by default. Rebind the key or set `tmux_management.enabled = false`.
- To keep the previous agent-only, read-only sidebar, add:

  ```toml
  [display]
  show_all_panes = false
  [tmux_management]
  enabled = false
  ```

### Assets

- Linux x86_64
- Linux aarch64
- macOS x86_64
- macOS aarch64
- SHA-256 checksums
