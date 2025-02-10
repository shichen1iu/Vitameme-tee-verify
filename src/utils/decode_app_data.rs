use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedData {
    pub hostname: String,
    pub request_url: String,
    pub request: String,
    pub response_header: String,
    pub response_body: String,
    pub semaphore_identity_commitment: String,
}

const SEMAPHORE_IDENTITY_HEADER: &str = "x-semaphore-identity"; // 修改常量名称

pub fn decode_app_data(hex_string: &str) -> DecodedData {
    // 移除所有空白字符
    let hex_string = hex_string.replace(char::is_whitespace, "");

    // 解码十六进制字符串
    let decoded_string = hex_string
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let hex_bytes = std::str::from_utf8(chunk).unwrap_or("");
            u8::from_str_radix(hex_bytes, 16).unwrap_or(0) as char
        })
        .collect::<String>();

    // 分割请求和响应
    let parts: Vec<&str> = decoded_string.split("\r\n\r\n").collect();
    let (request, response_header, response_body) = match parts.as_slice() {
        [req, res_header, res_body, ..] => (*req, *res_header, *res_body),
        _ => ("", "", ""),
    };

    // 解析请求行
    let lines: Vec<&str> = request.split("\r").collect();

    // 获取请求URL
    let request_url = lines
        .iter()
        .find(|line| line.starts_with("GET") || line.starts_with("POST"))
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            parts.get(1).unwrap_or(&"").to_string()
        })
        .unwrap_or_default();

    // 从 request_url 中提取 hostname
    let hostname = extract_hostname(&request_url);

    let semaphore_identity_commitment = lines
        .iter()
        .find(|line| {
            line.to_lowercase()
                .contains(&SEMAPHORE_IDENTITY_HEADER.to_lowercase())
        })
        .map(|line| line.split(": ").nth(1).unwrap_or("").trim().to_string())
        .unwrap_or_default();

    DecodedData {
        hostname,
        request_url,
        request: request.to_string(),
        response_header: response_header.to_string(),
        response_body: response_body.to_string(),
        semaphore_identity_commitment,
    }
}

