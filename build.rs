/// 在 编译期间运行来进行额外的处理
fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(["."]);
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
