use serde::{Deserialize, Serialize};

/// OwnTracks uses JSON format for its message payloads. The different payload
/// types are identified by a mandatory `_type` element. Depending on the app
/// platform, different payload types are supported.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum OwnTracksMessage {
    #[serde(rename = "location")]
    Location(LocationMessage),
    #[serde(rename = "lwt")]
    Lwt(LwtMessage),
    #[serde(rename = "waypoint")]
    Waypoint(WaypointMessage),
    #[serde(rename = "transition")]
    Transition(TransitionMessage),
    #[serde(rename = "configuration")]
    Configuration(ConfigurationMessage),
    #[serde(rename = "status")]
    Status(StatusMessage),
    #[serde(rename = "beacon")]
    Beacon(BeaconMessage),
    #[serde(rename = "cmd")]
    Cmd(CmdMessage),
    #[serde(rename = "steps")]
    Steps(StepsMessage),
    #[serde(rename = "card")]
    Card(CardMessage),
    #[serde(rename = "waypoints")]
    Waypoints(WaypointsMessage),
    #[serde(rename = "encrypted")]
    Encrypted(EncryptedMessage),
    #[serde(rename = "request")]
    Request(RequestMessage),
}

/// This location object describes the location of the device that reported it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMessage {
    /// Accuracy of the reported location in meters without unit
    /// (iOS,Android/integer/meters/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc: Option<i32>,

    /// Altitude measured above sea level (iOS,Android/integer/meters/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<i32>,

    /// Device battery level (iOS,Android/integer/percent/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batt: Option<i32>,

    /// Battery Status 0=unknown, 1=unplugged, 2=charging, 3=full (iOS, Android)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<i32>,

    /// Course over ground (iOS/integer/degree/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cog: Option<i32>,

    /// latitude (iOS,Android/float/degree/required)
    pub lat: f64,

    /// longitude (iOS,Android/float/degree/required)
    pub lon: f64,

    /// radius around the region when entering/leaving
    /// (iOS/integer/meters/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rad: Option<i32>,

    /// trigger for the location report (iOS,Android/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,

    /// Tracker ID used to display the initials of a user
    /// (iOS,Android/string/optional) required for `http` mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,

    /// UNIX epoch timestamp in seconds of the location fix
    /// (iOS,Android/integer/epoch/required)
    pub tst: i64,

    /// vertical accuracy of the `alt` element (iOS/integer/meters/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vac: Option<i32>,

    /// velocity (iOS,Android/integer/kmh/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vel: Option<i32>,

    /// barometric pressure (iOS/float/kPa/optional/extended data)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,

    /// point of interest name (iOS/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poi: Option<String>,

    /// Base64 encoded image associated with the poi (iOS/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    /// Name of the image associated with the poi (iOS/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagename: Option<String>,

    /// Internet connectivity status (route to host) when the message is created
    /// (iOS,Android/string/optional/extended data)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn: Option<String>,

    /// name of the tag (iOS/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// contains the original publish topic (e.g. `owntracks/jane/phone`)
    /// (iOS,Android >= 2.4,string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    /// contains a list of regions the device is currently in (e.g.
    /// `["Home","Garage"]`). Might be empty. (iOS,Android/list of
    /// strings/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inregions: Option<Vec<String>>,

    /// contains a list of region IDs the device is currently in (e.g.
    /// `["6da9cf","3defa7"]`). Might be empty. (iOS,Android/list of
    /// strings/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inrids: Option<Vec<String>>,

    /// if available, is the unique name of the WLAN. (iOS,string/optional)
    #[serde(rename = "SSID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,

    /// if available, identifies the access point. (iOS,string/optional)
    #[serde(rename = "BSSID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,

    /// identifies the time at which the message is constructed (if it differs
    /// from `tst`, which is the timestamp of the GPS fix)
    /// (iOS,Android/integer/epoch/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,

    /// identifies the monitoring mode at which the message is constructed
    /// (significant=`1`, move=`2`) (iOS/integer/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<i32>,

    /// random identifier to be used by consumers to correlate & distinguish
    /// send/return messages (Android/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,

    /// contains a list of motion states detected by iOS' motion manager
    /// (iOS/list of strings/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motionactivities: Option<Vec<String>>,

    /// total distance of the device (float/kilometers/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odometer: Option<f64>,

    /// total hours of operation (float/seconds/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmc: Option<f64>,

    /// voltage of the battery (float/volts/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubatt: Option<f64>,

    /// voltage of the external power source (float/volts/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uext: Option<f64>,

    /// vehicle identification number (string/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<String>,

    /// identification number (string/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,

    /// vehicle name (string/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// is duration since ignition on (float/seconds/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub don: Option<f64>,

    /// is duration since ignition off (float/seconds/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doff: Option<f64>,

    /// is analog input voltage (float/volts/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aiv: Option<f64>,

    /// is engine rounds per minute (float/rounds per minute/optional) -
    /// Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpm: Option<f64>,

    /// is fuel consumption (float/L per 100km/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcon: Option<f64>,

    /// is fuel level (float/percent/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flvl: Option<f64>,

    /// is number of analog inputs (integer/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anum: Option<i32>,

    /// is can data (string/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can: Option<String>,

    /// is status of digital input 1 (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub din1: Option<bool>,

    /// is status of digital input 2 (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub din2: Option<bool>,

    /// is status of digital output 1 (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dout1: Option<bool>,

    /// is status of digital output 2 (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dout2: Option<bool>,

    /// is status of ignition (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ign: Option<bool>,

    /// is motion status (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion: Option<bool>,

    /// is status of tow sensor (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tow: Option<bool>,

    /// is status of fake tow sensor (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fake: Option<bool>,

    /// is status of motion sensor (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sens: Option<bool>,

    /// is epoch when message was sent (integer/epoch/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<i64>,

    /// is mobile country code (integer/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<i32>,

    /// is mobile network code (integer/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnc: Option<i32>,

    /// is location area code (string/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lac: Option<String>,

    /// is cell id (string/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,

    /// is non movement detection status (boolean/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nmds: Option<bool>,

    /// queclink record id and type (integer/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rit: Option<i32>,

    /// queclink record type (integer/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rty: Option<i32>,

    /// queclink record id (integer/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<i32>,

    /// queclink motion state (integer/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mst: Option<i32>,

    /// is counter of message (string/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,

    /// raw data (string/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_line: Option<String>,

    /// number of ignored positions (integer/optional) - Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter: Option<i32>,

    /// indicates counter positions have been ignored (boolean/optional) -
    /// Queclink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,

    /// Allows for flexible inclusion of additional analog input fields -
    /// Queclink
    #[serde(flatten)]
    pub additional_fields: std::collections::HashMap<String, serde_json::Value>,
}

