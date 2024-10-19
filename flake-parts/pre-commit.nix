{inputs, ...}: {
  imports = [inputs.pre-commit-hooks.flakeModule ./formatting.nix];

  perSystem = {
    pkgs,
    self',
    ...
  }: {
    pre-commit = {
      check.enable = false;

      settings = {
        src = ../.;
        hooks = {
          alejandra.enable = true;
          rustfmt.enable = true;
        };
      };
    };
  };
}
