# README landing page (#150)

Docs-only: no Rust, no scripts, no version bump. `docs-skip.yml` covers the PR.

## Goal

- Lead with agenmux as a tmux manager that also follows your agents (the
  default since #145).
- Cut the 807-line README to a short landing page; move reference into `docs/`.
- Show the four agent states as an animated `site/states.svg` (braille drawn as
  dots, `SPIN` frames at 120 ms, 1 s blink, static under reduced motion).

## Layout

| File | Content moved from README |
| --- | --- |
| `docs/installation.md` | Installer details, TPM, manual install, requirements, upgrading from agents-mon, updating, release archives |
| `docs/usage.md` | Sidebar keys and details, popup mode, status line, tmux management, quick launchers, CLI |
| `docs/configuration.md` | tmux options, launcher bindings, `config.toml`, Settings view, reload, theme, keys, precedence |
| `docs/notifications.md` | Desktop notifications |
| `docs/custom-agents.md` | Adding / overriding agents |
| `docs/development.md` | Tests, dev and container harnesses, fixtures, runtime architecture |
| `docs/troubleshooting.md` | Daemon log, trace, known limits |

Moved text keeps every user-facing fact, rewritten as short sections, bullets,
and tables; internal implementation notes are dropped.

## Inbound links

README keeps short stub sections under the headings other files link to, so
nothing outside README and `docs/` changes:

- `#manual-install` (`site/index.html`)
- `#usage` and "README › Troubleshooting" / "README › Upgrading from agents-mon" (`install.sh`)
- `#adding--overriding-agents` (`CONTRIBUTING.md`)

## Verify

- Every relative link and anchor in README and `docs/*.md` resolves.
- `cargo test --locked` and `tests/run.sh` pass (`tests/no-stale-runtime-refs.sh` scans `docs/`).
