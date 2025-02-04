#[cfg(test)]
mod tests {
    #[test]
    fn test_format() {
        let code = r#"
        local a = 1
        local b = 2
        print(a+b)
        "#;
        let result = crate::reformat_code(code, "test.lua");
        let expected = "local a = 1\nlocal b = 2\nprint(a + b)\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_range_format() {
        let code = r#"
        local a         = 1
        local b = 2
        print(a+b)
        "#;
        let result = crate::range_format_code(code, "test.lua", 1, 1, 1, 1);
        let expected = "local a = 1\n";
        assert_eq!(result.text, expected);
    }

    #[test]
    fn test_check_code_style() {
        let code = r#"
        print(a+b)
        "#;

        let result = crate::check_code_style("test.lua", code);
        println!("{:?}", result);
    }

}