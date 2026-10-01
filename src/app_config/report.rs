//! `agenmux config` help, validation, and effective-settings output.

use super::*;

/// Lay names out in aligned columns so a long list stays readable in a pane
/// narrower than the list itself.
fn columns(names: &[&str], per_row: usize, indent: &str) -> String {
    let width = names.iter().map(|name| name.len()).max().unwrap_or(0);
    names
        .chunks(per_row)
        .map(|row| {
            let cells: Vec<String> = row.iter().map(|name| format!("{name:width$}")).collect();
            format!("{indent}{}", cells.join("  ").trim_end())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every configurable field, printed from one place so `--help` cannot drift
/// from the file the loader accepts.
pub fn help() -> i32 {
    let normal = |mode| {
        resolved_keys(mode, None)
            .unwrap_or_default()
            .into_keys()
            .map(|action| format!("{action:?}").to_lowercase())
            .collect::<Vec<_>>()
            .join(", ")
    };
    println!(
        r##"agenmux application configuration

  file      $XDG_CONFIG_HOME/agenmux/config.toml
            otherwise $HOME/.config/agenmux/config.toml
  commands  agenmux config check [--effective]   validate, and report sources
            agenmux config reload                apply an edited file

Every key is optional and omitting one keeps its default. A present @agenmux-*
tmux option still wins over the file.

[display]
  mode            split | popup                       (split)
  show_all_panes  true | false                        (true)
  show_frame      true | false                        (true)
  sidebar_width   1..=10000 cells                     (30)
  popup_width     1..=10000 cells                     (40)
  popup_height    "auto" or 1..=10000 cells           (auto)
  agent_label     icon-text | icon | text             (icon-text)

[behavior]
  notifications   true | false                        (true)
  hide_windows    glob for the prefix+w picker        (unset: picker untouched)

[tmux_management]
  enabled         true | false                        (true)
  confirm_delete  true | false                        (true)

[quick_launchers.<id>]
  sequence        one or two ASCII letters or digits
  label           printable help text                  (required for custom IDs)
  command         executable name or path               (required for custom IDs)
  args             array of arguments                   ([])
  working_directory selected | tmux session default    (selected)
  enabled         true | false                         (true)
  Defaults: nvim uses e; lazygit uses og. Entries with those IDs override
  their defaults. Set enabled = false to remove a binding. Custom launchers
  require sequence, label, and command. All launchers require tmux management.
  Conflicting active keys are rejected. These are independent of the tmux
  options that open Agenmux itself (@agenmux-key and @agenmux-popup-key).

[theme]
  base            dark | light | terminal             (dark)

[theme.colors]    "default", 0..=255, or "#RRGGBB"; the base fills the rest
{}

[keys]
  sequence_timeout_ms  positive integer milliseconds             (1000)

[keys.normal]     {}
[keys.search]     {}
  gg and G jump to the first and last visible agent. They are not configurable;
  binding one of those keys to an action replaces that jump.
  Each value replaces that action's default list, and [] unbinds it. Chords are
  one printable ASCII character, Space, Up/Down/Left/Right, Home/End,
  PageUp/PageDown, Enter, Escape, Tab, BSpace, or a C- chord. Reserved:
  C-c and C-d always exit, C-@/C-a/C-b/C-l carry the sidebar's own key
  packets, and C-h/C-j cannot be told apart from BSpace and Enter.
  Printable chords cannot be bound in search mode, where typing owns them."##,
        columns(&Palette::default().roles().map(|(name, _)| name), 3, "  ",),
        normal(KeyMode::Normal),
        normal(KeyMode::Search),
    );
    0
}

/// One resolved setting as `check --effective` reports it.
pub struct Row {
    pub name: String,
    pub value: String,
    pub source: String,
}

/// Every setting with its resolved value and the layer that decided it, in the
/// order `--help` documents rather than alphabetically, so the report reads
/// like the file it describes.
pub fn rows(config: &AppConfig) -> Vec<Row> {
    let chords = |list: &Vec<KeyChord>| {
        if list.is_empty() {
            "(unbound)".to_string()
        } else {
            list.iter()
                .map(|chord| chord.tmux_name())
                .collect::<Vec<_>>()
                .join(", ")
        }
    };
    let mut out: Vec<(String, String)> = vec![
        (
            "display.mode".into(),
            match config.mode {
                DisplayMode::Split => "split".into(),
                DisplayMode::Popup => "popup".to_string(),
            },
        ),
        (
            "display.show_all_panes".into(),
            config.show_all_panes.to_string(),
        ),
        ("display.show_frame".into(), config.show_frame.to_string()),
        (
            "display.sidebar_width".into(),
            config.sidebar_width.to_string(),
        ),
        ("display.popup_width".into(), config.popup_width.to_string()),
        (
            "display.popup_height".into(),
            match config.popup_height {
                PopupHeight::Auto(_) => "auto".into(),
                PopupHeight::Cells(n) => n.to_string(),
            },
        ),
        (
            "display.agent_label".into(),
            match config.agent_label {
                AgentLabel::IconText => "icon-text",
                AgentLabel::Icon => "icon",
                AgentLabel::Text => "text",
            }
            .into(),
        ),
        (
            "behavior.notifications".into(),
            config.notifications.to_string(),
        ),
        (
            "behavior.hide_windows".into(),
            // Validated, but still user text: escape before it reaches a terminal.
            config
                .hide_windows
                .as_deref()
                .map_or("(unset)".to_string(), escaped),
        ),
        (
            "tmux_management.enabled".into(),
            config.tmux_management_enabled.to_string(),
        ),
        (
            "tmux_management.confirm_delete".into(),
            config.tmux_management_confirm_delete.to_string(),
        ),
        (
            "theme.base".into(),
            match config.theme.base.unwrap_or(ThemeBase::Dark) {
                ThemeBase::Dark => "dark".into(),
                ThemeBase::Light => "light".into(),
                ThemeBase::Terminal => "terminal".to_string(),
            },
        ),
    ];
    for (role, ink) in Palette::resolve(&config.theme).roles() {
        out.push((format!("theme.colors.{role}"), ink.describe()));
    }
    out.push((
        "keys.sequence_timeout_ms".into(),
        config.sequence_timeout_ms.to_string(),
    ));
    for (action, list) in &config.normal {
        out.push((
            format!("keys.normal.{action:?}").to_lowercase(),
            chords(list),
        ));
    }
    for (action, list) in &config.search {
        out.push((
            format!("keys.search.{action:?}").to_lowercase(),
            chords(list),
        ));
    }
    for launcher in &config.quick_launchers {
        let prefix = format!("quick_launchers.{}", launcher.id);
        let mut args = toml_edit::Array::new();
        for arg in &launcher.args {
            args.push(arg.as_str());
        }
        out.extend([
            (format!("{prefix}.sequence"), launcher.sequence.clone()),
            (format!("{prefix}.label"), launcher.label.clone()),
            (format!("{prefix}.command"), launcher.command.clone()),
            (format!("{prefix}.args"), args.to_string()),
            (
                format!("{prefix}.working_directory"),
                match launcher.working_directory {
                    LauncherWorkingDirectory::Selected => "selected",
                    LauncherWorkingDirectory::Tmux => "tmux",
                }
                .into(),
            ),
            (format!("{prefix}.enabled"), launcher.enabled.to_string()),
        ]);
    }
    out.into_iter()
        .map(|(name, value)| {
            let source = config
                .sources
                .get(name.as_str())
                .cloned()
                .unwrap_or_else(|| "default".into());
            Row {
                name,
                value,
                source,
            }
        })
        .collect()
}

/// Settings the user actually decided: everything a bare default did not.
pub(super) fn customized(rows: &[Row]) -> usize {
    rows.iter()
        .filter(|row| row.source != "default" && row.source != "theme base")
        .count()
}

fn print_table(rows: &[Row]) {
    let name = rows.iter().map(|r| r.name.len()).max().unwrap_or(0).max(7);
    let value = rows.iter().map(|r| r.value.len()).max().unwrap_or(0).max(5);
    println!("{:name$}  {:value$}  source", "setting", "value");
    for row in rows {
        println!("{:name$}  {:value$}  {}", row.name, row.value, row.source);
    }
}

pub fn effective_check(all: bool) -> i32 {
    match current(None) {
        Ok(config) => {
            if let Some(path) = config_path() {
                println!("{}\n", escaped(&path.to_string_lossy()));
            }
            let all_rows = rows(&config);
            let shown: Vec<&Row> = if all {
                all_rows.iter().collect()
            } else {
                all_rows
                    .iter()
                    .filter(|row| row.source != "default" && row.source != "theme base")
                    .collect()
            };
            if shown.is_empty() {
                println!("every setting is at its default; --all lists them.");
                return 0;
            }
            let owned: Vec<Row> = shown
                .into_iter()
                .map(|row| Row {
                    name: row.name.clone(),
                    value: row.value.clone(),
                    source: row.source.clone(),
                })
                .collect();
            print_table(&owned);
            let rest = all_rows.len() - owned.len();
            if !all && rest > 0 {
                println!("\n{rest} settings are at their defaults; --all lists every one.");
            }
            0
        }
        Err(e) => {
            eprintln!("agenmux: {e}");
            e.exit_code()
        }
    }
}

/// Standalone validation: no tmux, detector, hook, or update dependencies.
pub fn check() -> i32 {
    let Some(path) = config_path() else {
        println!("no configuration file; using defaults");
        println!("no absolute XDG or HOME root to look in");
        return 0;
    };
    let shown = escaped(&path.to_string_lossy());
    let file = match load_path(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("agenmux: {e}");
            return e.exit_code();
        }
    };
    if !path.exists() {
        println!("no configuration file; using defaults");
        println!("looked for {shown}");
        return 0;
    }
    // No tmux here: count what the file alone decides, not what options win.
    let set = resolve(&file, &Default::default())
        .map(|config| customized(&rows(&config)))
        .unwrap_or(0);
    println!("{shown}: valid");
    match set {
        0 => println!("0 settings differ from the defaults"),
        n => println!(
            "{n} settings differ from the defaults; agenmux config check --effective shows them"
        ),
    }
    0
}
