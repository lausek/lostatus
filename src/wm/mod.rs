use super::*;

#[cfg(not(any(feature = "i3", feature = "dwm")))]
pub mod default;
#[cfg(feature = "dwm")]
pub mod dwm;
#[cfg(feature = "i3")]
pub mod i3;

#[cfg(not(any(feature = "i3", feature = "dwm")))]
pub use default::*;
#[cfg(feature = "dwm")]
pub use dwm::*;
#[cfg(feature = "i3")]
pub use i3::*;

#[macro_export]
macro_rules! sender {
    (system, $fn:expr) => {
        std::thread::Builder::new()
            .name("lostatus_system_sender".to_string())
            .spawn($fn)
            .expect("spawning system sender failed.")
    };
    (user, $fn:expr) => {
        std::thread::Builder::new()
            .name("lostatus_user_sender".to_string())
            .spawn($fn)
            .expect("spawning user sender failed.")
    };
}

pub fn spawn_senders(sender: Sender<UpdateEvent>) -> Vec<std::thread::JoinHandle<()>>
{
    let mut handles = vec![];

    if let Some(handle) = spawn_system_sender(sender.clone()) {
        handles.push(handle);
    }

    if let Some(handle) = spawn_user_sender(sender.clone()) {
        handles.push(handle);
    }

    handles
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Input
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
