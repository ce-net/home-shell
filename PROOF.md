# Proof (2026-11-18, foreman-ops-1)

home.shell@1 runs on foreman-ops-1 (`ce cell start home.shell`, key `cell-keys/home.shell/`, grant `delegate:1`).
`Layout{device}` as each test person, through apps.shell `/home/api/layout` (a grant wire.ws-door minted for them):

| person | rights | slots |
|---|---|---|
| t-home-all | Leif's apps.shell abilities + home.shell:open, home.shell:links, open.with:use | promises, approve, cells, links today (calls has nothing today and is dropped by apps.shell); 5 with tomorrow's window |
| t-home-promises | home.shell:open, open.with:use, crm.promises:read | promises |
| t-home-stranger | search.hybrid:find | 403, no tab |

Phone: every slot in area Main (one column). Layout answered in 34 to 51 ms warm. `cargo test`: 4.
The full slice proof (openers, refusal, stranger, unknown, lint, stop and start, schema hashes): repo:open-with PROOF.md.
