pub const KEYWORDS: [&str; 127] = [
    "ALL",
    "ALTER",
    "ANALYZE",
    "AND",
    "AS",
    "ASC",
    "AUTHORIZATION",
    "BACKUP",
    "BEGIN",
    "BETWEEN",
    "BY",
    "CASCADE",
    "CASE",
    "CAST",
    "CHECK",
    "CLUSTER",
    "COLLATE",
    "COLUMN",
    "COMMENT",
    "COMMIT",
    "CONSTRAINT",
    "CREATE",
    "CROSS",
    "CURRENT_DATE",
    "CURRENT_TIME",
    "CURRENT_TIMESTAMP",
    "CURRENT_USER",
    "DATABASE",
    "DATE",
    "DECIMAL",
    "DECLARE",
    "DEFAULT",
    "DEFERRABLE",
    "DESC",
    "DISTINCT",
    "DO",
    "DROP",
    "ELSE",
    "END",
    "EXCEPT",
    "EXISTS",
    "EXPLAIN",
    "EXTERNAL",
    "EXTRACT",
    "FALSE",
    "FETCH",
    "FOR",
    "FOREIGN",
    "FROM",
    "FULL",
    "FUNCTION",
    "GRANT",
    "GROUP",
    "HAVING",
    "IDENTITY",
    "IN",
    "INDEX",
    "INNER",
    "INSERT",
    "INTERSECT",
    "INTERVAL",
    "INTO",
    "IS",
    "JOIN",
    "KEY",
    "LATERAL",
    "LEFT",
    "LIKE",
    "LIMIT",
    "LOCALTIME",
    "LOCALTIMESTAMP",
    "LOGIN",
    "NOT",
    "NULL",
    "NULLIF",
    "OFFSET",
    "ON",
    "OPEN",
    "OR",
    "ORDER",
    "OUTER",
    "OVER",
    "PARTITION",
    "PRECISION",
    "PRIMARY",
    "PROCEDURE",
    "PUBLIC",
    "READ",
    "REAL",
    "RECURSIVE",
    "REFERENCES",
    "REINDEX",
    "RELEASE",
    "RENAME",
    "RESTRICT",
    "RETURN",
    "REVOKE",
    "RIGHT",
    "ROLLBACK",
    "ROWS",
    "SELECT",
    "SESSION_USER",
    "SET",
    "SOME",
    "STATISTICS",
    "TABLE",
    "TABLESPACE",
    "TEMPORARY",
    "THEN",
    "TO",
    "TRAILING",
    "TRANSACTION",
    "TRIGGER",
    "TRUE",
    "TRUNCATE",
    "UNION",
    "UNIQUE",
    "UPDATE",
    "USER",
    "USING",
    "VALUES",
    "VIEW",
    "WHEN",
    "WHERE",
    "WHILE",
    "WITH",
    "WRITE",
];

pub const DATA_TYPES: [&str; 43] = [
    "BIGINT",                   //	int8	signed eight-byte integer
    "BIGSERIAL",                //	serial8	autoincrementing eight-byte integer
    "BIT",                      // [ (n) ]	 	fixed-length bit string
    "BIT VARYING",              // varying [ (n) ]	varbit [ (n) ]	variable-length bit string
    "BOOLEAN",                  //	bool	logical Boolean (true/false)
    "BOX",                      //	 	rectangular box on a plane
    "BYTEA",                    //	 	binary data (“byte array”)
    "CHARACTER",                // [ (n) ]	char [ (n) ]	fixed-length character string
    "CHARACTER VARYING",        // varying [ (n) ]	varchar [ (n) ]	variable-length character string
    "CIDR",                     //	 	IPv4 or IPv6 network address
    "CIRCLE",                   //	 	circle on a plane
    "DATE",                     //	 	calendar date (year, month, day)
    "DOUBLE",                   // precision	float8	double precision floating-point number (8 bytes)
    "INET",                     //	 	IPv4 or IPv6 host address
    "INTEGER",                  //	int, int4	signed four-byte integer
    "INTERVAL",                 // [ fields ] [ (p) ]	 	time span
    "JSON",                     //	 	textual JSON data
    "JSONB",                    //	 	binary JSON data, decomposed
    "LINE",                     //	 	infinite line on a plane
    "LSEG",                     //	 	line segment on a plane
    "MACADDR",                  //	 	MAC (Media Access Control) address
    "MACADDR8",                 //	 	MAC (Media Access Control) address (EUI-64 format)
    "MONEY",                    //	 	currency amount
    "NUMERIC",                  // [ (p, s) ]	decimal [ (p, s) ]	exact numeric of selectable precision
    "PATH",                     //	 	geometric path on a plane
    "PG_LSN",                   //	 	PostgreSQL Log Sequence Number
    "PG_SNAPSHOT",              //	 	user-level transaction ID snapshot
    "POINT",                    //	 	geometric point on a plane
    "POLYGON",                  //	 	closed geometric path on a plane
    "REAL",                     //	float4	single precision floating-point number (4 bytes)
    "SMALLINT",                 //	int2	signed two-byte integer
    "SMALLSERIAL",              //	serial2	autoincrementing two-byte integer
    "SERIAL",                   //	serial4	autoincrementing four-byte integer
    "TEXT",                     //	 	variable-length character string
    "TIME",                     // [ (p) ] [ without time zone ]	 	time of day (no time zone)
    "TIME WITH TIME ZONE",      // [ (p) ] with time zone	timetz	time of day, including time zone
    "TIMESTAMP",                // [ (p) ] [ without time zone ]	 	date and time (no time zone)
    "TIMESTAMP WITH TIME ZONE", // [ (p) ] with time zone	timestamptz	date and time, including time zone
    "TSQUERY",                  //	 	text search query
    "TSVECTOR",                 //	 	text search document
    "TXID_SNAPSHOT",            //	 	user-level transaction ID snapshot (deprecated; see pg_snapshot)
    "UUID",                     //	 	universally unique identifier
    "XML",                      //	 	XML data
];

pub const OPERATORS_COMPARISON: [&str; 13] = [
    "IS",
    "ISNULL",
    "NOTNULL",
    "IS TRUE",
    "IS FALSE",
    "IS NULL",
    "IS DISTINCT FROM",
    "<",
    "<=",
    "=",
    ">=",
    ">",
    "<>",
];
