use mf_core::plugin::scan_plugins;
use mf_core::validation::error::MeltforgeError;

use crate::util::messages::print_messages;

pub fn print_scanned_plugins(plugins_dir: &str) -> Result<(), MeltforgeError> {
    let (manifests, messages) = scan_plugins(plugins_dir).map_err(MeltforgeError::from)?;
    print_messages(messages);

    for manifest in &manifests {
        println!(
            "  inputs: {:?}  outputs: {:?}",
            manifest.inputs, manifest.outputs
        );
    }

    Ok(())
}
