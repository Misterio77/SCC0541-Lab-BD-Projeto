{ config, lib, pkgs, ... }:

with lib;
let cfg = config.services.projeto-labbd;

in {
  options.services.projeto-labbd = {
    enable = mkEnableOption "projeto-labbd";
    package = mkOption {
      type = types.package;
      default = pkgs.projeto-labbd;
      defaultText = "pkgs.projeto-labbd";
      description = ''
        The package implementing projeto-labbd
      '';
    };
    database = mkOption {
      type = types.nullOr types.str;
      description = "Connection string for database.";
      default = null;
    };
    address = mkOption {
      type = types.str;
      default = "0.0.0.0";
      description = "Address to bind to.";
    };
    port = mkOption {
      type = types.int;
      default = 8080;
      description = "Port number to bind to.";
    };
    environmentFile = mkOption {
      type = types.nullOr types.path;
      description = "File path containing environment variables (secret key, for example) for the server";
      default = null;
    };
    openFirewall = mkOption {
      type = types.bool;
      default = false;
      description = "Whether to open port in the firewall for the server.";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.projeto-labbd = {
      description = "projeto-labbd";
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/projeto-labbd";
        Restart = "on-failure";
        User = "labbd";
        EnvironmentFile = "${cfg.environmentFile}";
      };
      environment = {
        ROCKET_ADDRESS = cfg.address;
        ROCKET_TEMPLATE_DIR = "${cfg.package}/etc/templates";
        ROCKET_PORT = toString cfg.port;
        ROCKET_DATABASES = ''{database={url="${cfg.database}"}}'';
      };
    };

    users = {
      users.labbd = {
        description = "projeto-labbd service user";
        isSystemUser = true;
        group = "labbd";
      };
      groups.labbd = { };
    };

    networking.firewall =
      mkIf cfg.openFirewall { allowedTCPPorts = [ cfg.port ]; };
  };
}
