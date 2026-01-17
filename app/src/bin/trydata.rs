////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Generic Datasource trait, and StdDS basic implementation
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use app::data::{product, product_ver};

use mysql::Pool;

use log::info;

const PROD_PKEY: u64 = 1;

fn main() {
    env_logger::init();

    println!("Hello World!");

    let url = "mysql://rusty_app:secret@localhost:3306/rusty";

    info!("Creating connection pool to {url:?}");
    let pool = Pool::new(url).unwrap();

    info!("Getting connection from pool");
    let mut conn = pool.get_conn().unwrap();

    info!("Got connection, about to try and get data source");

    let prod_ds = product();
    let prod_ver_ds = product_ver();

    info!("Created product datasource, about to try and retrieve product");

    {
        info!("Getting single product result");
        let res = prod_ds.get(&mut conn, PROD_PKEY);

        info!("Got result");

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
        info!("Getting joined results");
        let fk_field = String::from("fkey_prod");
        let res = prod_ver_ds.join(&mut conn, PROD_PKEY, &fk_field);

        info!("Got result");

        match res {
            Ok(product_vers) => {
                for product_ver in product_vers {
                    let pkey = product_ver.pkey;
                    let fkey_prod = product_ver.fkey_prod;
                    println!("Got product version {pkey} -> {fkey_prod}")
                }
            }
            Err(msg) => println!("FAILED! {msg}"),
        }
    }
}
