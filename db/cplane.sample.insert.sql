-- =====================================================================================================================
-- Load Basic SQL Data for testing and dev
-- =====================================================================================================================

-- A Couple of accounts

INSERT INTO customer (cust_id, cust_name)
VALUES ('SCARPERCO', 'Scarperco Premium Software Services');

INSERT INTO customer (cust_id, cust_name)
VALUES ('WBULL', 'The Weeping Bull, Public House');

INSERT INTO customer (cust_id, cust_name)
VALUES ('BFBG', 'Big Frankie''s Bar and Grill');

INSERT INTO customer (cust_id, cust_name)
VALUES ('MRICH', 'Le Maison Richeliu');

-- Some Products

INSERT INTO product (prod_id, prod_name)
VALUES ('CPLANE', 'Control Plane');

INSERT INTO product (prod_id, prod_name)
VALUES ('AION', 'Schedule Management Application');

-- Some Product Versions

-- CPLANE

INSERT INTO product_ver (fkey_prod, maj_ver, min_ver)
VALUES ((SELECT pkey FROM product WHERE prod_id = 'CPLANE'), 0, 0);

-- AION

INSERT INTO product_ver (fkey_prod, maj_ver, min_ver)
VALUES ((SELECT pkey FROM product WHERE prod_id = 'AION'), 0, 1);

-- Some Services

INSERT INTO service (svc_id, svc_name)
VALUES ('AUTH', 'Authorization Service');

INSERT INTO service (svc_id, svc_name)
VALUES ('SHIFTS', 'Shift Management Service (the main AION service)');

INSERT INTO service (svc_id, svc_name)
VALUES ('CP', 'Controll Plane');

INSERT INTO service (svc_id, svc_name)
VALUES ('CPUI', 'Control Plane UI');

-- Some Services Version

INSERT INTO service_ver (fkey_svc, maj_ver, min_ver)
VALUES ((SELECT pkey FROM service WHERE svc_id = 'AUTH'), 1, 0);

INSERT INTO service_ver (fkey_svc, maj_ver, min_ver)
VALUES ((SELECT pkey FROM service WHERE svc_id = 'SHIFTS'), 0, 0);

INSERT INTO service_ver (fkey_svc, maj_ver, min_ver)
VALUES ((SELECT pkey FROM service WHERE svc_id = 'CPUI'), 0, 0);

INSERT INTO service_ver (fkey_svc, maj_ver, min_ver)
VALUES ((SELECT pkey FROM service WHERE svc_id = 'CP'), 0, 0);

-- 
-- Bind some services to some versions
-- 

INSERT INTO product_service (fkey_prod_ver, fkey_svc_ver)
VALUES ( 1 /* CPLANE 0.0 */,  1 /* AUTH 1.0 */);

INSERT INTO product_service (fkey_prod_ver, fkey_svc_ver)
VALUES ( 5 /* AION 0.1 */,  1 /* AUTH 1.0 */);

INSERT INTO product_service (fkey_prod_ver, fkey_svc_ver)
VALUES ( 5 /* AION 0.1 */,  2 /* SHIFTS 0.0 */);

INSERT INTO product_service (fkey_prod_ver, fkey_svc_ver)
VALUES ( 1 /* CPLANE 0.0 */,  2 /* CPUI 0.0 */);

