pub struct Opacity {
    value: f32,
    min: f32,
    max: f32,
}

pub struct Layer {
    name: String,
    visible: bool,
    opacity: Opacity,
}