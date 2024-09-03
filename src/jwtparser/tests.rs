use crate::jwtparser::{JwtTool, Token};

#[test]
fn should_decode_the_base64_and_return_as_string() {
    let want = "hello world\n";
    let value: &str = "aGVsbG8gd29ybGQK";
    let got = JwtTool::decode(value);

    assert_eq!(want, got);
}

#[test]
#[should_panic(expected = "Failed to decode token: InvalidByte(6, 32)")]
fn should_fails_if_is_not_base64_and_return_failed_msg() {
    let want = "should fail";

    JwtTool::decode(want);
}

#[test]
#[should_panic(expected = "Failed to parse token, err: this is not a JWT token")]
fn should_fails_if_value_is_not_a_jwt_encoded() {
    JwtTool::parse_jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string());
}

#[test]
fn should_return_a_token_when_value_is_a_well_formatted_jwt() {
    let header = "{\n  \"alg\": \"HS256\",\n  \"typ\": \"JWT\"\n}".to_string();
    let payload =
        "{\n  \"iat\": 1516239022,\n  \"name\": \"John Doe\",\n  \"sub\": \"1234567890\"\n}"
            .to_string();

    let want = Token {
        header,
        payload,
        signature: "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c".to_string(),
    };

    let data = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    let got = JwtTool::parse_jwt(data.to_string());

    assert_eq!(want.header, got.header);
    assert_eq!(want.payload, got.payload);
    assert_eq!(want.signature, got.signature);
}