/// A last will and testament is published automatically by the MQTT broker when
/// it loses contact with the app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwtMessage {
    /// UNIX epoch timestamp at which the app first connected
    /// (iOS,Android/integer/epoch/required)
    pub tst: i64,
}

/// Waypoints / regions denote specific geographical regions that you want to
/// keep track of.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointMessage {
    /// Name of the waypoint that is included in the sent transition message
    /// (iOS,Android,string/required)
    pub desc: String,

    /// Latitude (iOS,Android/float/degree/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    /// Longitude (iOS,Android/float/degree/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,

    /// Radius around the latitude and longitude coordinates
    /// (iOS,Android/integer/meters/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rad: Option<i32>,

    /// Timestamp of creation of region (iOS,Android/integer/epoch/required)
    pub tst: i64,

    /// UUID of the BLE Beacon (iOS/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    /// Major number of the BLE Beacon (iOS/integer/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i32>,

    /// Minor number of the BLE Beacon (iOS/integer/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i32>,

    /// region ID, created automatically (iOS/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
}

/// A transition message is sent, when entering or leaving a previously
/// configured geographical region or BLE Beacon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionMessage {
    /// Timestamp of waypoint creation (iOS,Android/integer/epoch/required)
    pub wtst: i64,

    /// Latitude at which the event occured (iOS,Android/float/degree/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,

    /// Longitude at which the event occured (iOS,Android/float/degree/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,

    /// Timestamp at which the event occured
    /// (iOS,Android/integer/epoch/required)
    pub tst: i64,

    /// Accuracy of the geographical coordinates
    /// (iOS,Android/int/meters/required)
    pub acc: i32,

    /// Tracker ID (iOS/string/none/optional) required in http mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,

    /// Event that triggered the transition (iOS,Android/string/required)
    pub event: String,

    /// Name of the waypoint (iOS,Android/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    /// Trigger of the event (iOS,Android/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,

    /// Region ID (iOS/Android, after January 2021)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
}

