use serde::{Serialize, Deserialize};


#[allow(non_snake_case)]
#[derive(Clone, Serialize, Deserialize)]
pub struct PreviewMessage {
    #[serde(rename = "@id")]
	pub at_id: String,
    #[serde(rename = "@type")]
	pub at_type: String,
	pub accountId: String,
	pub createdAt: String,
	pub downloadUrl: String,
	pub from: From,
	pub hasAttachments: bool,
	pub id: String,
	pub intro: String,
	pub isDeleted: bool,
	pub msgid: String,
	pub seen: bool,
	pub size: f64,
	pub subject: String,
	pub to: Vec<To>,
	pub updatedAt: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct To {
	pub address: String,
	pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct From {
	pub address: String,
	pub name: String,
}

impl PartialEq for PreviewMessage {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "@context")]
	pub context: String,
    #[serde(rename = "@id")]
	pub at_id: String,
    #[serde(rename = "@type")]
	pub at_type: String,
	pub accountId: String,
	pub attachments: Vec<serde_json::Value>,
	pub bcc: Vec<serde_json::Value>,
	pub cc: Vec<serde_json::Value>,
	pub createdAt: String,
	pub downloadUrl: String,
	pub flagged: bool,
	pub from: From,
	pub hasAttachments: bool,
	pub html: Vec<String>,
	pub id: String,
	pub isDeleted: bool,
	pub msgid: String,
	pub retention: bool,
	pub retentionDate: String,
	pub seen: bool,
	pub size: f64,
	pub subject: String,
	pub text: String,
	pub to: Vec<To>,
	pub updatedAt: String,
	pub verifications: Vec<serde_json::Value>,
}
