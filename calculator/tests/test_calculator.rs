use calculator::calculator::parse_to_float;

#[test]
fn test_parse_to_float() {
    // Test valid float
    assert_eq!(parse_to_float("3.14"), Ok(3.14));
    assert_eq!(parse_to_float("0.5"), Ok(0.5));
    assert_eq!(parse_to_float("2.0"), Ok(2.0));

    // Test valid integer
    assert_eq!(parse_to_float("42"), Ok(42.0));
    assert_eq!(parse_to_float("0"), Ok(0.0));
    assert_eq!(parse_to_float("1000"), Ok(1000.0));

    // Test negative numbers
    assert_eq!(parse_to_float("-7.5"), Ok(-7.5));
    assert_eq!(parse_to_float("-42"), Ok(-42.0));

    // Test scientific notation
    assert_eq!(parse_to_float("1e3"), Ok(1000.0));
    assert_eq!(parse_to_float("1.5e-2"), Ok(0.015));

    // Test whitespace handling
    assert_eq!(parse_to_float("  3.14  "), Ok(3.14));
    assert_eq!(parse_to_float("\t42\n"), Ok(42.0));

    // Test invalid inputs
    assert!(parse_to_float("abc").is_err());
    assert!(parse_to_float("1.2.3").is_err());
    assert!(parse_to_float("").is_err());
    assert!(parse_to_float(" ").is_err());
    assert!(parse_to_float("3.14.15").is_err());
    assert!(parse_to_float("3,14").is_err());
    // assert!(parse_to_float("Infinity").is_err());
    // assert!(parse_to_float("NaN").is_err());

    // // Test error messages
    // assert_eq!(
    //     parse_to_float("abc"),
    //     Err("Error: abc is not a valid number".to_string())
    // );
    // assert_eq!(
    //     parse_to_float(""),
    //     Err("Error:  is not a valid number".to_string())
    // );
}