pub const GET_CONTACT_STMT: &str = "
SELECT
    ID,
    EMAIL,
    USERNAME
FROM PUBLIC.USER
WHERE ID != $1;    
";