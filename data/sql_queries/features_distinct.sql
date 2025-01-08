-- # Account Features List

-- NOTE: SQLX - these files are meant to be a single statement
--
-- ## Convenience Variables
-- SET @VAR_subdomain = 'pdt-ethan' COLLATE utf8mb4_unicode_ci;

-- ## Logging Variables
-- SET @VAR_inferred_region = IF(LOCATE('eu', @@hostname) > 0, 'EU', 'US');
-- SET @VAR_hostname = @@hostname;
-- SET @VAR_server_uuid = @@server_uuid;
-- SET @VAR_database = DATABASE();
-- SET @VAR_utc_request = UTC_TIMESTAMP();

-- ## Query Body
-- DESCRIBE
SELECT
    DISTINCT af.feature_name,
    COUNT(af.feature_name) AS feature_count,
    MIN(af.created_at) AS first_occurrence,
    MAX(af.created_at) AS last_occurrence,
    @VAR_inferred_region AS log_of_inferred_region,
    @VAR_hostname AS log_of_hostname,
    @VAR_server_uuid AS log_of_server_uuid,
    @VAR_database AS log_of_database,
    @VAR_utc_request AS log_of_query_start_time_utc
FROM account_features AS af
--     JOIN accounts a ON account_id = a.id
-- WHERE a.subdomain = @VAR_subdomain
GROUP BY feature_name
