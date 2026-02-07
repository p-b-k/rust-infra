////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Worker Bee for the Control Plane
//
// This looks for jobs to be run in the cp-svr and takes then and runs. It should locally store logs on the jobs,
// but return status updates and user facing logging info to the cp-svr.
//
// On startup it should register itself with the cp-svr, and on shutdown it should unregister itself. It should
// also provide a ping type endpoint to test for availability, and probably some kinds of statistics endpoints
// as well.  While we're at it we should probably add locally stored statistics as well.
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod state;

fn main() {
    println!("Hello, world!");
}
