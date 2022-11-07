use error_handling::ServiceError;
use std::collections::HashMap;

#[derive(Default)]
pub struct Pagination {
    pub offset: i32,
    pub limit: Option<i32>,
}

impl Pagination {
    /// ## Example usage
    /// ```rust
    /// let query_string_params = HashMap::new();
    /// query_string_params.push("offset", "1");
    /// query_string_params.push("limit", "100");
    /// let pagination = types::pagination::Pagination::parse_from_map(query_string_params).unwrap();
    /// assert_eq!(pagination.offset, 1);
    /// assert_eq!(pagination.limit, Some(100));
    /// ```
    pub fn parse_from_map(params: HashMap<String, String>) -> Result<Self, ServiceError> {
        if !params.contains_key("offset") || !params.contains_key("limit") {
            return Err(ServiceError::MissingParams);
        }

        let offset = params
            .get("offset")
            .unwrap()
            .parse::<u32>()
            .map_err(ServiceError::ParseError)?;
        let limit = params
            .get("limit")
            .unwrap()
            .parse::<u32>()
            .map_err(ServiceError::ParseError)?;

        Ok(Self {
            offset: offset as i32,
            limit: Some(limit as i32),
        })
    }
}
