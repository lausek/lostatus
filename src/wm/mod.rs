#[cfg(feature = "dwm")]
pub mod dwm;
#[cfg(feature = "i3")]
pub mod i3;

#[cfg(feature = "dwm")]
pub use dwm::*;
#[cfg(feature = "i3")]
pub use i3::*;

use super::*;

#[macro_export]
macro_rules! system_sender {
    ($fn:expr) => {
        std::thread::Builder::new()
            .name("lostatus_system_sender".to_string())
            .spawn($fn)
            .expect("spawning system sender failed.")
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct I3Input
{
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub instance: String,

    #[serde(default)]
    pub button: i64,
    #[serde(default)]
    pub modifiers: Vec<String>,

    #[serde(default)]
    pub x: i64,
    #[serde(default)]
    pub y: i64,
    #[serde(default)]
    pub relative_x: i64,
    #[serde(default)]
    pub relative_y: i64,
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}
