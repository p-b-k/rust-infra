////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic Datasource trait, and StdDS basic implementation
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use mysql::Pool;

use cplane::schema::build_datasource;

use log::{debug, info};

fn main() {
    env_logger::init();

    println!("Hello World!");

    let url = "mysql://cplane_app:secret@localhost:3306/cplane";

    info!("Creating connection pool to {url:?}");
    let pool = Pool::new(url).unwrap();

    info!("Getting connection from pool");
    let mut conn = pool.get_conn().unwrap();

    info!("Got connection, about to try and get data source");

    let ds = build_datasource();
    let prod_ds = ds.product;
    let prod_ver_ds = ds.product_ver;

    info!("Created product datasource, about to try and retrieve product");

    {
        debug!("Getting single account result");
        let res = ds.account.get(&mut conn, 0);

        debug!("Got result");

        match res {
            Ok(obj) => {
                println!("Got account {obj:?})")
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    {
        debug!("Getting single product result");
        let res = prod_ds.get(&mut conn, 0);

        debug!("Got result");

        match res {
            Ok(product) => {
                let id = product.prod_id;
                let name = product.prod_name;
                println!("Got product {id} ({name})")
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    {
        debug!("Getting single product version result");
        let res = prod_ver_ds.get(&mut conn, 4);

        debug!("Got result");

        match res {
            Ok(product) => {
                println!("Got product {product:?})")
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    {
        debug!("Getting single service result");
        let res = ds.service.get(&mut conn, 0);

        debug!("Got result");

        match res {
            Ok(obj) => {
                println!("Got service {obj:?})")
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    {
        debug!("Getting single service version result");
        let res = ds.service_ver.get(&mut conn, 0);

        debug!("Got result");

        match res {
            Ok(obj) => {
                println!("Got service version {obj:?})")
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    {
        info!("Getting joined results");
        let fk_field = String::from("fkey_prod");
        let res = prod_ver_ds.join(&mut conn, 1, &fk_field);

        debug!("Got result");

        match res {
            Ok(product_vers) => {
                for product_ver in product_vers {
                    let pkey = product_ver.pkey.unwrap();
                    let fkey_prod = product_ver.fkey_prod;
                    println!("Got product version {pkey} -> {fkey_prod}")
                }
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }

    {
        info!("Getting all products");
        let res = prod_ds.all(&mut conn);

        debug!("Got result");

        for prod in res {
            println!("Got product {prod:?}");
        }
    }
}
