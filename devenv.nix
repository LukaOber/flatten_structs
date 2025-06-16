{ ... }:

{
  languages.rust.enable = true;

  git-hooks.hooks = {
    nixfmt-rfc-style.enable = true;
    taplo.enable = true;
    rustfmt.enable = true;
    clippy = {
      enable = true;
      settings = {
        allFeatures = true;
        offline = false;
        denyWarnings = true;
        extraArgs = "--all-targets";
      };
    };
  };
}
