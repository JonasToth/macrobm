/// We parse configs cause we are so nice

use yaml_rust::Yaml;

/// Parse a yaml-Vector to strings, to use this list of strings as argument to start the wanted
/// process.
pub fn yaml_args_to_stringlist(args: &Vec<Yaml>) -> Vec<String>
{
    let mut result: Vec<String> = Vec::new();

    for arg_candidate in args {
        match arg_candidate {
            &Yaml::Real(ref a)   => result.push(a.clone()),
            &Yaml::Integer(ref a)=> result.push(a.to_string()),
            &Yaml::String(ref a) => result.push(a.clone()),
            &Yaml::Null          => (),
            _                    => panic!("Wrong argument type passed for that command!"),
        }
    }

    result
}
