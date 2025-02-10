use regex::Regex;

use crate::error::ApiError;

//todo : 获取ca的正则需要改
pub fn extract_ca(text: &str) -> Result<String, ApiError> {
    let re =
        Regex::new(r"(?i)ca\s*:\s*((?:0x[a-fA-F0-9]{40}|[1-9A-HJ-NP-Za-km-z]{44})\b)").unwrap();
    re.captures(text)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| ApiError::NotFound("ca not found".to_string()))
}

#[test]
fn test_extract_ca() {
    // 测试小写无空格
    let text1 = "Dobby thinks friends should ca:6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN always carry a sock";
    assert_eq!(matches!(extract_ca(text1), Ok(_)), true);
    assert_eq!(
        extract_ca(text1).unwrap(),
        "6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN"
    );

    // 测试大写有空格
    let text2 = "Start CA : ABC123 end";
    assert_eq!(
        matches!(extract_ca(text2), Err(ApiError::NotFound(_))),
        true
    );

    // 测试混合大小写
    let text3 = "Test Ca:XYZ789 test";
    assert_eq!(
        matches!(extract_ca(text3), Err(ApiError::NotFound(_))),
        true
    );

    // 测试没有 ca:
    let text4 = "Dobby thinks friends should always carry a sock";
    assert_eq!(
        matches!(extract_ca(text4), Err(ApiError::NotFound(_))),
        true
    );

    let text5 = "Dobby thinks ca:xxxxxxxx friends should always carry a sock in their pocket—you never know when freedom might come knocking! \\n\\nDobby once used a teacup to solve a big problem, proving even the smallest things can hold great power.\"";
    assert_eq!(
        matches!(extract_ca(text5), Err(ApiError::NotFound(_))),
        true
    );
    let text6 = "Dobby thinks friends should ca: always carry a sock";
    assert_eq!(
        matches!(extract_ca(text6), Err(ApiError::NotFound(_))),
        true
    );

    // 测试有效的 base58 格式
    let text7 = "Dobby thinks friends should ca:7mHCx9iXPJ7EJDbDAUGmej39Kme8cxZfeVi1EAvEpump";
    assert_eq!(
        extract_ca(text7).unwrap(),
        "7mHCx9iXPJ7EJDbDAUGmej39Kme8cxZfeVi1EAvEpump"
    );

    // 测试无效格式，应该返回错误
    let text8 = "Start CA : ABC123 end";
    assert!(matches!(extract_ca(text8), Err(ApiError::NotFound(_))));

    // 测试有效的 0x 格式
    let text9 = "Test Ca:0x85e58d0f9152669083bda1e6638fa6400898d0ee test";
    assert_eq!(
        extract_ca(text9).unwrap(),
        "0x85e58d0f9152669083bda1e6638fa6400898d0ee"
    );
}
