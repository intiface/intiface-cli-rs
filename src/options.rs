use getset::{CopyGetters, Getters};
use tracing::Level;

#[derive(CopyGetters, Getters, Default, Debug, Clone)]
pub struct EngineOptions {
  #[getset(get="pub")]
  sentry_api_key: Option<String>,
  #[getset(get="pub")]
  ipc_pipe_name: Option<String>,
  #[getset(get="pub")]
  device_config_json: Option<String>,
  #[getset(get="pub")]
  user_device_config_json: Option<String>,
  #[getset(get="pub")]
  server_name: String,
  #[getset(get_copy="pub")]
  crash_reporting: bool,
  #[getset(get_copy="pub")]
  websocket_use_all_interfaces: bool,
  #[getset(get_copy="pub")]
  websocket_port: Option<u16>,
  #[getset(get_copy="pub")]
  frontend_websocket_port: Option<u16>,
  #[getset(get_copy="pub")]
  frontend_in_process_channel: bool,
  #[getset(get_copy="pub")]
  max_ping_time: u32,
  #[getset(get="pub")]
  log_level: Option<String>,
  #[getset(get_copy="pub")]
  allow_raw_messages: bool,
  #[getset(get_copy="pub")]
  use_bluetooth_le: bool,
  #[getset(get_copy="pub")]
  use_serial_port: bool,
  #[getset(get_copy="pub")]
  use_hid: bool,
  #[getset(get_copy="pub")]
  use_lovense_dongle_serial: bool,
  #[getset(get_copy="pub")]
  use_lovense_dongle_hid: bool,
  #[getset(get_copy="pub")]
  use_xinput: bool,
  #[getset(get_copy="pub")]
  use_lovense_connect: bool,
  #[getset(get_copy="pub")]
  use_device_websocket_server: bool,
  #[getset(get_copy="pub")]
  device_websocket_server_port: Option<u16>,
  #[getset(get_copy="pub")]
  crash_main_thread: bool,
  #[getset(get_copy="pub")]
  crash_task_thread: bool,
}

#[derive(Default, Debug, Clone)]
pub struct EngineOptionsExternal {
  pub sentry_api_key: Option<String>,
  pub ipc_pipe_name: Option<String>,
  pub device_config_json: Option<String>,
  pub user_device_config_json: Option<String>,
  pub server_name: String,
  pub crash_reporting: bool,
  pub websocket_use_all_interfaces: bool,
  pub websocket_port: Option<u16>,
  pub frontend_websocket_port: Option<u16>,
  pub frontend_in_process_channel: bool,
  pub max_ping_time: u32,
  pub log_level: Option<String>,
  pub allow_raw_messages: bool,
  pub use_bluetooth_le: bool,
  pub use_serial_port: bool,
  pub use_hid: bool,
  pub use_lovense_dongle_serial: bool,
  pub use_lovense_dongle_hid: bool,
  pub use_xinput: bool,
  pub use_lovense_connect: bool,
  pub use_device_websocket_server: bool,
  pub device_websocket_server_port: Option<u16>,
  pub crash_main_thread: bool,
  pub crash_task_thread: bool,
}

impl From<EngineOptionsExternal> for EngineOptions {
  fn from(other: EngineOptionsExternal) -> Self {
    Self {
      sentry_api_key: other.sentry_api_key,
      ipc_pipe_name: other.ipc_pipe_name,
      device_config_json: other.device_config_json,
      user_device_config_json: other.user_device_config_json,
      server_name: other.server_name,
      crash_reporting: other.crash_reporting,
      websocket_use_all_interfaces: other.websocket_use_all_interfaces,
      websocket_port: other.websocket_port,
      frontend_websocket_port: other.frontend_websocket_port,
      frontend_in_process_channel: other.frontend_in_process_channel,
      max_ping_time: other.max_ping_time,
      log_level: other.log_level,
      allow_raw_messages: other.allow_raw_messages,
      use_bluetooth_le: other.use_bluetooth_le,
      use_serial_port: other.use_serial_port,
      use_hid: other.use_hid,
      use_lovense_dongle_serial: other.use_lovense_dongle_serial,
      use_lovense_dongle_hid: other.use_lovense_dongle_hid,
      use_xinput: other.use_xinput,
      use_lovense_connect: other.use_lovense_connect,
      use_device_websocket_server: other.use_device_websocket_server,
      device_websocket_server_port: other.device_websocket_server_port,
      crash_main_thread: other.crash_main_thread,
      crash_task_thread: other.crash_task_thread
    }
  }
}

