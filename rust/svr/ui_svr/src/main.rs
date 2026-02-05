////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// UI Server for the Control Plane.
//
// User facing interface, including the following
// 1. Static Cached files (html, css, js, svg)
// 2. Login page and action, with user preferences and session history
// 3. Passes json requestes through to cp_svr for processing
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod state;
pub mod cache_routers;
pub mod ui_routers;
pub mod passthrough_router;

fn main() {
    println!("Hello, world!");
}
