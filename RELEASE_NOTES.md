# agenmux v0.7.0

## What's changed

v0.7.0 makes agenmux a tmux manager first: the sidebar opens on every session,
window, and pane, with agent status inline and management actions enabled.

### Tmux manager by default

- The sidebar shows all sessions, windows, and panes by default, and create, rename, and delete actions are enabled; deletes still confirm inline ([#145](https://github.com/snirt/agenmux/pull/145)).
- Set `display.show_all_panes = false` for the agent-only list and `tmux_management.enabled = false` for a read-only sidebar; each works independently ([#145](https://github.com/snirt/agenmux/pull/145)).
- The nvim quick launcher moved from `e` to `oe`, so every launcher shares the `o` prefix ([#145](https://github.com/snirt/agenmux/pull/145)).
- Session and split-window headers are collapsible branches, with `▼`/`▶` markers that tint to the most urgent hidden agent, and secondary text is easier to read ([#143](https://github.com/snirt/agenmux/pull/143)).

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
