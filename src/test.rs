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
        println!("{}", result);
    }
}