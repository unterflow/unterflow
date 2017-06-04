error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Utf8Error(::std::string::FromUtf8Error);
    }
}
