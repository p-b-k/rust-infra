////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// sample data
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use infra::version::Version;
use mysql::PooledConn;

use crate::tabs::{
    customer::{CUSTOMER_FACTORY, Customer},
    product::{PRODUCT_FACTORY, Product},
    product_service::{PRODUCT_SERVICE_FACTORY, ProductService},
    product_ver::{PRODUCT_VER_FACTORY, ProductVer},
    service::{SERVICE_FACTORY, Service},
    service_ver::{SERVICE_VER_FACTORY, ServiceVer},
};

pub fn load_sample_data(conn: &mut PooledConn) {
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
        fkey_prod: p_cplane.pkey.unwrap(),
        prod_ver: Version::from_string("0.0.0.1").unwrap(),
    });
    pv_cplane_0_0.sync(conn);

    let mut pv_aion_0_1 = PRODUCT_VER_FACTORY.new(ProductVer {
        fkey_prod: p_aion.pkey.unwrap(),
        prod_ver: Version::from_string("0.1").unwrap(),
    });
    println!("before sync, pv_aion_0_1.pkey = {:?}", pv_aion_0_1.pkey);
    pv_aion_0_1.sync(conn);
    println!("after sync, pv_aion_0_1.pkey = {:?}", pv_aion_0_1.pkey);

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
    let mut sv_auth_0_1 = SERVICE_VER_FACTORY.new(ServiceVer {
        fkey_svc: s_auth.pkey.unwrap(),
        svc_ver: Version::from_string("0.1").unwrap(),
    });
    sv_auth_0_1.sync(conn);

    let mut sv_aiondb_0_1 = SERVICE_VER_FACTORY.new(ServiceVer {
        fkey_svc: s_aiondb.pkey.unwrap(),
        svc_ver: Version::from_string("0.1").unwrap(),
    });
    sv_aiondb_0_1.sync(conn);

    let mut sv_aionbl_0_1 = SERVICE_VER_FACTORY.new(ServiceVer {
        fkey_svc: s_aionbl.pkey.unwrap(),
        svc_ver: Version::from_string("0.1").unwrap(),
    });
    sv_aionbl_0_1.sync(conn);

    let mut sv_aionui_0_1 = SERVICE_VER_FACTORY.new(ServiceVer {
        fkey_svc: s_aionui.pkey.unwrap(),
        svc_ver: Version::from_string("0.1").unwrap(),
    });
    sv_aionui_0_1.sync(conn);

    // Product Services
    println!("pv_aion_0_1.pkey = {:?}", pv_aion_0_1.pkey);
    let mut ps_aion_auth = PRODUCT_SERVICE_FACTORY.new(ProductService {
        fkey_prod_ver: pv_aion_0_1.pkey.unwrap(),
        fkey_svc_ver: sv_auth_0_1.pkey.unwrap(),
    });
    ps_aion_auth.sync(conn);

    let mut ps_aion_aionbl = PRODUCT_SERVICE_FACTORY.new(ProductService {
        fkey_prod_ver: pv_aion_0_1.pkey.unwrap(),
        fkey_svc_ver: sv_aionbl_0_1.pkey.unwrap(),
    });
    ps_aion_aionbl.sync(conn);

    let mut ps_aion_aionui = PRODUCT_SERVICE_FACTORY.new(ProductService {
        fkey_prod_ver: pv_aion_0_1.pkey.unwrap(),
        fkey_svc_ver: sv_aionui_0_1.pkey.unwrap(),
    });
    ps_aion_aionui.sync(conn);

    let mut ps_aion_aiondb = PRODUCT_SERVICE_FACTORY.new(ProductService {
        fkey_prod_ver: pv_aion_0_1.pkey.unwrap(),
        fkey_svc_ver: sv_aiondb_0_1.pkey.unwrap(),
    });
    ps_aion_aiondb.sync(conn);
}
