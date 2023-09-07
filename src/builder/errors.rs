//! Errors for the builder module

use thiserror::Error;
/// Errors when parsing a Talk json
#[derive(Error, Debug, PartialEq, Eq)]
pub enum JsonError {
    /// Serde failed to parse the json
    #[error("serde failed to parse the json: {0}")]
    BadParse(String),
    /// The Talk json is not valid
    #[error("the script is not valid: {0:?}")]
    Validation(Vec<String>),
}

/// Errors when building a Talk
#[derive(Error, Debug, PartialEq, Eq)]
pub enum TalkError {
    /// The actor id is duplicated
    #[error("the actor id {0} is duplicated")]
    DuplicateActorId(String),
    /// An action has a non-existent actor
    #[error("the action {0} has specified a non existent actor {1}")]
    InvalidActor(i32, String),
    /// Multiple actions have same id error
    #[error("multiple actions have same id: {0}")]
    DuplicateActionId(i32),
    /// An action has the next field pointing to a non-existent action
    #[error("the action {0} is pointing to id {1} which was not found")]
    InvalidNextAction(i32, i32),
    /// The Handle did not have a Talk loaded
    #[error("the handle did not have a Talk loaded")]
    RawTalkNotLoaded,
}
