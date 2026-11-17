//! SigmaOS-remmina-plugins — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_remmina_plugins_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-remmina-plugins: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-remmina-plugins: ok");
}
