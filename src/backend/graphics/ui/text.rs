use wgpu::{Device, Queue, TextureView, CommandEncoder, SwapChainFrame, SwapChainDescriptor, util::StagingBelt};
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};

pub fn render_text<'a>(device: &Device, 
                   queue: &Queue, 
                   mut encoder: &'a mut CommandEncoder, 
                   view: &TextureView, 
                   sc_desc: &SwapChainDescriptor,
                   text: String, 
                   screen_position: (f32, f32)) {

    let font = ab_glyph::FontArc::try_from_slice(include_bytes!("../../../../res/font/Inconsolata-Bold.ttf"))
        .expect("Load font");

    let mut glyph_brush = GlyphBrushBuilder::using_font(font)
        .build(&device, wgpu::TextureFormat::Bgra8UnormSrgb);

    let section = Section {
        screen_position,
        text: vec![Text::new(text.as_str())
            .with_color([1.0, 1.0, 1.0, 1.0])
            .with_scale(50.0)],
        ..Section::default()
    };

    let mut staging_belt = StagingBelt::new(1024);

    glyph_brush.queue(section);
    
    glyph_brush.draw_queued(
        &device,
        &mut staging_belt,
        &mut encoder,
        &view,
        sc_desc.width,
        sc_desc.height,
    );

    staging_belt.finish();
}
