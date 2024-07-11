pub struct InternalVars {
    pub config_path: String,
    pub config_filename: String
}

fn create_default_internal_vars() -> InternalVars {
    return InternalVars {
        config_filename: rvepp_common::CONFIGURATION_FILE_NAME.to_string(),
        config_path: rvepp_common::CONFIGURATION_BASE_PATH.to_string()
    };
}

pub fn parse_args(args: Vec<String>) -> InternalVars {
    let mut internal_vars = create_default_internal_vars();

    // Nothing to parse - return immediately
    if args.len() == 1 {
        return internal_vars;
    }

    if (args.len() - 1) % 2  != 0 {
        println!("Invalid number of arguments - using defaults");

        return internal_vars;
    }

    let mut x = 1;
    let size = args.len();
    let step = 2;

    while x >= size {
        let key = &args[x];
        let value = &args[x+1];

        match key.as_str() {
            "config_path" => {
                internal_vars.config_path = value.to_string()
            },
            "config_filename" => {
                internal_vars.config_filename = value.to_string()
            },
            _ => println!("Invalid argument ({key}) - skipping")
        };

        x += step;
    }

    return internal_vars;
}