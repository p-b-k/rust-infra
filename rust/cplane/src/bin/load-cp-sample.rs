////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// load-cp-sample -- load the sample data into an empty database
// User requires INSERT, SELECT, DELETE, UPDATE rights on the tables
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use cplane::app::DbConfig;
use cplane::schema::{
    CUSTOMER_FACTORY, Customer, PRODUCT_FACTORY, Product, SERVICE_FACTORY, Service,
    PRODUCT_SERVICE_FACTORY, ProductService, PRODUCT_VER_FACTORY, ProductVer, SERVICE_VER_FACTORY, ServiceVer
};

use mysql::{ Pool, PooledConn };

use std::env;

use log::debug;

struct AppConfig {
    pub db: DbConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            db: DbConfig {
                name: String::from("cp"),
                user: String::from("cp_app"),
                pass: String::from("secret"),
                host: String::from("localhost"),
                port: 3306,
            },
        }
    }

    pub fn as_url(&self) -> String {
        let host = self.db.host.clone();
        let name = self.db.name.clone();
        let port = self.db.port;
        let user = self.db.user.clone();
        let pass = self.db.pass.clone();

        format!("mysql://{user}:{pass}@{host}:{port}/{name}")
    }
}

fn main() {
    env_logger::init();

    println!("Initializing ...");
    let mut cfg = AppConfig::new();

    println!("Processing Parameters ...");
    process_parameters(&mut cfg);

    let url = cfg.as_url();
    println!("Database root url = {url:?}");

    println!("About to get user pool ...");
    let pool = Pool::new(url.as_str()).unwrap();

    println!("About to get user connection ...");
    let mut conn = pool.get_conn().unwrap();

    load_sample_data(&mut conn);
}

fn load_sample_data(conn: &mut PooledConn) {
    // Customers
    let mut c_spss = CUSTOMER_FACTORY.new(Customer {
        cust_id: String::from("SPSS"),
        cust_name: String::from("Scaperco Premium Software Services"),
    });
    c_spss.sync(conn);
    c_spss.sync(conn);

    let mut c_wbull = CUSTOMER_FACTORY.new(Customer {
        cust_id: String::from("WBULL"),
        cust_name: String::from("The Weeping Bull, Public House"),
    });
    c_wbull.sync(conn);

    let mut c_bfbg = CUSTOMER_FACTORY.new(Customer {
        cust_id: String::from("BFBG"),
        cust_name: String::from("Big Frankie's Bar and Grill"),
    });
    c_bfbg.sync(conn);

    let mut c_mrich = CUSTOMER_FACTORY.new(Customer {
        cust_id: String::from("MRICH"),
        cust_name: String::from("La Maison Richeliu"),
    });
    c_mrich.sync(conn);

    // Products
    let mut p_cplane = PRODUCT_FACTORY.new(Product {
        prod_id: String::from("CPLANE"),
        prod_name: String::from("Control Plane"),
    });
    p_cplane.sync(conn);

    let mut p_aion = PRODUCT_FACTORY.new(Product {
        prod_id: String::from("AION"),
        prod_name: String::from("Schedule Management Application"),
    });
    p_aion.sync(conn);

    // Product Versions
    let mut pv_cplane_0_0 = PRODUCT_VER_FACTORY.new(ProductVer {
        fkey_prod : p_cplane.pkey.unwrap(),
        maj_ver : 0,
        min_ver : 0,
        rel_ver : None,
        bld_ver : None,
        bld_tag : None
    });
    pv_cplane_0_0.sync(conn);

    let mut pv_aion_0_1 = PRODUCT_VER_FACTORY.new(ProductVer {
        fkey_prod : p_aion.pkey.unwrap(),
        maj_ver : 0,
        min_ver : 1,
        rel_ver : None,
        bld_ver : None,
        bld_tag : None
    });
    pv_aion_0_1.sync(conn);

    // Services
    let mut s_auth = SERVICE_FACTORY.new(Service {
        svc_id: String::from("AUTH"),
        svc_name: String::from("Authorization Service"),
        is_global: String::from("Y"),
    });
    s_auth.sync(conn);

    let mut s_aionbl = SERVICE_FACTORY.new(Service {
        svc_id: String::from("AIONBL"),
        svc_name: String::from("AION Business Logic"),
        is_global: String::from("N"),
    });
    s_aionbl.sync(conn);

    let mut s_aionui = SERVICE_FACTORY.new(Service {
        svc_id: String::from("AIONUI"),
        svc_name: String::from("AION User Interface"),
        is_global: String::from("N"),
    });
    s_aionui.sync(conn);

    let mut s_aiondb = SERVICE_FACTORY.new(Service {
        svc_id: String::from("AIONDB"),
        svc_name: String::from("AION Database"),
        is_global: String::from("N"),
    });
    s_aiondb.sync(conn);

    // Service Versions
    let mut sv_auth_0_1 = SERVICE_VER_FACTORY.new(ServiceVer{
        fkey_svc : s_auth.pkey.unwrap(),
        maj_ver : 0,
        min_ver : 1,
        rel_ver : None,
        bld_ver : None,
        bld_tag : None
    });
    sv_auth_0_1.sync(conn);

    let mut sv_aiondb_0_1 = SERVICE_VER_FACTORY.new(ServiceVer{
        fkey_svc : s_aiondb.pkey.unwrap(),
        maj_ver : 0,
        min_ver : 1,
        rel_ver : None,
        bld_ver : None,
        bld_tag : None
    });
    sv_auth_0_1.sync(conn);

    let mut sv_aionbl_0_1 = SERVICE_VER_FACTORY.new(ServiceVer{
        fkey_svc : s_aionbl.pkey.unwrap(),
        maj_ver : 0,
        min_ver : 1,
        rel_ver : None,
        bld_ver : None,
        bld_tag : None
    });
    sv_auth_0_1.sync(conn);

    let mut sv_aionui_0_1 = SERVICE_VER_FACTORY.new(ServiceVer{
        fkey_svc : s_aionui.pkey.unwrap(),
        maj_ver : 0,
        min_ver : 1,
        rel_ver : None,
        bld_ver : None,
        bld_tag : None
    });
    sv_auth_0_1.sync(conn);

    // Product Services
    let mut ps_aion_auth = PRODUCT_SERVICE_FACTORY.new(ProductService{
        fkey_prod_ver : pv_aion_0_1.pkey.unwrap(),
        fkey_svc_ver : sv_auth_0_1.pkey.unwrap(),
    });
    ps_aion_auth.sync(conn);

    
}

fn process_parameters(cfg: &mut AppConfig) {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        let next = &args[i];
        // debug!("arg = {next:?}");
        if next == "--port" {
            i = i + 1;
            let port_str = &args[i];
            debug!(target: "read_parameters", "port_str = {port_str}");
            cfg.db.port = port_str.parse().unwrap();
        } else if next == "--host" {
            i = i + 1;
            cfg.db.host = args[i].clone();
            debug!(target: "read_parameters", "host = {}", cfg.db.host);
        } else if next == "--name" {
            i = i + 1;
            cfg.db.name = args[i].clone();
            debug!(target: "read_parameters", "name = {}", cfg.db.name);
        } else if next == "--user" {
            i = i + 1;
            cfg.db.user = args[i].clone();
            debug!(target: "read_parameters", "user = {}", cfg.db.user);
        } else if next == "--pass" {
            i = i + 1;
            cfg.db.pass = args[i].clone();
            debug!(target: "read_parameters", "pass = {}", cfg.db.pass);
        } else {
            panic!("Unknown paramater: {next}");
        }

        i = i + 1;
    }
}
