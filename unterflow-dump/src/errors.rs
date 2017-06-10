error_chain! {
    foreign_links {
        Cli(::clap::Error);
        Logging(::log::SetLoggerError);
        Io(::std::io::Error);
        Protocol(::unterflow_protocol::errors::Error);
    }
}
