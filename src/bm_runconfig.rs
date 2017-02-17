/// Define values used to configure a benchmark run.
pub struct RunConfig
{
    name: String,
    description: String,
    runcount: i64

    command: String,
    directory: String, // optional
    environment: String, // optional

}
