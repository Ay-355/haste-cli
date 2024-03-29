use clap::*;
use std::path::Path;

pub fn build_app() -> App<'static, 'static> {
    let mut app = App::new("haste")
        .version(crate_version!())
        .about(crate_description!())
        .after_help(
            "If you invoke the subcommand without an argument or pipe, it will open interactively. \
            Type text there and press Ctrl-d on unix or Ctrl-z on Windows to exit and paste the content. \
            \n\nReport issues at https://github.com/Ay-355/haste-cli"
        )
        .setting(AppSettings::DisableHelpSubcommand) 
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
        .arg(
            Arg::with_name("raw")
                .help("Get the raw content link instead")
                .takes_value(false)
                .multiple(false)
                .short("r")
                .long("raw")
        )

        if Path::new("view").exists() {
            app 
        } else {
        .subcommand(
            SubCommand::with_name("view")
                .about("Get the contents of a paste.")
        ) }
}