#[derive(Default)]
pub struct EngineOptionsBuilder {
  options: EngineOptions
}

impl EngineOptionsBuilder {
  pub fn sentry_api_key(&mut self, value: &str) -> &mut Self {
    self.options.sentry_api_key = Some(value.to_owned());
    self
  }

  pub fn ipc_pipe_name(&mut self, value: &str) -> &mut Self {
    self.options.sentry_api_key = Some(value.to_owned());
    self
  }

  pub fn device_config_json(&mut self, value: &str) -> &mut Self {
    self.options.device_config_json = Some(value.to_owned());
    self
  }

  pub fn user_device_config_json(&mut self, value: &str) -> &mut Self {
    self.options.user_device_config_json = Some(value.to_owned());
    self
  }

  pub fn server_name(&mut self, value: &str) -> &mut Self {
    self.options.server_name = value.to_owned();
    self
  }

  pub fn crash_main_thread(&mut self, value: bool) -> &mut Self {
    #[cfg(debug_assertions)]
    {
      self.options.crash_main_thread = value;
    }
    self
  }

  pub fn crash_task_thread(&mut self, value: bool) -> &mut Self {
    #[cfg(debug_assertions)]
    {
      self.options.crash_main_thread = value;
    }
    self
  }

  pub fn crash_reporting(&mut self, value: bool) -> &mut Self {
    self.options.crash_reporting = value;
    self
  }

  pub fn websocket_use_all_interfaces(&mut self, value: bool) -> &mut Self {
    self.options.websocket_use_all_interfaces = value;
    self
  }

  pub fn allow_raw_messages(&mut self, value: bool) -> &mut Self {
    self.options.allow_raw_messages = value;
    self
  }

  pub fn use_bluetooth_le(&mut self, value: bool) -> &mut Self {
    self.options.use_bluetooth_le = value;
    self
  }

  pub fn use_serial_port(&mut self, value: bool) -> &mut Self {
    self.options.use_serial_port = value;
    self
  }

  pub fn use_hid(&mut self, value: bool) -> &mut Self {
    self.options.use_hid = value;
    self
  }

  pub fn use_lovense_dongle_serial(&mut self, value: bool) -> &mut Self {
    self.options.use_lovense_dongle_serial = value;
    self
  }

  pub fn use_lovense_dongle_hid(&mut self, value: bool) -> &mut Self {
    self.options.use_lovense_dongle_hid = value;
    self
  }

  pub fn use_xinput(&mut self, value: bool) -> &mut Self {
    self.options.use_xinput = value;
    self
  }

  pub fn use_lovense_connect(&mut self, value: bool) -> &mut Self {
    self.options.use_lovense_connect = value;
    self
  }

  pub fn use_device_websocket_server(&mut self, value: bool) -> &mut Self {
    self.options.use_device_websocket_server = value;
    self
  }

  pub fn websocket_port(&mut self, port: u16) -> &mut Self {
    self.options.websocket_port = Some(port);
    self
  }

  pub fn frontend_websocket_port(&mut self, port: u16) -> &mut Self {
    self.options.frontend_websocket_port = Some(port);
    self
  }

  pub fn frontend_in_process_channel(&mut self, value: bool) -> &mut Self {
    self.options.frontend_in_process_channel = value;
    self
  }

  pub fn device_websocket_server_port(&mut self, port: u16) -> &mut Self {
    self.options.device_websocket_server_port = Some(port);
    self
  }

  pub fn max_ping_time(&mut self, value: u32) -> &mut Self {
    self.options.max_ping_time = value;
    self
  }

  pub fn log_level(&mut self, level: Level) -> &mut Self {
    self.options.log_level = Some(level.to_string());
    self
  }

  pub fn finish(&mut self) -> EngineOptions {
    self.options.clone()
  }
}