/// The device configuration can be imported and exported as JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationMessage {
    /// time in minutes of non-movement before switching from move to
    /// significant mode (iOS/integer/minutes/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapt: Option<i32>,

    /// Respond to reportLocation cmd message (iOS/boolean)
    #[serde(rename = "allowRemoteLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_location: Option<bool>,

    /// disable TLS certificate checks **insecure** (iOS/boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowinvalidcerts: Option<bool>,

    /// Use `username` and `password` for endpoint authentication
    /// (iOS,Android/boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<bool>,

    /// Autostart the app on device boot (Android/boolean)
    #[serde(rename = "autostartOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart_on_boot: Option<bool>,

    /// MQTT endpoint clean session (iOS,Android/boolean)
    #[serde(rename = "cleanSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_session: Option<bool>,

    /// client id to use for MQTT connect (iOS,Android/string)
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /// Name of the client pkcs12 file (iOS/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clientpkcs: Option<String>,

    /// Respond to cmd messages (iOS,Android/boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<bool>,

    /// TCP timeout for establishing a connection to the MQTT / HTTP broker
    /// (Android/int)
    #[serde(rename = "connectionTimeoutSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_timeout_seconds: Option<i32>,

    /// Number of days to keep locations stored locally (iOS/integer/days)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,

    /// whether or not debug logs should be shown in the log viewer / exporter
    /// activity (Android/bool)
    #[serde(rename = "debugLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_log: Option<bool>,

    /// id of the device used for `pubTopicBase` and `clientId` construction
    /// (iOS,Android/string)
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,

    /// battery level below which to downgrade monitoring from move mode
    /// (iOS/integer/percent/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downgrade: Option<i32>,

    /// the secret key used for payload encryption (iOS,Android/string)
    #[serde(rename = "encryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,

    /// Add extended data attributes to location messages (iOS,Android/boolean)
    #[serde(rename = "extendedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_data: Option<bool>,

    /// MQTT endpoint host (iOS,Android/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// extra HTTP headers (iOS only/string)
    #[serde(rename = "httpHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_headers: Option<String>,

    /// Location accuracy below which reports are supressed
    /// (iOS,Android/integer/meters)
    #[serde(rename = "ignoreInaccurateLocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_inaccurate_locations: Option<i32>,

    /// Number of days after which location updates are assumed stale
    /// (iOS,Android/integer/days)
    #[serde(rename = "ignoreStaleLocations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_stale_locations: Option<i32>,

    /// MQTT endpoint keepalive (iOS,Android/integer/seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepalive: Option<i32>,

    /// maximum distance between location source updates
    /// (iOS,Android/integer/meters)
    #[serde(rename = "locatorDisplacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator_displacement: Option<i32>,

    /// maximum interval between location source updates
    /// (iOS,Android/integer/seconds)
    #[serde(rename = "locatorInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator_interval: Option<i32>,

    /// source/power setting for location updates (Android/integer)
    #[serde(rename = "locatorPriority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator_priority: Option<i32>,

    /// Locks settings screen on device for editing (iOS/boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    /// Number of notifications to store historically (iOS/integer)
    #[serde(rename = "maxHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_history: Option<i32>,

    /// Endpoint protocol mode (iOS,Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,

    /// Location reporting mode (iOS,Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<i32>,

    /// MQTT broker protocol level (iOS,Android/integer)
    #[serde(rename = "mqttProtocolLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mqtt_protocol_level: Option<i32>,

    /// Show last reported location in ongoing notification (Android/boolean)
    #[serde(rename = "notificationLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_location: Option<bool>,

    /// API key for alternate Geocoding provider (Android/string)
    #[serde(rename = "opencageApiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opencage_api_key: Option<String>,

    /// URL template for alternate tile provider (iOS/string)
    #[serde(rename = "osmTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osm_template: Option<String>,

    /// Attribution text shown with OSM map (iOS/string)
    #[serde(rename = "osmCopyright")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osm_copyright: Option<String>,

    /// Passphrase of the client pkcs12 file (iOS/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,

    /// Endpoint password (iOS,Android/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// if true, requests that that the device provide locations no faster than
    /// the specified interval (Android/bool)
    #[serde(rename = "pegLocatorFastestIntervalToInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peg_locator_fastest_interval_to_interval: Option<bool>,

    /// Interval in which location messages of with `t`:`p` are reported
    /// (Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping: Option<i32>,

    /// MQTT endpoint port (iOS,Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,

    /// Number of locations to keep for friends and own device and display
    /// (iOS/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<i32>,

    /// MQTT topic base to which the app publishes (iOS,Android/string)
    #[serde(rename = "pubTopicBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_topic_base: Option<String>,

    /// MQTT retain flag for reported messages (iOS,Android/boolean)
    #[serde(rename = "pubRetain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_retain: Option<bool>,

    /// MQTT QoS level for reported messages (iOS,Android/integer)
    #[serde(rename = "pubQos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_qos: Option<i32>,

    /// Beacon ranging (iOS/boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranging: Option<bool>,

    /// Allow remote configuration by sending a `setConfiguration` cmd message
    /// (Android/boolean)
    #[serde(rename = "remoteConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_configuration: Option<bool>,

    /// subscribe to `subTopic` via MQTT (iOS,Android/boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<bool>,

    /// A whitespace separated list of MQTT topics to which the app subscribes
    /// (iOS,Android/string)
    #[serde(rename = "subTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_topic: Option<String>,

    /// MQTT subscription QoS (iOS,Android/boolean)
    #[serde(rename = "subQos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_qos: Option<bool>,

    /// Two digit Tracker ID used to display short name and default face of a
    /// user (iOS,Android/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,

    /// MQTT endpoint TLS connection (iOS,Android/boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,

    /// Passphrase of the client pkcs12 file (Android/string)
    #[serde(rename = "tlsClientCrtPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_client_crt_password: Option<String>,

    /// HTTP endpoint URL to which messages are POSTed (iOS,Android/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Endpoint username (iOS,Android/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// use MQTT over Websocket (iOS,Android/boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<bool>,

    /// Array of waypoint messages (iOS,Android/array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waypoints: Option<Vec<WaypointMessage>>,
}

