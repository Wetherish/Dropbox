// DEFINE TABLE Files TYPE NORMAL SCHEMALESS PERMISSIONS NONE;

// -- ------------------------------
// -- FIELDS
// -- ------------------------------ 

// DEFINE FIELD file_type ON Files TYPE string PERMISSIONS FULL;
// DEFINE FIELD is_file ON Files TYPE bool DEFAULT false PERMISSIONS FULL;
// DEFINE FIELD is_starred ON Files TYPE bool DEFAULT false PERMISSIONS FULL;
// DEFINE FIELD name ON Files TYPE string PERMISSIONS FULL;
// DEFINE FIELD owner ON Files TYPE record<User> PERMISSIONS FULL;
// DEFINE FIELD parent ON Files TYPE option<record<Files>> PERMISSIONS FULL;
// DEFINE FIELD size ON Files TYPE int DEFAULT 0 PERMISSIONS FULL;
// DEFINE FIELD thumbnail_url ON Files TYPE option<string> PERMISSIONS FULL;
