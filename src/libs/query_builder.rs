use sql_query_builder::Select;

pub struct WherePair {
    pub key: String,
    pub value: String,
}

impl WherePair {
    pub fn new(
        key: &str,
        value: &str,
    ) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

pub fn create_select(table_name: &str) -> Select {
    Select::new()
        .select("*")
        .from(table_name)
}

pub fn generate_where_clauses(pairs: &Vec<WherePair>) -> Vec<String> {
    let mut clauses: Vec<String> = Vec::new();

    for (i, pair) in pairs.iter().enumerate() {
        clauses.push(format!("{} = ${}", pair.key, i + 1))
    }

    clauses
}

pub fn generate_single_clause_select(table_name: &str, clause_key: &str) -> String {
    create_select(table_name)
        .where_clause(&format!("{} = $1", clause_key).clone())
        .as_string()
}

pub fn generate_multi_clause_select(table_name: &str, pairs: &Vec<WherePair>) -> String {
    let clauses: Vec<String> = generate_where_clauses(&pairs);

    let mut query = create_select(table_name);

    for (i, clause) in clauses.iter().enumerate() {
        query = match i == 0 {
            true => query.where_clause(clause),
            false => query.and(clause),
        };
    }

    query.as_string()
}
