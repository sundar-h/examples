use std::time::Duration;

use plugin::{BasePlugin, Sink};
use paho_mqtt as mqtt;

pub struct MqttSink {
}

impl MqttSink {
    fn new() -> Self {
    }
}

impl BasePlugin for MqttSink {
    fn init(&self, config: &str) -> Result<(), plugin::PluginError> {
        let host = "";
        let mu cli = mqtt::Client::new(host)?;
        cli.set_timeout(Duration::from_secs(secs));
    }

    fn finalize(&self) -> Result<(), plugin::PluginError> {
        // todo!()
    }

    fn name(&self) -> String {
        "MQTT".to_string()
    }

    fn plugin_type(&self) -> Result<plugin::base_plugin::PluginType, plugin::PluginError> {
        "Sink"
    }
}

impl Sink for MqttSink {
    fn send(&self, payload: String) -> Result<(), plugin::PluginError> {
        todo!()
    }
}
