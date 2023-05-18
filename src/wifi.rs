use anyhow::{bail, Result};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripheral;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop, wifi::BlockingWifi, wifi::EspWifi,
    nvs::EspDefaultNvsPartition
};
use log::info;

pub fn wifi(
    ssid: &str,
    password: &str,
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> Result<BlockingWifi<EspWifi<'static>>> {
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        bail!("Missing WiFi name")
    }
    if password.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }
    let esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    info!("Wifi initialized");
    
    let mut wifi = BlockingWifi::wrap(esp_wifi, sysloop)?;
    info!("Wifi wrapped");

    let config: Configuration = Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        bssid: None,
        auth_method,
        password: password.into(),
        channel: None,
    });

    wifi.set_configuration(&config)?;

    info!("Starting wifi...");
    wifi.start()?;

    info!("Connecting wifi...");
    wifi.connect()?;

    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(wifi)
}
