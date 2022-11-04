use error_handling::ServiceError;
use std::collections::HashMap;

pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

impl Pagination {
    /// ## Example usage
    /// ```rust
    /// let query_string_params = HashMap::new();
    /// query_string_params.push("start", "1");
    /// query_string_params.push("end", "100");
    /// let pagination = types::pagination::Pagination::parse_from_map(query_string_params).unwrap();
    /// assert_eq!(pagination.start, 1);
    /// assert_eq!(pagination.start, 100);
    /// ```
    pub fn parse_from_map(params: HashMap<String, String>) -> Result<Self, ServiceError> {
        if !params.contains_key("start") || !params.contains_key("end") {
            return Err(ServiceError::MissingParams);
        }

        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(ServiceError::ParseError)?;
        let end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(ServiceError::ParseError)?;

        if start > end {
            return Err(ServiceError::InvalidParamsRange);
        }

        Ok(Self { start, end })
    }
}
