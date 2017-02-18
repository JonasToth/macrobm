/// Define values used to configure a benchmark run.
#[derive(Debug)]
pub struct RunConfig
{
    pub name: String,
    pub description: String,
    pub count: i64,

    pub command: String,
    pub args: Vec<String>, // empty vector if no args were configured
    pub directory: String, // optional
    pub environment: Vec<String>, // optional

}
