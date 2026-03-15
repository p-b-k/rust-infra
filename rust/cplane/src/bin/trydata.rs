////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic Datasource trait, and StdDS basic implementation
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use mysql::Pool;

use cplane::schema::{
    CUSTOMER_FACTORY, PRODUCT_FACTORY, PRODUCT_VER_FACTORY, SERVICE_FACTORY, SERVICE_VER_FACTORY,
};

use log::{debug, info};

fn main() {
    env_logger::init();

    let url = "mysql://cplane_app:secret@localhost:3306/cplane";

    info!("Creating connection pool to {url:?}");
    let pool = Pool::new(url).unwrap();

    info!("Getting connection from pool");
    let mut conn = pool.get_conn().unwrap();

    info!("Got connection, about to try and get data source");

    info!("Created product datasource, about to try and retrieve product");

    let get_cust_0 = false;
    let get_prod_0 = false;
    let get_prod_ver_0 = false;
    let get_svc_0 = false;
    let get_svc_ver_0 = false;
    let get_prod_ver_join = true;
    let get_prod_all = true;

    if get_cust_0 {
        debug!("Getting single customer result");
        // let res = ds.customer.get(&mut conn, 0);
        let res = CUSTOMER_FACTORY.fetch(&mut conn, 0);

        debug!("Got result");

        match res {
            Ok(d_o) => {
                println!("Got customer {:?})", d_o.obj)
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    if get_prod_0 {
        debug!("Getting single product result");
        let res = PRODUCT_FACTORY.fetch(&mut conn, 0);

        debug!("Got result");

        match res {
            Ok(product) => {
                let id = product.obj.prod_id;
                let name = product.obj.prod_name;
                println!("Got product {id} ({name})")
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    if get_prod_ver_0 {
        debug!("Getting single product version result");
        let res = PRODUCT_VER_FACTORY.fetch(&mut conn, 4);

        debug!("Got result");

        match res {
            Ok(product) => {
                println!("Got product {:?})", product.obj)
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    if get_svc_0 {
        debug!("Getting single service result");
        let res = SERVICE_FACTORY.fetch(&mut conn, 0);

        debug!("Got result");

        match res {
            Ok(svc) => {
                println!("Got service {:?})", svc.obj)
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    if get_svc_ver_0 {
        debug!("Getting single service version result");
        let res = SERVICE_VER_FACTORY.fetch(&mut conn, 0);

        debug!("Got result");

        match res {
            Ok(sv) => {
                println!("Got service version {:?})", sv.obj)
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    if get_prod_ver_join {
        info!("Getting joined results");
        let fk_field = String::from("fkey_prod");
        let res = PRODUCT_VER_FACTORY.join(&mut conn, 1, &fk_field);

        debug!("Got result");

        match res {
            Ok(product_vers) => {
                for product_ver in product_vers {
                    let pkey = product_ver.pkey.unwrap();
                    let fkey_prod = product_ver.obj.fkey_prod;
                    println!("Got product version {pkey} -> {fkey_prod}")
                }
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    if get_prod_all {
        info!("Getting all products");
        let res = PRODUCT_FACTORY.all(&mut conn).unwrap();

        debug!("Got result");

        for prod in res {
            println!("Got product {:?}/{:?}", prod.pkey, prod.obj);
        }
    }
}
