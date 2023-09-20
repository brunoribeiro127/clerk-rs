/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamlConnection {
	#[serde(rename = "object")]
	pub object: Object,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "domain")]
	pub domain: String,
	#[serde(rename = "idp_entity_id", deserialize_with = "Option::deserialize")]
	pub idp_entity_id: Option<String>,
	#[serde(rename = "idp_sso_url", deserialize_with = "Option::deserialize")]
	pub idp_sso_url: Option<String>,
	#[serde(rename = "idp_certificate", deserialize_with = "Option::deserialize")]
	pub idp_certificate: Option<String>,
	#[serde(rename = "acs_url")]
	pub acs_url: String,
	#[serde(rename = "sp_entity_id")]
	pub sp_entity_id: String,
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "provider")]
	pub provider: String,
	#[serde(rename = "user_count")]
	pub user_count: i32,
	#[serde(rename = "sync_user_attributes")]
	pub sync_user_attributes: bool,
	/// Unix timestamp of creation.
	#[serde(rename = "created_at")]
	pub created_at: i64,
	/// Unix timestamp of last update.
	#[serde(rename = "updated_at")]
	pub updated_at: i64,
}

impl SamlConnection {
	pub fn new(
		object: Object,
		id: String,
		name: String,
		domain: String,
		idp_entity_id: Option<String>,
		idp_sso_url: Option<String>,
		idp_certificate: Option<String>,
		acs_url: String,
		sp_entity_id: String,
		active: bool,
		provider: String,
		user_count: i32,
		sync_user_attributes: bool,
		created_at: i64,
		updated_at: i64,
	) -> SamlConnection {
		SamlConnection {
			object,
			id,
			name,
			domain,
			idp_entity_id,
			idp_sso_url,
			idp_certificate,
			acs_url,
			sp_entity_id,
			active,
			provider,
			user_count,
			sync_user_attributes,
			created_at,
			updated_at,
		}
	}
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "saml_connection")]
	SamlConnection,
}

impl Default for Object {
	fn default() -> Object {
		Self::SamlConnection
	}
}