pub mod core;
pub mod dispatch;
pub mod scheduled;

use core::{channel::Channel, hello::Hello, log::Logger, login::Login, pong::Pong};
use marv_api::plugins::{DynamicPluginVec, Plugin};
use once_cell::sync::OnceCell;

static PLUGINS: OnceCell<DynamicPluginVec> = OnceCell::new();

pub fn initialize(custom: DynamicPluginVec) {
    let mut default = vec![
        Logger::new(),
        Login::new(),
        Pong::new(),
        Channel::new(),
        Hello::new(),
    ];

    default.extend(custom);
    PLUGINS.set(default);
}

fn default_plugins() -> &'static DynamicPluginVec {
    PLUGINS.get().unwrap()
}

fn default_schedulables() -> Vec<(String, &'static Box<dyn Plugin>)> {
    let mut candidates: Vec<(String, &'static Box<dyn Plugin>)> = Vec::new();
    let plugins = default_plugins();

    for plugin in plugins.iter() {
        if let Some(appointment) = plugin.schedule() {
            candidates.push((appointment, plugin));
        }
    }

    candidates
}
