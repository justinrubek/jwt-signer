{self, ...}: {
  perSystem = {
    self',
    pkgs,
    lib,
    system,
    inputs',
    ...
  }: {
    packages = {
      "jwt-signer/docker" = pkgs.dockerTools.buildImage {
        name = "lockpad";
        tag = self.rev or "dirty";

        copyToRoot = pkgs.buildEnv {
          name = "image-root";
          paths = [
            self'.packages.cli
          ];
          pathsToLink = ["/bin" "/migrations"];
        };

        config = {
          Cmd = ["/bin/lockpad-cli"];
          WorkingDir = "/app";
        };
      };
    };
  };
}
