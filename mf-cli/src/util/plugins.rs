use mf_core::plugin::scan_plugins;
use mf_core::validation::error::MeltforgeError;

pub fn print_scanned_plugins(dir: &str) -> Result<(), MeltforgeError> {
    let plugins = scan_plugins(dir)?;
    println!("Plugins found: {}", plugins.len());

    for man in &plugins {
        println!("- {} v{} at {}", man.name, man.version, man.path.display());
        println!("  inputs:  {:?}", man.inputs);
        println!("  outputs: {:?}", man.outputs);
    }

    Ok(())
}
