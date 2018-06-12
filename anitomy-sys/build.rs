extern crate cc;

fn main() {
    let mut cc_build = cc::Build::new();
    cc_build.cpp(true);
    let compiler = cc_build.get_compiler();

    if !compiler.is_like_msvc() {
        cc_build.flag("-std=c++14");
        
        if compiler.is_like_clang() && cfg!(target_os = "macos") {
            cc_build.cpp_set_stdlib(Some("c++"));
            cc_build.flag("-mmacosx-version-min=10.7");
        }
    }

    cc_build
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
