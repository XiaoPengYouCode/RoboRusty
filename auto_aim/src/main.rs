use dora_node_api::{DoraNode, Event};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (_node, mut events) = DoraNode::init_from_env()?;

    while let Some(event) = events.recv() {
        match event {
            Event::Input {
                id,
                metadata: _,
                data: _,
            } => match id.as_str() {
                "race_time" => {
                    println!("get race time:")
                }
                other => eprintln!("Received input `{other}`"),
            },
            _ => {}
        }
    }

    Ok(())
}
