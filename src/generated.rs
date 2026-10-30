// ASAM OpenDRIVE V1.9.0
//
// © by ASAM e.V., 2026
//
// ASAM OpenDRIVE defines a file format for the precise analytical description of road networks
//
//
// Any use is Limited to the scope described in the ASAM license terms.
// This file is distributable in accordance with the ASAM license terms.
// See www.asam.net/license.html for further details.

pub type OpenDrive = OpenDriveElementType;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct OpenDriveElementType {
    #[cfg_attr(feature = "serde", serde(rename = "header"))]
    pub header: THeader,
    #[cfg_attr(feature = "serde", serde(default, rename = "road"))]
    pub road: ::std::vec::Vec<TRoad>,
    #[cfg_attr(feature = "serde", serde(default, rename = "controller"))]
    pub controller: ::std::vec::Vec<TController>,
    #[cfg_attr(feature = "serde", serde(default, rename = "junction"))]
    pub junction: ::std::vec::Vec<TJunction>,
    #[cfg_attr(feature = "serde", serde(default, rename = "junctionGroup"))]
    pub junction_group: ::std::vec::Vec<TJunctionGroup>,
    #[cfg_attr(feature = "serde", serde(default, rename = "station"))]
    pub station: ::std::vec::Vec<TStation>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "vmsGroup"))]
    pub vms_group: ::std::vec::Vec<TSignalGroupVmsGroup>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct OpenDriveElement;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EAccessRestrictionType {
    #[cfg_attr(feature = "serde", serde(rename = "simulator"))]
    Simulator,
    #[cfg_attr(feature = "serde", serde(rename = "autonomousTraffic"))]
    AutonomousTraffic,
    #[cfg_attr(feature = "serde", serde(rename = "pedestrian"))]
    Pedestrian,
    #[cfg_attr(feature = "serde", serde(rename = "passengerCar"))]
    PassengerCar,
    #[cfg_attr(feature = "serde", serde(rename = "bus"))]
    Bus,
    #[cfg_attr(feature = "serde", serde(rename = "delivery"))]
    Delivery,
    #[cfg_attr(feature = "serde", serde(rename = "emergency"))]
    Emergency,
    #[cfg_attr(feature = "serde", serde(rename = "taxi"))]
    Taxi,
    #[cfg_attr(feature = "serde", serde(rename = "throughTraffic"))]
    ThroughTraffic,
    #[cfg_attr(feature = "serde", serde(rename = "truck"))]
    Truck,
    #[cfg_attr(feature = "serde", serde(rename = "bicycle"))]
    Bicycle,
    #[cfg_attr(feature = "serde", serde(rename = "motorcycle"))]
    Motorcycle,
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "trucks"))]
    Trucks,
    #[cfg_attr(feature = "serde", serde(rename = "HOV"))]
    Hov,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EBorderType {
    #[cfg_attr(feature = "serde", serde(rename = "concrete"))]
    Concrete,
    #[cfg_attr(feature = "serde", serde(rename = "curb"))]
    Curb,
    #[cfg_attr(feature = "serde", serde(rename = "paint"))]
    Paint,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EBridgeType {
    #[cfg_attr(feature = "serde", serde(rename = "concrete"))]
    Concrete,
    #[cfg_attr(feature = "serde", serde(rename = "steel"))]
    Steel,
    #[cfg_attr(feature = "serde", serde(rename = "brick"))]
    Brick,
    #[cfg_attr(feature = "serde", serde(rename = "wood"))]
    Wood,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EConnectionType {
    #[cfg_attr(feature = "serde", serde(rename = "default"))]
    Default,
    #[cfg_attr(feature = "serde", serde(rename = "virtual"))]
    Virtual,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EContactPoint {
    #[cfg_attr(feature = "serde", serde(rename = "start"))]
    Start,
    #[cfg_attr(feature = "serde", serde(rename = "end"))]
    End,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "::std::string::String", into = "::std::string::String")
)]
pub enum ECountryCode {
    #[cfg_attr(feature = "serde", serde(rename = "OpenDRIVE"))]
    OpenDrive,
    #[cfg_attr(feature = "serde", serde(rename = "Austria"))]
    Austria,
    #[cfg_attr(feature = "serde", serde(rename = "Brazil"))]
    Brazil,
    #[cfg_attr(feature = "serde", serde(rename = "China"))]
    China,
    #[cfg_attr(feature = "serde", serde(rename = "France"))]
    France,
    #[cfg_attr(feature = "serde", serde(rename = "Germany"))]
    Germany,
    #[cfg_attr(feature = "serde", serde(rename = "Italy"))]
    Italy,
    #[cfg_attr(feature = "serde", serde(rename = "Switzerland"))]
    Switzerland,
    #[cfg_attr(feature = "serde", serde(rename = "USA"))]
    Usa,
    String(::std::string::String),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ECountryCodeDeprecated {
    #[cfg_attr(feature = "serde", serde(rename = "OpenDRIVE"))]
    OpenDrive,
    #[cfg_attr(feature = "serde", serde(rename = "Austria"))]
    Austria,
    #[cfg_attr(feature = "serde", serde(rename = "Brazil"))]
    Brazil,
    #[cfg_attr(feature = "serde", serde(rename = "China"))]
    China,
    #[cfg_attr(feature = "serde", serde(rename = "France"))]
    France,
    #[cfg_attr(feature = "serde", serde(rename = "Germany"))]
    Germany,
    #[cfg_attr(feature = "serde", serde(rename = "Italy"))]
    Italy,
    #[cfg_attr(feature = "serde", serde(rename = "Switzerland"))]
    Switzerland,
    #[cfg_attr(feature = "serde", serde(rename = "USA"))]
    Usa,
}
pub type ECountryCodeIso3166Alpha2 = ::std::string::String;
pub type ECountryCodeIso3166Alpha3Deprecated = ::std::string::String;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EDataQualityRawDataPostProcessing {
    #[cfg_attr(feature = "serde", serde(rename = "raw"))]
    Raw,
    #[cfg_attr(feature = "serde", serde(rename = "cleaned"))]
    Cleaned,
    #[cfg_attr(feature = "serde", serde(rename = "processed"))]
    Processed,
    #[cfg_attr(feature = "serde", serde(rename = "fused"))]
    Fused,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EDataQualityRawDataSource {
    #[cfg_attr(feature = "serde", serde(rename = "sensor"))]
    Sensor,
    #[cfg_attr(feature = "serde", serde(rename = "cadaster"))]
    Cadaster,
    #[cfg_attr(feature = "serde", serde(rename = "custom"))]
    Custom,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EDirection {
    #[cfg_attr(feature = "serde", serde(rename = "same"))]
    Same,
    #[cfg_attr(feature = "serde", serde(rename = "opposite"))]
    Opposite,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EElementDir {
    #[cfg_attr(feature = "serde", serde(rename = "+"))]
    Plus,
    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    Minus,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EJunctionGroupType {
    #[cfg_attr(feature = "serde", serde(rename = "roundabout"))]
    Roundabout,
    #[cfg_attr(feature = "serde", serde(rename = "unknown"))]
    Unknown,
    #[cfg_attr(feature = "serde", serde(rename = "complexJunction"))]
    ComplexJunction,
    #[cfg_attr(feature = "serde", serde(rename = "highwayInterchange"))]
    HighwayInterchange,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EJunctionSegmentType {
    #[cfg_attr(feature = "serde", serde(rename = "joint"))]
    Joint,
    #[cfg_attr(feature = "serde", serde(rename = "lane"))]
    Lane,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EJunctionType {
    #[cfg_attr(feature = "serde", serde(rename = "default"))]
    Default,
    #[cfg_attr(feature = "serde", serde(rename = "virtual"))]
    Virtual,
    #[cfg_attr(feature = "serde", serde(rename = "direct"))]
    Direct,
    #[cfg_attr(feature = "serde", serde(rename = "crossing"))]
    Crossing,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ELaneAdvisory {
    #[cfg_attr(feature = "serde", serde(rename = "both"))]
    Both,
    #[cfg_attr(feature = "serde", serde(rename = "inner"))]
    Inner,
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "outer"))]
    Outer,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ELaneType {
    #[cfg_attr(feature = "serde", serde(rename = "shoulder"))]
    Shoulder,
    #[cfg_attr(feature = "serde", serde(rename = "border"))]
    Border,
    #[cfg_attr(feature = "serde", serde(rename = "driving"))]
    Driving,
    #[cfg_attr(feature = "serde", serde(rename = "stop"))]
    Stop,
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "restricted"))]
    Restricted,
    #[cfg_attr(feature = "serde", serde(rename = "parking"))]
    Parking,
    #[cfg_attr(feature = "serde", serde(rename = "median"))]
    Median,
    #[cfg_attr(feature = "serde", serde(rename = "biking"))]
    Biking,
    #[cfg_attr(feature = "serde", serde(rename = "shared"))]
    Shared,
    #[cfg_attr(feature = "serde", serde(rename = "sidewalk"))]
    Sidewalk,
    #[cfg_attr(feature = "serde", serde(rename = "curb"))]
    Curb,
    #[cfg_attr(feature = "serde", serde(rename = "exit"))]
    Exit,
    #[cfg_attr(feature = "serde", serde(rename = "entry"))]
    Entry,
    #[cfg_attr(feature = "serde", serde(rename = "onRamp"))]
    OnRamp,
    #[cfg_attr(feature = "serde", serde(rename = "offRamp"))]
    OffRamp,
    #[cfg_attr(feature = "serde", serde(rename = "connectingRamp"))]
    ConnectingRamp,
    #[cfg_attr(feature = "serde", serde(rename = "bidirectional"))]
    Bidirectional,
    #[cfg_attr(feature = "serde", serde(rename = "special1"))]
    Special1,
    #[cfg_attr(feature = "serde", serde(rename = "special2"))]
    Special2,
    #[cfg_attr(feature = "serde", serde(rename = "special3"))]
    Special3,
    #[cfg_attr(feature = "serde", serde(rename = "roadWorks"))]
    RoadWorks,
    #[cfg_attr(feature = "serde", serde(rename = "tram"))]
    Tram,
    #[cfg_attr(feature = "serde", serde(rename = "rail"))]
    Rail,
    #[cfg_attr(feature = "serde", serde(rename = "bus"))]
    Bus,
    #[cfg_attr(feature = "serde", serde(rename = "taxi"))]
    Taxi,
    #[cfg_attr(feature = "serde", serde(rename = "HOV"))]
    Hov,
    #[cfg_attr(feature = "serde", serde(rename = "mwyEntry"))]
    MwyEntry,
    #[cfg_attr(feature = "serde", serde(rename = "mwyExit"))]
    MwyExit,
    #[cfg_attr(feature = "serde", serde(rename = "walking"))]
    Walking,
    #[cfg_attr(feature = "serde", serde(rename = "slipLane"))]
    SlipLane,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ELaneDirection {
    #[cfg_attr(feature = "serde", serde(rename = "both"))]
    Both,
    #[cfg_attr(feature = "serde", serde(rename = "reversed"))]
    Reversed,
    #[cfg_attr(feature = "serde", serde(rename = "standard"))]
    Standard,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ELayerType {
    #[cfg_attr(feature = "serde", serde(rename = "temporary"))]
    Temporary,
    #[cfg_attr(feature = "serde", serde(rename = "permanent"))]
    Permanent,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EMaxSpeedString {
    #[cfg_attr(feature = "serde", serde(rename = "no limit"))]
    NoLimit,
    #[cfg_attr(feature = "serde", serde(rename = "undefined"))]
    Undefined,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EObjectType {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "obstacle"))]
    Obstacle,
    #[cfg_attr(feature = "serde", serde(rename = "car"))]
    Car,
    #[cfg_attr(feature = "serde", serde(rename = "pole"))]
    Pole,
    #[cfg_attr(feature = "serde", serde(rename = "tree"))]
    Tree,
    #[cfg_attr(feature = "serde", serde(rename = "vegetation"))]
    Vegetation,
    #[cfg_attr(feature = "serde", serde(rename = "barrier"))]
    Barrier,
    #[cfg_attr(feature = "serde", serde(rename = "building"))]
    Building,
    #[cfg_attr(feature = "serde", serde(rename = "parkingSpace"))]
    ParkingSpace,
    #[cfg_attr(feature = "serde", serde(rename = "patch"))]
    Patch,
    #[cfg_attr(feature = "serde", serde(rename = "railing"))]
    Railing,
    #[cfg_attr(feature = "serde", serde(rename = "trafficIsland"))]
    TrafficIsland,
    #[cfg_attr(feature = "serde", serde(rename = "crosswalk"))]
    Crosswalk,
    #[cfg_attr(feature = "serde", serde(rename = "streetLamp"))]
    StreetLamp,
    #[cfg_attr(feature = "serde", serde(rename = "gantry"))]
    Gantry,
    #[cfg_attr(feature = "serde", serde(rename = "soundBarrier"))]
    SoundBarrier,
    #[cfg_attr(feature = "serde", serde(rename = "van"))]
    Van,
    #[cfg_attr(feature = "serde", serde(rename = "bus"))]
    Bus,
    #[cfg_attr(feature = "serde", serde(rename = "trailer"))]
    Trailer,
    #[cfg_attr(feature = "serde", serde(rename = "bike"))]
    Bike,
    #[cfg_attr(feature = "serde", serde(rename = "motorbike"))]
    Motorbike,
    #[cfg_attr(feature = "serde", serde(rename = "tram"))]
    Tram,
    #[cfg_attr(feature = "serde", serde(rename = "train"))]
    Train,
    #[cfg_attr(feature = "serde", serde(rename = "pedestrian"))]
    Pedestrian,
    #[cfg_attr(feature = "serde", serde(rename = "wind"))]
    Wind,
    #[cfg_attr(feature = "serde", serde(rename = "roadMark"))]
    RoadMark,
    #[cfg_attr(feature = "serde", serde(rename = "roadSurface"))]
    RoadSurface,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EOrientation {
    #[cfg_attr(feature = "serde", serde(rename = "+"))]
    Plus,
    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    Minus,
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EOutlineFillType {
    #[cfg_attr(feature = "serde", serde(rename = "grass"))]
    Grass,
    #[cfg_attr(feature = "serde", serde(rename = "concrete"))]
    Concrete,
    #[cfg_attr(feature = "serde", serde(rename = "cobble"))]
    Cobble,
    #[cfg_attr(feature = "serde", serde(rename = "asphalt"))]
    Asphalt,
    #[cfg_attr(feature = "serde", serde(rename = "pavement"))]
    Pavement,
    #[cfg_attr(feature = "serde", serde(rename = "gravel"))]
    Gravel,
    #[cfg_attr(feature = "serde", serde(rename = "soil"))]
    Soil,
    #[cfg_attr(feature = "serde", serde(rename = "paint"))]
    Paint,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EParamPoly3PRange {
    #[cfg_attr(feature = "serde", serde(rename = "arcLength"))]
    ArcLength,
    #[cfg_attr(feature = "serde", serde(rename = "normalized"))]
    Normalized,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EPersonCategory {
    #[cfg_attr(feature = "serde", serde(rename = "pedestrian"))]
    Pedestrian,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadMarkColor {
    #[cfg_attr(feature = "serde", serde(rename = "standard"))]
    Standard,
    #[cfg_attr(feature = "serde", serde(rename = "blue"))]
    Blue,
    #[cfg_attr(feature = "serde", serde(rename = "green"))]
    Green,
    #[cfg_attr(feature = "serde", serde(rename = "red"))]
    Red,
    #[cfg_attr(feature = "serde", serde(rename = "white"))]
    White,
    #[cfg_attr(feature = "serde", serde(rename = "yellow"))]
    Yellow,
    #[cfg_attr(feature = "serde", serde(rename = "black"))]
    Black,
    #[cfg_attr(feature = "serde", serde(rename = "orange"))]
    Orange,
    #[cfg_attr(feature = "serde", serde(rename = "violet"))]
    Violet,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadMarkRule {
    #[cfg_attr(feature = "serde", serde(rename = "no passing"))]
    NoPassing,
    #[cfg_attr(feature = "serde", serde(rename = "caution"))]
    Caution,
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadMarkType {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "solid"))]
    Solid,
    #[cfg_attr(feature = "serde", serde(rename = "broken"))]
    Broken,
    #[cfg_attr(feature = "serde", serde(rename = "solid solid"))]
    SolidSolid,
    #[cfg_attr(feature = "serde", serde(rename = "solid broken"))]
    SolidBroken,
    #[cfg_attr(feature = "serde", serde(rename = "broken solid"))]
    BrokenSolid,
    #[cfg_attr(feature = "serde", serde(rename = "broken broken"))]
    BrokenBroken,
    #[cfg_attr(feature = "serde", serde(rename = "botts dots"))]
    BottsDots,
    #[cfg_attr(feature = "serde", serde(rename = "grass"))]
    Grass,
    #[cfg_attr(feature = "serde", serde(rename = "curb"))]
    Curb,
    #[cfg_attr(feature = "serde", serde(rename = "custom"))]
    Custom,
    #[cfg_attr(feature = "serde", serde(rename = "edge"))]
    Edge,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadMarkWeight {
    #[cfg_attr(feature = "serde", serde(rename = "standard"))]
    Standard,
    #[cfg_attr(feature = "serde", serde(rename = "bold"))]
    Bold,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadType {
    #[cfg_attr(feature = "serde", serde(rename = "unknown"))]
    Unknown,
    #[cfg_attr(feature = "serde", serde(rename = "rural"))]
    Rural,
    #[cfg_attr(feature = "serde", serde(rename = "motorway"))]
    Motorway,
    #[cfg_attr(feature = "serde", serde(rename = "town"))]
    Town,
    #[cfg_attr(feature = "serde", serde(rename = "lowSpeed"))]
    LowSpeed,
    #[cfg_attr(feature = "serde", serde(rename = "pedestrian"))]
    Pedestrian,
    #[cfg_attr(feature = "serde", serde(rename = "bicycle"))]
    Bicycle,
    #[cfg_attr(feature = "serde", serde(rename = "townExpressway"))]
    TownExpressway,
    #[cfg_attr(feature = "serde", serde(rename = "townCollector"))]
    TownCollector,
    #[cfg_attr(feature = "serde", serde(rename = "townArterial"))]
    TownArterial,
    #[cfg_attr(feature = "serde", serde(rename = "townPrivate"))]
    TownPrivate,
    #[cfg_attr(feature = "serde", serde(rename = "townLocal"))]
    TownLocal,
    #[cfg_attr(feature = "serde", serde(rename = "townPlayStreet"))]
    TownPlayStreet,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadLanesLaneSectionLcrLaneRoadMarkLaneChange {
    #[cfg_attr(feature = "serde", serde(rename = "increase"))]
    Increase,
    #[cfg_attr(feature = "serde", serde(rename = "decrease"))]
    Decrease,
    #[cfg_attr(feature = "serde", serde(rename = "both"))]
    Both,
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadLanesLaneSectionLrLaneAccessRule {
    #[cfg_attr(feature = "serde", serde(rename = "allow"))]
    Allow,
    #[cfg_attr(feature = "serde", serde(rename = "deny"))]
    Deny,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadLinkElementType {
    #[cfg_attr(feature = "serde", serde(rename = "road"))]
    Road,
    #[cfg_attr(feature = "serde", serde(rename = "junction"))]
    Junction,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadObjectsObjectParkingSpaceAccess {
    #[cfg_attr(feature = "serde", serde(rename = "all"))]
    All,
    #[cfg_attr(feature = "serde", serde(rename = "car"))]
    Car,
    #[cfg_attr(feature = "serde", serde(rename = "women"))]
    Women,
    #[cfg_attr(feature = "serde", serde(rename = "handicapped"))]
    Handicapped,
    #[cfg_attr(feature = "serde", serde(rename = "bus"))]
    Bus,
    #[cfg_attr(feature = "serde", serde(rename = "truck"))]
    Truck,
    #[cfg_attr(feature = "serde", serde(rename = "electric"))]
    Electric,
    #[cfg_attr(feature = "serde", serde(rename = "residents"))]
    Residents,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadRailroadSwitchPosition {
    #[cfg_attr(feature = "serde", serde(rename = "dynamic"))]
    Dynamic,
    #[cfg_attr(feature = "serde", serde(rename = "straight"))]
    Straight,
    #[cfg_attr(feature = "serde", serde(rename = "turn"))]
    Turn,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadSignalsDisplayType {
    #[cfg_attr(feature = "serde", serde(rename = "LED"))]
    Led,
    #[cfg_attr(feature = "serde", serde(rename = "monochromGraphic"))]
    MonochromGraphic,
    #[cfg_attr(feature = "serde", serde(rename = "rotatingPrismHorizontal"))]
    RotatingPrismHorizontal,
    #[cfg_attr(feature = "serde", serde(rename = "rotatingPrismVertical"))]
    RotatingPrismVertical,
    #[cfg_attr(feature = "serde", serde(rename = "simpleMatrix"))]
    SimpleMatrix,
    #[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadSignalsSignalReferenceElementType {
    #[cfg_attr(feature = "serde", serde(rename = "object"))]
    Object,
    #[cfg_attr(feature = "serde", serde(rename = "signal"))]
    Signal,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadSurfaceCrgMode {
    #[cfg_attr(feature = "serde", serde(rename = "attached"))]
    Attached,
    #[cfg_attr(feature = "serde", serde(rename = "attached0"))]
    Attached0,
    #[cfg_attr(feature = "serde", serde(rename = "genuine"))]
    Genuine,
    #[cfg_attr(feature = "serde", serde(rename = "global"))]
    Global,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ERoadSurfaceCrgPurpose {
    #[cfg_attr(feature = "serde", serde(rename = "elevation"))]
    Elevation,
    #[cfg_attr(feature = "serde", serde(rename = "friction"))]
    Friction,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ESideType {
    #[cfg_attr(feature = "serde", serde(rename = "left"))]
    Left,
    #[cfg_attr(feature = "serde", serde(rename = "right"))]
    Right,
    #[cfg_attr(feature = "serde", serde(rename = "front"))]
    Front,
    #[cfg_attr(feature = "serde", serde(rename = "rear"))]
    Rear,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ESignalsSemanticsLane {
    #[cfg_attr(feature = "serde", serde(rename = "noOvertakeCars"))]
    NoOvertakeCars,
    #[cfg_attr(feature = "serde", serde(rename = "noOvertakeCarsEnd"))]
    NoOvertakeCarsEnd,
    #[cfg_attr(feature = "serde", serde(rename = "noOvertakeTrucks"))]
    NoOvertakeTrucks,
    #[cfg_attr(feature = "serde", serde(rename = "noOvertakeTrucksEnd"))]
    NoOvertakeTrucksEnd,
    #[cfg_attr(feature = "serde", serde(rename = "priorityOverOncoming"))]
    PriorityOverOncoming,
    #[cfg_attr(feature = "serde", serde(rename = "roundabout"))]
    Roundabout,
    #[cfg_attr(feature = "serde", serde(rename = "yieldForOncoming"))]
    YieldForOncoming,
    #[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ESignalsSemanticsPriority {
    #[cfg_attr(feature = "serde", serde(rename = "4way"))]
    _4Way,
    #[cfg_attr(feature = "serde", serde(rename = "keepClearLine"))]
    KeepClearLine,
    #[cfg_attr(feature = "serde", serde(rename = "noParkingLine"))]
    NoParkingLine,
    #[cfg_attr(feature = "serde", serde(rename = "noTurnOnRed"))]
    NoTurnOnRed,
    #[cfg_attr(feature = "serde", serde(rename = "priorityRoad"))]
    PriorityRoad,
    #[cfg_attr(feature = "serde", serde(rename = "priorityRoadEnd"))]
    PriorityRoadEnd,
    #[cfg_attr(feature = "serde", serde(rename = "priorityToTheRightRule"))]
    PriorityToTheRightRule,
    #[cfg_attr(feature = "serde", serde(rename = "stop"))]
    Stop,
    #[cfg_attr(feature = "serde", serde(rename = "stopLine"))]
    StopLine,
    #[cfg_attr(feature = "serde", serde(rename = "turnOnRedAllowed"))]
    TurnOnRedAllowed,
    #[cfg_attr(feature = "serde", serde(rename = "trafficLight"))]
    TrafficLight,
    #[cfg_attr(feature = "serde", serde(rename = "waitingLine"))]
    WaitingLine,
    #[cfg_attr(feature = "serde", serde(rename = "yield"))]
    Yield,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ESignalsSemanticsSpeed {
    #[cfg_attr(feature = "serde", serde(rename = "maximum"))]
    Maximum,
    #[cfg_attr(feature = "serde", serde(rename = "maximumEnd"))]
    MaximumEnd,
    #[cfg_attr(feature = "serde", serde(rename = "minimum"))]
    Minimum,
    #[cfg_attr(feature = "serde", serde(rename = "minimumEnd"))]
    MinimumEnd,
    #[cfg_attr(feature = "serde", serde(rename = "recommended"))]
    Recommended,
    #[cfg_attr(feature = "serde", serde(rename = "recommendedEnd"))]
    RecommendedEnd,
    #[cfg_attr(feature = "serde", serde(rename = "zone"))]
    Zone,
    #[cfg_attr(feature = "serde", serde(rename = "zoneEnd"))]
    ZoneEnd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ESignalsSemanticsSupplementaryDistance {
    #[cfg_attr(feature = "serde", serde(rename = "for"))]
    For,
    #[cfg_attr(feature = "serde", serde(rename = "in"))]
    In,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ESignalsSemanticsSupplementaryEnvironment {
    #[cfg_attr(feature = "serde", serde(rename = "fog"))]
    Fog,
    #[cfg_attr(feature = "serde", serde(rename = "rain"))]
    Rain,
    #[cfg_attr(feature = "serde", serde(rename = "snow"))]
    Snow,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ESignalsSemanticsSupplementaryTime {
    #[cfg_attr(feature = "serde", serde(rename = "day"))]
    Day,
    #[cfg_attr(feature = "serde", serde(rename = "time"))]
    Time,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EStationPlatformSegmentSide {
    #[cfg_attr(feature = "serde", serde(rename = "left"))]
    Left,
    #[cfg_attr(feature = "serde", serde(rename = "right"))]
    Right,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EStationType {
    #[cfg_attr(feature = "serde", serde(rename = "small"))]
    Small,
    #[cfg_attr(feature = "serde", serde(rename = "medium"))]
    Medium,
    #[cfg_attr(feature = "serde", serde(rename = "large"))]
    Large,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EStripMode {
    #[cfg_attr(feature = "serde", serde(rename = "independent"))]
    Independent,
    #[cfg_attr(feature = "serde", serde(rename = "relative"))]
    Relative,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ETrafficRule {
    #[cfg_attr(feature = "serde", serde(rename = "RHT"))]
    Rht,
    #[cfg_attr(feature = "serde", serde(rename = "LHT"))]
    Lht,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum ETunnelType {
    #[cfg_attr(feature = "serde", serde(rename = "standard"))]
    Standard,
    #[cfg_attr(feature = "serde", serde(rename = "underpass"))]
    Underpass,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EUnit {
    #[cfg_attr(feature = "serde", serde(rename = "m"))]
    M,
    #[cfg_attr(feature = "serde", serde(rename = "km"))]
    Km,
    #[cfg_attr(feature = "serde", serde(rename = "ft"))]
    Ft,
    #[cfg_attr(feature = "serde", serde(rename = "mile"))]
    Mile,
    #[cfg_attr(feature = "serde", serde(rename = "m/s"))]
    MS,
    #[cfg_attr(feature = "serde", serde(rename = "mph"))]
    Mph,
    #[cfg_attr(feature = "serde", serde(rename = "km/h"))]
    KmH,
    #[cfg_attr(feature = "serde", serde(rename = "kg"))]
    Kg,
    #[cfg_attr(feature = "serde", serde(rename = "t"))]
    T,
    #[cfg_attr(feature = "serde", serde(rename = "%"))]
    Percent,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EUnitDistance {
    #[cfg_attr(feature = "serde", serde(rename = "m"))]
    M,
    #[cfg_attr(feature = "serde", serde(rename = "km"))]
    Km,
    #[cfg_attr(feature = "serde", serde(rename = "ft"))]
    Ft,
    #[cfg_attr(feature = "serde", serde(rename = "mile"))]
    Mile,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EUnitMass {
    #[cfg_attr(feature = "serde", serde(rename = "kg"))]
    Kg,
    #[cfg_attr(feature = "serde", serde(rename = "t"))]
    T,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EUnitSlope {
    #[cfg_attr(feature = "serde", serde(rename = "%"))]
    Percent,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EUnitSpeed {
    #[cfg_attr(feature = "serde", serde(rename = "m/s"))]
    MS,
    #[cfg_attr(feature = "serde", serde(rename = "mph"))]
    Mph,
    #[cfg_attr(feature = "serde", serde(rename = "km/h"))]
    KmH,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum EVehicleCategory {
    #[cfg_attr(feature = "serde", serde(rename = "bicycle"))]
    Bicycle,
    #[cfg_attr(feature = "serde", serde(rename = "car"))]
    Car,
    #[cfg_attr(feature = "serde", serde(rename = "semiTrailer"))]
    SemiTrailer,
    #[cfg_attr(feature = "serde", serde(rename = "trailer"))]
    Trailer,
    #[cfg_attr(feature = "serde", serde(rename = "train"))]
    Train,
    #[cfg_attr(feature = "serde", serde(rename = "tram"))]
    Tram,
    #[cfg_attr(feature = "serde", serde(rename = "van"))]
    Van,
    #[cfg_attr(feature = "serde", serde(rename = "bus"))]
    Bus,
    #[cfg_attr(feature = "serde", serde(rename = "heavyTruck"))]
    HeavyTruck,
    #[cfg_attr(feature = "serde", serde(rename = "microMobilityDevice"))]
    MicroMobilityDevice,
    #[cfg_attr(feature = "serde", serde(rename = "motorcycle"))]
    Motorcycle,
    #[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
    #[cfg_attr(feature = "serde", serde(rename = "landVehicle"))]
    LandVehicle,
    #[cfg_attr(feature = "serde", serde(rename = "semiTractor"))]
    SemiTractor,
    #[cfg_attr(feature = "serde", serde(rename = "standupScooter"))]
    StandupScooter,
    #[cfg_attr(feature = "serde", serde(rename = "workMachine"))]
    WorkMachine,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TBool {
    #[cfg_attr(feature = "serde", serde(rename = "true"))]
    True,
    #[cfg_attr(feature = "serde", serde(rename = "false"))]
    False,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TController {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sequence"))]
    pub sequence: ::core::option::Option<::core::primitive::usize>,
    #[cfg_attr(feature = "serde", serde(default, rename = "control"))]
    pub control: ::std::vec::Vec<TControllerControl>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TControllerControl {
    #[cfg_attr(feature = "serde", serde(rename = "@signalId"))]
    pub signal_id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TDataQuality {
    #[cfg_attr(feature = "serde", serde(default, rename = "error"))]
    pub error: ::core::option::Option<TDataQualityError>,
    #[cfg_attr(feature = "serde", serde(default, rename = "rawData"))]
    pub raw_data: ::core::option::Option<TDataQualityRawData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TDataQualityError {
    #[cfg_attr(feature = "serde", serde(rename = "@xyAbsolute"))]
    pub xy_absolute: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@zAbsolute"))]
    pub z_absolute: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@xyRelative"))]
    pub xy_relative: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@zRelative"))]
    pub z_relative: ::core::primitive::f64,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TDataQualityRawData {
    #[cfg_attr(feature = "serde", serde(rename = "@date"))]
    pub date: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@source"))]
    pub source: EDataQualityRawDataSource,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sourceComment"))]
    pub source_comment: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@postProcessing"))]
    pub post_processing: EDataQualityRawDataPostProcessing,
    #[cfg_attr(feature = "serde", serde(default, rename = "@postProcessingComment"))]
    pub post_processing_comment: ::core::option::Option<::std::string::String>,
}
pub type TGrEqZero = ::core::primitive::f64;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "::std::string::String", into = "::std::string::String")
)]
pub enum TGrEqZeroOrContactPoint {
    #[cfg_attr(feature = "serde", serde(rename = "start"))]
    Start,
    #[cfg_attr(feature = "serde", serde(rename = "end"))]
    End,
    F64(::core::primitive::f64),
}
pub type TGrZero = ::core::primitive::f64;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct THeader {
    #[cfg_attr(feature = "serde", serde(rename = "@revMajor"))]
    pub rev_major: ::core::primitive::i32,
    #[cfg_attr(feature = "serde", serde(rename = "@revMinor"))]
    pub rev_minor: ::core::primitive::i32,
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@version"))]
    pub version: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@date"))]
    pub date: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@north"))]
    pub north: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@south"))]
    pub south: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@east"))]
    pub east: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@west"))]
    pub west: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@vendor"))]
    pub vendor: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "geoReference"))]
    pub geo_reference: ::core::option::Option<THeaderGeoReference>,
    #[cfg_attr(feature = "serde", serde(default, rename = "offset"))]
    pub offset: ::core::option::Option<THeaderOffset>,
    #[cfg_attr(feature = "serde", serde(default, rename = "license"))]
    pub license: ::core::option::Option<TLicense>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "defaultRegulations"))]
    pub default_regulations: ::core::option::Option<THeaderDefaultRegulations>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct THeaderGeoReference {
    #[cfg_attr(feature = "serde", serde(default, rename = "$text"))]
    pub text: ::core::option::Option<::std::string::String>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct THeaderOffset {
    #[cfg_attr(feature = "serde", serde(rename = "@x"))]
    pub x: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@y"))]
    pub y: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@z"))]
    pub z: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@hdg"))]
    pub hdg: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct THeaderDefaultRegulations {
    #[cfg_attr(feature = "serde", serde(default, rename = "roadRegulations"))]
    pub road_regulations: ::std::vec::Vec<THeaderRoadRegulation>,
    #[cfg_attr(feature = "serde", serde(default, rename = "signalRegulations"))]
    pub signal_regulations: ::std::vec::Vec<THeaderSignalRegulation>,
    #[cfg_attr(feature = "serde", serde(rename = "_OpenDriveElement"))]
    pub open_drive_element: OpenDriveElement,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct THeaderRoadRegulation {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<ERoadType>,
    #[cfg_attr(feature = "serde", serde(rename = "_OpenDriveElement"))]
    pub open_drive_element: OpenDriveElement,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "semantics"))]
    pub semantics: ::core::option::Option<TSignalsSemantics>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct THeaderSignalRegulation {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@subtype"))]
    pub subtype: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "_OpenDriveElement"))]
    pub open_drive_element: OpenDriveElement,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "semantics"))]
    pub semantics: ::core::option::Option<TSignalsSemantics>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TInclude {
    #[cfg_attr(feature = "serde", serde(rename = "@file"))]
    pub file: ::std::string::String,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunction {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionGroup {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: EJunctionGroupType,
    #[cfg_attr(feature = "serde", serde(default, rename = "junctionReference"))]
    pub junction_reference: ::std::vec::Vec<TJunctionGroupJunctionReference>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionGroupJunctionReference {
    #[cfg_attr(feature = "serde", serde(rename = "@junction"))]
    pub junction: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionBoundary {
    #[cfg_attr(feature = "serde", serde(default, rename = "segment"))]
    pub segment: ::std::vec::Vec<TJunctionBoundarySegment>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionBoundarySegment {
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadId"))]
    pub road_id: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionBoundarySegmentJoint {
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadId"))]
    pub road_id: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EJunctionSegmentType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@contactPoint"))]
    pub contact_point: ::core::option::Option<TGrEqZeroOrContactPoint>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@jointLaneStart"))]
    pub joint_lane_start: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@jointLaneEnd"))]
    pub joint_lane_end: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@transitionLength"))]
    pub transition_length: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionBoundarySegmentLane {
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadId"))]
    pub road_id: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EJunctionSegmentType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@boundaryLane"))]
    pub boundary_lane: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sStart"))]
    pub s_start: ::core::option::Option<TGrEqZeroOrContactPoint>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sEnd"))]
    pub s_end: ::core::option::Option<TGrEqZeroOrContactPoint>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionCommon {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EJunctionType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "connection"))]
    pub connection: ::std::vec::Vec<TJunctionConnectionCommon>,
    #[cfg_attr(feature = "serde", serde(default, rename = "crossPath"))]
    pub cross_path: ::std::vec::Vec<TJunctionCrossPath>,
    #[cfg_attr(feature = "serde", serde(default, rename = "priority"))]
    pub priority: ::std::vec::Vec<TJunctionPriority>,
    #[cfg_attr(feature = "serde", serde(default, rename = "controller"))]
    pub controller: ::std::vec::Vec<TJunctionController>,
    #[cfg_attr(feature = "serde", serde(default, rename = "surface"))]
    pub surface: ::core::option::Option<TRoadSurface>,
    #[cfg_attr(feature = "serde", serde(default, rename = "planView"))]
    pub plan_view: ::core::option::Option<TRoadPlanView>,
    #[cfg_attr(feature = "serde", serde(default, rename = "objects"))]
    pub objects: ::core::option::Option<TRoadObjects>,
    #[cfg_attr(feature = "serde", serde(default, rename = "boundary"))]
    pub boundary: ::core::option::Option<TJunctionBoundary>,
    #[cfg_attr(feature = "serde", serde(default, rename = "elevationGrid"))]
    pub elevation_grid: ::core::option::Option<TJunctionElevationGrid>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionConnection {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@incomingRoad"))]
    pub incoming_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@contactPoint"))]
    pub contact_point: ::core::option::Option<EContactPoint>,
    #[cfg_attr(feature = "serde", serde(default, rename = "laneLink"))]
    pub lane_link: ::std::vec::Vec<TJunctionConnectionLaneLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionConnectionCommon {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@incomingRoad"))]
    pub incoming_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@contactPoint"))]
    pub contact_point: ::core::option::Option<EContactPoint>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@connectingRoad"))]
    pub connecting_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "laneLink"))]
    pub lane_link: ::std::vec::Vec<TJunctionConnectionLaneLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionConnectionDirect {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@incomingRoad"))]
    pub incoming_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@contactPoint"))]
    pub contact_point: ::core::option::Option<EContactPoint>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@linkedRoad"))]
    pub linked_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "laneLink"))]
    pub lane_link: ::std::vec::Vec<TJunctionConnectionLaneLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionConnectionLaneLink {
    #[cfg_attr(feature = "serde", serde(rename = "@from"))]
    pub from: ::core::primitive::i32,
    #[cfg_attr(feature = "serde", serde(rename = "@to"))]
    pub to: ::core::primitive::i32,
    #[cfg_attr(feature = "serde", serde(default, rename = "@overlapZone"))]
    pub overlap_zone: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@fromLayer"))]
    pub from_layer: ::core::option::Option<ELayerType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@toLayer"))]
    pub to_layer: ::core::option::Option<ELayerType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionConnectionVirtual {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@incomingRoad"))]
    pub incoming_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@contactPoint"))]
    pub contact_point: ::core::option::Option<EContactPoint>,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: EConnectionType,
    #[cfg_attr(feature = "serde", serde(default, rename = "@connectingRoad"))]
    pub connecting_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "laneLink"))]
    pub lane_link: ::std::vec::Vec<TJunctionConnectionLaneLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(rename = "predecessor"))]
    pub predecessor: TJunctionPredecessorSuccessor,
    #[cfg_attr(feature = "serde", serde(rename = "successor"))]
    pub successor: TJunctionPredecessorSuccessor,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionConnectionVirtualDefault {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@incomingRoad"))]
    pub incoming_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@contactPoint"))]
    pub contact_point: ::core::option::Option<EContactPoint>,
    #[cfg_attr(
        feature = "serde",
        serde(
            default = "TJunctionConnectionVirtualDefault::default_type_",
            rename = "@type"
        )
    )]
    pub type_: EConnectionType,
    #[cfg_attr(feature = "serde", serde(default, rename = "@connectingRoad"))]
    pub connecting_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "laneLink"))]
    pub lane_link: ::std::vec::Vec<TJunctionConnectionLaneLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
impl TJunctionConnectionVirtualDefault {
    #[must_use]
    pub fn default_type_() -> EConnectionType {
        EConnectionType::Default
    }
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionController {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sequence"))]
    pub sequence: ::core::option::Option<::core::primitive::usize>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionCrossPath {
    #[cfg_attr(feature = "serde", serde(default, rename = "@crossingRoad"))]
    pub crossing_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadAtEnd"))]
    pub road_at_end: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadAtStart"))]
    pub road_at_start: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "startLaneLink"))]
    pub start_lane_link: TJunctionCrossPathLaneLink,
    #[cfg_attr(feature = "serde", serde(rename = "endLaneLink"))]
    pub end_lane_link: TJunctionCrossPathLaneLink,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionCrossPathLaneLink {
    #[cfg_attr(feature = "serde", serde(default, rename = "@s"))]
    pub s: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@from"))]
    pub from: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@to"))]
    pub to: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionCrossing {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EJunctionType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "roadSection"))]
    pub road_section: ::std::vec::Vec<TJunctionRoadSection>,
    #[cfg_attr(feature = "serde", serde(default, rename = "priority"))]
    pub priority: ::std::vec::Vec<TJunctionPriority>,
    #[cfg_attr(feature = "serde", serde(default, rename = "controller"))]
    pub controller: ::std::vec::Vec<TJunctionController>,
    #[cfg_attr(feature = "serde", serde(default, rename = "surface"))]
    pub surface: ::core::option::Option<TRoadSurface>,
    #[cfg_attr(feature = "serde", serde(default, rename = "planView"))]
    pub plan_view: ::core::option::Option<TRoadPlanView>,
    #[cfg_attr(feature = "serde", serde(default, rename = "objects"))]
    pub objects: ::core::option::Option<TRoadObjects>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionDirect {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EJunctionType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "connection"))]
    pub connection: ::std::vec::Vec<TJunctionConnectionDirect>,
    #[cfg_attr(feature = "serde", serde(default, rename = "priority"))]
    pub priority: ::std::vec::Vec<TJunctionPriority>,
    #[cfg_attr(feature = "serde", serde(default, rename = "controller"))]
    pub controller: ::std::vec::Vec<TJunctionController>,
    #[cfg_attr(feature = "serde", serde(default, rename = "surface"))]
    pub surface: ::core::option::Option<TRoadSurface>,
    #[cfg_attr(feature = "serde", serde(default, rename = "planView"))]
    pub plan_view: ::core::option::Option<TRoadPlanView>,
    #[cfg_attr(feature = "serde", serde(default, rename = "objects"))]
    pub objects: ::core::option::Option<TRoadObjects>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionElevationGrid {
    #[cfg_attr(feature = "serde", serde(default, rename = "@sStart"))]
    pub s_start: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@gridSpacing"))]
    pub grid_spacing: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "elevation"))]
    pub elevation: ::std::vec::Vec<TJunctionElevationGridElevation>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionElevationGridElevation {
    #[cfg_attr(feature = "serde", serde(default, rename = "@left"))]
    pub left: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@center"))]
    pub center: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@right"))]
    pub right: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TJunctionGridPositionList(pub ::std::vec::Vec<::core::primitive::f64>);
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionPredecessorSuccessor {
    #[cfg_attr(feature = "serde", serde(rename = "@elementType"))]
    pub element_type: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@elementId"))]
    pub element_id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@elementS"))]
    pub element_s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@elementDir"))]
    pub element_dir: ::core::option::Option<EElementDir>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionPriority {
    #[cfg_attr(feature = "serde", serde(rename = "@high"))]
    pub high: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@low"))]
    pub low: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionRoadSection {
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadId"))]
    pub road_id: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sStart"))]
    pub s_start: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sEnd"))]
    pub s_end: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TJunctionVirtual {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EJunctionType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@mainRoad"))]
    pub main_road: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sStart"))]
    pub s_start: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sEnd"))]
    pub s_end: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@orientation"))]
    pub orientation: ::core::option::Option<EOrientation>,
    #[cfg_attr(feature = "serde", serde(default, rename = "connection"))]
    pub connection: ::std::vec::Vec<TJunctionConnection>,
    #[cfg_attr(feature = "serde", serde(default, rename = "crossPath"))]
    pub cross_path: ::std::vec::Vec<TJunctionCrossPath>,
    #[cfg_attr(feature = "serde", serde(default, rename = "priority"))]
    pub priority: ::std::vec::Vec<TJunctionPriority>,
    #[cfg_attr(feature = "serde", serde(default, rename = "controller"))]
    pub controller: ::std::vec::Vec<TJunctionController>,
    #[cfg_attr(feature = "serde", serde(default, rename = "surface"))]
    pub surface: ::core::option::Option<TRoadSurface>,
    #[cfg_attr(feature = "serde", serde(default, rename = "planView"))]
    pub plan_view: ::core::option::Option<TRoadPlanView>,
    #[cfg_attr(feature = "serde", serde(default, rename = "objects"))]
    pub objects: ::core::option::Option<TRoadObjects>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TLicense {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@spdxid"))]
    pub spdxid: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@text"))]
    pub text: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@resource"))]
    pub resource: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "::std::string::String", into = "::std::string::String")
)]
pub enum TMaxSpeed {
    #[cfg_attr(feature = "serde", serde(rename = "no limit"))]
    NoLimit,
    #[cfg_attr(feature = "serde", serde(rename = "undefined"))]
    Undefined,
    F64(::core::primitive::f64),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoad {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@length"))]
    pub length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@junction"))]
    pub junction: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@rule"))]
    pub rule: ::core::option::Option<ETrafficRule>,
    #[cfg_attr(feature = "serde", serde(default, rename = "link"))]
    pub link: ::core::option::Option<TRoadLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "type"))]
    pub type_: ::std::vec::Vec<TRoadType>,
    #[cfg_attr(feature = "serde", serde(rename = "planView"))]
    pub plan_view: TRoadPlanView,
    #[cfg_attr(feature = "serde", serde(default, rename = "elevationProfile"))]
    pub elevation_profile: ::core::option::Option<TRoadElevationProfile>,
    #[cfg_attr(feature = "serde", serde(default, rename = "lateralProfile"))]
    pub lateral_profile: ::core::option::Option<TRoadLateralProfile>,
    #[cfg_attr(feature = "serde", serde(default, rename = "lanes"))]
    pub lanes: ::std::vec::Vec<TRoadLanes>,
    #[cfg_attr(feature = "serde", serde(default, rename = "objects"))]
    pub objects: ::core::option::Option<TRoadObjects>,
    #[cfg_attr(feature = "serde", serde(default, rename = "signals"))]
    pub signals: ::core::option::Option<TRoadSignals>,
    #[cfg_attr(feature = "serde", serde(default, rename = "surface"))]
    pub surface: ::core::option::Option<TRoadSurface>,
    #[cfg_attr(feature = "serde", serde(default, rename = "railroad"))]
    pub railroad: ::core::option::Option<TRoadRailroad>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadElevationProfile {
    #[cfg_attr(feature = "serde", serde(default, rename = "elevation"))]
    pub elevation: ::std::vec::Vec<TRoadElevationProfileElevation>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadElevationProfileElevation {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@a"))]
    pub a: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@b"))]
    pub b: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@c"))]
    pub c: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@d"))]
    pub d: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanes {
    #[cfg_attr(feature = "serde", serde(default, rename = "@layer"))]
    pub layer: ::core::option::Option<ELayerType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "laneOffset"))]
    pub lane_offset: ::std::vec::Vec<TRoadLanesLaneOffset>,
    #[cfg_attr(feature = "serde", serde(default, rename = "laneSection"))]
    pub lane_section: ::std::vec::Vec<TRoadLanesLaneSection>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneOffset {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@a"))]
    pub a: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@b"))]
    pub b: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@c"))]
    pub c: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@d"))]
    pub d: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSection {
    #[cfg_attr(feature = "serde", serde(default, rename = "@length"))]
    pub length: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@singleSide"))]
    pub single_side: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "left"))]
    pub left: ::core::option::Option<TRoadLanesLaneSectionLeft>,
    #[cfg_attr(feature = "serde", serde(rename = "center"))]
    pub center: TRoadLanesLaneSectionCenter,
    #[cfg_attr(feature = "serde", serde(default, rename = "right"))]
    pub right: ::core::option::Option<TRoadLanesLaneSectionRight>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionCenter {
    #[cfg_attr(feature = "serde", serde(rename = "lane"))]
    pub lane: TRoadLanesLaneSectionCenterLane,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionCenterLane {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::core::primitive::i32,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<ELaneType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@level"))]
    pub level: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "link"))]
    pub link: ::core::option::Option<TRoadLanesLaneSectionLcrLaneLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "roadMark"))]
    pub road_mark: ::std::vec::Vec<TRoadLanesLaneSectionLcrLaneRoadMark>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLcrLaneLink {
    #[cfg_attr(feature = "serde", serde(default, rename = "predecessor"))]
    pub predecessor: ::std::vec::Vec<TRoadLanesLaneSectionLcrLaneLinkPredecessorSuccessor>,
    #[cfg_attr(feature = "serde", serde(default, rename = "successor"))]
    pub successor: ::std::vec::Vec<TRoadLanesLaneSectionLcrLaneLinkPredecessorSuccessor>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLcrLaneLinkPredecessorSuccessor {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::core::primitive::i32,
    #[cfg_attr(feature = "serde", serde(default, rename = "@layer"))]
    pub layer: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMark {
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ERoadMarkType,
    #[cfg_attr(feature = "serde", serde(default, rename = "@weight"))]
    pub weight: ::core::option::Option<ERoadMarkWeight>,
    #[cfg_attr(feature = "serde", serde(rename = "@color"))]
    pub color: ERoadMarkColor,
    #[cfg_attr(feature = "serde", serde(default, rename = "@material"))]
    pub material: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@laneChange"))]
    pub lane_change: ::core::option::Option<ERoadLanesLaneSectionLcrLaneRoadMarkLaneChange>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@height"))]
    pub height: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "sway"))]
    pub sway: ::std::vec::Vec<TRoadLanesLaneSectionLcrLaneRoadMarkSway>,
    #[cfg_attr(feature = "serde", serde(default, rename = "type"))]
    pub type_element: ::core::option::Option<TRoadLanesLaneSectionLcrLaneRoadMarkType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "explicit"))]
    pub explicit: ::core::option::Option<TRoadLanesLaneSectionLcrLaneRoadMarkExplicit>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkExplicit {
    #[cfg_attr(feature = "serde", serde(default, rename = "line"))]
    pub line: ::std::vec::Vec<TRoadLanesLaneSectionLcrLaneRoadMarkExplicitLine>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkExplicitLine {
    #[cfg_attr(feature = "serde", serde(rename = "@length"))]
    pub length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@tOffset"))]
    pub t_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@rule"))]
    pub rule: ::core::option::Option<ERoadMarkRule>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkSway {
    #[cfg_attr(feature = "serde", serde(rename = "@ds"))]
    pub ds: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@a"))]
    pub a: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@b"))]
    pub b: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@c"))]
    pub c: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@d"))]
    pub d: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkType {
    #[cfg_attr(feature = "serde", serde(rename = "@name"))]
    pub name: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@width"))]
    pub width: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "line"))]
    pub line: ::std::vec::Vec<TRoadLanesLaneSectionLcrLaneRoadMarkTypeLine>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLcrLaneRoadMarkTypeLine {
    #[cfg_attr(feature = "serde", serde(rename = "@length"))]
    pub length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@space"))]
    pub space: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@tOffset"))]
    pub t_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@rule"))]
    pub rule: ::core::option::Option<ERoadMarkRule>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@color"))]
    pub color: ::core::option::Option<ERoadMarkColor>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLeft {
    #[cfg_attr(feature = "serde", serde(default, rename = "lane"))]
    pub lane: ::std::vec::Vec<TRoadLanesLaneSectionLeftLane>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLeftLane {
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ELaneType,
    #[cfg_attr(feature = "serde", serde(default, rename = "@advisory"))]
    pub advisory: ::core::option::Option<ELaneAdvisory>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@direction"))]
    pub direction: ::core::option::Option<ELaneDirection>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@dynamicLaneType"))]
    pub dynamic_lane_type: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@dynamicLaneDirection"))]
    pub dynamic_lane_direction: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@level"))]
    pub level: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadWorks"))]
    pub road_works: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::core::num::NonZeroUsize,
    #[cfg_attr(feature = "serde", serde(default, rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadLanesLaneSectionLeftLaneContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadLanesLaneSectionLeftLaneContent {
    #[cfg_attr(feature = "serde", serde(rename = "link"))]
    Link(TRoadLanesLaneSectionLcrLaneLink),
    #[cfg_attr(feature = "serde", serde(rename = "border"))]
    Border(TRoadLanesLaneSectionLrLaneBorder),
    #[cfg_attr(feature = "serde", serde(rename = "width"))]
    Width(TRoadLanesLaneSectionLrLaneWidth),
    #[cfg_attr(feature = "serde", serde(rename = "roadMark"))]
    RoadMark(::std::boxed::Box<TRoadLanesLaneSectionLcrLaneRoadMark>),
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    Material(TRoadLanesLaneSectionLrLaneMaterial),
    #[cfg_attr(feature = "serde", serde(rename = "speed"))]
    Speed(TRoadLanesLaneSectionLrLaneSpeed),
    #[cfg_attr(feature = "serde", serde(rename = "access"))]
    Access(TRoadLanesLaneSectionLrLaneAccess),
    #[cfg_attr(feature = "serde", serde(rename = "height"))]
    Height(TRoadLanesLaneSectionLrLaneHeight),
    #[cfg_attr(feature = "serde", serde(rename = "rule"))]
    Rule(TRoadLanesLaneSectionLrLaneRule),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLane {
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ELaneType,
    #[cfg_attr(feature = "serde", serde(default, rename = "@advisory"))]
    pub advisory: ::core::option::Option<ELaneAdvisory>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@direction"))]
    pub direction: ::core::option::Option<ELaneDirection>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@dynamicLaneType"))]
    pub dynamic_lane_type: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@dynamicLaneDirection"))]
    pub dynamic_lane_direction: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@level"))]
    pub level: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadWorks"))]
    pub road_works: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadLanesLaneSectionLrLaneContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadLanesLaneSectionLrLaneContent {
    #[cfg_attr(feature = "serde", serde(rename = "link"))]
    Link(TRoadLanesLaneSectionLcrLaneLink),
    #[cfg_attr(feature = "serde", serde(rename = "border"))]
    Border(TRoadLanesLaneSectionLrLaneBorder),
    #[cfg_attr(feature = "serde", serde(rename = "width"))]
    Width(TRoadLanesLaneSectionLrLaneWidth),
    #[cfg_attr(feature = "serde", serde(rename = "roadMark"))]
    RoadMark(::std::boxed::Box<TRoadLanesLaneSectionLcrLaneRoadMark>),
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    Material(TRoadLanesLaneSectionLrLaneMaterial),
    #[cfg_attr(feature = "serde", serde(rename = "speed"))]
    Speed(TRoadLanesLaneSectionLrLaneSpeed),
    #[cfg_attr(feature = "serde", serde(rename = "access"))]
    Access(TRoadLanesLaneSectionLrLaneAccess),
    #[cfg_attr(feature = "serde", serde(rename = "height"))]
    Height(TRoadLanesLaneSectionLrLaneHeight),
    #[cfg_attr(feature = "serde", serde(rename = "rule"))]
    Rule(TRoadLanesLaneSectionLrLaneRule),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLaneAccess {
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@rule"))]
    pub rule: ::core::option::Option<ERoadLanesLaneSectionLrLaneAccessRule>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@restriction"))]
    pub restriction: ::core::option::Option<EAccessRestrictionType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "restriction"))]
    pub restriction_element: ::std::vec::Vec<TRoadLanesLaneSectionLrLaneAccessRestriction>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLaneAccessRestriction {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EAccessRestrictionType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLaneBorder {
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@a"))]
    pub a: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@b"))]
    pub b: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@c"))]
    pub c: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@d"))]
    pub d: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLaneHeight {
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@inner"))]
    pub inner: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@outer"))]
    pub outer: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLaneMaterial {
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@surface"))]
    pub surface: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@friction"))]
    pub friction: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roughness"))]
    pub roughness: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLaneRule {
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@value"))]
    pub value: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLaneSpeed {
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@max"))]
    pub max: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@unit"))]
    pub unit: ::core::option::Option<EUnitSpeed>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionLrLaneWidth {
    #[cfg_attr(feature = "serde", serde(rename = "@sOffset"))]
    pub s_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@a"))]
    pub a: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@b"))]
    pub b: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@c"))]
    pub c: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@d"))]
    pub d: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionRight {
    #[cfg_attr(feature = "serde", serde(default, rename = "lane"))]
    pub lane: ::std::vec::Vec<TRoadLanesLaneSectionRightLane>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLanesLaneSectionRightLane {
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ELaneType,
    #[cfg_attr(feature = "serde", serde(default, rename = "@advisory"))]
    pub advisory: ::core::option::Option<ELaneAdvisory>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@direction"))]
    pub direction: ::core::option::Option<ELaneDirection>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@dynamicLaneType"))]
    pub dynamic_lane_type: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@dynamicLaneDirection"))]
    pub dynamic_lane_direction: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@level"))]
    pub level: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadWorks"))]
    pub road_works: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::core::num::NonZeroIsize,
    #[cfg_attr(feature = "serde", serde(default, rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadLanesLaneSectionRightLaneContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadLanesLaneSectionRightLaneContent {
    #[cfg_attr(feature = "serde", serde(rename = "link"))]
    Link(TRoadLanesLaneSectionLcrLaneLink),
    #[cfg_attr(feature = "serde", serde(rename = "border"))]
    Border(TRoadLanesLaneSectionLrLaneBorder),
    #[cfg_attr(feature = "serde", serde(rename = "width"))]
    Width(TRoadLanesLaneSectionLrLaneWidth),
    #[cfg_attr(feature = "serde", serde(rename = "roadMark"))]
    RoadMark(::std::boxed::Box<TRoadLanesLaneSectionLcrLaneRoadMark>),
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    Material(TRoadLanesLaneSectionLrLaneMaterial),
    #[cfg_attr(feature = "serde", serde(rename = "speed"))]
    Speed(TRoadLanesLaneSectionLrLaneSpeed),
    #[cfg_attr(feature = "serde", serde(rename = "access"))]
    Access(TRoadLanesLaneSectionLrLaneAccess),
    #[cfg_attr(feature = "serde", serde(rename = "height"))]
    Height(TRoadLanesLaneSectionLrLaneHeight),
    #[cfg_attr(feature = "serde", serde(rename = "rule"))]
    Rule(TRoadLanesLaneSectionLrLaneRule),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfile {
    #[cfg_attr(feature = "serde", serde(default, rename = "superelevation"))]
    pub superelevation: ::std::vec::Vec<TRoadLateralProfileSuperelevation>,
    #[cfg_attr(feature = "serde", serde(default, rename = "shape"))]
    pub shape: ::std::vec::Vec<TRoadLateralProfileShape>,
    #[cfg_attr(feature = "serde", serde(default, rename = "crossSectionSurface"))]
    pub cross_section_surface: ::core::option::Option<TRoadLateralProfileCrossSectionSurface>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurface {
    #[cfg_attr(feature = "serde", serde(default, rename = "tOffset"))]
    pub t_offset: ::core::option::Option<TRoadLateralProfileCrossSectionSurfaceTOffset>,
    #[cfg_attr(feature = "serde", serde(rename = "surfaceStrips"))]
    pub surface_strips: TRoadLateralProfileCrossSectionSurfaceSurfaceStrip,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceCoefficients {
    #[cfg_attr(feature = "serde", serde(default, rename = "@a"))]
    pub a: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@b"))]
    pub b: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@c"))]
    pub c: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@d"))]
    pub d: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@s"))]
    pub s: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceStrip {
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@mode"))]
    pub mode: ::core::option::Option<EStripMode>,
    #[cfg_attr(feature = "serde", serde(default, rename = "width"))]
    pub width: ::core::option::Option<TRoadLateralProfileCrossSectionSurfaceStripWidth>,
    #[cfg_attr(feature = "serde", serde(default, rename = "constant"))]
    pub constant: ::core::option::Option<TRoadLateralProfileCrossSectionSurfaceStripConstant>,
    #[cfg_attr(feature = "serde", serde(default, rename = "linear"))]
    pub linear: ::core::option::Option<TRoadLateralProfileCrossSectionSurfaceStripLinear>,
    #[cfg_attr(feature = "serde", serde(default, rename = "quadratic"))]
    pub quadratic: ::core::option::Option<TRoadLateralProfileCrossSectionSurfaceStripQuadratic>,
    #[cfg_attr(feature = "serde", serde(default, rename = "cubic"))]
    pub cubic: ::core::option::Option<TRoadLateralProfileCrossSectionSurfaceStripCubic>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceStripConstant {
    #[cfg_attr(feature = "serde", serde(default, rename = "coefficients"))]
    pub coefficients: ::std::vec::Vec<TRoadLateralProfileCrossSectionSurfaceCoefficients>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceStripCubic {
    #[cfg_attr(feature = "serde", serde(default, rename = "coefficients"))]
    pub coefficients: ::std::vec::Vec<TRoadLateralProfileCrossSectionSurfaceCoefficients>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceStripLinear {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "coefficients"))]
    pub coefficients: ::std::vec::Vec<TRoadLateralProfileCrossSectionSurfaceCoefficients>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceStripQuadratic {
    #[cfg_attr(feature = "serde", serde(default, rename = "coefficients"))]
    pub coefficients: ::std::vec::Vec<TRoadLateralProfileCrossSectionSurfaceCoefficients>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceStripWidth {
    #[cfg_attr(feature = "serde", serde(default, rename = "coefficients"))]
    pub coefficients: ::std::vec::Vec<TRoadLateralProfileCrossSectionSurfaceCoefficients>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceSurfaceStrip {
    #[cfg_attr(feature = "serde", serde(default, rename = "strip"))]
    pub strip: ::std::vec::Vec<TRoadLateralProfileCrossSectionSurfaceStrip>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileCrossSectionSurfaceTOffset {
    #[cfg_attr(feature = "serde", serde(default, rename = "coefficients"))]
    pub coefficients: ::std::vec::Vec<TRoadLateralProfileCrossSectionSurfaceCoefficients>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileShape {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@t"))]
    pub t: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@a"))]
    pub a: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@b"))]
    pub b: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@c"))]
    pub c: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@d"))]
    pub d: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLateralProfileSuperelevation {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@a"))]
    pub a: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@b"))]
    pub b: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@c"))]
    pub c: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@d"))]
    pub d: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLink {
    #[cfg_attr(feature = "serde", serde(default, rename = "predecessor"))]
    pub predecessor: ::core::option::Option<TRoadLinkPredecessorSuccessor>,
    #[cfg_attr(feature = "serde", serde(default, rename = "successor"))]
    pub successor: ::core::option::Option<TRoadLinkPredecessorSuccessor>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadLinkPredecessorSuccessor {
    #[cfg_attr(feature = "serde", serde(rename = "@elementId"))]
    pub element_id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@elementType"))]
    pub element_type: ::core::option::Option<ERoadLinkElementType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@contactPoint"))]
    pub contact_point: ::core::option::Option<EContactPoint>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@elementS"))]
    pub element_s: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@elementDir"))]
    pub element_dir: ::core::option::Option<EElementDir>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjects {
    #[cfg_attr(feature = "serde", serde(default, rename = "object"))]
    pub object: ::std::vec::Vec<TRoadObjectsObject>,
    #[cfg_attr(feature = "serde", serde(default, rename = "objectReference"))]
    pub object_reference: ::std::vec::Vec<TRoadObjectsObjectReference>,
    #[cfg_attr(feature = "serde", serde(default, rename = "tunnel"))]
    pub tunnel: ::std::vec::Vec<TRoadObjectsTunnel>,
    #[cfg_attr(feature = "serde", serde(default, rename = "bridge"))]
    pub bridge: ::std::vec::Vec<TRoadObjectsBridge>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsBridge {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@length"))]
    pub length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: EBridgeType,
    #[cfg_attr(feature = "serde", serde(default, rename = "validity"))]
    pub validity: ::std::vec::Vec<TRoadObjectsObjectLaneValidity>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObject {
    #[cfg_attr(feature = "serde", serde(rename = "@t"))]
    pub t: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@zOffset"))]
    pub z_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EObjectType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@validLength"))]
    pub valid_length: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@orientation"))]
    pub orientation: ::core::option::Option<EOrientation>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@subtype"))]
    pub subtype: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@dynamic"))]
    pub dynamic: ::core::option::Option<TYesNo>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@hdg"))]
    pub hdg: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@pitch"))]
    pub pitch: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roll"))]
    pub roll: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@height"))]
    pub height: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@length"))]
    pub length: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@radius"))]
    pub radius: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@perpToRoad"))]
    pub perp_to_road: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@invalidated"))]
    pub invalidated: ::core::option::Option<::core::primitive::bool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@temporary"))]
    pub temporary: ::core::option::Option<::core::primitive::bool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "repeat"))]
    pub repeat: ::std::vec::Vec<TRoadObjectsObjectRepeat>,
    #[cfg_attr(feature = "serde", serde(default, rename = "outline"))]
    pub outline: ::core::option::Option<TRoadObjectsObjectOutlinesOutline>,
    #[cfg_attr(feature = "serde", serde(default, rename = "outlines"))]
    pub outlines: ::core::option::Option<TRoadObjectsObjectOutlines>,
    #[cfg_attr(feature = "serde", serde(default, rename = "material"))]
    pub material: ::std::vec::Vec<TRoadObjectsObjectMaterial>,
    #[cfg_attr(feature = "serde", serde(default, rename = "validity"))]
    pub validity: ::std::vec::Vec<TRoadObjectsObjectLaneValidity>,
    #[cfg_attr(feature = "serde", serde(default, rename = "parkingSpace"))]
    pub parking_space: ::core::option::Option<TRoadObjectsObjectParkingSpace>,
    #[cfg_attr(feature = "serde", serde(default, rename = "markings"))]
    pub markings: ::core::option::Option<TRoadObjectsObjectMarkings>,
    #[cfg_attr(feature = "serde", serde(default, rename = "borders"))]
    pub borders: ::core::option::Option<TRoadObjectsObjectBorders>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "surface"))]
    pub surface: ::core::option::Option<TRoadObjectsObjectSurface>,
    #[cfg_attr(feature = "serde", serde(default, rename = "skeleton"))]
    pub skeleton: ::core::option::Option<TRoadObjectsObjectSkeleton>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectReference {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@t"))]
    pub t: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@zOffset"))]
    pub z_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@validLength"))]
    pub valid_length: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(rename = "@orientation"))]
    pub orientation: EOrientation,
    #[cfg_attr(feature = "serde", serde(default, rename = "validity"))]
    pub validity: ::std::vec::Vec<TRoadObjectsObjectLaneValidity>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectBorders {
    #[cfg_attr(feature = "serde", serde(default, rename = "border"))]
    pub border: ::std::vec::Vec<TRoadObjectsObjectBordersBorder>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectBordersBorder {
    #[cfg_attr(feature = "serde", serde(rename = "@width"))]
    pub width: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: EBorderType,
    #[cfg_attr(feature = "serde", serde(rename = "@outlineId"))]
    pub outline_id: ::core::primitive::usize,
    #[cfg_attr(feature = "serde", serde(default, rename = "@useCompleteOutline"))]
    pub use_complete_outline: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "cornerReference"))]
    pub corner_reference: ::std::vec::Vec<TRoadObjectsObjectMarkingsMarkingCornerReference>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectLaneValidity {
    #[cfg_attr(feature = "serde", serde(rename = "@fromLane"))]
    pub from_lane: ::core::primitive::i32,
    #[cfg_attr(feature = "serde", serde(rename = "@toLane"))]
    pub to_lane: ::core::primitive::i32,
    #[cfg_attr(feature = "serde", serde(default, rename = "@layer"))]
    pub layer: ::core::option::Option<ELayerType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectMarkings {
    #[cfg_attr(feature = "serde", serde(default, rename = "marking"))]
    pub marking: ::std::vec::Vec<TRoadObjectsObjectMarkingsMarking>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectMarkingsMarking {
    #[cfg_attr(feature = "serde", serde(default, rename = "@side"))]
    pub side: ::core::option::Option<ESideType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@weight"))]
    pub weight: ::core::option::Option<ERoadMarkWeight>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(rename = "@color"))]
    pub color: ERoadMarkColor,
    #[cfg_attr(feature = "serde", serde(default, rename = "@zOffset"))]
    pub z_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(rename = "@spaceLength"))]
    pub space_length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@lineLength"))]
    pub line_length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@startOffset"))]
    pub start_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@stopOffset"))]
    pub stop_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "cornerReference"))]
    pub corner_reference: ::std::vec::Vec<TRoadObjectsObjectMarkingsMarkingCornerReference>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectMarkingsMarkingCornerReference {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::core::primitive::usize,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectMaterial {
    #[cfg_attr(feature = "serde", serde(default, rename = "@surface"))]
    pub surface: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@friction"))]
    pub friction: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roughness"))]
    pub roughness: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roadMarkColor"))]
    pub road_mark_color: ::core::option::Option<ERoadMarkColor>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectOutlines {
    #[cfg_attr(feature = "serde", serde(default, rename = "outline"))]
    pub outline: ::std::vec::Vec<TRoadObjectsObjectOutlinesOutline>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectOutlinesOutline {
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::core::primitive::usize>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@fillType"))]
    pub fill_type: ::core::option::Option<EOutlineFillType>,
    #[cfg_attr(
        feature = "serde",
        serde(
            default = "TRoadObjectsObjectOutlinesOutline::default_outer",
            rename = "@outer"
        )
    )]
    pub outer: TBool,
    #[cfg_attr(feature = "serde", serde(default, rename = "@closed"))]
    pub closed: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@laneType"))]
    pub lane_type: ::core::option::Option<ELaneType>,
    #[cfg_attr(feature = "serde", serde(rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadObjectsObjectOutlinesOutlineContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadObjectsObjectOutlinesOutlineContent {
    #[cfg_attr(feature = "serde", serde(rename = "cornerRoad"))]
    CornerRoad(TRoadObjectsObjectOutlinesOutlineCornerRoad),
    #[cfg_attr(feature = "serde", serde(rename = "cornerLocal"))]
    CornerLocal(TRoadObjectsObjectOutlinesOutlineCornerLocal),
    #[cfg_attr(feature = "serde", serde(rename = "curveLocal"))]
    CurveLocal(TRoadObjectsObjectOutlinesOutlineCurveLocal),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
    #[cfg_attr(feature = "serde", serde(rename = "markings"))]
    Markings(TRoadObjectsObjectMarkings),
}
impl TRoadObjectsObjectOutlinesOutline {
    #[must_use]
    pub fn default_outer() -> TBool {
        TBool::True
    }
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectOutlinesOutlineCornerLocal {
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::core::primitive::usize>,
    #[cfg_attr(feature = "serde", serde(rename = "@u"))]
    pub u: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@v"))]
    pub v: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@z"))]
    pub z: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@height"))]
    pub height: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectOutlinesOutlineCornerRoad {
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::core::primitive::usize>,
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@t"))]
    pub t: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@dz"))]
    pub dz: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@height"))]
    pub height: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectOutlinesOutlineCurveLocal {
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::core::primitive::usize>,
    #[cfg_attr(feature = "serde", serde(rename = "@u"))]
    pub u: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@v"))]
    pub v: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@z"))]
    pub z: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@height"))]
    pub height: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@length"))]
    pub length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@hdg"))]
    pub hdg: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "$value"))]
    pub content: TRoadObjectsObjectOutlinesOutlineCurveLocalContent,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadObjectsObjectOutlinesOutlineCurveLocalContent {
    #[cfg_attr(feature = "serde", serde(rename = "arc"))]
    Arc(TRoadObjectsObjectOutlinesOutlineCurveLocalArc),
    #[cfg_attr(feature = "serde", serde(rename = "line"))]
    Line(TRoadObjectsObjectOutlinesOutlineCurveLocalLine),
    #[cfg_attr(feature = "serde", serde(rename = "paramPoly3"))]
    ParamPoly3(TRoadObjectsObjectOutlinesOutlineCurveLocalParamPoly3),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectOutlinesOutlineCurveLocalArc {
    #[cfg_attr(feature = "serde", serde(rename = "@curvature"))]
    pub curvature: ::core::primitive::f64,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectOutlinesOutlineCurveLocalLine;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectOutlinesOutlineCurveLocalParamPoly3 {
    #[cfg_attr(feature = "serde", serde(rename = "@aU"))]
    pub au: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@bU"))]
    pub bu: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@cU"))]
    pub cu: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@dU"))]
    pub du: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@aV"))]
    pub av: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@bV"))]
    pub bv: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@cV"))]
    pub cv: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@dV"))]
    pub dv: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@pRange"))]
    pub p_range: EParamPoly3PRange,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectParkingSpace {
    #[cfg_attr(feature = "serde", serde(rename = "@access"))]
    pub access: ERoadObjectsObjectParkingSpaceAccess,
    #[cfg_attr(feature = "serde", serde(default, rename = "@restrictions"))]
    pub restrictions: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectRepeat {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@length"))]
    pub length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@distance"))]
    pub distance: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@tStart"))]
    pub t_start: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@tEnd"))]
    pub t_end: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@heightStart"))]
    pub height_start: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@heightEnd"))]
    pub height_end: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@zOffsetStart"))]
    pub z_offset_start: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@zOffsetEnd"))]
    pub z_offset_end: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@widthStart"))]
    pub width_start: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@widthEnd"))]
    pub width_end: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@lengthStart"))]
    pub length_start: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@lengthEnd"))]
    pub length_end: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@radiusStart"))]
    pub radius_start: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@radiusEnd"))]
    pub radius_end: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@detachFromReferenceLine"))]
    pub detach_from_reference_line: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@bT"))]
    pub bt: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@cT"))]
    pub ct: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@dT"))]
    pub dt: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectSkeleton {
    #[cfg_attr(feature = "serde", serde(default, rename = "polyline"))]
    pub polyline: ::std::vec::Vec<TRoadObjectsObjectSkeletonPolyline>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectSkeletonPolyline {
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::core::num::NonZeroUsize>,
    #[cfg_attr(feature = "serde", serde(rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadObjectsObjectSkeletonPolylineContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadObjectsObjectSkeletonPolylineContent {
    #[cfg_attr(feature = "serde", serde(rename = "vertexRoad"))]
    VertexRoad(TRoadObjectsObjectSkeletonPolylineVertexRoad),
    #[cfg_attr(feature = "serde", serde(rename = "vertexLocal"))]
    VertexLocal(TRoadObjectsObjectSkeletonPolylineVertexLocal),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectSkeletonPolylineVertexLocal {
    #[cfg_attr(feature = "serde", serde(default, rename = "@radius"))]
    pub radius: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@intersectionPoint"))]
    pub intersection_point: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::core::num::NonZeroUsize>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@u"))]
    pub u: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@v"))]
    pub v: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@z"))]
    pub z: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectSkeletonPolylineVertexRoad {
    #[cfg_attr(feature = "serde", serde(default, rename = "@dz"))]
    pub dz: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@radius"))]
    pub radius: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@intersectionPoint"))]
    pub intersection_point: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::core::num::NonZeroUsize>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@s"))]
    pub s: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@t"))]
    pub t: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectSurface {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "CRG"))]
    pub crg: ::core::option::Option<TRoadObjectsObjectSurfaceCrg>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsObjectSurfaceCrg {
    #[cfg_attr(feature = "serde", serde(default, rename = "@file"))]
    pub file: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@hideRoadSurfaceCRG"))]
    pub hide_road_surface_crg: ::core::option::Option<TBool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@zScale"))]
    pub z_scale: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadObjectsTunnel {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@length"))]
    pub length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ETunnelType,
    #[cfg_attr(feature = "serde", serde(default, rename = "@lighting"))]
    pub lighting: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@daylight"))]
    pub daylight: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "validity"))]
    pub validity: ::std::vec::Vec<TRoadObjectsObjectLaneValidity>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadPlanView {
    #[cfg_attr(feature = "serde", serde(default, rename = "geometry"))]
    pub geometry: ::std::vec::Vec<TRoadPlanViewGeometry>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadPlanViewGeometry {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@x"))]
    pub x: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@y"))]
    pub y: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@hdg"))]
    pub hdg: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@length"))]
    pub length: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadPlanViewGeometryContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadPlanViewGeometryContent {
    #[cfg_attr(feature = "serde", serde(rename = "line"))]
    Line(TRoadPlanViewGeometryLine),
    #[cfg_attr(feature = "serde", serde(rename = "spiral"))]
    Spiral(TRoadPlanViewGeometrySpiral),
    #[cfg_attr(feature = "serde", serde(rename = "arc"))]
    Arc(TRoadPlanViewGeometryArc),
    #[cfg_attr(feature = "serde", serde(rename = "poly3"))]
    Poly3(TRoadPlanViewGeometryPoly3),
    #[cfg_attr(feature = "serde", serde(rename = "paramPoly3"))]
    ParamPoly3(TRoadPlanViewGeometryParamPoly3),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadPlanViewGeometryArc {
    #[cfg_attr(feature = "serde", serde(rename = "@curvature"))]
    pub curvature: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadPlanViewGeometryLine {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadPlanViewGeometryParamPoly3 {
    #[cfg_attr(feature = "serde", serde(rename = "@aU"))]
    pub au: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@bU"))]
    pub bu: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@cU"))]
    pub cu: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@dU"))]
    pub du: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@aV"))]
    pub av: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@bV"))]
    pub bv: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@cV"))]
    pub cv: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@dV"))]
    pub dv: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@pRange"))]
    pub p_range: EParamPoly3PRange,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadPlanViewGeometryPoly3 {
    #[cfg_attr(feature = "serde", serde(rename = "@a"))]
    pub a: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@b"))]
    pub b: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@c"))]
    pub c: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@d"))]
    pub d: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadPlanViewGeometrySpiral {
    #[cfg_attr(feature = "serde", serde(rename = "@curvStart"))]
    pub curv_start: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@curvEnd"))]
    pub curv_end: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadRailroad {
    #[cfg_attr(feature = "serde", serde(default, rename = "switch"))]
    pub switch: ::std::vec::Vec<TRoadRailroadSwitch>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadRailroadSwitch {
    #[cfg_attr(feature = "serde", serde(rename = "@name"))]
    pub name: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@position"))]
    pub position: ERoadRailroadSwitchPosition,
    #[cfg_attr(feature = "serde", serde(rename = "mainTrack"))]
    pub main_track: TRoadRailroadSwitchMainTrack,
    #[cfg_attr(feature = "serde", serde(rename = "sideTrack"))]
    pub side_track: TRoadRailroadSwitchSideTrack,
    #[cfg_attr(feature = "serde", serde(default, rename = "partner"))]
    pub partner: ::core::option::Option<TRoadRailroadSwitchPartner>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadRailroadSwitchMainTrack {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@dir"))]
    pub dir: EElementDir,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadRailroadSwitchPartner {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadRailroadSwitchSideTrack {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@dir"))]
    pub dir: EElementDir,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignals {
    #[cfg_attr(feature = "serde", serde(default, rename = "signal"))]
    pub signal: ::std::vec::Vec<TRoadSignalsSignalRoad>,
    #[cfg_attr(feature = "serde", serde(default, rename = "signalReference"))]
    pub signal_reference: ::std::vec::Vec<TRoadSignalsSignalReference>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsBoard {
    #[cfg_attr(feature = "serde", serde(default, rename = "validity"))]
    pub validity: ::std::vec::Vec<TRoadObjectsObjectLaneValidity>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dependency"))]
    pub dependency: ::std::vec::Vec<TRoadSignalsSignalDependency>,
    #[cfg_attr(feature = "serde", serde(default, rename = "reference"))]
    pub reference: ::std::vec::Vec<TRoadSignalsSignalReferenceLink>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsBoardSign {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@dynamic"))]
    pub dynamic: TYesNo,
    #[cfg_attr(feature = "serde", serde(rename = "@orientation"))]
    pub orientation: EOrientation,
    #[cfg_attr(feature = "serde", serde(default, rename = "@country"))]
    pub country: ::core::option::Option<ECountryCode>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@countryRevision"))]
    pub country_revision: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@subtype"))]
    pub subtype: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@value"))]
    pub value: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@text"))]
    pub text: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@unit"))]
    pub unit: ::core::option::Option<EUnit>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@height"))]
    pub height: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@length"))]
    pub length: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@hOffset"))]
    pub h_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roll"))]
    pub roll: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@pitch"))]
    pub pitch: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@invalidated"))]
    pub invalidated: ::core::option::Option<::core::primitive::bool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@temporary"))]
    pub temporary: ::core::option::Option<::core::primitive::bool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@v"))]
    pub v: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@z"))]
    pub z: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadSignalsBoardSignContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadSignalsBoardSignContent {
    #[cfg_attr(feature = "serde", serde(rename = "validity"))]
    Validity(TRoadObjectsObjectLaneValidity),
    #[cfg_attr(feature = "serde", serde(rename = "dependency"))]
    Dependency(TRoadSignalsSignalDependency),
    #[cfg_attr(feature = "serde", serde(rename = "reference"))]
    Reference(TRoadSignalsSignalReferenceLink),
    #[cfg_attr(feature = "serde", serde(rename = "positionInertial"))]
    PositionInertial(TRoadSignalsSignalPositionInertial),
    #[cfg_attr(feature = "serde", serde(rename = "positionRoad"))]
    PositionRoad(TRoadSignalsSignalPositionRoad),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
    #[cfg_attr(feature = "serde", serde(rename = "semantics"))]
    Semantics(::std::boxed::Box<TSignalsSemantics>),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsDisplayArea {
    #[cfg_attr(feature = "serde", serde(default, rename = "@index"))]
    pub index: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@height"))]
    pub height: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@v"))]
    pub v: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@z"))]
    pub z: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsSignal {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@dynamic"))]
    pub dynamic: TYesNo,
    #[cfg_attr(feature = "serde", serde(rename = "@orientation"))]
    pub orientation: EOrientation,
    #[cfg_attr(feature = "serde", serde(default, rename = "@country"))]
    pub country: ::core::option::Option<ECountryCode>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@countryRevision"))]
    pub country_revision: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@subtype"))]
    pub subtype: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@value"))]
    pub value: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@text"))]
    pub text: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@unit"))]
    pub unit: ::core::option::Option<EUnit>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@height"))]
    pub height: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@length"))]
    pub length: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@hOffset"))]
    pub h_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roll"))]
    pub roll: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@pitch"))]
    pub pitch: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@invalidated"))]
    pub invalidated: ::core::option::Option<::core::primitive::bool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@temporary"))]
    pub temporary: ::core::option::Option<::core::primitive::bool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadSignalsSignalContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadSignalsSignalContent {
    #[cfg_attr(feature = "serde", serde(rename = "validity"))]
    Validity(TRoadObjectsObjectLaneValidity),
    #[cfg_attr(feature = "serde", serde(rename = "dependency"))]
    Dependency(TRoadSignalsSignalDependency),
    #[cfg_attr(feature = "serde", serde(rename = "reference"))]
    Reference(TRoadSignalsSignalReferenceLink),
    #[cfg_attr(feature = "serde", serde(rename = "positionInertial"))]
    PositionInertial(TRoadSignalsSignalPositionInertial),
    #[cfg_attr(feature = "serde", serde(rename = "positionRoad"))]
    PositionRoad(TRoadSignalsSignalPositionRoad),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
    #[cfg_attr(feature = "serde", serde(rename = "semantics"))]
    Semantics(::std::boxed::Box<TSignalsSemantics>),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsSignalReference {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@t"))]
    pub t: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@orientation"))]
    pub orientation: EOrientation,
    #[cfg_attr(feature = "serde", serde(default, rename = "validity"))]
    pub validity: ::std::vec::Vec<TRoadObjectsObjectLaneValidity>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsSignalDependency {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsSignalPositionInertial {
    #[cfg_attr(feature = "serde", serde(rename = "@x"))]
    pub x: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@y"))]
    pub y: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@z"))]
    pub z: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@hdg"))]
    pub hdg: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@pitch"))]
    pub pitch: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roll"))]
    pub roll: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsSignalPositionRoad {
    #[cfg_attr(feature = "serde", serde(rename = "@roadId"))]
    pub road_id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@t"))]
    pub t: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@zOffset"))]
    pub z_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@hOffset"))]
    pub h_offset: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(default, rename = "@pitch"))]
    pub pitch: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roll"))]
    pub roll: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsSignalReferenceLink {
    #[cfg_attr(feature = "serde", serde(rename = "@elementType"))]
    pub element_type: ERoadSignalsSignalReferenceElementType,
    #[cfg_attr(feature = "serde", serde(rename = "@elementId"))]
    pub element_id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsSignalRoad {
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@dynamic"))]
    pub dynamic: TYesNo,
    #[cfg_attr(feature = "serde", serde(rename = "@orientation"))]
    pub orientation: EOrientation,
    #[cfg_attr(feature = "serde", serde(default, rename = "@country"))]
    pub country: ::core::option::Option<ECountryCode>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@countryRevision"))]
    pub country_revision: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@subtype"))]
    pub subtype: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@value"))]
    pub value: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@text"))]
    pub text: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@unit"))]
    pub unit: ::core::option::Option<EUnit>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@height"))]
    pub height: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@length"))]
    pub length: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@hOffset"))]
    pub h_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@width"))]
    pub width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@roll"))]
    pub roll: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@pitch"))]
    pub pitch: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@invalidated"))]
    pub invalidated: ::core::option::Option<::core::primitive::bool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@temporary"))]
    pub temporary: ::core::option::Option<::core::primitive::bool>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@s"))]
    pub s: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@t"))]
    pub t: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@zOffset"))]
    pub z_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "$value"))]
    pub content: ::std::vec::Vec<TRoadSignalsSignalRoadContent>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TRoadSignalsSignalRoadContent {
    #[cfg_attr(feature = "serde", serde(rename = "validity"))]
    Validity(TRoadObjectsObjectLaneValidity),
    #[cfg_attr(feature = "serde", serde(rename = "dependency"))]
    Dependency(TRoadSignalsSignalDependency),
    #[cfg_attr(feature = "serde", serde(rename = "reference"))]
    Reference(TRoadSignalsSignalReferenceLink),
    #[cfg_attr(feature = "serde", serde(rename = "positionInertial"))]
    PositionInertial(TRoadSignalsSignalPositionInertial),
    #[cfg_attr(feature = "serde", serde(rename = "positionRoad"))]
    PositionRoad(TRoadSignalsSignalPositionRoad),
    #[cfg_attr(feature = "serde", serde(rename = "dataQuality"))]
    DataQuality(TDataQuality),
    #[cfg_attr(feature = "serde", serde(rename = "include"))]
    Include(TInclude),
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    UserData(TUserData),
    #[cfg_attr(feature = "serde", serde(rename = "semantics"))]
    Semantics(::std::boxed::Box<TSignalsSemantics>),
    #[cfg_attr(feature = "serde", serde(rename = "staticBoard"))]
    StaticBoard(TRoadSignalsStaticBoard),
    #[cfg_attr(feature = "serde", serde(rename = "vmsBoard"))]
    VmsBoard(TRoadSignalsVmsBoard),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsStaticBoard {
    #[cfg_attr(feature = "serde", serde(default, rename = "validity"))]
    pub validity: ::std::vec::Vec<TRoadObjectsObjectLaneValidity>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dependency"))]
    pub dependency: ::std::vec::Vec<TRoadSignalsSignalDependency>,
    #[cfg_attr(feature = "serde", serde(default, rename = "reference"))]
    pub reference: ::std::vec::Vec<TRoadSignalsSignalReferenceLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "sign"))]
    pub sign: ::std::vec::Vec<TRoadSignalsBoardSign>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSignalsVmsBoard {
    #[cfg_attr(feature = "serde", serde(default, rename = "@displayType"))]
    pub display_type: ::core::option::Option<ERoadSignalsDisplayType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@displayHeight"))]
    pub display_height: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@displayWidth"))]
    pub display_width: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@v"))]
    pub v: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@z"))]
    pub z: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "validity"))]
    pub validity: ::std::vec::Vec<TRoadObjectsObjectLaneValidity>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dependency"))]
    pub dependency: ::std::vec::Vec<TRoadSignalsSignalDependency>,
    #[cfg_attr(feature = "serde", serde(default, rename = "reference"))]
    pub reference: ::std::vec::Vec<TRoadSignalsSignalReferenceLink>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "displayArea"))]
    pub display_area: ::std::vec::Vec<TRoadSignalsDisplayArea>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSurface {
    #[cfg_attr(feature = "serde", serde(default, rename = "CRG"))]
    pub crg: ::std::vec::Vec<TRoadSurfaceCrg>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadSurfaceCrg {
    #[cfg_attr(feature = "serde", serde(rename = "@file"))]
    pub file: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@sStart"))]
    pub s_start: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@sEnd"))]
    pub s_end: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@orientation"))]
    pub orientation: EDirection,
    #[cfg_attr(feature = "serde", serde(rename = "@mode"))]
    pub mode: ERoadSurfaceCrgMode,
    #[cfg_attr(feature = "serde", serde(default, rename = "@purpose"))]
    pub purpose: ::core::option::Option<ERoadSurfaceCrgPurpose>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@sOffset"))]
    pub s_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@tOffset"))]
    pub t_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@xOffset"))]
    pub x_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@yOffset"))]
    pub y_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@zOffset"))]
    pub z_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@zScale"))]
    pub z_scale: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@hOffset"))]
    pub h_offset: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadType {
    #[cfg_attr(feature = "serde", serde(rename = "@s"))]
    pub s: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@type"))]
    pub type_: ERoadType,
    #[cfg_attr(feature = "serde", serde(default, rename = "@country"))]
    pub country: ::core::option::Option<ECountryCode>,
    #[cfg_attr(feature = "serde", serde(default, rename = "speed"))]
    pub speed: ::core::option::Option<TRoadTypeSpeed>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TRoadTypeSpeed {
    #[cfg_attr(feature = "serde", serde(rename = "@max"))]
    pub max: TMaxSpeed,
    #[cfg_attr(feature = "serde", serde(default, rename = "@unit"))]
    pub unit: ::core::option::Option<EUnitSpeed>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalGroupVmsBoardReference {
    #[cfg_attr(feature = "serde", serde(default, rename = "@signalId"))]
    pub signal_id: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@vmsIndex"))]
    pub vms_index: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@groupIndex"))]
    pub group_index: ::core::option::Option<::core::primitive::i32>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalGroupVmsGroup {
    #[cfg_attr(feature = "serde", serde(default, rename = "@id"))]
    pub id: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(default, rename = "vmsBoardReference"))]
    pub vms_board_reference: ::std::vec::Vec<TSignalGroupVmsBoardReference>,
    #[cfg_attr(feature = "serde", serde(rename = "_OpenDriveElement"))]
    pub open_drive_element: OpenDriveElement,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemantics {
    #[cfg_attr(feature = "serde", serde(default, rename = "speed"))]
    pub speed: ::std::vec::Vec<TSignalsSemanticsSpeed>,
    #[cfg_attr(feature = "serde", serde(default, rename = "lane"))]
    pub lane: ::std::vec::Vec<TSignalsSemanticsLane>,
    #[cfg_attr(feature = "serde", serde(default, rename = "priority"))]
    pub priority: ::std::vec::Vec<TSignalsSemanticsPriority>,
    #[cfg_attr(feature = "serde", serde(default, rename = "prohibited"))]
    pub prohibited: ::std::vec::Vec<TSignalsSemanticsProhibited>,
    #[cfg_attr(feature = "serde", serde(default, rename = "warning"))]
    pub warning: ::std::vec::Vec<TSignalsSemanticsWarning>,
    #[cfg_attr(feature = "serde", serde(default, rename = "routing"))]
    pub routing: ::std::vec::Vec<TSignalsSemanticsRouting>,
    #[cfg_attr(feature = "serde", serde(default, rename = "streetname"))]
    pub streetname: ::std::vec::Vec<TSignalsSemanticsStreetname>,
    #[cfg_attr(feature = "serde", serde(default, rename = "parking"))]
    pub parking: ::std::vec::Vec<TSignalsSemanticsParking>,
    #[cfg_attr(feature = "serde", serde(default, rename = "tourist"))]
    pub tourist: ::std::vec::Vec<TSignalsSemanticsTourist>,
    #[cfg_attr(feature = "serde", serde(default, rename = "supplementaryTime"))]
    pub supplementary_time: ::std::vec::Vec<TSignalsSemanticsSupplementaryTime>,
    #[cfg_attr(feature = "serde", serde(default, rename = "supplementaryAllows"))]
    pub supplementary_allows: ::std::vec::Vec<TSignalsSemanticsSupplementaryAllows>,
    #[cfg_attr(feature = "serde", serde(default, rename = "supplementaryProhibits"))]
    pub supplementary_prohibits: ::std::vec::Vec<TSignalsSemanticsSupplementaryProhibits>,
    #[cfg_attr(feature = "serde", serde(default, rename = "supplementaryDistance"))]
    pub supplementary_distance: ::std::vec::Vec<TSignalsSemanticsSupplementaryDistance>,
    #[cfg_attr(feature = "serde", serde(default, rename = "supplementaryEnvironment"))]
    pub supplementary_environment: ::std::vec::Vec<TSignalsSemanticsSupplementaryEnvironment>,
    #[cfg_attr(feature = "serde", serde(default, rename = "supplementaryExplanatory"))]
    pub supplementary_explanatory: ::std::vec::Vec<TSignalsSemanticsSupplementaryExplanatory>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsAnimal;
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsLane {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<ESignalsSemanticsLane>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsParking {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsPerson {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: EPersonCategory,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsPriority {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<ESignalsSemanticsPriority>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsProhibited {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "animal"))]
    pub animal: ::std::vec::Vec<TSignalsSemanticsAnimal>,
    #[cfg_attr(feature = "serde", serde(default, rename = "person"))]
    pub person: ::std::vec::Vec<TSignalsSemanticsPerson>,
    #[cfg_attr(feature = "serde", serde(default, rename = "vehicle"))]
    pub vehicle: ::std::vec::Vec<TSignalsSemanticsVehicle>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsRouting {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsSpeed {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<ESignalsSemanticsSpeed>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@value"))]
    pub value: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@unit"))]
    pub unit: ::core::option::Option<EUnitSpeed>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsStreetname {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsSupplementaryAllows {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "animal"))]
    pub animal: ::std::vec::Vec<TSignalsSemanticsAnimal>,
    #[cfg_attr(feature = "serde", serde(default, rename = "person"))]
    pub person: ::std::vec::Vec<TSignalsSemanticsPerson>,
    #[cfg_attr(feature = "serde", serde(default, rename = "vehicle"))]
    pub vehicle: ::std::vec::Vec<TSignalsSemanticsVehicle>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsSupplementaryDistance {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<ESignalsSemanticsSupplementaryDistance>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@value"))]
    pub value: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@unit"))]
    pub unit: ::core::option::Option<EUnitDistance>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsSupplementaryEnvironment {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<ESignalsSemanticsSupplementaryEnvironment>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsSupplementaryExplanatory {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsSupplementaryProhibits {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
    #[cfg_attr(feature = "serde", serde(default, rename = "animal"))]
    pub animal: ::std::vec::Vec<TSignalsSemanticsAnimal>,
    #[cfg_attr(feature = "serde", serde(default, rename = "person"))]
    pub person: ::std::vec::Vec<TSignalsSemanticsPerson>,
    #[cfg_attr(feature = "serde", serde(default, rename = "vehicle"))]
    pub vehicle: ::std::vec::Vec<TSignalsSemanticsVehicle>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsSupplementaryTime {
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<ESignalsSemanticsSupplementaryTime>,
    #[cfg_attr(feature = "serde", serde(default, rename = "@value"))]
    pub value: ::core::option::Option<::core::primitive::f64>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsTourist {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsVehicle {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: EVehicleCategory,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TSignalsSemanticsWarning {
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TStation {
    #[cfg_attr(feature = "serde", serde(rename = "@name"))]
    pub name: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@type"))]
    pub type_: ::core::option::Option<EStationType>,
    #[cfg_attr(feature = "serde", serde(default, rename = "platform"))]
    pub platform: ::std::vec::Vec<TStationPlatform>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TStationPlatform {
    #[cfg_attr(feature = "serde", serde(default, rename = "@name"))]
    pub name: ::core::option::Option<::std::string::String>,
    #[cfg_attr(feature = "serde", serde(rename = "@id"))]
    pub id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "segment"))]
    pub segment: ::std::vec::Vec<TStationPlatformSegment>,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TStationPlatformSegment {
    #[cfg_attr(feature = "serde", serde(rename = "@roadId"))]
    pub road_id: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(rename = "@sStart"))]
    pub s_start: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@sEnd"))]
    pub s_end: ::core::primitive::f64,
    #[cfg_attr(feature = "serde", serde(rename = "@side"))]
    pub side: EStationPlatformSegmentSide,
    #[cfg_attr(feature = "serde", serde(default, rename = "dataQuality"))]
    pub data_quality: ::core::option::Option<TDataQuality>,
    #[cfg_attr(feature = "serde", serde(default, rename = "include"))]
    pub include: ::std::vec::Vec<TInclude>,
    #[cfg_attr(feature = "serde", serde(default, rename = "userData"))]
    pub user_data: ::std::vec::Vec<TUserData>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TUserData {
    #[cfg_attr(feature = "serde", serde(rename = "@code"))]
    pub code: ::std::string::String,
    #[cfg_attr(feature = "serde", serde(default, rename = "@value"))]
    pub value: ::core::option::Option<::std::string::String>,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum TYesNo {
    #[cfg_attr(feature = "serde", serde(rename = "yes"))]
    Yes,
    #[cfg_attr(feature = "serde", serde(rename = "no"))]
    No,
}
pub type TZeroOne = ::core::primitive::f64;

impl ::core::convert::TryFrom<::std::string::String> for ECountryCode {
    type Error = ::std::string::String;

    fn try_from(value: ::std::string::String) -> ::core::result::Result<Self, Self::Error> {
        if value == "OpenDRIVE" {
            return ::core::result::Result::Ok(Self::OpenDrive);
        }
        if value == "Austria" {
            return ::core::result::Result::Ok(Self::Austria);
        }
        if value == "Brazil" {
            return ::core::result::Result::Ok(Self::Brazil);
        }
        if value == "China" {
            return ::core::result::Result::Ok(Self::China);
        }
        if value == "France" {
            return ::core::result::Result::Ok(Self::France);
        }
        if value == "Germany" {
            return ::core::result::Result::Ok(Self::Germany);
        }
        if value == "Italy" {
            return ::core::result::Result::Ok(Self::Italy);
        }
        if value == "Switzerland" {
            return ::core::result::Result::Ok(Self::Switzerland);
        }
        if value == "USA" {
            return ::core::result::Result::Ok(Self::Usa);
        }
        ::core::result::Result::Ok(Self::String(value))
    }
}

impl ::core::convert::From<ECountryCode> for ::std::string::String {
    fn from(value: ECountryCode) -> Self {
        match value {
            ECountryCode::OpenDrive => "OpenDRIVE".into(),
            ECountryCode::Austria => "Austria".into(),
            ECountryCode::Brazil => "Brazil".into(),
            ECountryCode::China => "China".into(),
            ECountryCode::France => "France".into(),
            ECountryCode::Germany => "Germany".into(),
            ECountryCode::Italy => "Italy".into(),
            ECountryCode::Switzerland => "Switzerland".into(),
            ECountryCode::Usa => "USA".into(),
            ECountryCode::String(value) => value,
        }
    }
}

impl ::core::convert::TryFrom<::std::string::String> for TGrEqZeroOrContactPoint {
    type Error = ::std::string::String;

    fn try_from(value: ::std::string::String) -> ::core::result::Result<Self, Self::Error> {
        if value == "start" {
            return ::core::result::Result::Ok(Self::Start);
        }
        if value == "end" {
            return ::core::result::Result::Ok(Self::End);
        }
        if let ::core::result::Result::Ok(value) = value.as_str().parse::<::core::primitive::f64>()
        {
            return ::core::result::Result::Ok(Self::F64(value));
        }
        ::core::result::Result::Err(format!("invalid TGrEqZeroOrContactPoint value: {value}"))
    }
}

impl ::core::convert::From<TGrEqZeroOrContactPoint> for ::std::string::String {
    fn from(value: TGrEqZeroOrContactPoint) -> Self {
        match value {
            TGrEqZeroOrContactPoint::Start => "start".into(),
            TGrEqZeroOrContactPoint::End => "end".into(),
            TGrEqZeroOrContactPoint::F64(value) => value.to_string(),
        }
    }
}

impl ::core::convert::TryFrom<::std::string::String> for TMaxSpeed {
    type Error = ::std::string::String;

    fn try_from(value: ::std::string::String) -> ::core::result::Result<Self, Self::Error> {
        if value == "no limit" {
            return ::core::result::Result::Ok(Self::NoLimit);
        }
        if value == "undefined" {
            return ::core::result::Result::Ok(Self::Undefined);
        }
        if let ::core::result::Result::Ok(value) = value.as_str().parse::<::core::primitive::f64>()
        {
            return ::core::result::Result::Ok(Self::F64(value));
        }
        ::core::result::Result::Err(format!("invalid TMaxSpeed value: {value}"))
    }
}

impl ::core::convert::From<TMaxSpeed> for ::std::string::String {
    fn from(value: TMaxSpeed) -> Self {
        match value {
            TMaxSpeed::NoLimit => "no limit".into(),
            TMaxSpeed::Undefined => "undefined".into(),
            TMaxSpeed::F64(value) => value.to_string(),
        }
    }
}
