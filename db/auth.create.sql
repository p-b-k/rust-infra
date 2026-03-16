-- =====================================================================================================================
-- Create an Authorization Schema
-- =====================================================================================================================

CREATE TABLE user
( pkey INTEGER PRIMARY KEY AUTO_INCREMENT
, user_id VARCHAR (64)
, user_name VARCHAR (256)
, password VARCHAR (256)
);

