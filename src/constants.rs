//lpnl
pub const LPNL_MAIN_DIR : &str              = "/etc/lpnl";
pub const LPNL_BACKUP_DIR: &str             = "/etc/lpnl/backups";
pub const LPNL_TMP_DIR: &str                = "/etc/lpnl/tmp";

// nginx
    // pub const NGINX_MAIN_DIR: &str              = "/etc/nginx";
pub const NGINX_CONFIG: &str                = "/etc/nginx/nginx.conf";
pub const NGINX_SITES_DISABLED_DIR: &str    = "/etc/nginx/sites-disabled";
pub const NGINX_SITES_ENABLED_DIR: &str     = "/etc/nginx/sites-enabled";
pub const LAST_MAJOR_VER: u32               = 1;
pub const LAST_MINOR_VER: u32               = 30;
pub const LAST_PATCH_VER: u32               = 4;

// test file dir
pub const TEST_FILE_DIR: &str               = "/etc/lpnl/tmp/run_test.txt";

// init
pub const DEFAULT_PORT: u16                 = 8080;
pub const DEFAULT_DOMAIN: &str              = "localhost";
pub const WWW_ROOT_DIR: &str                = "/var/www"; 

// other
pub const OS_RELEASE_DIR: &str              = "/etc/os-release";