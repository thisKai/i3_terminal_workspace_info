#![feature(use_nested_groups)]
extern crate i3ipc;

use std::io::{self, Write};

use i3ipc::{
    I3Connection,
    I3EventListener,
    Subscription,
    reply::{
        Outputs,
        Output,
        Workspaces,
        Workspace,
    }
};

fn debug_workspaces(connection: &mut I3Connection) -> Result<(), ::i3ipc::MessageError> {
    let &Outputs { ref outputs } = &connection.get_outputs().unwrap();
    let screens = outputs.iter().filter(|output| output.current_workspace.is_some());
    let Workspaces { ref workspaces } = connection.get_workspaces()?;
    for &Output { ref name, .. } in screens {
        print!("[ {}: ", name);
        let workspaces_on_this_screen = workspaces.iter().filter(|ws| &ws.output == name);
        for &Workspace { ref name, focused, .. } in workspaces_on_this_screen {
            if focused {
                print!("<{}>", name);
            } else {
                print!(" {} ", name);
            }
        }
        print!(" ] ");
    }
    println!();
    let _ = io::stdout().flush();
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
