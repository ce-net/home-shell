# home-shell

`home.shell@1`: the layout of Home, the first tab of apps.ce-net.com (repo:ce/docs/CE-REF-OPENERS-AND-HOME-2026-11-18.md
section 5, slice 1 in section 8 C). It owns a layout and nothing else: no data, no panel code. Every slot is another
cell's view; every item in a slot is a ce-ref opened through open.with.

- `crates/home-shell-api`: `Layout{device: Phone|Desk|Wall} -> Vec<Slot{name, cell, major, view, area, order, offer}>`,
  `HealthOp`. The same shape as foreman.shell@2 `Layout`.
- `crates/shell-layout`: the one Layout routine both shells answer with (slot lines filtered by the viewer's chain,
  placed per device, ordered by area, urgency, order). foreman.shell@2 depends on it instead of writing a second one.
- `crates/home-shell`: slot lines are repo:ce-net-net `HOME_SLOTS` (calls, promises, approve, cells, links); a slot
  whose `needs` the caller's chain does not hold is not in the answer. `offer` is filled by apps.shell, which holds
  the viewer's grant for the data cells and drops a slot whose `when` op has nothing for the viewer.

Views of the slots live in repo:apps-shell `site/v/` (`calls-today.js`, `promises-due.js`, `pending.js`,
`cells-summary.js`, `links.js`). Keys: `<ce data dir>/cell-keys/home.shell/` (`delegate:1` only; it calls nothing).
`cargo test`: 4. Proof: [PROOF.md](PROOF.md). Apache-2.0.
