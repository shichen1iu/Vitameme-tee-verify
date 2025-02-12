use crate::error::ApiError;
use crate::utils::deserialize_message::*;
use crate::utils::find_spec_attribute::*;
use crate::utils::verify_signature::*;

use super::generate_redeemcode_and_sign::*;

pub fn verify_and_sign(
    author_data_message: &str,
    post_data_message: &str,
) -> Result<Signedredeemcode, ApiError> {
    let post_data: VitaSignedSession = deserialize_message(post_data_message)?;
    let author_data: VitaSignedSession = deserialize_message(author_data_message)?;

    let VitaSignedSession {
        signature: post_signature,
        application_data: post_application_data,
        attributes: post_attributes,
        ..
    } = post_data;

    let VitaSignedSession {
        attributes: author_attributes,
        signature: author_signature,
        application_data: author_application_data,
        ..
    } = author_data;

    if post_application_data.is_empty() || author_application_data.is_empty() {
        return Err(ApiError::InvalidMessage(
            "No application data found".to_string(),
        ));
    }
    if post_signature.is_empty() || author_signature.is_empty() {
        return Err(ApiError::InvalidMessage("No signature found".to_string()));
    }
    if post_attributes.is_empty() || author_attributes.is_empty() {
        return Err(ApiError::InvalidMessage("No attributes found".to_string()));
    }

    // let decoded_data = decode_app_data(&post_application_data);

    let mut is_valid = true;

    for attribute in &post_attributes {
        let is_valid_ = verify_signature(
            &attribute.attribute_hex,
            &attribute.attribute_name,
            &attribute.signature,
        )?;

        if !is_valid_ {
            is_valid = false;
            break;
        }
    }

    for attribute in &author_attributes {
        let is_valid_ = verify_signature(
            &attribute.attribute_hex,
            &attribute.attribute_name,
            &attribute.signature,
        )?;

        if !is_valid_ {
            is_valid = false;
            break;
        }
    }

    if !is_valid {
        return Err(ApiError::InvalidMessage("Invalid signature".to_string()));
    }

    let post_auhtor_id = find_author_attribute(&post_attributes).expect("No author id found");
    let author_id = find_author_attribute(&author_attributes).expect("No author id found");

    if post_auhtor_id != author_id {
        return Err(ApiError::InvalidMessage("Invalid author".to_string()));
    }

    let signed_redeemcode = generate_redeemcode_and_sign(&post_attributes)?;
    Ok(signed_redeemcode)
}
