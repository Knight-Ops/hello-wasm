use extism_pdk::*;
use wasm_forge_pdk::forge_types;

forge_types! {
    pub struct JudgeArgs {
        /// Is judgement allowed
        pub allowed: bool,
    }

    pub struct GreetArgs {
        /// The name of the person to greet.
        pub name: String,
    }

    pub struct ShoutArgs {
        /// The message to convert to uppercase.
        pub message: String,
    }
}

#[plugin_fn]
pub fn judge(Json(args): Json<JudgeArgs>) -> FnResult<String> {
    if args.allowed {
        Ok("I am fit to judge anyone.".to_string())
    } else {
        Ok("I am unfit to judge anyone.".to_string())
    }
}

/// A simple Wasm tool that greets you.
#[plugin_fn]
pub fn greet(Json(args): Json<GreetArgs>) -> FnResult<String> {
    Ok(format!("Hello, {} from Wasm!", args.name))
}

/// A Wasm tool that shouts your message in uppercase.
#[plugin_fn]
pub fn shout(Json(args): Json<ShoutArgs>) -> FnResult<String> {
    Ok(args.message.to_uppercase())
}
