use clap::{App, SubCommand, AppSettings, Arg};

pub fn setup_command_line() -> App<'static, 'static> {
    let name = Arg::with_name("name")
                     .short("n")
                     .long("name")
                     .value_name("NAME")
                     .default_value("default")
                     .help("Server name");
    App::new("pqueue")
        .version("0.1")
        .author("samuel.lauren@iki.fi")
        .about("Queue commands for execution")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("server")
                .about("Start a job queue server")
                .setting(AppSettings::TrailingVarArg)
                .arg(name.clone())
                .arg(Arg::with_name("retries")
                     .short("r")
                     .long("retries")
                     .takes_value(true)
                     .value_name("COUNT")
                     .default_value("0")
                     .help("Retry count"))
                .arg(Arg::with_name("dir")
                     .short("c")
                     .long("cd")
                     .value_name("DIR")
                     .help("Execute COMMAND in DIR"))
                .arg(Arg::with_name("log")
                     .short("l")
                     .long("log")
                     .takes_value(true)
                     .value_name("PATH")
                     .help("Log to PATH instead of stdout"))
                .arg(Arg::with_name("daemon")
                     .short("d")
                     .long("daemon")
                     .requires("dir")
                     .help("Run the server as a daemon"))
                .arg(Arg::with_name("command")
                     .required(true)
                     .value_name("COMMAND")
                     .help("Command to execute"))
                .arg(Arg::with_name("template")
                     .multiple(true)
                     .value_name("TEMPLATE")
                     .help("Argument template")))
        .subcommand(
            SubCommand::with_name("send")
                .about("Queue a new job")
                .setting(AppSettings::TrailingVarArg)
                .arg(name.clone())
                .arg(Arg::with_name("args")
                     .multiple(true)
                     .help("Arguments to be combined with the template")))
        .subcommand(
            SubCommand::with_name("stop")
                .about("Stop a server")
                .arg(name.clone()))
        .subcommand(
            SubCommand::with_name("has")
                .about("Check if a server exists")
                .arg(name))
}

