use crate::error::ApiError;
use crate::utils::*;
use crate::CURRENT_VERSION;
use bs58;
use ed25519_dalek::{Signer, SigningKey};
use serde::Deserialize;
use serde::Serialize;
const CLIENT: &str = "twitter";

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedRedemcode {
    pub redemcode: String,
    pub signature: String,
}

pub fn generate_redemcode_and_sign(attributes: &[Attribute]) -> Result<SignedRedemcode, ApiError> {
    let bookmark_count = find_bookmark_count_attribute(attributes)?;
    let favorite_count = find_favorite_count_attribute(attributes)?;
    let retweet_count = find_retweet_count_attribute(attributes)?;

    //todo:engagement具体值需要修改
    let engagement = bookmark_count.parse::<u32>().unwrap()
        + favorite_count.parse::<u32>().unwrap()
        + retweet_count.parse::<u32>().unwrap()
        + 1;

    let post_id = find_post_id_attribute(attributes)?;

    let content = find_content_attribute(attributes)?;

    let ca = extract_ca(&content)?;

    let redemcode = format!(
        "{}-{}-{}-{}-{}",
        CURRENT_VERSION, CLIENT, post_id, ca, engagement
    );

    let key_pem = include_str!("../../ed25519key/private.pem");
    let key_bytes = bs58::decode(
        key_pem
            .trim()
            .strip_prefix("-----BEGIN PRIVATE KEY-----")
            .unwrap()
            .strip_suffix("-----END PRIVATE KEY-----")
            .unwrap()
            .replace('\n', ""),
    )
    .into_vec()
    .unwrap();

    let signing_key = SigningKey::from_keypair_bytes(&key_bytes.try_into().unwrap()).unwrap();

    // 3. 对 redemcode 进行签名
    let signature_hex = signing_key.sign(redemcode.as_bytes());
    let signature_hex_low = format!("{:x}", signature_hex);

    Ok(SignedRedemcode {
        redemcode,
        signature: signature_hex_low,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_redemcode_and_sign() {
        let attributes = vec![
            Attribute {
            attribute_hex: "617574686f723a20223132343836363830363531343839373330363122".to_string(),
            attribute_name: "author: \"1248668065148973061\"".to_string(),
            signature: "98e045ba2ddb0cc9cb6a98b1714032823e92b4ae4f7b59cf80058eba0250e9841edefcb508311af74984c58f2efd5bd7d121242364be0f38e2a668f5d5439fa5".to_string(),
        },
        Attribute {
            attribute_hex: "636f6e74656e743a2022446f626279207468696e6b7320667269656e64732073686f756c6420616c77617973206361727279206120736f636b20696e20746865697220706f636b6574e28094796f75206e65766572206b6e6f77207768656e2066726565646f6d206d6967687420636f6d65206b6e6f636b696e6721205c6e5c6e446f626279206f6e6365207573656420612074656163757020746f20736f6c76652061206269672070726f626c656d2c2070726f76696e67206576656e2074686520736d616c6c657374207468696e67732063616e20686f6c6420677265617420706f7765722e22".to_string(),
            attribute_name: "content: \"Dobby thinks ca:xxxxxxxx friends should always carry a sock in their pocket—you never know when freedom might come knocking! \\n\\nDobby once used a teacup to solve a big problem, proving even the smallest things can hold great power.\"".to_string(),
            signature: "f2319d9496fb42627d54a43724fb8a3a4d8c26e4a037c441b92255cfff80802ef58248f663f3fa4f7247307e31bf7c999502f0aedc26c33c7a1f4977e140f08d".to_string(),
        },
        Attribute {
            attribute_hex: "69643a20223138373934353633393734353433383532363522".to_string(),
            attribute_name: "id: \"111111111111111\"".to_string(),
            signature: "8c1c9b9ead611852f72ff2dd5cc4ff591bff766fc197aa19cebbae3e3a494e024885ca063f5601158649b6762c083c93f2e49132b05de1c0904ad74000126358".to_string(),
        },
        Attribute {
            attribute_hex: "626f6f6b6d61726b5f636f756e743a2030".to_string(),
            attribute_name: "bookmark_count: 0".to_string(),
            signature: "8bea50d146c6597c44e1a6292d74cc78653b25390d1be8fd352a915db7a01c428f0e68d8d1f5e27e018df019cd5814e1fdfcc959265d6d82e7ea45ff7ca1a32b".to_string(),
        },
        Attribute {
            attribute_hex: "6661766f726974655f636f756e743a2030".to_string(),
            attribute_name: "favorite_count: 0".to_string(),
            signature: "4301c5be9f7028ccab7212e22996edfcae32d0651506bdf3ada2413246ec22efc5217ed5769a220d84528ea7a883a5c82555651e6b1536951fd32b436de79821".to_string(),
        },
        Attribute {
            attribute_hex: "726574776565745f636f756e743a2030".to_string(),
            attribute_name: "retweet_count: 0".to_string(),
            signature: "e214455027c1740636a15725b91b25090625d1e2b68945a0e48b76e421f8e4a07cd86ca106a964c56f98747e5ebccb14f07e93ee6987e3f18d6b0d32bf78a40a".to_string(),
        },
        Attribute {
            attribute_hex: "637265617465645f61743a2022576564204a616e2031352030393a31313a3338202b30303030203230323522".to_string(),
            attribute_name: "created_at: \"Wed Jan 15 09:11:38 +0000 2025\"".to_string(),
            signature: "b7c8a45f03399544b8020e5bd65e017f01a683cfdf916bdda2458de573b8adda9b11a1cdf6bea01c420555064fd244881b639650755037399962e535e8c9ca0f".to_string(),
            },
        ];

        let redemcode = generate_redemcode_and_sign(&attributes).unwrap();
        println!("redemcode: {:?}", redemcode);
    }
}
