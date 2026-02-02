use qalculate_rs::Qalculate;

#[test]
fn test_basic_calculations() {
    let calc = Qalculate::new().expect("Failed to create calculator");

    let test_cases = vec![
        ("2 + 2", "4"),
        ("10 - 5", "5"),
        ("3 * 4", "12"),
        ("20 / 5", "4"),
        ("2^3", "8"),
        ("sqrt(16)", "4"),
        ("sin(0)", "0"),
    ];

    for (expression, expected) in test_cases {
        assert_eq!(calc.calculate_string(expression).unwrap(), expected);
    }
}
