extern crate cc;

fn main() {
    if cfg!(not(target_env = "msvc")) {
        std::env::set_var("CXXFLAGS", "-std=c++14");
    }

    cc::Build::new()
        .cpp(true)
        .file("anitomy-c/anitomy_c.cpp")
        .file("anitomy-c/anitomy/anitomy/anitomy.cpp")
        .file("anitomy-c/anitomy/anitomy/element.cpp")
        .file("anitomy-c/anitomy/anitomy/keyword.cpp")
        .file("anitomy-c/anitomy/anitomy/parser.cpp")
        .file("anitomy-c/anitomy/anitomy/parser_helper.cpp")
        .file("anitomy-c/anitomy/anitomy/parser_number.cpp")
        .file("anitomy-c/anitomy/anitomy/string.cpp")
        .file("anitomy-c/anitomy/anitomy/token.cpp")
        .file("anitomy-c/anitomy/anitomy/tokenizer.cpp")
        .compile("anitomy_c");
}
