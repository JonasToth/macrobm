#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;


#[derive(Serialize, Deserialize)]
struct RunConfig
{
    name: String,
    description: String,

    command: String,
    directory: String, // optional
    environment: String, // optional

    runcount: i32, // optional
}
