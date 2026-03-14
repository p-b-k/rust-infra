-- =====================================================================================================================
-- Load Basic SQL Data for testing and dev
-- =====================================================================================================================

-- A Couple of accounts

INSERT INTO customer (pkey, cust_id, cust_name)
VALUES (0, 'SCARPERCO', 'Scarperco Premium Software Services');

INSERT INTO customer (pkey, cust_id, cust_name)
VALUES (1, 'WBULL', 'The Weeping Bull, Public House');

INSERT INTO customer (pkey, cust_id, cust_name)
VALUES (2, 'BFBG', 'Big Frankie''s Bar and Grill');

INSERT INTO customer (pkey, cust_id, cust_name)
VALUES (3, 'MRICH', 'Le Maison Richeliu');

-- Some Products

INSERT INTO product (pkey, prod_id, prod_name)
VALUES (0, 'CPLANE', 'Control Plane');

INSERT INTO product (pkey, prod_id, prod_name)
VALUES (1, 'AION', 'Schedule Management Application');

-- Some Product Versions

-- CPLANE

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (0, (SELECT pkey FROM product WHERE prod_id = 'CPLANE'), 0, 0);

-- AION

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (4, (SELECT pkey FROM product WHERE prod_id = 'AION'), 0, 1);

-- Some Services

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (0, 'AUTH', 'Authorization Service');

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (1, 'SHIFTS', 'Shift Management Service (the main AION service)');

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (2, 'CP', 'Controll Plane');

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (3, 'CPUI', 'Control Plane UI');

-- Some Services Version

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (0, (SELECT pkey FROM service WHERE svc_id = 'AUTH'), 1, 0);

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (1, (SELECT pkey FROM service WHERE svc_id = 'SHIFTS'), 0, 0);

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (2, (SELECT pkey FROM service WHERE svc_id = 'CPUI'), 0, 0);

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (3, (SELECT pkey FROM service WHERE svc_id = 'CP'), 0, 0);

-- 
-- Bind some services to some versions
-- 

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 0, 0 /* CPLANE 0.0 */,  0 /* AUTH 1.0 */);

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 2, 4 /* AION 0.1 */,  0 /* AUTH 1.0 */);

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 3, 4 /* AION 0.1 */,  1 /* SHIFTS 0.0 */);

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 4, 0 /* CPLANE 0.0 */,  1 /* CPUI 0.0 */);

