use std::env;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Deserialize;
use serde_json::{Value, json};

use crate::Workspace;

#[derive(Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
enum Payload {
    #[serde(rename = "workspaces")]
    Workspace(Vec<WorkspaceData>),

    #[serde(rename = "windows")]
    Window(Vec<WindowData>),
}

#[derive(Deserialize, Debug)]
pub struct WorkspaceData {
    pub id: String,
    pub name: String,
    pub index: u32,
    pub output: String,
    pub layout: String,
    pub active: bool,
    pub focused: bool,
    pub occupied: bool,
    pub urgent: bool,
    pub window_count: u32,
    pub layout_override: bool,
    pub named: bool,
    pub focused_window: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WindowData {
    pub id: String,
    pub title: String,
    pub app_id: String,
    pub pid: u32,
    pub output: String,
    pub workspace: String,

    // Geometry and Layout (Positions can be negative)
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub column: u32,
    pub row: u32,

    // Window States
    pub active: bool,
    pub focused: bool,
    pub visible: bool,
    pub urgent: bool,
    pub floating: bool,
    pub fullscreen: bool,
    pub maximized: bool,
    pub maximized_to_edges: bool,
    pub pinned: bool,
    pub tabbed: bool,
    pub tab_hidden: bool,
    pub tab_index: i32, // Uses -1 when not tabbed
    pub title_bar: bool,
    pub xwayland: bool,

    // Metadata & Extensions
    pub content_type: String,
    pub xdg_tag: String,
    pub parent: String,
    pub scratchpad: String,
    pub snap: Option<Value>, // Catches Null or future snap states Safely

    // Nested Effect Objects
    pub border_effect: Effect,
    pub window_effect: Effect,
}

#[derive(Deserialize, Debug)]
pub struct Effect {
    pub name: String,
    pub source: String,
    pub pool: String,
    pub suppressed: bool,

    // Option protects window_effect which lacks this key completely
    pub overlay: Option<String>,
}

pub fn listen(mut on_change: impl FnMut(Vec<Workspace>)) {
    // write send
    let Some(mut stream) = send(r#"{"cmd":"subscribe","events":["workspaces","windows"]}"#) else {
        return;
    };

    for line in BufReader::new(stream).lines() {
        let Ok(line) = line else {
            return;
        };

        let Some(event) = parse(&line) else {
            continue;
        };
    }
}

// I could maybe send the request not formatted, but idk. would probably be a better idea, idk
pub fn send(request: &str) -> Option<UnixStream> {
    let path = env::var("UMBRIEL_SOCKET").ok()?;

    let payload = format!("{}\n", request);

    let mut stream = UnixStream::connect(path).ok()?;
    eprintln!("sending request: {}", payload);

    stream.write_all(payload.as_bytes()).ok()?;

    Some(stream)
}

fn request(stream: &mut UnixStream, command: &str) -> Option<String> {
    let result = stream.write_all(command.as_bytes());

    eprintln!("result: {:?}", result);

    let mut reply = String::new();

    stream.read_to_string(&mut reply).ok()?;

    Some(reply)
}

fn fetch(stream: &mut UnixStream) -> Option<Vec<Workspace>> {
    let workspaces = request(stream, r#"{"cmd":"workspaces"}"#)?;

    parse(&workspaces)
}

//TODO fix this later, for now i just want to be able to see that the info comes in
fn parse(workspaces: &str) -> Option<Vec<Workspace>> {
    match serde_json::from_str::<Payload>(workspaces) {
        Ok(Payload::Workspace(workspaces)) => {
            eprintln!("got {} workspaces.", workspaces.len());
            eprintln!("the first workspace is {:?}", workspaces.get(0));
        }
        Ok(Payload::Window(windows)) => {
            //eprintln!("got {} windows", windows.len());
            //eprintln!("first window is {:?}", windows.get(0));
        }
        _ => {
            eprintln!("wrong branch")
        }
    }
    //eprintln!("{:?}", workspaces);
    //eprintln!("--------------------------\n\n\n");

    let mut shown: Vec<Workspace> = Vec::new();
    //let mut focused = None;

    let mut list = Vec::new();

    Some(list)
}

pub fn focus_workspace(id: i64) {}
