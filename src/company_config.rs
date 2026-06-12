// Company customization for the internal RustDesk client.
// Only use this client for authorized office collaboration.

pub const COMPANY_ID_SERVER: &str = "82.156.75.4";
pub const COMPANY_RELAY_SERVER: &str = "82.156.75.4:21117";
pub const COMPANY_API_SERVER: &str = "";
pub const COMPANY_KEY: &str = "uKITwyLpZMhOBR5J7flMkOKf6smxFHJeutTMXcKYesQ=";

fn set_option(key: &str, value: &str) {
    crate::ui_interface::set_option(key.to_string(), value.to_string());
}

pub fn apply_company_server_config() {
    set_option("custom-rendezvous-server", COMPANY_ID_SERVER);
    set_option("relay-server", COMPANY_RELAY_SERVER);
    set_option("api-server", COMPANY_API_SERVER);
    set_option("key", COMPANY_KEY);

    // Hide server/network settings in the normal UI.
    set_option("hide-server-settings", "Y");
    set_option("hide-network-settings", "Y");

    // Prevent remote-side configuration tampering.
    set_option("allow-remote-config-modification", "N");
}
