use std::borrow::Cow;

pub fn encode_to_gbk(text: &str) -> Cow<'_, [u8]> {
    let (encoded, _, _) = encoding_rs::GBK.encode(text);
    encoded
}

pub fn decode_gbk(data: &[u8]) -> String {
    let (str, _, _) = encoding_rs::GBK.decode(data);
    str.to_string()
}
