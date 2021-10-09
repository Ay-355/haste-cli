use clap::*;

pub fn build_app() -> App<'static, 'static> {
    App::new("haste")
        .version(crate_version!())
        .about(crate_description!())
        .after_help(
            "If you invoke the subcommand without an argument or pipe, it will open interactively. \
            Type text there and press Ctrl-d on *nix or Ctrl-z on Windows to exit and paste the content. \
            \n\nReport issues at https://github.com/Ay-355/haste-cli"
        )
        .arg(
            Arg::with_name("FILE")
                .help("The file you want to paste. Use '-' or nothing for standard input.")
                .multiple(false)
                .empty_values(false)
        )
        .arg(
            Arg::with_name("language")
                .help("The syntax language of the file. Mainly for syntax highlighting, default is none.")
                .takes_value(true)
                .multiple(false)
                .empty_values(false)
                .short("l")
                .long("language")
        )
}
