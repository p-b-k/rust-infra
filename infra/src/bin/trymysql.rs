////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Test MySql pool
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use log::info;
use mysql::Pool;
use mysql::prelude::Queryable;

#[derive(Debug, PartialEq, Eq)]
struct Account {
    pkey: u64,
    acct_id: String,
    acct_name: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Product {
    pkey: u64,
    prod_id: String,
    prod_name: String,
}

// #[derive(Debug, PartialEq, Eq)]
// struct Service {
//     pkey: u64,
//     svc_id: String,
//     svc_name: String,
// }

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let url = "mysql://cplane_app:secret@localhost:3306/cplane";
    info!("Creating connection pool to {url}");
    let pool = Pool::new(url).unwrap();
    info!("Created connection pool");
    // let recs = sqlx::query!("SELECT * FROM account").fetch_all(pool).await;

    let query = "SELECT pkey, acct_id, acct_name FROM account";

    let mut conn = pool.get_conn().unwrap();

    let accounts = conn
        .query_map(query, |(pkey, acct_id, acct_name)| Account {
            pkey,
            acct_id,
            acct_name,
        })
        .unwrap();

    info!("Haven't failed after getting the accounts!");

    for account in accounts {
        info!("Account {} => {}", account.pkey, account.acct_id);
    }

    let query = "SELECT pkey, prod_id, prod_name FROM product";

    let products = conn
        .query_map(query, |(pkey, prod_id, prod_name)| Product {
            pkey,
            prod_id,
            prod_name,
        })
        .unwrap();

    info!("Haven't failed after getting the projects!");

    for product in products {
        info!("Product {} => {}", product.pkey, product.prod_id);
    }
}
