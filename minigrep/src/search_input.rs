use std::env;

pub struct SearchInput {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl SearchInput {
    pub fn build(args: Vec<String>) -> Result<SearchInput, &'static str> {
        let query = Self::read_string_variable(&args, "search-query").unwrap_or(String::new());
        let ignore_case = Self::read_boolean_variable(&args, "ignore-case");
        let file_path = Self::read_string_variable(&args, "file-path").ok_or("Missing file-path variable")?;

        Ok(SearchInput {
            query,
            file_path,
            ignore_case,
        })
    }

    fn read_boolean_variable(args: &Vec<String>, var_name: &str) -> bool {
        let (env_var_name, flag_name) = Self::var_name_variations(var_name);


        for flag in args.iter() {
            if flag.eq(&flag_name) {
                return true;
            }
        }

        match env::var(env_var_name) {
            Ok(value) => value.to_lowercase() != "false" && value != "0",
            Err(_) => false,
        }
    }


    fn read_string_variable(args: &Vec<String>, var_name: &str) -> Option<String> {
        let (env_var_name, flag_name) = Self::var_name_variations(var_name);

        for flag in args.iter() {
            if !flag.starts_with(&flag_name) {
                continue;
            }
            let flag_with_equal = format!("{}=", flag_name);
            let value = flag.replace(&flag_with_equal, "");
            if !value.is_empty() {
                return Some(value);
            }
        }

        env::var(env_var_name).ok()
    }

    fn var_name_variations(var_name: &str) -> (String, String) {
        let env_var_name = var_name.to_uppercase().replace("-", "_");
        let flag_name = format!("--{}", var_name.to_lowercase().replace("_", "-"));
        (env_var_name, flag_name)
    }
}