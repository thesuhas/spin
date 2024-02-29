use anyhow::{Context, Result};
use spin_sdk::{
    http::{IntoResponse, Request, Response},
    http_component, variables,
};

#[http_component]
fn handle_vault_variable_test(req: Request) -> Result<impl IntoResponse> {
    let attempt = std::str::from_utf8(req.body()).unwrap();
    let expected = variables::get("token").context("could not get variable")?;
    let response = if constant_time_compare(&expected, attempt) {
        "accepted"
    } else {
        "denied"
    };
    let response_json = format!("{{\"authentication\": \"{}\"}}", response);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(response_json)
        .build())
}

fn constant_time_compare(a: &str, b: &str) -> bool {
    let mut diff = 0;
    if a.len() != b.len() {
        return false;
    }
    for (a, b) in a.bytes().zip(b.bytes()) {
        diff |= a ^ b;
    }
    diff == 0
}

mod test {
    #[test]
    fn test_constant_time_compare() {
        assert!(super::constant_time_compare("test", "test"));
        assert!(!super::constant_time_compare("test", "test1"));
        assert!(!super::constant_time_compare("test", "some"));
    }
}
