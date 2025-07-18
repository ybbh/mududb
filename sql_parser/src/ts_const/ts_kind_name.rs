//
// When change grammar.js, rerun ``cargo build`` to generate this file
// Caution, do not change this file manually!!!
//

// kind name of Node

pub const S_KEYWORD_OPTION : &str = "keyword_option";
pub const S_KEYWORD_ORC : &str = "keyword_orc";
pub const S_KEYWORD_LATERAL : &str = "keyword_lateral";
pub const S_WINDOW_FRAME : &str = "window_frame";
pub const S_KEYWORD_SUPPORT : &str = "keyword_support";
pub const S_FUNCTION_LANGUAGE : &str = "function_language";
pub const S_KEYWORD_RANGE : &str = "keyword_range";
pub const S_INTERVAL_DEFINITIONS : &str = "interval_definitions";
pub const S_ORDERED_COLUMN : &str = "ordered_column";
pub const S_KEYWORD_MATERIALIZED : &str = "keyword_materialized";
pub const S__EXCLUDE_GROUP : &str = "_exclude_group";
pub const S_KEYWORD_UNBOUNDED : &str = "keyword_unbounded";
pub const S_KEYWORD_BTREE : &str = "keyword_btree";
pub const S_KEYWORD_CONFLICT : &str = "keyword_conflict";
pub const S__DROP_BEHAVIOR : &str = "_drop_behavior";
pub const S_KEYWORD_NOTHING : &str = "keyword_nothing";
pub const S__DEFAULT_EXPRESSION : &str = "_default_expression";
pub const S_DOLLAR_QUOTE : &str = "dollar_quote";
pub const S_KEYWORD_CACHED : &str = "keyword_cached";
pub const S__COLUMN_COMMENT : &str = "_column_comment";
pub const S_FUNCTION_BODY : &str = "function_body";
pub const S_KEYWORD_UNCACHED : &str = "keyword_uncached";
pub const S_KEYWORD_COST : &str = "keyword_cost";
pub const S_IMPLICIT_CAST : &str = "implicit_cast";
pub const S_KEYWORD_DO : &str = "keyword_do";
pub const S_KEYWORD_HAVING : &str = "keyword_having";
pub const S_KEYWORD_SEQUENCEFILE : &str = "keyword_sequencefile";
pub const S_EXISTS : &str = "exists";
pub const S_KEYWORD_TRIGGER : &str = "keyword_trigger";
pub const S_TABLE_PARTITION : &str = "table_partition";
pub const S_KEYWORD_UNION : &str = "keyword_union";
pub const S_VALUES : &str = "values";
pub const S_WINDOW_CLAUSE : &str = "window_clause";
pub const S_KEYWORD_BRIN : &str = "keyword_brin";
pub const S__IF_NOT_EXISTS : &str = "_if_not_exists";
pub const S_KEYWORD_SAFE : &str = "keyword_safe";
pub const S_CREATE_SEQUENCE : &str = "create_sequence";
pub const S_KEYWORD_INPUT : &str = "keyword_input";
pub const S__COMPUTE_STATS : &str = "_compute_stats";
pub const S__IF_EXISTS : &str = "_if_exists";
pub const S_KEYWORD_PLPGSQL : &str = "keyword_plpgsql";
pub const S__TRUNCATE_STATEMENT : &str = "_truncate_statement";
pub const S_KEYWORD_SORT : &str = "keyword_sort";
pub const S__CHECK_OPTION : &str = "_check_option";
pub const S_FUNCTION_DECLARATION : &str = "function_declaration";
pub const S__DOUBLE_QUOTE_STRING : &str = "_double_quote_string";
pub const S_KEYWORD_IMMUTABLE : &str = "keyword_immutable";
pub const S_KEYWORD_CHECK : &str = "keyword_check";
pub const S_CTE : &str = "cte";
pub const S_KEYWORD_INNER : &str = "keyword_inner";
pub const S_STORED_AS : &str = "stored_as";
pub const S_PARTITION_BY : &str = "partition_by";
pub const S_WINDOW_FUNCTION : &str = "window_function";
pub const S_KEYWORD_STRICT : &str = "keyword_strict";
pub const S_INTERVAL : &str = "interval";
pub const S_KEYWORD_PRECEDING : &str = "keyword_preceding";
pub const S_CREATE_ROLE : &str = "create_role";
pub const S_KEYWORD_JOIN : &str = "keyword_join";
pub const S__FUNCTION_RETURN : &str = "_function_return";
pub const S_KEYWORD_OVERWRITE : &str = "keyword_overwrite";
pub const S__TEMPORARY : &str = "_temporary";
pub const S_KEYWORD_SETOF : &str = "keyword_setof";
pub const S_TABLE_SORT : &str = "table_sort";
pub const S_KEYWORD_AVRO : &str = "keyword_avro";
pub const S__OR_REPLACE : &str = "_or_replace";
pub const S_KEYWORD_FUNCTION : &str = "keyword_function";
pub const S_KEYWORD_VOLATILE : &str = "keyword_volatile";
pub const S_KEYWORD_ATOMIC : &str = "keyword_atomic";
pub const S_FUNCTION_LEAKPROOF : &str = "function_leakproof";
pub const S_KEYWORD_LEFT : &str = "keyword_left";
pub const S__RENAME_STATEMENT : &str = "_rename_statement";
pub const S__SINGLE_QUOTE_STRING : &str = "_single_quote_string";
pub const S__IDENTIFIER : &str = "_identifier";
pub const S_KEYWORD_AVG : &str = "keyword_avg";
pub const S__KEY_CONSTRAINT : &str = "_key_constraint";
pub const S__CTE : &str = "_cte";
pub const S_KEYWORD_RECURSIVE : &str = "keyword_recursive";
pub const S__WITH_SETTINGS : &str = "_with_settings";
pub const S_KEYWORD_USE : &str = "keyword_use";
pub const S_KEYWORD_FOLLOWING : &str = "keyword_following";
pub const S_KEYWORD_CASCADED : &str = "keyword_cascaded";
pub const S_KEYWORD_LEAKPROOF : &str = "keyword_leakproof";
pub const S_KEYWORD_REPLACE : &str = "keyword_replace";
pub const S_FUNCTION_SUPPORT : &str = "function_support";
pub const S__VACUUM_TABLE : &str = "_vacuum_table";
pub const S__FUNCTION_BODY_STATEMENT : &str = "_function_body_statement";
pub const S_CREATE_QUERY : &str = "create_query";
pub const S_KEYWORD_GROUPS : &str = "keyword_groups";
pub const S_LATERAL_JOIN : &str = "lateral_join";
pub const S_KEYWORD_MAX : &str = "keyword_max";
pub const S_KEYWORD_END : &str = "keyword_end";
pub const S__ROLE_OPTIONS : &str = "_role_options";
pub const S__TABLE_SETTINGS : &str = "_table_settings";
pub const S__RENAME_TABLE_NAMES : &str = "_rename_table_names";
pub const S__ALTER_SPECIFICATIONS : &str = "_alter_specifications";
pub const S__NOT_NULL : &str = "_not_null";
pub const S_KEYWORD_PRESERVE : &str = "keyword_preserve";
pub const S__VACUUM_OPTION : &str = "_vacuum_option";
pub const S__ARRAY_SIZE_DEFINITION : &str = "_array_size_definition";
pub const S_CREATE_TYPE : &str = "create_type";
pub const S_KEYWORD_ROW : &str = "keyword_row";
pub const S_KEYWORD_ADMIN : &str = "keyword_admin";
pub const S_KEYWORD_OPTIONS : &str = "keyword_options";
pub const S__INNER_DEFAULT_EXPRESSION : &str = "_inner_default_expression";
pub const S_KEYWORD_IGNORE : &str = "keyword_ignore";
pub const S_KEYWORD_CSV : &str = "keyword_csv";
pub const S_DELETE : &str = "delete";
pub const S__DEFAULT_NULL : &str = "_default_null";
pub const S_FUNCTION_STRICTNESS : &str = "function_strictness";
pub const S_KEYWORD_LOW_PRIORITY : &str = "keyword_low_priority";
pub const S_KEYWORD_RCFILE : &str = "keyword_rcfile";
pub const S_KEYWORD_TERMINATED : &str = "keyword_terminated";
pub const S_SET_OPERATION : &str = "set_operation";
pub const S__OPTIMIZE_TABLE : &str = "_optimize_table";
pub const S_ENUM_ELEMENTS : &str = "enum_elements";
pub const S_RETURNING : &str = "returning";
pub const S_KEYWORD_EXCEPT : &str = "keyword_except";
pub const S_KEYWORD_CASE : &str = "keyword_case";
pub const S__PRIMARY_KEY : &str = "_primary_key";
pub const S_KEYWORD_TIES : &str = "keyword_ties";
pub const S_KEYWORD_TBLPROPERTIES : &str = "keyword_tblproperties";
pub const S_INDEX_HINT : &str = "index_hint";
pub const S__PARTITION_SPEC : &str = "_partition_spec";
pub const S__ALTER_STATEMENT : &str = "_alter_statement";
pub const S_KEYWORD_OTHERS : &str = "keyword_others";
pub const S_KEYWORD_WINDOW : &str = "keyword_window";
pub const S_KEYWORD_GIST : &str = "keyword_gist";
pub const S_KEYWORD_ROWS : &str = "keyword_rows";
pub const S_CASE : &str = "case";
pub const S_KEYWORD_SPGIST : &str = "keyword_spgist";
pub const S_KEYWORD_RETURNING : &str = "keyword_returning";
pub const S_KEYWORD_LANGUAGE : &str = "keyword_language";
pub const S__KEY_VALUE_PAIR : &str = "_key_value_pair";
pub const S__EXCLUDE_NO_OTHERS : &str = "_exclude_no_others";
pub const S_CROSS_JOIN : &str = "cross_join";
pub const S_KEYWORD_OUTER : &str = "keyword_outer";
pub const S_KEYWORD_PARTITIONED : &str = "keyword_partitioned";
pub const S_STORAGE_LOCATION : &str = "storage_location";
pub const S_KEYWORD_RETURNS : &str = "keyword_returns";
pub const S_CREATE_VIEW : &str = "create_view";
pub const S_KEYWORD_DECLARE : &str = "keyword_declare";
pub const S_KEYWORD_SQL : &str = "keyword_sql";
pub const S_KEYWORD_CROSS : &str = "keyword_cross";
pub const S_CREATE_MATERIALIZED_VIEW : &str = "create_materialized_view";
pub const S_KEYWORD_DELAYED : &str = "keyword_delayed";
pub const S_CREATE_FUNCTION : &str = "create_function";
pub const S_KEYWORD_LOCATION : &str = "keyword_location";
pub const S_ROW_FORMAT : &str = "row_format";
pub const S__CURRENT_ROW : &str = "_current_row";
pub const S_CREATE_INDEX : &str = "create_index";
pub const S_WINDOW_SPECIFICATION : &str = "window_specification";
pub const S_GROUP_BY : &str = "group_by";
pub const S_KEYWORD_ESCAPED : &str = "keyword_escaped";
pub const S_KEYWORD_RIGHT : &str = "keyword_right";
pub const S_KEYWORD_RESTRICTED : &str = "keyword_restricted";
pub const S_FRAME_DEFINITION : &str = "frame_definition";
pub const S_KEYWORD_CALLED : &str = "keyword_called";
pub const S__CONSTRAINT_LITERAL : &str = "_constraint_literal";
pub const S_KEYWORD_HIGH_PRIORITY : &str = "keyword_high_priority";
pub const S_KEYWORD_UNSAFE : &str = "keyword_unsafe";
pub const S__OPTIMIZE_STATEMENT : &str = "_optimize_statement";
pub const S_FUNCTION_COST : &str = "function_cost";
pub const S_KEYWORD_RETURN : &str = "keyword_return";
pub const S_KEYWORD_STABLE : &str = "keyword_stable";
pub const S__INTERVAL_DEFINITION : &str = "_interval_definition";
pub const S__CREATE_STATEMENT : &str = "_create_statement";
pub const S_KEYWORD_STORED : &str = "keyword_stored";
pub const S_CREATE_SCHEMA : &str = "create_schema";
pub const S__MERGE_STATEMENT : &str = "_merge_statement";
pub const S__USER_ACCESS_ROLE_CONFIG : &str = "_user_access_role_config";
pub const S__EXCLUDE_CURRENT_ROW : &str = "_exclude_current_row";
pub const S_FUNCTION_VOLATILITY : &str = "function_volatility";
pub const S_KEYWORD_EXCLUDE : &str = "keyword_exclude";
pub const S__MYSQL_UPDATE_STATEMENT : &str = "_mysql_update_statement";
pub const S_KEYWORD_HASH : &str = "keyword_hash";
pub const S_KEYWORD_JSONFILE : &str = "keyword_jsonfile";
pub const S_KEYWORD_REPLICATION : &str = "keyword_replication";
pub const S_KEYWORD_FIELDS : &str = "keyword_fields";
pub const S__POSTGRES_UPDATE_STATEMENT : &str = "_postgres_update_statement";
pub const S_KEYWORD_PARQUET : &str = "keyword_parquet";
pub const S_KEYWORD_LINES : &str = "keyword_lines";
pub const S_KEYWORD_MIN : &str = "keyword_min";
pub const S_KEYWORD_AUTHORIZATION : &str = "keyword_authorization";
pub const S_KEYWORD_ELSE : &str = "keyword_else";
pub const S__EXCLUDE_TIES : &str = "_exclude_ties";
pub const S_KEYWORD_FORMAT : &str = "keyword_format";
pub const S_FUNCTION_ROWS : &str = "function_rows";
pub const S_KEYWORD_OVER : &str = "keyword_over";
pub const S_KEYWORD_TEXTFILE : &str = "keyword_textfile";
pub const S_KEYWORD_INTERSECT : &str = "keyword_intersect";
pub const S_LATERAL_CROSS_JOIN : &str = "lateral_cross_join";
pub const S_JOIN : &str = "join";
pub const S_KEYWORD_GIN : &str = "keyword_gin";
pub const S_CREATE_DATABASE : &str = "create_database";
pub const S__HAVING : &str = "_having";
pub const S_KEYWORD_DELIMITED : &str = "keyword_delimited";
pub const S_FUNCTION_SAFETY : &str = "function_safety";
pub const S_KEYWORD_SELECT : &str = "keyword_select";
pub const S_KEYWORD_DELETE : &str = "keyword_delete";
pub const S_KEYWORD_INSERT : &str = "keyword_insert";
pub const S_KEYWORD_COPY : &str = "keyword_copy";
pub const S_KEYWORD_UPDATE : &str = "keyword_update";
pub const S_KEYWORD_TRUNCATE : &str = "keyword_truncate";
pub const S_KEYWORD_MERGE : &str = "keyword_merge";
pub const S_KEYWORD_INTO : &str = "keyword_into";
pub const S_KEYWORD_VALUES : &str = "keyword_values";
pub const S_KEYWORD_VALUE : &str = "keyword_value";
pub const S_KEYWORD_MATCHED : &str = "keyword_matched";
pub const S_KEYWORD_SET : &str = "keyword_set";
pub const S_KEYWORD_FROM : &str = "keyword_from";
pub const S_KEYWORD_FULL : &str = "keyword_full";
pub const S_KEYWORD_ON : &str = "keyword_on";
pub const S_KEYWORD_WHERE : &str = "keyword_where";
pub const S_KEYWORD_ORDER : &str = "keyword_order";
pub const S_KEYWORD_GROUP : &str = "keyword_group";
pub const S_KEYWORD_PARTITION : &str = "keyword_partition";
pub const S_KEYWORD_BY : &str = "keyword_by";
pub const S_KEYWORD_DESC : &str = "keyword_desc";
pub const S_KEYWORD_ASC : &str = "keyword_asc";
pub const S_KEYWORD_LIMIT : &str = "keyword_limit";
pub const S_KEYWORD_OFFSET : &str = "keyword_offset";
pub const S_KEYWORD_PRIMARY : &str = "keyword_primary";
pub const S_KEYWORD_CREATE : &str = "keyword_create";
pub const S_KEYWORD_ALTER : &str = "keyword_alter";
pub const S_KEYWORD_CHANGE : &str = "keyword_change";
pub const S_KEYWORD_ANALYZE : &str = "keyword_analyze";
pub const S_KEYWORD_EXPLAIN : &str = "keyword_explain";
pub const S_KEYWORD_VERBOSE : &str = "keyword_verbose";
pub const S_KEYWORD_MODIFY : &str = "keyword_modify";
pub const S_KEYWORD_DROP : &str = "keyword_drop";
pub const S_KEYWORD_ADD : &str = "keyword_add";
pub const S_KEYWORD_TABLE : &str = "keyword_table";
pub const S_KEYWORD_TABLES : &str = "keyword_tables";
pub const S_KEYWORD_VIEW : &str = "keyword_view";
pub const S_KEYWORD_COLUMN : &str = "keyword_column";
pub const S_KEYWORD_COLUMNS : &str = "keyword_columns";
pub const S_KEYWORD_TABLESPACE : &str = "keyword_tablespace";
pub const S_KEYWORD_SEQUENCE : &str = "keyword_sequence";
pub const S_KEYWORD_INCREMENT : &str = "keyword_increment";
pub const S_KEYWORD_MINVALUE : &str = "keyword_minvalue";
pub const S_KEYWORD_MAXVALUE : &str = "keyword_maxvalue";
pub const S_KEYWORD_NONE : &str = "keyword_none";
pub const S_KEYWORD_OWNED : &str = "keyword_owned";
pub const S_KEYWORD_START : &str = "keyword_start";
pub const S_KEYWORD_RESTART : &str = "keyword_restart";
pub const S_KEYWORD_KEY : &str = "keyword_key";
pub const S_KEYWORD_AS : &str = "keyword_as";
pub const S_KEYWORD_DISTINCT : &str = "keyword_distinct";
pub const S_KEYWORD_CONSTRAINT : &str = "keyword_constraint";
pub const S_KEYWORD_FILTER : &str = "keyword_filter";
pub const S_KEYWORD_CAST : &str = "keyword_cast";
pub const S_KEYWORD_SEPARATOR : &str = "keyword_separator";
pub const S_KEYWORD_WHEN : &str = "keyword_when";
pub const S_KEYWORD_THEN : &str = "keyword_then";
pub const S_KEYWORD_IN : &str = "keyword_in";
pub const S_KEYWORD_AND : &str = "keyword_and";
pub const S_KEYWORD_OR : &str = "keyword_or";
pub const S_KEYWORD_IS : &str = "keyword_is";
pub const S_KEYWORD_NOT : &str = "keyword_not";
pub const S_KEYWORD_FORCE : &str = "keyword_force";
pub const S_KEYWORD_USING : &str = "keyword_using";
pub const S_KEYWORD_INDEX : &str = "keyword_index";
pub const S_KEYWORD_FOR : &str = "keyword_for";
pub const S_KEYWORD_IF : &str = "keyword_if";
pub const S_KEYWORD_EXISTS : &str = "keyword_exists";
pub const S_KEYWORD_AUTO_INCREMENT : &str = "keyword_auto_increment";
pub const S_KEYWORD_GENERATED : &str = "keyword_generated";
pub const S_KEYWORD_ALWAYS : &str = "keyword_always";
pub const S_KEYWORD_COLLATE : &str = "keyword_collate";
pub const S_KEYWORD_ENGINE : &str = "keyword_engine";
pub const S_KEYWORD_DEFAULT : &str = "keyword_default";
pub const S_KEYWORD_CASCADE : &str = "keyword_cascade";
pub const S_KEYWORD_RESTRICT : &str = "keyword_restrict";
pub const S_KEYWORD_NO : &str = "keyword_no";
pub const S_KEYWORD_DATA : &str = "keyword_data";
pub const S_KEYWORD_TYPE : &str = "keyword_type";
pub const S_KEYWORD_RENAME : &str = "keyword_rename";
pub const S_KEYWORD_TO : &str = "keyword_to";
pub const S_KEYWORD_DATABASE : &str = "keyword_database";
pub const S_KEYWORD_SCHEMA : &str = "keyword_schema";
pub const S_KEYWORD_OWNER : &str = "keyword_owner";
pub const S_KEYWORD_USER : &str = "keyword_user";
pub const S_KEYWORD_PASSWORD : &str = "keyword_password";
pub const S_KEYWORD_ENCRYPTED : &str = "keyword_encrypted";
pub const S_KEYWORD_VALID : &str = "keyword_valid";
pub const S_KEYWORD_UNTIL : &str = "keyword_until";
pub const S_KEYWORD_CONNECTION : &str = "keyword_connection";
pub const S_KEYWORD_ROLE : &str = "keyword_role";
pub const S_KEYWORD_RESET : &str = "keyword_reset";
pub const S_KEYWORD_TEMP : &str = "keyword_temp";
pub const S_KEYWORD_TEMPORARY : &str = "keyword_temporary";
pub const S_KEYWORD_UNLOGGED : &str = "keyword_unlogged";
pub const S_KEYWORD_LOGGED : &str = "keyword_logged";
pub const S_KEYWORD_CYCLE : &str = "keyword_cycle";
pub const S_KEYWORD_ALL : &str = "keyword_all";
pub const S_KEYWORD_ANY : &str = "keyword_any";
pub const S_KEYWORD_SOME : &str = "keyword_some";
pub const S_KEYWORD_BEGIN : &str = "keyword_begin";
pub const S_KEYWORD_COMMIT : &str = "keyword_commit";
pub const S_KEYWORD_ROLLBACK : &str = "keyword_rollback";
pub const S_KEYWORD_TRANSACTION : &str = "keyword_transaction";
pub const S_KEYWORD_NULLS : &str = "keyword_nulls";
pub const S_KEYWORD_FIRST : &str = "keyword_first";
pub const S_KEYWORD_AFTER : &str = "keyword_after";
pub const S_KEYWORD_BEFORE : &str = "keyword_before";
pub const S_KEYWORD_LAST : &str = "keyword_last";
pub const S_KEYWORD_BETWEEN : &str = "keyword_between";
pub const S_KEYWORD_CURRENT : &str = "keyword_current";
pub const S_KEYWORD_ONLY : &str = "keyword_only";
pub const S_KEYWORD_UNIQUE : &str = "keyword_unique";
pub const S_KEYWORD_FOREIGN : &str = "keyword_foreign";
pub const S_KEYWORD_REFERENCES : &str = "keyword_references";
pub const S_KEYWORD_CONCURRENTLY : &str = "keyword_concurrently";
pub const S_KEYWORD_SIMILAR : &str = "keyword_similar";
pub const S_KEYWORD_UNSIGNED : &str = "keyword_unsigned";
pub const S_KEYWORD_ZEROFILL : &str = "keyword_zerofill";
pub const S_KEYWORD_LOCAL : &str = "keyword_local";
pub const S_KEYWORD_CURRENT_TIMESTAMP : &str = "keyword_current_timestamp";
pub const S_KEYWORD_VACUUM : &str = "keyword_vacuum";
pub const S_KEYWORD_WAIT : &str = "keyword_wait";
pub const S_KEYWORD_NOWAIT : &str = "keyword_nowait";
pub const S_KEYWORD_ATTRIBUTE : &str = "keyword_attribute";
pub const S_KEYWORD_PARALLEL : &str = "keyword_parallel";
pub const S_KEYWORD_EXTERNAL : &str = "keyword_external";
pub const S_KEYWORD_COMPUTE : &str = "keyword_compute";
pub const S_KEYWORD_STATS : &str = "keyword_stats";
pub const S_KEYWORD_STATISTICS : &str = "keyword_statistics";
pub const S_KEYWORD_OPTIMIZE : &str = "keyword_optimize";
pub const S_KEYWORD_REWRITE : &str = "keyword_rewrite";
pub const S_KEYWORD_BIN_PACK : &str = "keyword_bin_pack";
pub const S_KEYWORD_INCREMENTAL : &str = "keyword_incremental";
pub const S_KEYWORD_COMMENT : &str = "keyword_comment";
pub const S_KEYWORD_CACHE : &str = "keyword_cache";
pub const S_KEYWORD_METADATA : &str = "keyword_metadata";
pub const S_KEYWORD_NOSCAN : &str = "keyword_noscan";
pub const S_KEYWORD_NULL : &str = "keyword_null";
pub const S_KEYWORD_TRUE : &str = "keyword_true";
pub const S_KEYWORD_FALSE : &str = "keyword_false";
pub const S_KEYWORD_BOOLEAN : &str = "keyword_boolean";
pub const S_KEYWORD_BIT : &str = "keyword_bit";
pub const S_KEYWORD_BINARY : &str = "keyword_binary";
pub const S_KEYWORD_VARBINARY : &str = "keyword_varbinary";
pub const S_KEYWORD_IMAGE : &str = "keyword_image";
pub const S_KEYWORD_DECIMAL : &str = "keyword_decimal";
pub const S_KEYWORD_NUMERIC : &str = "keyword_numeric";
pub const S_KEYWORD_FLOAT : &str = "keyword_float";
pub const S_KEYWORD_DOUBLE : &str = "keyword_double";
pub const S_KEYWORD_PRECISION : &str = "keyword_precision";
pub const S_KEYWORD_INET : &str = "keyword_inet";
pub const S_KEYWORD_MONEY : &str = "keyword_money";
pub const S_KEYWORD_SMALLMONEY : &str = "keyword_smallmoney";
pub const S_KEYWORD_VARYING : &str = "keyword_varying";
pub const S_KEYWORD_NCHAR : &str = "keyword_nchar";
pub const S_KEYWORD_NVARCHAR : &str = "keyword_nvarchar";
pub const S_KEYWORD_TEXT : &str = "keyword_text";
pub const S_KEYWORD_STRING : &str = "keyword_string";
pub const S_KEYWORD_UUID : &str = "keyword_uuid";
pub const S_KEYWORD_JSON : &str = "keyword_json";
pub const S_KEYWORD_JSONB : &str = "keyword_jsonb";
pub const S_KEYWORD_XML : &str = "keyword_xml";
pub const S_KEYWORD_BYTEA : &str = "keyword_bytea";
pub const S_KEYWORD_ENUM : &str = "keyword_enum";
pub const S_KEYWORD_DATE : &str = "keyword_date";
pub const S_KEYWORD_DATETIME : &str = "keyword_datetime";
pub const S_KEYWORD_DATETIME2 : &str = "keyword_datetime2";
pub const S_KEYWORD_SMALLDATETIME : &str = "keyword_smalldatetime";
pub const S_KEYWORD_DATETIMEOFFSET : &str = "keyword_datetimeoffset";
pub const S_KEYWORD_INTERVAL : &str = "keyword_interval";
pub const S_KEYWORD_GEOMETRY : &str = "keyword_geometry";
pub const S_KEYWORD_GEOGRAPHY : &str = "keyword_geography";
pub const S_KEYWORD_BOX2D : &str = "keyword_box2d";
pub const S_KEYWORD_BOX3D : &str = "keyword_box3d";
pub const S_KEYWORD_OID : &str = "keyword_oid";
pub const S_KEYWORD_NAME : &str = "keyword_name";
pub const S_KEYWORD_REGCLASS : &str = "keyword_regclass";
pub const S_KEYWORD_REGNAMESPACE : &str = "keyword_regnamespace";
pub const S_KEYWORD_REGPROC : &str = "keyword_regproc";
pub const S_KEYWORD_REGTYPE : &str = "keyword_regtype";
pub const S_KEYWORD_ARRAY : &str = "keyword_array";
pub const S_NATURAL_NUMBER : &str = "natural_number";
pub const S_BANG : &str = "bang";
pub const S_PROGRAM : &str = "program";
pub const S_KEYWORD_CHARACTER : &str = "keyword_character";
pub const S_KEYWORD_WITH : &str = "keyword_with";
pub const S_KEYWORD_LIKE : &str = "keyword_like";
pub const S_IS_NOT : &str = "is_not";
pub const S_NOT_LIKE : &str = "not_like";
pub const S_SIMILAR_TO : &str = "similar_to";
pub const S_NOT_SIMILAR_TO : &str = "not_similar_to";
pub const S_DISTINCT_FROM : &str = "distinct_from";
pub const S_NOT_DISTINCT_FROM : &str = "not_distinct_from";
pub const S_DIRECTION : &str = "direction";
pub const S_KEYWORD_SMALLSERIAL : &str = "keyword_smallserial";
pub const S_KEYWORD_SERIAL : &str = "keyword_serial";
pub const S_KEYWORD_BIGSERIAL : &str = "keyword_bigserial";
pub const S_KEYWORD_TINYINT : &str = "keyword_tinyint";
pub const S_KEYWORD_SMALLINT : &str = "keyword_smallint";
pub const S_KEYWORD_MEDIUMINT : &str = "keyword_mediumint";
pub const S_KEYWORD_INT : &str = "keyword_int";
pub const S_KEYWORD_BIGINT : &str = "keyword_bigint";
pub const S_KEYWORD_REAL : &str = "keyword_real";
pub const S_KEYWORD_CHAR : &str = "keyword_char";
pub const S_KEYWORD_VARCHAR : &str = "keyword_varchar";
pub const S_KEYWORD_TIME : &str = "keyword_time";
pub const S_KEYWORD_TIMESTAMP : &str = "keyword_timestamp";
pub const S_KEYWORD_TIMESTAMPTZ : &str = "keyword_timestamptz";
pub const S_DATA_TYPE : &str = "data_type";
pub const S_DATA_TYPE_KIND : &str = "data_type_kind";
pub const S_ARRAY_SIZE_DEFINITION : &str = "array_size_definition";
pub const S_TINYINT : &str = "tinyint";
pub const S_SMALLINT : &str = "smallint";
pub const S_MEDIUMINT : &str = "mediumint";
pub const S_INT : &str = "int";
pub const S_BIGINT : &str = "bigint";
pub const S_BIT : &str = "bit";
pub const S_BINARY : &str = "binary";
pub const S_VARBINARY : &str = "varbinary";
pub const S_FLOAT : &str = "float";
pub const S_DOUBLE : &str = "double";
pub const S_DECIMAL : &str = "decimal";
pub const S_NUMERIC : &str = "numeric";
pub const S_CHAR : &str = "char";
pub const S_VARCHAR : &str = "varchar";
pub const S_NCHAR : &str = "nchar";
pub const S_NVARCHAR : &str = "nvarchar";
pub const S_DATETIMEOFFSET : &str = "datetimeoffset";
pub const S_TIME : &str = "time";
pub const S_ENUM : &str = "enum";
pub const S_ARRAY : &str = "array";
pub const S_COMMENT : &str = "comment";
pub const S_MARGINALIA : &str = "marginalia";
pub const S_STATEMENT_TRANSACTION : &str = "statement_transaction";
pub const S_BEGIN_TRANSACTION : &str = "begin_transaction";
pub const S_COMMIT_TRANSACTION : &str = "commit_transaction";
pub const S_ROLLBACK_TRANSACTION : &str = "rollback_transaction";
pub const S_STATEMENT : &str = "statement";
pub const S_COPY_STMT : &str = "copy_stmt";
pub const S_COPY_FROM : &str = "copy_from";
pub const S_COPY_TO : &str = "copy_to";
pub const S_FILE_PATH : &str = "file_path";
pub const S_DDL_STMT : &str = "ddl_stmt";
pub const S_DML_WRITE_STMT : &str = "dml_write_stmt";
pub const S_DML_READ_STMT : &str = "dml_read_stmt";
pub const S_SELECT_STATEMENT : &str = "select_statement";
pub const S_SELECT : &str = "select";
pub const S_SELECT_EXPRESSION : &str = "select_expression";
pub const S_TERM : &str = "term";
pub const S_DELETE_STATEMENT : &str = "delete_statement";
pub const S_CREATE_TABLE_STATEMENT : &str = "create_table_statement";
pub const S_ALTER_TABLE : &str = "alter_table";
pub const S_ADD_COLUMN : &str = "add_column";
pub const S_ADD_CONSTRAINT : &str = "add_constraint";
pub const S_ALTER_COLUMN : &str = "alter_column";
pub const S_MODIFY_COLUMN : &str = "modify_column";
pub const S_CHANGE_COLUMN : &str = "change_column";
pub const S_COLUMN_POSITION : &str = "column_position";
pub const S_DROP_COLUMN : &str = "drop_column";
pub const S_RENAME_COLUMN : &str = "rename_column";
pub const S_ALTER_VIEW : &str = "alter_view";
pub const S_ALTER_SCHEMA : &str = "alter_schema";
pub const S_ALTER_DATABASE : &str = "alter_database";
pub const S_ALTER_ROLE : &str = "alter_role";
pub const S_SET_CONFIGURATION : &str = "set_configuration";
pub const S_ALTER_INDEX : &str = "alter_index";
pub const S_ALTER_SEQUENCE : &str = "alter_sequence";
pub const S_ALTER_TYPE : &str = "alter_type";
pub const S_DROP_STATEMENT : &str = "drop_statement";
pub const S_DROP_TABLE : &str = "drop_table";
pub const S_DROP_VIEW : &str = "drop_view";
pub const S_DROP_SCHEMA : &str = "drop_schema";
pub const S_DROP_DATABASE : &str = "drop_database";
pub const S_DROP_ROLE : &str = "drop_role";
pub const S_DROP_TYPE : &str = "drop_type";
pub const S_DROP_SEQUENCE : &str = "drop_sequence";
pub const S_DROP_INDEX : &str = "drop_index";
pub const S_RENAME_OBJECT : &str = "rename_object";
pub const S_SET_SCHEMA : &str = "set_schema";
pub const S_CHANGE_OWNERSHIP : &str = "change_ownership";
pub const S_OBJECT_REFERENCE : &str = "object_reference";
pub const S_INSERT_STATEMENT : &str = "insert_statement";
pub const S_INSERT_VALUES : &str = "insert_values";
pub const S_TYPED_ROW_VALUE_EXPR_LIST : &str = "typed_row_value_expr_list";
pub const S_SET_VALUES : &str = "set_values";
pub const S_COLUMN_LIST : &str = "column_list";
pub const S_COLUMN : &str = "column";
pub const S_UPDATE_STATEMENT : &str = "update_statement";
pub const S_WHEN_CLAUSE : &str = "when_clause";
pub const S_ASSIGNMENT : &str = "assignment";
pub const S_TABLE_OPTION : &str = "table_option";
pub const S_COLUMN_DEFINITIONS : &str = "column_definitions";
pub const S_COLUMN_DEFINITION : &str = "column_definition";
pub const S_COLUMN_CONSTRAINT : &str = "column_constraint";
pub const S_CONSTRAINTS : &str = "constraints";
pub const S_CONSTRAINT : &str = "constraint";
pub const S_PRIMARY_KEY_CONSTRAINT : &str = "primary_key_constraint";
pub const S_ORDERED_COLUMNS : &str = "ordered_columns";
pub const S_ALL_FIELDS : &str = "all_fields";
pub const S_PARAMETER : &str = "parameter";
pub const S_FIELD : &str = "field";
pub const S_QUALIFIED_FIELD : &str = "qualified_field";
pub const S_CAST : &str = "cast";
pub const S_FILTER_EXPRESSION : &str = "filter_expression";
pub const S_INVOCATION : &str = "invocation";
pub const S_ALIAS_NAME : &str = "alias_name";
pub const S_FROM : &str = "from";
pub const S_RELATION : &str = "relation";
pub const S_WHERE : &str = "where";
pub const S_ORDER_BY : &str = "order_by";
pub const S_ORDER_TARGET : &str = "order_target";
pub const S_LIMIT : &str = "limit";
pub const S_OFFSET : &str = "offset";
pub const S_EXPRESSION : &str = "expression";
pub const S_BINARY_EXPRESSION : &str = "binary_expression";
pub const S_UNARY_EXPRESSION : &str = "unary_expression";
pub const S_BETWEEN_EXPRESSION : &str = "between_expression";
pub const S_NOT_IN : &str = "not_in";
pub const S_SUBQUERY : &str = "subquery";
pub const S_LIST : &str = "list";
pub const S_LITERAL : &str = "literal";
pub const S_LITERAL_STRING : &str = "literal_string";
pub const S_INTEGER : &str = "integer";
pub const S_DECIMAL_NUMBER : &str = "decimal_number";
pub const S_IDENTIFIER : &str = "identifier";
