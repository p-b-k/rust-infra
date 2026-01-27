-- =====================================================================================================================
-- Create a Control Plane schema
-- =====================================================================================================================

CREATE TABLE account
( pkey INTEGER PRIMARY KEY
, acct_id VARCHAR (64) UNIQUE NOT NULL
, acct_name VARCHAR (256) UNIQUE NOT NULL
);

CREATE TABLE product
( pkey INTEGER PRIMARY KEY
, prod_id VARCHAR(32) UNIQUE NOT NULL
, prod_name VARCHAR(256) UNIQUE NOT NULL
);

CREATE TABLE product_ver
( pkey INTEGER PRIMARY KEY
, fkey_prod INTEGER NOT NULL
, maj_ver INTEGER NOT NULL
, min_ver INTEGER NOT NULL
, rel_ver INTEGER
, bld_ver INTEGER
, bld_tag VARCHAR(128)
);

CREATE TABLE service
( pkey INTEGER PRIMARY KEY
, svc_id VARCHAR (32) UNIQUE NOT NULL
, svc_name VARCHAR (128) UNIQUE NOT NULL
);

CREATE TABLE service_ver
( pkey INTEGER PRIMARY KEY
, fkey_svc INTEGER NOT NULL
, maj_ver INTEGER NOT NULL
, min_ver INTEGER NOT NULL
, rel_ver INTEGER
, bld_ver INTEGER
, bld_tag VARCHAR(128)
, schema_def TEXT
);

CREATE TABLE product_service 
( pkey INTEGER PRIMARY KEY
, fkey_prod_ver INTEGER NOT NULL
, fkey_svc_ver INTEGER NOT NULL
);


CREATE TABLE request
( pkey INTEGER PRIMARY KEY
, req_type VARCHAR(64)
, req_start VARCHAR(64)
, req_status VARCHAR(64)
);

CREATE TABLE task
( pkey INTEGER PRIMARY KEY
, fkey_req INTEGER NOT NULL
);
