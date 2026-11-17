pub fn application_icon() -> eframe::egui::IconData {
    let ico = include_bytes!("../assets/QuickRemoteToolkit.ico");
    let count = u16::from_le_bytes([ico[4], ico[5]]) as usize;
    let entry = 6 + (count - 1) * 16;
    let size = u32::from_le_bytes(ico[entry + 8..entry + 12].try_into().unwrap()) as usize;
    let offset = u32::from_le_bytes(ico[entry + 12..entry + 16].try_into().unwrap()) as usize;
    eframe::icon_data::from_png_bytes(&ico[offset..offset + size])
        .expect("Embedded application icon")
}

pub fn themed_icon(accent: [u8; 3]) -> eframe::egui::IconData {
    let mut icon = application_icon();
    for pixel in icon.rgba.as_chunks_mut::<4>().0 {
        // The SVG consists of an accent tile and white artwork. Preserve white
        // coverage and antialiasing while replacing the original blue accent.
        let white = (f32::from(pixel[0]) - 37.).max(0.) / 218.;
        for channel in 0..3 {
            pixel[channel] = (f32::from(accent[channel])
                + (255. - f32::from(accent[channel])) * white)
                .round() as u8;
        }
    }
    icon
}
