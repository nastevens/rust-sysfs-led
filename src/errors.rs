error_chain! {
    errors {
        InvalidDevicePath(path: String) {
            description("invalid device path")
            display("invalid device path: '{}'", path)
        }
        UnsupportedTrigger(trigger: String) {
            description("trigger unsupported (kernel driver missing?)")
            display("trigger unsupported: '{}'", trigger)
        }
    }

    foreign_links {
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}
