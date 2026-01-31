-- =====================================================================================================================
-- Load Basic SQL Data for testing and dev
-- =====================================================================================================================

-- A Couple of accounts

INSERT INTO account (pkey, acct_id, acct_name)
VALUES (0, 'ACT1', 'First Account');

INSERT INTO account (pkey, acct_id, acct_name)
VALUES (1, 'ACT2', 'Second Account');

-- Some Products

INSERT INTO product (pkey, prod_id, prod_name)
VALUES (0, 'PIXTORE', 'Photo Storage');

INSERT INTO product (pkey, prod_id, prod_name)
VALUES (1, 'UTUNE', 'Music Library and Storage');

INSERT INTO product (pkey, prod_id, prod_name)
VALUES (2, 'WEBED', 'Web Editor');

-- Some Product Versions

-- PIXTORE

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (0, (SELECT pkey FROM product WHERE prod_id = 'PIXTORE'), 0, 1);

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (1, (SELECT pkey FROM product WHERE prod_id = 'PIXTORE'), 0, 2);

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver, bld_tag)
VALUES (2, (SELECT pkey FROM product WHERE prod_id = 'PIXTORE'), 1, 0, 'beta');

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (3, (SELECT pkey FROM product WHERE prod_id = 'PIXTORE'), 1, 0);

-- UTUNE

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (4, (SELECT pkey FROM product WHERE prod_id = 'UTUNE'), 0, 1);

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver, bld_tag)
VALUES (5, (SELECT pkey FROM product WHERE prod_id = 'UTUNE'), 1, 0, 'beta');

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (6, (SELECT pkey FROM product WHERE prod_id = 'UTUNE'), 1, 0);

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (7, (SELECT pkey FROM product WHERE prod_id = 'UTUNE'), 1, 1);

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (8, (SELECT pkey FROM product WHERE prod_id = 'UTUNE'), 1, 2);

-- WEBED

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (9, (SELECT pkey FROM product WHERE prod_id = 'WEBED'), 0, 1);

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver, bld_tag)
VALUES (10, (SELECT pkey FROM product WHERE prod_id = 'WEBED'), 1, 0, 'beta');

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (11, (SELECT pkey FROM product WHERE prod_id = 'WEBED'), 1, 0);

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (12, (SELECT pkey FROM product WHERE prod_id = 'WEBED'), 1, 1);

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver, bld_tag)
VALUES (13, (SELECT pkey FROM product WHERE prod_id = 'WEBED'), 2, 0, 'beta');

INSERT INTO product_ver (pkey, fkey_prod, maj_ver, min_ver)
VALUES (14, (SELECT pkey FROM product WHERE prod_id = 'WEBED'), 2, 0);

-- Some Services

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (0, 'AUTH', 'Authorization Service');

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (1, 'OSTORE', 'Data/Object Storage');

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (2, 'REPT', 'Reporting Service');

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (3, 'WEBUI', 'Web UI Service');

INSERT INTO service (pkey, svc_id, svc_name)
VALUES (4, 'SCHED', 'CPM Scheduling Service');

-- Some Services Version

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (0, (SELECT pkey FROM service WHERE svc_id = 'AUTH'), 1, 0);

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (1, (SELECT pkey FROM service WHERE svc_id = 'OSTORE'), 1, 0);

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (2, (SELECT pkey FROM service WHERE svc_id = 'REPT'), 1, 0);

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (3, (SELECT pkey FROM service WHERE svc_id = 'WEBUI'), 1, 0);

INSERT INTO service_ver (pkey, fkey_svc, maj_ver, min_ver)
VALUES (4, (SELECT pkey FROM service WHERE svc_id = 'SCHED'), 1, 0);

-- Bind some services to some versions

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 0, 14 /* WEBDEV 2.0 */,  0 /* AUTH 1.0 */);

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 1, 14 /* WEBDEV 2.0 */,  3 /* WEBUI 1.0 */);

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 2, 8 /* UTUNE 1.2 */,  0 /* AUTH 1.0 */);

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 3, 8 /* UTUNE 1.2 */,  3 /* WEBUI 1.0 */);

INSERT INTO product_service (pkey, fkey_prod_ver, fkey_svc_ver)
VALUES ( 4, 8 /* UTUNE 1.2 */,  1 /* OSTORE 1.0 */);


