use crate::error::ApiError;

use super::Attribute;

pub fn find_content_attribute(attributes: &[Attribute]) -> Result<String, ApiError> {
    attributes
        .iter()
        .find(|attr| attr.attribute_name.starts_with("content:"))
        .map(|attr| {
            let content = attr
                .attribute_name
                .splitn(2, ':') // 最多分割成两部分
                .nth(1) // 取第二部分
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .to_string();
            content
        })
        .ok_or_else(|| ApiError::NotFound("Message content is missing".to_string()))
}

pub fn find_author_attribute(attributes: &[Attribute]) -> Result<String, ApiError> {
    attributes
        .iter()
        .find(|attr| attr.attribute_name.starts_with("author:"))
        .map(|attr| {
            attr.attribute_name
                .split(':') // 用冒号分割
                .nth(1) // 取第二部分
                .unwrap_or("") // 如果没有则返回空字符串
                .trim() // 移除前后空白
                .trim_matches('"') // 移除引号
                .to_string()
        })
        .ok_or_else(|| ApiError::NotFound("Author information is missing".to_string()))
}

pub fn find_post_id_attribute(attributes: &[Attribute]) -> Result<String, ApiError> {
    attributes
        .iter()
        .find(|attr| attr.attribute_name.starts_with("id:"))
        .map(|attr| {
            attr.attribute_name
                .split(':') // 用冒号分割
                .nth(1) // 取第二部分
                .unwrap_or("") // 如果没有则返回空字符串
                .trim() // 移除前后空白
                .trim_matches('"') // 移除引号
                .to_string()
        })
        .ok_or_else(|| ApiError::NotFound("Post ID is missing".to_string()))
}

pub fn find_bookmark_count_attribute(attributes: &[Attribute]) -> Result<String, ApiError> {
    attributes
        .iter()
        .find(|attr| attr.attribute_name.starts_with("bookmark_count:"))
        .map(|attr| {
            attr.attribute_name
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .to_string()
        })
        .ok_or_else(|| ApiError::NotFound("Bookmark count is missing".to_string()))
}

pub fn find_favorite_count_attribute(attributes: &[Attribute]) -> Result<String, ApiError> {
    attributes
        .iter()
        .find(|attr| attr.attribute_name.starts_with("favorite_count:"))
        .map(|attr| {
            attr.attribute_name
                .split(':') // 用冒号分割
                .nth(1) // 取第二部分
                .unwrap_or("") // 如果没有则返回空字符串
                .trim() // 移除前后空白
                .trim_matches('"') // 移除引号
                .to_string()
        })
        .ok_or_else(|| ApiError::NotFound("Like count is missing".to_string()))
}

pub fn find_retweet_count_attribute(attributes: &[Attribute]) -> Result<String, ApiError> {
    attributes
        .iter()
        .find(|attr| attr.attribute_name.starts_with("retweet_count:"))
        .map(|attr| {
            attr.attribute_name
                .split(':') // 用冒号分割
                .nth(1) // 取第二部分
                .unwrap_or("") // 如果没有则返回空字符串
                .trim() // 移除前后空白
                .trim_matches('"') // 移除引号
                .to_string()
        })
        .ok_or_else(|| ApiError::NotFound("Share count is missing".to_string()))
}

#[test]
fn test_find_content_attribute() {
    let attributes = vec![Attribute {
            attribute_hex: "test".to_string(),
            attribute_name: "content: \"Dobby thinks ca:xxxxxxxx friends should always carry a sock in their pocket—you never know when freedom might come knocking! \\n\\nDobby once used a teacup to solve a big problem, proving even the smallest things can hold great power.\"".to_string(),
            signature: "test".to_string(),
        },
        Attribute{
            attribute_hex: "321312312".to_string(),
            attribute_name: "dsant: \"Dobby thinks friends should always carry a sock in their pocket—you never know when freedom might come knocking! \\n\\nDobby once used a teacup to solve a big problem, proving even the smallest things can hold great power.\"".to_string(),
            signature: "test".to_string(),
        },
        Attribute{
            attribute_hex: "617574686f723a20223132343836363830363531343839373330363122".to_string(),
            attribute_name: "dsant: \"Dobby thinks friends should always carry a sock in their pocket—you never know when freedom might come knocking! \\n\\nDobby once used a teacup to solve a big problem, proving even the smallest things can hold great power.\"".to_string(),
            signature: "test".to_string(),
        },
        Attribute{
            attribute_hex: "637265617465645f61743a2022576564204a616e2031352030393a31313a3338202b30303030203230323522".to_string(),
            attribute_name: "author: \"1248668065148973061\"".to_string(),
            signature: "test".to_string(),
        },
        Attribute{
            attribute_hex: "69643a202231383739343536393734353433383532363522".to_string(),
            attribute_name: "id: \"187945697454385265\"".to_string(),
            signature: "test".to_string(),
        },
        Attribute{
            attribute_hex: "626f6f6b6d61726b5f636f756e743a2030".to_string(),
            attribute_name: "bookmark_count: 0".to_string(),
            signature: "test".to_string(),
        },
        Attribute{
            attribute_hex: "6661766f726974655f636f756e743a2030".to_string(),
            attribute_name: "favorite_count: 0".to_string(),
            signature: "test".to_string(),
        },
        Attribute{
            attribute_hex: "726574776565745f636f756e743a2030".to_string(),
            attribute_name: "retweet_count: 0".to_string(),
            signature: "test".to_string(),
        },
    ];

    let content = find_content_attribute(&attributes).expect("content not found");
    println!("content: is {:?}", content);

    let author = find_author_attribute(&attributes).expect("author not found");
    println!("author: is {:?}", author);

    let post_id = find_post_id_attribute(&attributes).expect("post_id not found");
    println!("post_id: is {:?}", post_id);

    let bookmark_count =
        find_bookmark_count_attribute(&attributes).expect("bookmark_count not found");
    println!("bookmark_count: is {:?}", bookmark_count);

    let favorite_count =
        find_favorite_count_attribute(&attributes).expect("favorite_count not found");
    println!("favorite_count: is {:?}", favorite_count);

    let retweet_count = find_retweet_count_attribute(&attributes).expect("retweet_count not found");
    println!("retweet_count: is {:?}", retweet_count);
}
