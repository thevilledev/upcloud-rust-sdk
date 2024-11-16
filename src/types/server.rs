use serde::{Deserialize, Serialize};
use crate::types::common::{Labels, Label, Tags};

// Add these new constants
pub const SERVER_STATE_STARTED: &str = "started";
pub const SERVER_STATE_STOPPED: &str = "stopped";
pub const SERVER_STATE_MAINTENANCE: &str = "maintenance";
pub const SERVER_STATE_ERROR: &str = "error";

pub const VIDEO_MODEL_VGA: &str = "vga";
pub const VIDEO_MODEL_CIRRUS: &str = "cirrus";

pub const NIC_MODEL_E1000: &str = "e1000";
pub const NIC_MODEL_VIRTIO: &str = "virtio";
pub const NIC_MODEL_RTL8139: &str = "rtl8139";

pub const STOP_TYPE_SOFT: &str = "soft";
pub const STOP_TYPE_HARD: &str = "hard";

pub const PASSWORD_DELIVERY_NONE: &str = "none";
pub const PASSWORD_DELIVERY_EMAIL: &str = "email";
pub const PASSWORD_DELIVERY_SMS: &str = "sms";

pub const CREATE_SERVER_STORAGE_DEVICE_ACTION_CREATE: &str = "create";
pub const CREATE_SERVER_STORAGE_DEVICE_ACTION_CLONE: &str = "clone";
pub const CREATE_SERVER_STORAGE_DEVICE_ACTION_ATTACH: &str = "attach";


#[derive(Debug, Serialize, Deserialize)]
pub struct ServerResponse {
    pub servers: ServerList,
}

