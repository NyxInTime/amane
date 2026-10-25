use std::env;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

use serde::Deserialize;
use serde_json::json;

use crate::Workspace;

#[derive(Deserialize, Debug)]
struct Payload {
    pub data: Vec<WorkspaceData>,
    #[allow(unused)]
    pub event: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WorkspacesResponse {
    pub ok: Vec<WorkspaceData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WorkspaceData {
    #[allow(unused)]
    pub id: String,
    pub name: String,
    pub index: u32,
    pub output: String,
    #[allow(unused)]
    pub layout: String,
    pub active: bool,
    pub focused: bool,
    #[allow(unused)]
    pub occupied: bool,
    pub urgent: bool,
    pub window_count: u32,
    #[allow(unused)]
    pub layout_override: bool,
    #[allow(unused)]
    pub named: bool,
    #[allow(unused)]
    pub focused_window: Option<String>,
}

pub fn listen(mut on_change: impl FnMut(Vec<Workspace>)) {
    // write send
    let Some(stream) = send(r#"{"cmd":"subscribe","events":["workspaces"]}"#) else {
        return;
    };

    for line in BufReader::new(stream).lines() {
        let Ok(line) = line else {
            return;
        };

        let Some(event) = parse(&line) else {
            continue;
        };

        let mut workspaces = Vec::new();
        event.iter().for_each(|w| {
            workspaces.push(Workspace {
                id: workspaces.len() as i64,
                index: w.index,
                name: Some(w.name.clone()),
                output: Some(w.output.clone()),
                active: w.active,
                focused: w.focused,
                urgent: w.urgent,
                windows: w.window_count,
            });
        });

        on_change(workspaces);
    }
}

// I could maybe send the request not formatted, but idk. would probably be a better idea, idk
pub fn send(request: &str) -> Option<UnixStream> {
    let path = env::var("UMBRIEL_SOCKET").ok()?;

    let payload = format!("{}\n", request);

    let mut stream = UnixStream::connect(path).ok()?;

    stream.write_all(payload.as_bytes()).ok()?;

    Some(stream)
}

//TODO fix this later, for now i just want to be able to see that the info comes in
fn parse(payload: &str) -> Option<Vec<WorkspaceData>> {
    match serde_json::from_str::<Payload>(payload) {
        Ok(response) => Some(response.data),
        _ => None,
    }
}
fn parse_workspaces(workspaces: &str) -> Option<Vec<WorkspaceData>> {
    match serde_json::from_str::<WorkspacesResponse>(workspaces) {
        Ok(response) => Some(response.ok),
        Err(_) => None,
    }
}

pub fn focus_workspace(id: i64) {
    let Some(stream) = send(r#"{"cmd":"workspaces"}"#) else {
        return;
    };
    let mut reply = String::new();
    let _ = BufReader::new(stream).read_line(&mut reply);
    let Some(workspaces) = parse_workspaces(&reply) else {
        return;
    };
    let Some(workspace): Option<WorkspaceData> = workspaces.get(id as usize).cloned() else {
        return;
    };
    let arg = format!("workspace-switch:{}/{}", workspace.index, workspace.output);
    let request = json!({
    "cmd": "msg",
    "arg": arg,
    }); //r#"{"cmd":"msg","arg":"workspace-switch:3/HDMI-A-1"}"#;
    let Some(stream) = send(&request.to_string()) else {
        return;
    };
    let mut reply = String::new();

    let _ = BufReader::new(stream).read_line(&mut reply);
}
