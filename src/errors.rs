error_chain! {
    errors {
        InvalidDevicePath(s: String) {
            description("invalid device path")
            display("invalid device path: '{}'", s)
        }
    }

    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}
