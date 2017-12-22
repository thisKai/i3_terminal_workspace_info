extern crate i3ipc;

use i3ipc::{
    I3Connection,
    I3EventListener,
    Subscription,
};

fn debug_workspaces(connection: &mut I3Connection) -> Result<(), ::i3ipc::MessageError> {
    Ok(())
}

fn main() {
    let mut connection = I3Connection::connect().unwrap();

    let _ = debug_workspaces(&mut connection);

    let mut listener = I3EventListener::connect().unwrap();

    let subs = [Subscription::Workspace];
    listener.subscribe(&subs).unwrap();

    for _event in listener.listen() {
        let _ = debug_workspaces(&mut connection);
    }
}
