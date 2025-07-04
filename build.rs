use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SpirvBuilder::new("shader", "spirv-unknown-vulkan1.4")
        .print_metadata(MetadataPrintout::Full)
        .build()?;
    Ok(())
}
