error_chain! {
    foreign_links {
        Cli(::clap::Error);
        Logging(::log::SetLoggerError);
        Protocol(::unterflow_protocol::errors::Error);
    }
}
