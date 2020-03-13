use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Debug, Deserialize)]
pub struct Authorization {
    pub auth_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelInfo {
    pub name: String,
    pub manufacturer: String,
    pub model: String,
    pub firmware_version: String,
    #[serde(rename = "serialNo")]
    pub serial_number: String,
    pub state: State,
    pub effects: Effects,
    pub panel_layout: PanelLayout,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub color_mode: String,
    pub brightness: Range,
    pub ct: Range,
    pub hue: Range,
    pub sat: Range,
    pub on: On,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct On {
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub max: u32,
    pub min: u32,
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effects {
    pub effects_list: Vec<String>,
    pub select: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelLayout {
    pub global_orientation: GlobalOrientation,
    pub layout: Layout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalOrientation {
    pub max: u32,
    pub min: u32,
    pub value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    pub num_panels: u32,
    pub side_length: u32,
    pub position_data: Vec<Position>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ShapeType {
    Triangle = 0,
    Rhythm = 1,
    Square = 2,
    ControlSquarePrimary = 3,
    ControlSquarePassive = 4,
    PowerSupply = 5,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "panelId")]
    pub panel_id: u32,
    pub o: i32,
    pub x: i32,
    pub y: i32,
    // Not in firmware_version: "3.0.5"
    #[serde(rename = "shapeType")]
    pub shape_type: Option<ShapeType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Brightness {
    Increment { increment: i32 },
    Set { value: u32 },
    SetWithDuration { value: u32, duration: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetRange {
    Increment(i32),
    Set { value: u32 },
}

// Effects Endpoint
//
// This endpoint is driven by PUTing various "commands"

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    Color,
    Rhythm,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimType {
    Custom,
    Explode,
    Fade,
    Flow,
    Highlight,
    Plugin,
    Random,
    Static,
    Wheel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    pub hue: u32,
    pub saturation: u32,
    pub brightness: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinearDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RadialDirection {
    In,
    Out,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Circular Direction, around the layout center.
pub enum RotationalDirection {
    /// Clockwise
    Cw,
    /// Counter Clockwise
    Ccw,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ColorType {
    HSB,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name", content = "value")]
pub enum PluginOption {
    /// Generally, the time it takes to go from one palette colour to another, in tenths of a
    /// second.
    #[serde(rename = "transTime")]
    TransitionTime(u16),
    /// Indicates whether an animation should loop or not.
    #[serde(rename = "loop")]
    Loop(bool),
    /// Linear direction, based on user’s global orientation.
    #[serde(rename = "linDirection")]
    LinearDirection(LinearDirection),
    /// Radial direction, based on layout center.
    #[serde(rename = "radDirection")]
    RadialDirection(RadialDirection),
    /// Circular Direction, around the layout center.
    #[serde(rename = "rotDirection")]
    RotationalDirection(RotationalDirection),
    /// Indicates how long the plugin will dwell on a palette colour, in tenths of a second.
    #[serde(rename = "delayTime")]
    DelayTime(u16),
    /// Modifier that indicates how much of a palette is shown on the layout. 50 is a global limit,
    /// but generally the max will be dynamically set by the App to the size of the user’s palette.
    #[serde(rename = "nColorsPerFrame")]
    NColorsPerFrame(u8),
    /// Probability of background colour being used
    #[serde(rename = "mainColorProb")]
    MainColorProb(f32),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    #[serde(rename = "animType")]
    pub animation_type: AnimType,
    pub color_type: ColorType,
    #[serde(rename = "animName")]
    pub name: String,
    pub palette: Vec<Palette>,
    pub plugin_type: Option<PluginType>,
    pub plugin_options: Option<Vec<PluginOption>>,
    pub plugin_uuid: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animations {
    pub animations: Vec<Effect>,
}
