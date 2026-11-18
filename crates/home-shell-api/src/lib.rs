//! `home.shell@1`: the layout of Home, and nothing else (repo:ce/docs/CE-REF-OPENERS-AND-HOME-2026-11-18.md section 5.1).
//! `Layout` names the slots this viewer may have on this device, each another cell's view; the same shape as
//! foreman.shell@2 `Layout` (repo:home-shell crates/shell-layout is the one routine both answer with). `offer` is
//! filled by the shell that holds the viewer's grant for the data cells (apps.shell), from each slot's `when` op.
#![forbid(unsafe_code)]

ce_wire::api! {
    api HomeShell = "home.shell", version 1;
    types {
        /// Health contract: these three fields, in this order, in every cell.
        pub struct Health { pub ok: bool, pub version: String, pub uptime_s: u64 }
        pub enum Device { Phone, Desk, Wall }
        pub enum Area { Head, Main, Side }
        /// What a slot has for the viewer now: how many, how urgent (0 overdue .. 4 quiet), until when (unix s).
        pub struct Offer { pub count: u32, pub urgency: u8, pub until: Option<u64> }
        pub struct LayoutReq { pub device: Device }
        /// One slot: mount `view` of `cell@major` in `area`, at `order`.
        pub struct Slot { pub name: String, pub cell: String, pub major: u16, pub view: String, pub area: Area, pub order: u16, pub offer: Option<Offer> }
    }
    ops {
        1 => HealthOp(()) -> Health, needs "home.shell:health";
        2 => Layout(LayoutReq) -> Vec<Slot>, needs "home.shell:open";
    }
}