pub struct ServerRequest {
    pub server: Vec<Server>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerList {
    pub server: Vec<Server>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub core_number: Option<String>,
    pub created: i64,
    pub host: i64,
    pub hostname: String,
    #[serde(default)]
    pub labels: Labels,
    pub license: f64,
    pub memory_amount: String,
    pub plan: String,
    pub plan_ipv4_bytes: String,
    pub plan_ipv6_bytes: String,
    #[serde(default)]
    pub server_group: Option<String>,
    pub simple_backup: String,
    pub state: String,
    #[serde(default)]
    pub tags: Tags,
    pub title: String,
    pub uuid: String,
    pub zone: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ServerDetails {
    pub boot_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_optional_number_as_string")]
    pub core_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    pub hostname: String,
    #[serde(rename = "ip_addresses")]
    pub ip_addresses: IPAddressWrapper,
    pub license: Option<f64>,
    #[serde(serialize_with = "serialize_number_as_string")]
    pub memory_amount: i32,
    pub nic_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub plan: Option<String>,
    pub plan_ipv4_bytes: Option<String>,
    pub plan_ipv6_bytes: Option<String>,
    pub simple_backup: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "storage_devices")]
    pub storage_devices: StorageDeviceListWrapper,
    pub tags: TagWrapper,
    pub timezone: Option<String>,
    pub title: Option<String>,
    pub uuid: String,
    pub video_model: Option<String>,
    #[serde(serialize_with = "serialize_bool_as_yes_no")]
    pub remote_access_enabled: bool,
    pub remote_access_password: Option<String>,
    pub zone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageDeviceListWrapper {
    pub storage_device: Vec<ServerStorageDevice>,
}

impl Default for StorageDeviceListWrapper {
    fn default() -> Self {
        Self {
            storage_device: Vec::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagWrapper {
    pub tag: Vec<String>,
}

impl Default for TagWrapper {
    fn default() -> Self {
        Self {
            tag: Vec::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPAddress {
    pub access: String,
    pub family: String,
    pub address: String,
    pub ptr_record: String,
    pub server: String,
    pub floating: Option<bool>,
    pub mac: Option<String>,
    pub network: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStorageDevice {
    pub address: String,
    #[serde(rename = "part_of_plan")]
    pub part_of_plan: String,
    pub labels: Vec<Label>,
    pub storage: String,
    #[serde(rename = "storage_size")]
    pub storage_size: i32,
    #[serde(rename = "storage_encrypted")]
    pub storage_encrypted: String,
    #[serde(rename = "storage_tier")]
    pub storage_tier: String,
    #[serde(rename = "storage_title")]
    pub storage_title: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub storage_type: Option<String>,
    #[serde(rename = "boot_disk")]
    pub boot_disk: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupRule {
    pub interval: String,
    pub time: String,
    pub retention: i32,
}

#[derive(Debug, Serialize, Deserialize)]    
pub struct NetworkInterface {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<IPAddress>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub interface_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerNetworking {
    pub interfaces: Vec<NetworkInterface>,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateServerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_host: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    pub hostname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    pub login_user: LoginUser,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<CreateServerNetworking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_delivery: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<String>,
    #[serde(rename = "storage_devices")]
    pub storage_devices: StorageDeviceWrapper,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_password: Option<String>,
    pub zone: String,
}

#[derive(Debug, Default, Serialize)]
pub struct CreateServerRequestWrapper {
    pub server: CreateServerRequest,
}

impl CreateServerRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_zone(mut self, zone: impl Into<String>) -> Self {
        self.zone = zone.into();
        self
    }

    pub fn with_hostname(mut self, hostname: impl Into<String>) -> Self {
        self.hostname = hostname.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    // Builder pattern methods for optional fields
    pub fn with_plan(mut self, plan: String) -> Self {
        self.plan = Some(plan);
        self
    }

    pub fn with_core_number(mut self, core_number: i32) -> Self {
        self.core_number = Some(core_number);
        self
    }

    pub fn with_memory_amount(mut self, memory_amount: i32) -> Self {
        self.memory_amount = Some(memory_amount);
        self
    }

    pub fn with_storage_device(mut self, device: CreateServerStorageDevice) -> Self {
        self.storage_devices.storage_device.push(device);
        self
    }

    pub fn with_networking(mut self, networking: CreateServerNetworking) -> Self {
        self.networking = Some(networking);
        self
    }

    pub fn with_login_user(mut self, login_user: LoginUser) -> Self {
        self.login_user = login_user;
        self
    }

    pub fn with_user_data(mut self, user_data: String) -> Self {
        self.user_data = Some(user_data);
        self
    }

    pub fn with_avoid_host(mut self, avoid_host: i32) -> Self {
        self.avoid_host = Some(avoid_host);
        self
    }

    pub fn with_host(mut self, host: i32) -> Self {
        self.host = Some(host);
        self
    }

    pub fn with_boot_order(mut self, boot_order: String) -> Self {
        self.boot_order = Some(boot_order);
        self
    }

    pub fn with_firewall(mut self, firewall: String) -> Self {
        self.firewall = Some(firewall);
        self
    }

    pub fn with_labels(mut self, labels: Vec<Label>) -> Self {
        self.labels = Some(labels);
        self
    }

    pub fn with_metadata(mut self, metadata: String) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_nic_model(mut self, nic_model: String) -> Self {
        self.nic_model = Some(nic_model);
        self
    }

    pub fn with_password_delivery(mut self, password_delivery: String) -> Self {
        self.password_delivery = Some(password_delivery);
        self
    }

    pub fn with_server_group(mut self, server_group: String) -> Self {
        self.server_group = Some(server_group);
        self
    }

    pub fn with_simple_backup(mut self, simple_backup: String) -> Self {
        self.simple_backup = Some(simple_backup);
        self
    }

    pub fn with_timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    pub fn with_video_model(mut self, video_model: String) -> Self {
        self.video_model = Some(video_model);
        self
    }

    pub fn with_remote_access(
        mut self,
        enabled: bool,
        access_type: Option<String>,
        password: Option<String>,
    ) -> Self {
        self.remote_access_enabled = Some(enabled);
        self.remote_access_type = access_type;
        self.remote_access_password = password;
        self
    }

    pub fn build(self) -> CreateServerRequestWrapper {
        CreateServerRequestWrapper {
            server: self
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CreateServerStorageDevice {
    pub action: String,
    pub address: Option<String>,
    pub encrypted: Option<bool>,
    pub storage: String,
    pub title: Option<String>,
    // Storage size in gigabytes
    pub size: Option<i32>,
    pub tier: Option<String>,
    #[serde(rename = "type")]
    pub storage_type: Option<String>,
    pub backup_rule: Option<BackupRule>,
}

#[derive(Debug, Serialize)]
pub struct CreateServerNetworking {
    pub interfaces: InterfaceWrapper,
}

#[derive(Debug, Default, Serialize)]
pub struct InterfaceWrapper {
    pub interface: Vec<CreateServerInterface>,
}

#[derive(Debug, Serialize)]
pub struct CreateServerInterface {
    pub ip_addresses: IPAddressWrapper,
    #[serde(rename = "type")]
    pub interface_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServerIPAddress {
    pub family: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IPAddressWrapper {
    pub ip_address: Vec<CreateServerIPAddress>,
}

#[derive(Debug, Serialize)]
pub struct LoginUser {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_password: Option<String>,
}

impl Default for LoginUser {
    fn default() -> Self {
        Self {
            username: "root".to_string(),
            ssh_keys: None,
            create_password: Some("yes".to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StartServerRequest {
    pub host: Option<i32>,
    pub avoid_host: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct StopServerRequest {
    pub stop_type: Option<String>,
    pub timeout: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct RestartServerRequest {
    pub stop_type: Option<String>,
    pub timeout: Option<i64>,
    pub timeout_action: Option<String>,
    pub host: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ModifyServerRequest {
    pub boot_order: Option<String>,
    pub core_number: Option<i32>,
    pub memory_amount: Option<i32>,
    pub title: Option<String>,
    pub zone: Option<String>,
    // Add other fields as needed
}

#[derive(Debug, Serialize)]
pub struct StorageDeviceWrapper {
    pub storage_device: Vec<CreateServerStorageDevice>,
}

impl Default for StorageDeviceWrapper {
    fn default() -> Self {
        Self {
            storage_device: Vec::new()
        }
    }
}

// Add these serializer helper functions
fn serialize_number_as_string<S>(num: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&num.to_string())
}

fn serialize_bool_as_yes_no<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(if *value { "yes" } else { "no" })
}

// Add this new serializer function
fn serialize_optional_number_as_string<S>(num: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match num {
        Some(n) => serializer.serialize_str(&n.to_string()),
        None => serializer.serialize_none(),
    }
}