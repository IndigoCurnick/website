use crate::blog::BlogJson;

const JSON: &str = r#"
{
    "title": "After the Welfare State",
    "date": "2022-12-19",
    "desc": "A summary of the failings of the welfare state and where we go from here",
    "slug": "after-the-welfare-state",
    "tags": ["economics", "politics"]
}
"#;

#[test]
fn test_blogjson() {
    let json_data: BlogJson = match serde_json::from_str(JSON) {
        Ok(x) => x,
        Err(y) => panic!("{}", y),
    };

    println!("{:?}", json_data);
}
