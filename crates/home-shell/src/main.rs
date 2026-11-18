//! `home-shell serve`: the home.shell cell behind this machine's door. `Layout{device}` is repo:ce-net-net HOME_SLOTS
//! through crates/shell-layout for the caller's chain (the viewer grant apps.shell had wire.ws-door mint for exactly
//! that viewer): a slot whose `needs` the chain does not hold is not in the answer. No data, no panel code.
use ce_wire::Server;
use home_shell_api::*;
use std::time::Instant;

const CELL: &str = "home.shell";

fn device(d: &Device) -> shell_layout::Device {
    match d {
        Device::Phone => shell_layout::Device::Phone,
        Device::Desk => shell_layout::Device::Desk,
        Device::Wall => shell_layout::Device::Wall,
    }
}

fn area(a: shell_layout::Area) -> Area {
    match a {
        shell_layout::Area::Head => Area::Head,
        shell_layout::Area::Main => Area::Main,
        shell_layout::Area::Side => Area::Side,
    }
}

async fn serve() -> Result<(), String> {
    let me = ui_chain::init(CELL)?.root();
    let started = Instant::now();
    let gate = ce_gate::Gate::<HomeShell>::new(me, ce_gate::mesh_roots()).into_fn();
    Server::<HomeShell>::new(gate)
        .on::<HealthOp, _, _>(move |_: (), _| async move { Ok(Health { ok: true, version: env!("CARGO_PKG_VERSION").into(), uptime_s: started.elapsed().as_secs() }) })
        .on::<Layout, _, _>(|req: LayoutReq, who| async move {
            let holds = |a: &str| ui_chain::holds(&who, a);
            let slots: Vec<Slot> = shell_layout::layout(ce_net_net::HOME_SLOTS, &holds, device(&req.device))
                .into_iter()
                .map(|p| Slot { name: p.name.into(), cell: p.cell.into(), major: p.major, view: p.view.into(), area: area(p.area), order: p.order, offer: None })
                .collect();
            eprintln!("home.shell: {} {:?}: {} slot(s)", who.name, req.device, slots.len());
            Ok(slots)
        })
        .audit(ce_gate::audit::file())
        .serve_door()
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    let r = match std::env::args().nth(1).as_deref() {
        Some("serve") => tokio::runtime::Builder::new_multi_thread().enable_all().build().map_err(|e| e.to_string()).and_then(|rt| rt.block_on(serve())),
        _ => Err("usage: home-shell serve".into()),
    };
    if let Err(e) = r {
        eprintln!("home-shell: {e}");
        std::process::exit(1);
    }
}