/// The device status contains information about the settings on the device
/// which are not configured in the app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMessage {
    /// iOS specific elements
    #[serde(rename = "iOS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios: Option<IosStatus>,

    /// Android specific elements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<AndroidStatus>,

    /// random identifier to be used by consumers to correlate & distinguish
    /// send/return messages (Android/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
}

/// iOS specific status elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosStatus {
    /// used for steps (iOS/string)
    #[serde(rename = "altimeterAuthorizationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altimeter_authorization_status: Option<String>,

    /// used for steps (iOS/boolean)
    #[serde(rename = "altimeterIsRelativeAltitudeAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altimeter_is_relative_altitude_available: Option<bool>,

    /// background refresh status (iOS/boolean)
    #[serde(rename = "backgroundRefreshStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_refresh_status: Option<bool>,

    /// used for default DeviceID (iOS/string)
    #[serde(rename = "deviceIdentifierForVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_identifier_for_vendor: Option<String>,

    /// device model (iOS/string)
    #[serde(rename = "deviceModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,

    /// device system name (iOS/string)
    #[serde(rename = "deviceSystemName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_system_name: Option<String>,

    /// device system version (iOS/string)
    #[serde(rename = "deviceSystemVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_system_version: Option<String>,

    /// device user interface idiom (iOS/string)
    #[serde(rename = "deviceUserInterfaceIdiom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_user_interface_idiom: Option<String>,

    /// the current locale on the device (iOS/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// uses metric system (iOS/boolean)
    #[serde(rename = "localeUsesMetricSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_uses_metric_system: Option<bool>,

    /// used for all locations (iOS/string)
    #[serde(rename = "locationManagerAuthorizationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_manager_authorization_status: Option<String>,

    /// used for motionactivities (iOS/string)
    #[serde(rename = "motionActivityManagerAuthorizationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_activity_manager_authorization_status: Option<String>,

    /// used for motionactivities (iOS/boolean)
    #[serde(rename = "motionActivityManagerIsActivityAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_activity_manager_is_activity_available: Option<bool>,

    /// pedometer distance available (iOS/boolean)
    #[serde(rename = "pedometerIsDistanceAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pedometer_is_distance_available: Option<bool>,

    /// pedometer floor counting available (iOS/boolean)
    #[serde(rename = "pedometerIsFloorCountingAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pedometer_is_floor_counting_available: Option<bool>,

    /// pedometer step counting available (iOS/boolean)
    #[serde(rename = "pedometerIsStepCountingAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pedometer_is_step_counting_available: Option<bool>,

    /// version of the OwnTracks app (iOS/string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Android specific status elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidStatus {
    /// app can hibernate if not used (Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hib: Option<i32>,

    /// app is configured with battery optimizations (Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bo: Option<i32>,

    /// app location permissions (Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc: Option<i32>,

    /// phone power save mode (Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<i32>,

    /// wifi is on/off (Android/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi: Option<i32>,
}

