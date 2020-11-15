{
  description = "Taskpilot";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/007126eef72271480cb7670e19e501a1ad2c1ff2";

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.taskpilot = nixpkgs.legacyPackages.x86_64-linux.callPackage ./default.nix {};
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.taskpilot;
  };
}
