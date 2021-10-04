use clap::*;

pub fn build_app() -> App<'static, 'static> {
    App::new("haste")
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("FILE")
                .help("The file you want to paste")
                .required(true)
                .multiple(false)
                .empty_values(false)
        )
}
