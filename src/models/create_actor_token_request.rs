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
pub struct CreateActorTokenRequest {
	/// The ID of the user that can use the newly created sign in token.
	#[serde(rename = "user_id")]
	pub user_id: String,
	/// The actor payload. It needs to include a sub property which should contain the ID of the actor. This whole payload will be also included in the JWT session token.
	#[serde(rename = "actor")]
	pub actor: serde_json::Value,
	/// Optional parameter to specify the life duration of the actor token in seconds. By default, the duration is 1 hour.
	#[serde(rename = "expires_in_seconds", skip_serializing_if = "Option::is_none")]
	pub expires_in_seconds: Option<i64>,
	/// The maximum duration that the session which will be created by the generated actor token should last. By default, the duration of a session created via an actor token, lasts 30 minutes.
	#[serde(rename = "session_max_duration_in_seconds", skip_serializing_if = "Option::is_none")]
	pub session_max_duration_in_seconds: Option<i64>,
}

impl CreateActorTokenRequest {
	pub fn new(user_id: String, actor: serde_json::Value) -> CreateActorTokenRequest {
		CreateActorTokenRequest {
			user_id,
			actor,
			expires_in_seconds: None,
			session_max_duration_in_seconds: None,
		}
	}
}
