use dora_node_api::{dora_core::config::DataId, DoraNode, Event, IntoArrow};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let output = DataId::from("race_tick".to_string());
    let (mut node, mut events) = DoraNode::init_from_env()?;

    while let Some(event) = events.recv() {
        match event {
            Event::Input {
                id,
                metadata,
                data: _,
            } => match id.as_str() {
                "system_tick" => {
                    node.send_output(output.clone(), metadata.parameters, 666.into_arrow())?;
                    println!("get tick from system_tick");
                }
                other => eprintln!("Received input `{other}`"),
            },
            _ => {}
        }
    }

    Ok(())
}
