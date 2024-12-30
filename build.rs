fn main() {
    std::env::set_var("CC_LOG", "1");

    build_emmyluacodestyle();
}

fn build_emmyluacodestyle() {
    let mut builder = cc::Build::new();
    builder.cpp(true);
    builder
        .include("3rd/EmmyLuaCodeStyle/Util/include")
        .include("3rd/EmmyLuaCodeStyle/CodeFormatCLib/include")
        .include("3rd/EmmyLuaCodeStyle/CodeFormatCore/include")
        .include("3rd/EmmyLuaCodeStyle/LuaParser/include")
        .include("3rd/EmmyLuaCodeStyle/3rd/wildcards/include")
        .include("3rd/lua");

    let file_patterns = vec![
        "3rd/EmmyLuaCodeStyle/CodeFormatCLib/src/*.cpp",
        "3rd/EmmyLuaCodeStyle/LuaParser/src/**/*.cpp",
        "3rd/EmmyLuaCodeStyle/Util/src/StringUtil.cpp",
        "3rd/EmmyLuaCodeStyle/Util/src/Utf8.cpp",
        "3rd/EmmyLuaCodeStyle/Util/src/SymSpell/*.cpp",
        "3rd/EmmyLuaCodeStyle/Util/src/InfoTree/*.cpp",
        "3rd/EmmyLuaCodeStyle/CodeFormatCore/src/**/*.cpp",
    ];

    for pattern in file_patterns {
        if pattern.contains("*") {
            builder.files(glob::glob(pattern).unwrap().filter_map(|path| path.ok()));
        } else {
            builder.file(pattern);
        }
    }

    if cfg!(windows) {
        builder.flag("/utf-8");
        builder.flag("/std:c++17");
    } else {
        builder.flag("-std=c++17");
    }

    builder.compile("EmmyLuaCodeStyle");
}