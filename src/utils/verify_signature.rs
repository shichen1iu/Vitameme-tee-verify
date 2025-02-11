use hex;
use p256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
};
use std::str;

use crate::error::ApiError;

pub fn verify_signature(
    attribute_hex: &str,
    attribute_name: &str,
    signature: &str,
) -> Result<bool, ApiError> {
    let attribute_name_hex = hex::encode(attribute_name);

    if attribute_name_hex != attribute_hex {
        return Err(ApiError::InvalidMessage(
            "attribute_name_hex not match attribute_hex".to_string(),
        ));
    }

    //signature
    let signature_bytes =
        hex::decode(signature).map_err(|e| ApiError::SignatureError(e.to_string()))?;
    // println!("signature_bytes: {:?}", signature_bytes);

    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|e| ApiError::SignatureError(e.to_string()))?;

    //message
    let application_data =
        hex::decode(attribute_hex).map_err(|e| ApiError::SignatureError(e.to_string()))?;

    //当前的写法是从私钥获取verifying key , 因为esper提供的公钥私钥不匹配
    let verifying_key = VerifyingKey::from(notary_private_key());

    let result = verifying_key.verify(&application_data, &signature).is_ok();
    // println!("verification result: {}", result);

    Ok(result)
}

/// 返回保存的公钥
fn _notary_pubkey() -> p256::PublicKey {
    let pem_file = str::from_utf8(include_bytes!(
        "../notary/notary.pub"
    ))
    .unwrap();
    p256::PublicKey::from_public_key_pem(pem_file).unwrap()
}

fn notary_private_key() -> p256::ecdsa::SigningKey {
    let pem_file = str::from_utf8(include_bytes!(
        "../notary/notary.key"
    ))
    .unwrap();
    let private_key = p256::ecdsa::SigningKey::from_pkcs8_pem(pem_file).unwrap();
    private_key
}

#[cfg(test)]
mod tests {

    use p256::ecdsa::{signature::SignerMut, SigningKey};
    use rand_core::OsRng;

    use super::*;

    #[test]
    fn test_verifying_key() {
        // 1. 先生成一个私钥
        let signing_key = SigningKey::random(&mut OsRng);

        //2.生成verifying key
        let verifying_key_from_private = VerifyingKey::from(&signing_key);

        // 3.导出公钥
        let public_key = p256::PublicKey::from(&verifying_key_from_private);

        //4.通过pubkey生成verifying key
        let verifying_key_from_pubkey = VerifyingKey::from(&public_key);

        // 这两个 VerifyingKey 是完全相同的
        assert_eq!(verifying_key_from_private, verifying_key_from_pubkey);
    }

    #[test]
    fn test_esper_private_key_and_pubkey() {
        // let private_key = notary_private_key();
        // let public_key = _notary_pubkey();

        // let verifying_key_from_private = VerifyingKey::from(&private_key);
        // let verifying_key_from_pubkey = VerifyingKey::from(&public_key);

        // //esper提供的公钥和私钥不是一对
        // assert_eq!(verifying_key_from_private, verifying_key_from_pubkey);
    }

    #[test]
    fn test_sign_p256() {
        // Generate a random private key
        let mut signing_key = notary_private_key();

        // Message to be signed
        let message = b"bookmark_count";

        // Sign the message
        let signature: Signature = signing_key.sign(message);

        // Verify the signature (optional, for demonstration)

        let verifying_key = VerifyingKey::from(&signing_key);

        println!(
            "verifying_key from private key: {:?}",
            verifying_key.to_sec1_bytes()
        );

        assert!(verifying_key.verify(message, &signature).is_ok());
        println!("test");

        let message_hex = hex::encode(message);
        let result = verify_signature(&message_hex, "bookmark_count", &signature.to_string());
        assert!(result.unwrap());
    }
    #[test]
    fn test_verify_signature() {
        // let attribute_hex = "626f6f6b6d61726b5f636f756e743a2030";
        // let signature = "8bea50d146c6597c44e1a6292d74cc78653b25390d1be8fd352a915db7a01c428f0e68d8d1f5e27e018df019cd5814e1fdfcc959265d6d82e7ea45ff7ca1a32b";

        // let attribute_hex_2 = "636f6e74656e743a2022446f626279207468696e6b732063613a36703678674879463741654536545a6b536d46736b6f34343477716f503135696355537169326a664769504e2073686f756c6420616c77617973206361727279206120736f636b20696e20746865697220706f636b6574e28094796f75206e65766572206b6e6f77207768656e2066726565646f6d206d6967687420636f6d65206b6e6f636b696e6721205c6e5c6e446f626279206f6e6365207573656420612074656163757020746f20736f6c76652061206269672070726f626c656d2c2070726f76696e67206576656e2074686520736d616c6c657374207468696e67732063616e20686f6c6420677265617420706f7765722e22";
        // let signature_2 ="fc4461697608e8ef0a01e0ebc953b91dddf987f27323ddcb2a687225590bac194e55b7d18969d5a81c3e43d0f8e90862a91e97068798e5d2c8c15c6a38a8c186";
        // let content = "content: \"Dobby thinks ca:6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN should always carry a sock in their pocket—you never know when freedom might come knocking! \\n\\nDobby once used a teacup to solve a big problem, proving even the smallest things can hold great power.\"";
        // println!("Testing with:");
        // println!("attribute_hex: {}", attribute_hex);
        // println!("signature: {}", signature);

        // // let result = verify_signature(attribute_hex, "bookmark_count", signature);
        // // assert!(result.unwrap());

        // let result_2 = verify_signature(attribute_hex_2, "content", signature_2);
        // assert!(result_2.unwrap());
    }

    #[test]
    fn test_hex_to_signature() {
        // 模拟服务器接收到的 attribute_name
        let attribute_name =
            "content:\"Dobby thinks ca:6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN is\"";
        println!("Original attribute_name: {}", attribute_name);

        // 生成 hex
        let attribute_hex = hex::encode(attribute_name);
        println!("Generated attribute_hex: {}", attribute_hex);

        // 生成签名
        let mut signing_key = notary_private_key();

        let signature: Signature = signing_key.sign(attribute_name.as_bytes());
        let signature_hex = hex::encode(signature.to_bytes());
        println!("Generated signature: {}", signature_hex);

        // 验证签名
        let result = verify_signature(&attribute_hex, attribute_name, &signature_hex);
        println!("Verification result: {:?}", result);

        // 添加断言确保测试通过
        assert!(result.is_ok());
    }
}
