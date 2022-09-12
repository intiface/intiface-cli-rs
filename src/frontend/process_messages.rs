use serde::{Serialize, Deserialize};

// Everything in this struct is an object, even if it has null contents. This is to make other
// languages happy when trying to recompose JSON into objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineMessage {
  MessageVersion{version: u32},
  EngineLog{message: String},
  EngineStarted{},
  EngineError{error: String},
  EngineStopped{},
  ClientConnected{client_name: String},
  ClientDisconnected{},
  DeviceConnected { 
    name: String, 
    index: u32, 
    address: String, 
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String> 
  },
  DeviceDisconnected{index: u32},
  ClientRejected{reason: String}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntifaceMessage {
  Stop
}

/*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineLogMessage {
  
}
*/