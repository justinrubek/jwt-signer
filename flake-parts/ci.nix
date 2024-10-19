{
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    ciPackages = [
      inputs'.bomper.packages.bomper
    ];
in {
    devShells.ci = pkgs.mkShell rec {
      packages = ciPackages;

      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
    };

    legacyPackages = {
      inherit ciPackages;
    };
  };
}
