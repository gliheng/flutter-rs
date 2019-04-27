//! This plugin is used for navigation in an app.
//! It handles flutter/navigation type messages.

use super::{PlatformMessage, Plugin, PluginName};
use crate::{
    channel::{Channel, JsonMethodChannel},
    codec::{MethodCall, MethodCallResult},
    desktop_window_state::RuntimeData,
};

use std::sync::Weak;

use log::info;
use serde_json::{json, Value};

pub const PLUGIN_NAME: &str = "flutter-engine::plugins::navigation";
pub const CHANNEL_NAME: &str = "flutter/navigation";

pub struct NavigationPlugin {
    channel: JsonMethodChannel,
}

impl PluginName for NavigationPlugin {
    fn plugin_name() -> &'static str {
        PLUGIN_NAME
    }
}

impl NavigationPlugin {
    pub fn new() -> Self {
        Self {
            channel: JsonMethodChannel::new(CHANNEL_NAME),
        }
    }

    pub fn set_initial_route(&self, initial_route: &str) {
        self.channel.invoke_method(MethodCall {
            method: String::from("setInitialRoute"),
            args: json!(initial_route),
        });
    }

    pub fn push_route(&self, route: &str) {
        self.channel.invoke_method(MethodCall {
            method: String::from("pushRoute"),
            args: json!(route),
        });
    }

    pub fn pop_route(&self) {
        self.channel.invoke_method(MethodCall {
            method: String::from("popRoute"),
            args: Value::Null,
        });
    }
}

impl Plugin for NavigationPlugin {
    fn init_channel(&mut self, runtime_data: Weak<RuntimeData>) {
        self.channel.init(runtime_data);
    }

    fn handle(&mut self, msg: &mut PlatformMessage, _: &mut glfw::Window) {
        let decoded = self.channel.decode_method_call(msg).unwrap();

        info!(
            "navigation method {:?} called with args {:?}",
            decoded.method, decoded.args
        );
        self.channel
            .send_method_call_response(&mut msg.response_handle, MethodCallResult::NotImplemented);
    }
}