/// These messages are published when beacon ranging (iOS only) is enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconMessage {
    /// name of the seen beacon (iOS/String)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    /// UUID of the seen beacon (iOS/String)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    /// Major number of the seen beacon (iOS/integer/epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i32>,

    /// Minor number of the seen beacon (iOS/integer/epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i32>,

    /// Timestamp at which the beacon was seen (iOS/integer/epoch)
    pub tst: i64,

    /// Accuracy of the proximity value (iOS/integer/meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc: Option<i32>,

    /// Received signal strength of the beacon (iOS/integer/decibel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rssi: Option<i32>,

    /// Relative distance to the beacon (iOS/integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prox: Option<i32>,
}

/// Command message for remote device control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmdMessage {
    /// action to be performed by the device (iOS,Android/string)
    pub action: String,

    /// Configuration message to import (iOS,Android/required) - for
    /// setConfiguration action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationMessage>,

    /// Array of waypoint messages to import (iOS,Android/array/required) - for
    /// setWaypoints action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waypoints: Option<WaypointsMessage>,

    /// Timestamp (iOS/epoch/optional) - for reportSteps action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// Timestamp (iOS/epoch/optional) - for reportSteps action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Step counter data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepsMessage {
    /// Timestamp of the request (iOS/integer/epoch)
    pub tst: i64,

    /// Steps walked with the device in the specfied time period
    /// (iOS/integer/steps)
    pub steps: i32,

    /// Effective start of time period (iOS/integer/epoch)
    pub from: i64,

    /// Effective end of time period (iOS/integer/epoch)
    pub to: i64,
}

/// Apps read Card to display a name and icon for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardMessage {
    /// Tracker ID to associate the card with (iOS,Android/string/required)
    pub tid: String,

    /// Name to identify a user (iOS,Android/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Base64 encoded PNG image that is displayed instead of the Tracker ID
    /// (iOS,Android/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<String>,
}

/// The app can export a list of configured waypoints to the endpoint
/// ../waypoints (plural).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointsMessage {
    /// Identification of what created the array. Ignored by the apps
    /// (iOS,Android/string/optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _creator: Option<String>,

    /// Array of waypoint messages (iOS,Android/array/required)
    pub waypoints: Vec<WaypointMessage>,
}

/// Apps can optionally encrypt outgoing messages with a shared symmetric key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Encrypted and Base64 encoded original JSON message
    /// (iOS,Android/string/required)
    pub data: String,
}

/// Apps can request the creation of tours which elicit a `cmd` response from
/// the Recorder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMessage {
    /// Type of request (string)
    pub request: String,

    /// Tour configuration data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tour: Option<serde_json::Value>,
}
