use graphene::LayerId;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct FrontendDocumentDetails {
	#[serde(rename = "isAutoSaved")]
	pub is_auto_saved: bool,
	#[serde(rename = "isSaved")]
	pub is_saved: bool,
	pub name: String,
	pub id: u64,
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct FrontendImageData {
	pub path: Vec<LayerId>,
	pub mime: String,
	#[serde(skip)]
	pub image_data: std::sync::Arc<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum MouseCursorIcon {
	#[default]
	Default,
	None,
	ZoomIn,
	ZoomOut,
	Grabbing,
	Crosshair,
	Text,
	Move,
	NSResize,
	EWResize,
	NESWResize,
	NWSEResize,
	Rotate,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum FileType {
	#[default]
	Png,
	Jpg,
	Svg,
}

impl FileType {
	pub fn to_mime(self) -> &'static str {
		match self {
			FileType::Png => "image/png",
			FileType::Jpg => "image/jpeg",
			FileType::Svg => "image/svg+xml",
		}
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExportBounds {
	#[default]
	AllArtwork,
	Selection,
	Artboard(LayerId),
}
