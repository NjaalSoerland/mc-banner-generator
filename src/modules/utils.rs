use image::Rgba;

pub const COLORS: [(Rgba<u8>, &str); 16] = [
    (Rgba([25, 25, 25, 255]), "Black"),
    (Rgba([76, 76, 76, 255]), "Dark Grey"),
    (Rgba([153, 153, 153, 255]), "Grey"),
    (Rgba([255, 255, 255, 255]), "White"),
    (Rgba([242, 127, 165, 255]), "Pink"),
    (Rgba([178, 76, 216, 255]), "Magenta"),
    (Rgba([127, 63, 178, 255]), "Purple"),
    (Rgba([51, 76, 178, 255]), "Blue"),
    (Rgba([76, 127, 153, 255]), "Cyan"),
    (Rgba([102, 153, 216, 255]), "Light Blue"),
    (Rgba([102, 127, 51, 255]), "Green"),
    (Rgba([127, 204, 25, 255]), "Lime"),
    (Rgba([229, 229, 51, 255]), "Yellow"),
    (Rgba([216, 127, 51, 255]), "Orange"),
    (Rgba([102, 76, 51, 255]), "Brown"),
    (Rgba([153, 51, 51, 255]), "Red"),
];