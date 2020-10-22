{ stdenv, lib, rustPlatform, sqlite }:

rustPlatform.buildRustPackage rec {
  pname = "taskpilot";
  version = "0.0.0";

  src = ./.;

  cargoSha256 = "0p4dw092bv5d8kgwknhb7hfxqrl0pcyx38laa27s7xmd6niy14if";

  propagatedBuildInputs = [ sqlite ];

  meta = with lib; {
    description = "Database tool for working with Rust projects that use Diesel";
    homepage = "https://github.com/diesel-rs/diesel/tree/master/diesel_cli";
    license = with licenses; [ mit asl20 ];
    maintainers = with maintainers; [ ];
  };
}