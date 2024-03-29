use vergen::{vergen, Config, ShaKind};

fn main() {
  prost_build::compile_protos(&["src/IntifaceGui.proto"], &["src/"]).unwrap();
  let mut config = Config::default();
  // Change the SHA output to the short variant
  *config.git_mut().sha_kind_mut() = ShaKind::Short;
  // Generate the default 'cargo:' instruction output
  vergen(config).unwrap();
}
