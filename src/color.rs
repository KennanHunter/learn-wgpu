//  https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB
/// hue saturation and value are all expected to be in range [0, 1]
pub fn hue_to_rgb(hue: f64, saturation: f64, value: f64) -> [f64; 3] {
    let hue_step = hue * 6.0;

    let chroma = saturation * value;

    let variant = chroma * (1.0 - ((hue_step % 2.0) - 1.0).abs());

    match hue_step {
        0.0..=1.0 => [chroma, variant, 0.0],
        1.0..=2.0 => [variant, chroma, 0.0],
        2.0..=3.0 => [0.0, chroma, variant],
        3.0..=4.0 => [0.0, variant, chroma],
        4.0..=5.0 => [variant, 0.0, chroma],
        5.0..=6.0 => [chroma, 0.0, variant],
        _ => panic!("Hue has to be somewhere between 0.0 and 1.0"),
    }
}
