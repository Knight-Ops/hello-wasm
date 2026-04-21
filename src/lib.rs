use extism_pdk::*;

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    Ok(format!("Hello, {} from Wasm!", name))
}

#[plugin_fn]
pub fn shout(input: String) -> FnResult<String> {
    Ok(input.to_uppercase())
}
