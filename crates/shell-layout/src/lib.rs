//! The one Layout routine of the shells (repo:ce/docs/CE-REF-OPENERS-AND-HOME-2026-11-18.md section 5.1): home.shell@1
//! and foreman.shell@2 both answer `Layout{device}` with it, so one shell routine draws both. A shell owns a layout and
//! nothing else: the slot lines come from the systems (repo:ce-net-net), the data from each slot's own cell.
//!
//! [`layout`]: the lines whose `needs` the viewer's chain holds (a slot the viewer may not have is never named to them),
//! placed for the device (Phone: one column, side slots join main), ordered by area then `order`. [`order`] re-sorts
//! once the shell knows each slot's offer: area, then urgency (overdue, due today, waiting on the viewer, news, quiet),
//! then `order`. A slot whose `when` op says there is nothing for the viewer is dropped by the shell (it alone holds
//! the viewer's grant for the data cells).
#![forbid(unsafe_code)]

pub use ce_net_net::{Area, SlotLine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Device {
    Phone,
    Desk,
    Wall,
}

/// One slot as laid out for a device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placed {
    pub name: &'static str,
    pub cell: &'static str,
    pub major: u16,
    pub view: &'static str,
    pub area: Area,
    pub order: u16,
    pub when: Option<&'static str>,
}

/// Where a slot sits on a device: a phone has one column under the head.
pub fn area_on(device: Device, area: Area) -> Area {
    match (device, area) {
        (Device::Phone, Area::Side) => Area::Main,
        (_, a) => a,
    }
}

pub fn layout(lines: &[SlotLine], holds: &dyn Fn(&str) -> bool, device: Device) -> Vec<Placed> {
    let mut out: Vec<Placed> = lines
        .iter()
        .filter(|l| holds(l.needs))
        .map(|l| Placed { name: l.name, cell: l.cell, major: l.major, view: l.view, area: area_on(device, l.area), order: l.order, when: l.when })
        .collect();
    out.sort_by_key(|p| (p.area, p.order));
    out
}

/// Urgency of an offer, most urgent first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Urgency {
    Overdue,
    DueToday,
    WaitingOnViewer,
    News,
    Quiet,
}

/// Area, then urgency, then order.
pub fn order<T>(slots: &mut [T], key: impl Fn(&T) -> (Area, Urgency, u16)) {
    slots.sort_by_key(key);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ce_net_net::HOME_SLOTS;

    #[test]
    fn every_right_five_slots_in_area_then_order() {
        let p = layout(HOME_SLOTS, &|_| true, Device::Desk);
        let names: Vec<&str> = p.iter().map(|s| s.name).collect();
        assert_eq!(names, ["approve", "calls", "promises", "cells", "links"]);
    }

    #[test]
    fn a_viewer_gets_only_the_slots_their_chain_holds() {
        let p = layout(HOME_SLOTS, &|a| a == "crm.promises:read", Device::Desk);
        assert_eq!(p.iter().map(|s| s.name).collect::<Vec<_>>(), ["promises"]);
        assert!(layout(HOME_SLOTS, &|_| false, Device::Phone).is_empty());
    }

    #[test]
    fn a_phone_has_one_column() {
        let p = layout(HOME_SLOTS, &|_| true, Device::Phone);
        assert!(p.iter().all(|s| s.area == Area::Main));
        assert_eq!(p.iter().map(|s| s.name).collect::<Vec<_>>(), ["approve", "calls", "promises", "cells", "links"]);
    }

    #[test]
    fn urgency_comes_before_order_within_an_area() {
        let mut p = layout(HOME_SLOTS, &|_| true, Device::Desk);
        let urg = |n: &str| if n == "promises" { Urgency::Overdue } else { Urgency::Quiet };
        order(&mut p, |s| (s.area, urg(s.name), s.order));
        assert_eq!(p.iter().map(|s| s.name).collect::<Vec<_>>(), ["promises", "approve", "calls", "cells", "links"]);
    }
}
