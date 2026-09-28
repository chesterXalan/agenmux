# Sidebar visual hierarchy and accessibility (#142)

Goal: selection, hierarchy, and previews stay readable without color, with
the same compact layout and no new metadata. Status keeps its existing glyphs:
working animates, but blocked and done still differ by color alone.

## Decisions

- **Disclosure markers:** session and window headers in the all-pane tree show
  `▼` open and `▶` collapsed, in place of the Nerd Font window icon. The
  agent-only view keeps plain session headers; it has no branches.
- **Collapse (#118):** headers are cursor stops in all-pane mode whether or
  not tmux management is on. `Space` or a click on the selected header
  toggles; `h`/`Left` collapses or steps to the parent header; `Right`
  expands a header; `l`/`Enter` expand a header too, but jump from a pane or
  while filtering, where `Right` does nothing; `z`/`Z` fold or unfold everything. These keys are fixed defaults,
  like `G` and `.`, and a configured chord wins. State is a daemon-lifetime
  set of session/window ids. Filtering ignores it without changing it, and a
  hidden selection falls back to its nearest visible header. A collapsed
  `▶` takes the most urgent hidden status colour (blocked > done > working),
  blinking against the accent. Sidebar rows have no spare
  width, so it adds no glyph or count; the attention filter lists them all.
- **Unclaimed keys:** tmux's `Any` binding now sends a no-op protocol byte
  instead of Space, which toggles a branch.
- **Status glyphs:** unchanged. Blocked keeps its blinking red `⣿`.
- **Muted text:** muted styling drops SGR dim. The dark base defaults
  `muted_fg` to 256-color 245. Light and terminal bases keep their explicit
  values, and a configured `muted_fg` renders exactly as set.
- **Previews:** prompt/activity lines start with `↳ ` at the agent-label
  column in both views.
- **Not done:** agent-name column alignment, extra counters, tree guides.

## Out of scope

- The tmux status-bar summary (`#{agenmux}`) keeps its colored `⣿` counts.
- A config default for initially collapsed branches.

## Verification

- Update the renderer golden frames and unit assertions.
- Run `cargo test --locked` and `tests/run.sh`.
