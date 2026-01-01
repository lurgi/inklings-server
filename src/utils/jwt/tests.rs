use super::*;

#[test]
fn test_generate_and_verify_token() {
    let secret = "test_secret_key_min_32_chars_long";
    let user_id = 123;
    let expiration_hours = 24;

    let token = generate_token(user_id, secret, expiration_hours).unwrap();
    let claims = verify_token(&token, secret).unwrap();

    assert_eq!(claims.sub, "123");
    assert!(claims.exp > claims.iat);
}

#[test]
fn test_invalid_secret_fails() {
    let secret = "test_secret_key_min_32_chars_long";
    let wrong_secret = "wrong_secret_key_min_32_chars_long";

    let token = generate_token(1, secret, 24).unwrap();
    let result = verify_token(&token, wrong_secret);

    assert!(result.is_err());
}