fn extract_hostname(url: &str) -> String {
    // 修改正则表达式以匹配 TypeScript 版本的模式
    let re = Regex::new(r"^(?:https?://)?(?:[^@\n]+@)?(?:www\.)?([^:/\n?]+)")
        .unwrap_or(Regex::new(r"").unwrap());

    re.captures(url)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_app_data() {
        let hex_string = "4745542068747470733a2f2f782e636f6d2f692f6170692f6772617068716c2f457a366b5250795862714e6c684277634e4d70552d512f547765657444657461696c3f7661726961626c65733d253742253232666f63616c5477656574496425323225334125323231383739343536333937343534333835323635253232253243253232776974685f7275785f696e6a656374696f6e7325323225334166616c736525324325323272616e6b696e674d6f646525323225334125323252656c6576616e6365253232253243253232696e636c75646550726f6d6f746564436f6e74656e742532322533417472756525324325323277697468436f6d6d756e6974792532322533417472756525324325323277697468517569636b50726f6d6f7465456c69676962696c69747954776565744669656c647325323225334174727565253243253232776974684269726477617463684e6f7465732532322533417472756525324325323277697468566f696365253232253341747275652537442666656174757265733d25374225323270726f66696c655f6c6162656c5f696d70726f76656d656e74735f7063665f6c6162656c5f696e5f706f73745f656e61626c656425323225334174727565253243253232727765625f7469706a61725f636f6e73756d7074696f6e5f656e61626c656425323225334174727565253243253232726573706f6e736976655f7765625f6772617068716c5f6578636c7564655f6469726563746976655f656e61626c65642532322533417472756525324325323276657269666965645f70686f6e655f6c6162656c5f656e61626c656425323225334166616c736525324325323263726561746f725f737562736372697074696f6e735f74776565745f707265766965775f6170695f656e61626c656425323225334174727565253243253232726573706f6e736976655f7765625f6772617068716c5f74696d656c696e655f6e617669676174696f6e5f656e61626c656425323225334174727565253243253232726573706f6e736976655f7765625f6772617068716c5f736b69705f757365725f70726f66696c655f696d6167655f657874656e73696f6e735f656e61626c656425323225334174727565253243253232726573706f6e736976655f7765625f6a65746675656c5f6672616d6525323225334166616c7365253243253232726573706f6e736976655f7765625f67726f6b5f73686172655f6174746163686d656e745f656e61626c65642532322533417472756525324325323261727469636c65735f707265766965775f656e61626c656425323225334174727565253243253232726573706f6e736976655f7765625f656469745f74776565745f6170695f656e61626c6564253232253341747275652532432532326772617068716c5f69735f7472616e736c617461626c655f727765625f74776565745f69735f7472616e736c617461626c655f656e61626c656425323225334174727565253243253232766965775f636f756e74735f657665727977686572655f6170695f656e61626c6564253232253341747275652532432532326c6f6e67666f726d5f6e6f74657477656574735f636f6e73756d7074696f6e5f656e61626c656425323225334174727565253243253232726573706f6e736976655f7765625f747769747465725f61727469636c655f74776565745f636f6e73756d7074696f6e5f656e61626c65642532322533417472756525324325323274776565745f6177617264735f7765625f74697070696e675f656e61626c656425323225334166616c7365253243253232726573706f6e736976655f7765625f67726f6b5f616e616c797369735f627574746f6e5f66726f6d5f6261636b656e642532322533417472756525324325323263726561746f725f737562736372697074696f6e735f71756f74655f74776565745f707265766965775f656e61626c656425323225334166616c736525324325323266726565646f6d5f6f665f7370656563685f6e6f745f72656163685f66657463685f656e61626c6564253232253341747275652532432532327374616e64617264697a65645f6e75646765735f6d6973696e666f2532322533417472756525324325323274776565745f776974685f7669736962696c6974795f726573756c74735f7072656665725f67716c5f6c696d697465645f616374696f6e735f706f6c6963795f656e61626c656425323225334174727565253243253232727765625f766964656f5f74696d657374616d70735f656e61626c6564253232253341747275652532432532326c6f6e67666f726d5f6e6f74657477656574735f726963685f746578745f726561645f656e61626c6564253232253341747275652532432532326c6f6e67666f726d5f6e6f74657477656574735f696e6c696e655f6d656469615f656e61626c656425323225334174727565253243253232726573706f6e736976655f7765625f67726f6b5f696d6167655f616e6e6f746174696f6e5f656e61626c656425323225334174727565253243253232726573706f6e736976655f7765625f656e68616e63655f63617264735f656e61626c656425323225334166616c7365253744266669656c64546f67676c65733d2537422532327769746841727469636c6552696368436f6e74656e745374617465253232253341747275652532432532327769746841727469636c65506c61696e5465787425323225334166616c73652532432532327769746847726f6b416e616c797a6525323225334166616c736525324325323277697468446973616c6c6f7765645265706c79436f6e74726f6c7325323225334166616c736525374420485454502f312e310d0a636f6f6b69653a206e696768745f6d6f64653d323b206b64743d61334e6a427070315137465462744d6e4130494a4133376e6d654d46487a353366376843524c55303b205f67613d4741312e322e313135383438393135322e313733363738333531373b20646e743d313b2067756573745f69643d76312533413137333638313733313231383635343332343b2067756573745f69645f6d61726b6574696e673d76312533413137333638313733313231383635343332343b2067756573745f69645f6164733d76312533413137333638313733313231383635343332343b20617574685f746f6b656e3d353666623763656564663361656637623332363966346561316666623335353230323433303263653b206374303d613834383833656339613631346633633564396135396533343532666535306461303536363030303531313131633537396638313735366463356462343531333533333535363534616365623639306562366435396536356566643663353763386639653962663632363537336236366166636539313061346664303937623630306565656638303837356531336361653739353930643562626432363661310d0a747769643d75253344313234383636383036353134383937333036313b20706572736f6e616c697a6174696f6e5f69643d2276315f6d4d396b66376e5567554b6f4f686652364b656c68413d3d223b206465735f6f70745f696e3d593b2070685f7068635f545864706f63624756655a566d35564A6d417348544d72436f664251753365306b4e3848474d4e475456575f706f7374686f673d25374225323264697374696e63745f696425323225334125323230313934363466372d663238322d373838342d386338352d32336239313464376666616625323225324325323225323473657369642532322533412535423137333732393330383635363625324325323230313934376562632d393637612d376139612d386562312d383234396237303866613966253232253243313733373239333037363039302535442537443b205f67615f4b45575a3147354d42333d4753312e322e313733373533363739302e342e302e313733373533363739302e36302e302e303b206c616e673d656e0d0a636f6e74656e742d747970653a206170706c69636174696f6e2f6a736f6e0d0a782d747769747465722d617574682d747970653a204f417574683253657373696f6e0d0a7365632d63682d75612d706c6174666f726d3a20226d61634f53220d0a636f6e6e656374696f6e3a20636c6f73650d0a7365632d63682d75612d6d6f62696c653a203f300d0a782d636c69656e742d7472616e73616374696f6e2d69643a206c7a45702b7676384e7a566e626349636a34366d73362b53446548636f49464d6c6a7864732f63475454625a59735a583169696757756d3767472f6b347134684971616d785a514f305579365377622f3543497154387a66523454716c410d0a7365632d63682d75613a20224e6f742041284272616e64223b763d2238222c20224368726f6d69756d223b763d22313332222c2022476f6f676c65204368726f6d65223b763d22313332220d0a7365632d66657463682d736974653a2073616d652d6f726967696e0d0a6163636570742d656e636f64696e673a206964656e746974790d0a6163636570742d6c616e67756167653a20656e2d55532c656e3b713d302e392c7a682d54573b713d302e382c7a682d434e3b713d302e372c7a683b713d302e362c72753b713d302e352c65733b713d302e342c6a613b713d302e330d0a782d747769747465722d636c69656e742d6c616e67756167653a20656e0d0a7365632d66657463682d6d6f64653a20636f72730d0a782d73656d6170686f72652d6964656e746974793a20313637623063643533393335376365376633633535366338303534656566646230656564613138343730306238656230343065376331616132353633643033612c313966326665386661363032663137303734633835633734653264336232616536316238356261363731316338626338636263383865663632646638353864610d0a782d636c69656e742d757569643a2039333132666231642d323564302d343837352d393037372d6631386166663738383664640d0a757365722d6167656e743a204d6f7a696c6c612f352e3020284d6163696e746f73683b20496e74656c204d6163204f5320582031305f31355f3729204170706c655765624b69742f3533372e333620284b48544d4c2c206c696b65204765636b6f29204368726f6d652f3133322e302e302e30205361666172692f3533372e33360d0a782d747769747465722d6163746976652d757365723a207965730d0a782d637372662d746f6b656e3a20613834383833656339613631346633633564396135396533343532666535306461303536363030303531313131633537396638313735366463356462343531333533333535363534616365623639306562366435396536356566643663353763386639653962663632363537336236366166636539313061346664303937623630306565656638303837356531336361653739353930643562626432363661310d0a617574686f72697a6174696f6e3a20426561726572204141414141414141414141414141414141414141414e52494c674141414141416e4e77497a55656a52434f75483545364938786e5a7a3470755473253344315a76377474666b384c463831495571313663486a684c54764a753446413333414757576a4370546e410d0a686f73743a20782e636f6d0d0a7365632d66657463682d646573743a20656d7074790d0a6163636570743a202a2f2a0d0a726566657265723a2068747470733a2f2f782e636f6d2f676f676f6475636b313031312f7374617475732f313837393435363339373435343338353236350d0a0d0a485454502f312e3120323030204f4b0d0a646174653a205475652c2030342046656220323032352030343a31363a333520474d540d0a706572663a20373430323832373130340d0a766172793a204f726967696e0d0a707261676d613a206e6f2d63616368650d0a7365727665723a207473615f6d0d0a657870697265733a205475652c203331204d617220313938312030353a30303a303020474d540d0a636f6e74656e742d747970653a206170706c69636174696f6e2f6a736f6e3b20636861727365743d7574662d380d0a63616368652d636f6e74726f6c3a206e6f2d63616368652c206e6f2d73746f72652c206d7573742d726576616c69646174652c207072652d636865636b3d302c20706f73742d636865636b3d300d0a6c6173742d6d6f6469666965643a205475652c2030342046656220323032352030343a31363a333620474d540d0a636f6e74656e742d6c656e6774683a20333430380d0a782d6672616d652d6f7074696f6e733a2053414d454f524947494e0d0a782d7472616e73616374696f6e2d69643a20363062613235633635623765653266610d0a782d7873732d70726f74656374696f6e3a20300d0a782d726174652d6c696d69742d6c696d69743a203135300d0a782d726174652d6c696d69742d72657365743a20313733383634323938390d0a636f6e74656e742d646973706f736974696f6e3a206174746163686d656e743b2066696c656e616d653d6a736f6e2e6a736f6e0d0a782d7466652d70726573657276652d626f64793a20747275650d0a782d636f6e74656e742d747970652d6f7074696f6e733a206e6f736e6966660d0a782d726174652d6c696d69742d72656d61696e696e673a203134350d0a782d747769747465722d726573706f6e73652d746167733a20426f756e636572436f6d706c69616e740d0a7374726963742d7472616e73706f72742d73656375726974793a206d61782d6167653d3633313133383531390d0a782d726573706f6e73652d74696d653a203139380d0a782d636f6e6e656374696f6e2d686173683a20343234613665383337666662303736623065663663393537653261626338663539616165656661346363353539333034633562393730313735363531393330300d0a636f6e6e656374696f6e3a20636c6f73650d0a0d0a7b2264617461223a7b2274687265616465645f636f6e766572736174696f6e5f776974685f696e6a656374696f6e735f7632223a7b22696e737472756374696f6e73223a5b7b2274797065223a2254696d656c696e65416464456e7472696573222c22656e7472696573223a5b7b22656e7472794964223a2274776565742d31383739343536333937343534333835323635222c22736f7274496e646578223a2237333433393135363339343030333930353432222c22636f6e74656e74223a7b22656e74727954797065223a2254696d656c696e6554696d656c696e654974656d222c225f5f747970656e616d65223a2254696d656c696e6554696d656c696e654974656d222c226974656d436f6e74656e74223a7b226974656d54797065223a2254696d656c696e655477656574222c225f5f747970656e616d65223a2254696d656c696e655477656574222c2274776565745f726573756c7473223a7b22726573756c74223a7b225f5f747970656e616d65223a2255736572222c22726573745f6964223a2231383739343536333937343534333835323635222c226861735f6269726477617463685f6e6f746573223a66616c73652c22636f7265223a7b22757365725f726573756c7473223a7b22726573756c74223a7b225f5f747970656e616d65223a2255736572222c226964223a2256584e6c636a6f784d6a51344e6a59344d4459314d5451344f54637a4d445978222c22726573745f6964223a2231323438363638303635313438393733303631222c22616666696c69617465735f686967686c6967687465645f6c6162656c223a7b7d2c226861735f6772616475617465645f616363657373223a747275652c227061726f64795f636f6d6d656e746172795f66616e5f6c6162656c223a224e6f6e65222c2269735f626c75655f7665726966696564223a66616c73652c2270726f66696c655f696d6167655f7368617065223a22436972636c65222c226c6567616379223a7b22666f6c6c6f77696e67223a66616c73652c2263616e5f646d223a747275652c2263616e5f6d656469615f746167223a747275652c22637265617465645f6174223a22467269204170722031302031373a34343a3130202b303030302032303230222c2264656661756c745f70726f66696c65223a747275652c2264656661756c745f70726f66696c655f696d616765223a66616c73652c226465736372697074696f6e223a22676f20676f206475636b21222c22656e746974696573223a7b226465736372697074696f6e223a7b2275726c73223a5b5d7d7d2c22666173745f666f6c6c6f776572735f636f756e74223a302c226661766f72697465735f636f756e74223a332c22666f6c6c6f776572735f636f756e74223a332c22667269656e64735f636f756e74223a31372c226861735f637573746f6d5f74696d656c696e6573223a66616c73652c2269735f7472616e736c61746f72223a66616c73652c226c69737465645f636f756e74223a302c226c6f636174696f6e223a224c6f7320416e67656c65732c204341222c226d656469615f636f756e74223a302c226e616d65223a22476f476f4475636b222c226e656564735f70686f6e655f766572696669636174696f6e223a66616c73652c226e6f726d616c5f666f6c6c6f776572735f636f756e74223a332c2270696e6e65645f74776565745f6964735f737472223a5b5d2c22706f737369626c795f73656e736974697665223a66616c73652c2270726f66696c655f62616e6e65725f75726c223a2268747470733a2f2f7062732e7477696d672e636f6d2f70726f66696c655f62616e6e6572732f313234383636383036353134383937333036312f31373337333635393632222c2270726f66696c655f696d6167655f75726c5f7477656574223a2268747470733a2f2f7062732e7477696d672e636f6d2f70726f66696c655f696d616765732f313838313237353233353039353533313532302f6256314141765a505f6e6f726d616c2e6a7067222c2270726f66696c655f696e7465727374697469616c5f74797065223a22222c2273637265656e5f6e616d65223a22676f676f6475636b31303131222c2273746174757365735f636f756e74223a392c227472616e736c61746f725f74797065223a226e6f6e65222c227665726966696564223a66616c73652c2277616e745f7265747765657473223a66616c73652c227769746868656c645f696e5f636f756e7472696573223a5b5d7d2c227469706a61725f73657474696e6773223a7b7d7d7d7d2c22756e6d656e74696f6e5f64617461223a7b7d2c22656469745f636f6e74726f6c223a7b22656469745f74776565745f696473223a5b2231383739343536333937343534333835323635225d2c226564697461626c655f756e74696c5f6d73656373223a2231373336393335383938303030222c2269735f656469745f656c696769626c65223a747275652c2265646974735f72656d61696e696e67223a2235227d2c2269735f7472616e736c617461626c65223a66616c73652c227669657773223a7b22636f756e74223a223636222c227374617465223a22456e61626c656457697468436f756e74227d2c22736f75726365223a223c6120687265663d5c2268747470733a2f2f747769747465722e636f6d5c222072656c3d5c226e6f666f6c6c6f775c223e54776565744465636b20576562204170703c2f613e222c2267726f6b5f616e616c797369735f627574746f6e223a747275652c226c6567616379223a7b22626f6f6b6d61726b5f636f756e74223a302c22626f6f6b6d61726b6564223a66616c73652c22637265617465645f6174223a22576564204a616e2031352030393a31313a3338202b303030302032303235222c22636f6e766572736174696f6e5f69645f737472223a2231383739343536333937343534333835323635222c22646973706c617954797065223a5b302c3231385d2c22656e746974696573223a7b226861736874616773223a5b5d2c2273796d626f6c73223a5b5d2c2274696d657374616d7073223a5b5d2c2275726c73223a5b5d2c22757365725f6d656e74696f6e73223a5b5d7d2c226661766f726974655f636f756e74223a302c226661766f7269746564223a66616c73652c2266756c6c5f74657874223a22446f626279207468696e6b7320667269656e64732073686f756c6420616c77617973206361727279206120736f636b20696e20746865697220706f636b6574e28094796f75206e65766572206b6e6f77207768656e2066726565646f6d206d6967687420636f6d65206b6e6f636b696e6721205c6e5c6e446f626279206f6e6365207573656420612074656163757020746f20736f6c76652061206269672070726f626c656d2c2070726f76696e67206576656e2074686520736d616c6c657374207468696e67732063616e20686f6c6420677265617420706f7765722e222c2269735f71756f74655f737461747573223a66616c73652c226c616e67223a22656e222c2271756f74655f636f756e74223a302c227265706c795f636f756e74223a302c22726574776565745f636f756e74223a302c22757365725f69645f737472223a2231323438363638303635313438393733303631222c2269645f737472223a2231383739343536333937343534333835323635227d2c22717569636b5f70726f6d6f74655f656c69676962696c697479223a7b22656c69676962696c697479223a22496e656c696769626c654e6f7450726f66657373696f6e616c227d7d7d2c227477656574446973706c617954797065223a225477656574222c226861734d6f646572617465645265706c696573223a66616c73657d7d7d2c7b22656e7472794964223a22747765657464657461696c72656c617465647477656574732d34303835353330333736333238333334373237222c22736f7274496e646578223a2231222c22636f6e74656e74223a7b22656e74727954797065223a2254696d656c696e6554696d656c696e654d6f64756c65222c225f5f747970656e616d65223a2254696d656c696e6554696d656c696e654d6f64756c65222c226974656d73223a5b5d2c22646973706c617954797065223a22566572746963616c222c22686561646572223a7b22646973706c617954797065223a22436c6173736963222c2274657874223a22446973636f766572206d6f7265222c22736f6369616c436f6e74657874223a7b2274797065223a2254696d656c696e6547656e6572616c436f6e74657874222c22636f6e7465787454797065223a22546578744f6e6c79222c2274657874223a22536f75726365642066726f6d206163726f73732058227d2c22737469636b79223a747275657d2c22636c69656e744576656e74496e666f223a7b22636f6d706f6e656e74223a2272656c617465645f7477656574222c2264657461696c73223a7b22636f6e766572736174696f6e44657461696c73223a7b22636f6e766572736174696f6e53656374696f6e223a2252656c617465645477656574227d7d7d7d5d7d2c7b2274797065223a2254696d656c696e655465726d696e61746554696d656c696e65222c22646972656374696f6e223a22546f70227d5d7d7d7d";

        let result = decode_app_data(hex_string);

        println!("解码结果:");
        println!("----------------------------------------");
        println!("主机名: {}", result.hostname);
        println!("请求URL: {}", result.request_url);
        println!("----------------------------------------");
        println!("请求内容:\n{}", result.request);
        println!("----------------------------------------");
        println!("响应头:\n{}", result.response_header);
        println!("----------------------------------------");
        println!("响应体:\n{}", result.response_body);
        println!("----------------------------------------");
        println!(
            "Semaphore Identity Commitment:\n{}",
            result.semaphore_identity_commitment
        );
        println!("----------------------------------------");
    }

    #[test]
    fn test_extract_hostname() {
        assert_eq!(extract_hostname("https://example.com/test"), "example.com");
        assert_eq!(extract_hostname("http://www.example.com"), "example.com");
        assert_eq!(extract_hostname("example.com/path"), "example.com");
        assert_eq!(extract_hostname(""), "");
    }
}

