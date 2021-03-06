use clap::{Command, Arg};

/// Build the CLI instance
pub fn build_cli() -> Command<'static> {
    Command::new("apt-gen-list-rs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("AOSC-Dev")
        .about(
            "Utility for generating APT sources.list from available repository configurations."
        )
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set-branch")
                .about("Set APT repository branch (e.g., stable)")
                .arg(
                    Arg::new("BRANCH")
                        .help("Input branch name here")
                        .max_values(1)
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("set-mirror")
                .about("Set APT repository mirror")
                .arg(
                    Arg::new("MIRROR")
                        .help("source.list mirror")
                        .max_values(1)
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("add-mirror")
                .about("Add additional APT repository mirror")
                .arg(
                    Arg::new("MIRROR")
                        .help("source.list mirror")
                        .min_values(1)
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("remove-mirror")
                .about("Remove APT repository mirror")
                .arg(
                    Arg::new("MIRROR")
                        .help("remove source.list mirror")
                        .min_values(1)
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("status")
                .about("Show apt-gen-list status")
        )
        .subcommand(
            Command::new("add-component")
                .about("Set APT repository component")
                .arg(
                    Arg::new("COMPONENT")
                        .help("Input component name")
                        .min_values(1)
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("remove-component")
                .about("Remove APT repository component")
                .arg(
                    Arg::new("COMPONENT")
                        .help("Input component name to be removed")
                        .min_values(1)
                        .required(true)
                        .takes_value(true),
                )
        )
        .subcommand(
            Command::new("add-custom-mirror")
                .about("Add custom repository mirror")
                .arg(
                    Arg::new("MIRROR_NAME")
                        .help("custom repository mirror name")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("MIRROR_URL")
                    .help("custom repository mirror url")
                    .required(true)
                    .takes_value(true),
                )
                .arg(
                    Arg::new("also-set-mirror")
                    .help("also set mirror as default")
                    .long("also-set-mirror")
                    .short('s')
                    .requires("MIRROR_NAME")
                    .requires("MIRROR_URL")
                )
                .arg(
                    Arg::new("also-add-mirror")
                    .help("also add mirror to list")
                    .long("also-add-mirror")
                    .short('a')
                    .requires("MIRROR_NAME")
                    .requires("MIRROR_URL")
                    .conflicts_with("also-set-mirror")
                )
        )
        .subcommand(
            Command::new("remove-custom-mirror")
                .about("Remove custom repository mirror")
                .arg(
                    Arg::new("MIRROR")
                        .help("Input custom repository mirror name to remove from the list of custom mirrors")
                        .min_values(1)
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("speedtest")
                .about("Run speed-test on available mirrors")
                .arg(
                    Arg::new("parallel")
                    .help("Test mirror performance concurrently, test will take a shorter amount of time, but results will only serve as a rough estimate and could vary between runs")
                    .long("parallel")
                    .short('p')
                )
        )
        .subcommand(
            Command::new("list-mirrors")
                .about("Show available mirror list")
        )
        .subcommand(
            Command::new("set-fastest-mirror-as-default")
                .about("Set fastest mirror as default")
        )
        .subcommands({
            if cfg!(feature = "aosc") {
                vec![Command::new("reset-mirror").about("Reset mirror to default")]
            } else {
                vec![]
            }
        })
}
