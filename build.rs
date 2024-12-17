use miette::{IntoDiagnostic, Result};
use wgsl_bindgen::{GlamWgslTypeMap, WgslBindgenOptionBuilder, WgslTypeSerializeStrategy};

fn main() -> Result<()> {
    WgslBindgenOptionBuilder::default()
        .workspace_root("src")
        .add_entry_point("src/shader.wgsl")
        .skip_hash_check(true)
        .serialization_strategy(WgslTypeSerializeStrategy::Encase)
        .type_map(GlamWgslTypeMap)
        .output("src/shader.rs")
        .build()?
        .generate()
        .into_diagnostic()
}
