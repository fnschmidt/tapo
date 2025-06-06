/// KE100 TRV Example
use std::env;

use log::info;
use tapo::requests::TemperatureUnitKE100;
use tapo::{ApiClient, HubDevice};

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let tapo_username = env::var("TAPO_USERNAME")?;
    let tapo_password = env::var("TAPO_PASSWORD")?;
    let ip_address = env::var("IP_ADDRESS")?;
    // Name of the KE100 device.
    // Can be obtained from the Tapo App or by executing `get_child_device_component_list()` on the hub device.
    let device_name = env::var("DEVICE_NAME")?;
    let target_temperature: u8 = env::var("TARGET_TEMPERATURE")?.parse()?;

    let hub = ApiClient::new(tapo_username, tapo_password)
        .h100(ip_address)
        .await?;

    // Get a handler for the child device
    let device = hub.ke100(HubDevice::ByNickname(device_name)).await?;

    // Get the device info of the child device
    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    // Set target temperature.
    // KE100 currently only supports Celsius as temperature unit.
    info!("Setting target temperature to {target_temperature} degrees Celsius...");
    device
        .set_target_temperature(target_temperature, TemperatureUnitKE100::Celsius)
        .await?;

    // Get the device info of the child device
    let device_info = device.get_device_info().await?;
    info!("Device info: {device_info:?}");

    Ok(())
}
