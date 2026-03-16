-- =====================================================================================================================
-- Create a Control Plane schema
-- =====================================================================================================================

--
-- Customer Information
-- 

CREATE TABLE customer
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, cust_id VARCHAR (64) UNIQUE NOT NULL
, cust_name VARCHAR (256) UNIQUE NOT NULL
);

--
-- Basic Product and Service Definitions
-- 

CREATE TABLE product
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, prod_id VARCHAR(32) UNIQUE NOT NULL
, prod_name VARCHAR(256) UNIQUE NOT NULL
);

CREATE TABLE product_ver
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, fkey_prod INTEGER NOT NULL
, maj_ver INTEGER NOT NULL
, min_ver INTEGER NOT NULL
, rel_ver INTEGER
, bld_ver INTEGER
, bld_tag VARCHAR(128)
);

CREATE TABLE service
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, is_global VARCHAR(1) NOT NULL DEFAULT 'Y'
, svc_id VARCHAR (32) UNIQUE NOT NULL
, svc_name VARCHAR (128) UNIQUE NOT NULL
);

CREATE TABLE service_ver
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, fkey_svc INTEGER NOT NULL
, maj_ver INTEGER NOT NULL
, min_ver INTEGER NOT NULL
, rel_ver INTEGER
, bld_ver INTEGER
, bld_tag VARCHAR(128)
, schema_def LONGTEXT
);

CREATE TABLE product_service 
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, fkey_prod_ver INTEGER NOT NULL
, fkey_svc_ver INTEGER NOT NULL
);

--
-- Tenancies
-- 

CREATE TABLE tenant
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, fkey_cust INTEGER NOT NULL
);

CREATE TABLE product_tenant
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, fkey_tnet INTEGER NOT NULL
, fkey_prod_ver INTEGER NOT NULL
);

--
-- Request Processing
-- 

CREATE TABLE request
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, req_type VARCHAR(64)
, req_start VARCHAR(64)
, req_status VARCHAR(64)
);

CREATE TABLE task
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, fkey_req INTEGER NOT NULL
);

CREATE TABLE worker
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, name VARCHAR(32) UNIQUE NOT NULL
, host VARCHAR(128) NOT NULL
, port INTEGER NOT NULL
, status INTEGER
-- , last_check TIMESTAMP
);
