use layer_shika::prelude::*;

slint::include_modules!();

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let platform = LayerShell::new().unwrap();

    // Per-window config by title.
    // Exclusive zone auto-derives from the Slint window height for top/bottom anchors.
    platform
        .window("TopBar")
        .set_namespace("top-bar")
        .set_anchor(AnchorEdges::top_bar());

    platform
        .window("BottomBar")
        .set_namespace("bottom-bar")
        .set_layer(Layer::Overlay)
        .set_anchor(AnchorEdges::bottom_bar())
        .set_exclusive_zone(0);

    slint::platform::set_platform(platform).unwrap();

    let top = TopBar::new().unwrap();
    let bottom = BottomBar::new().unwrap();

    top.show().unwrap();
    bottom.run().unwrap();

    Ok(())
}
