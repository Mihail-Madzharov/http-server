use std::str::FromStr;

pub enum Method {
    PUT,
    GET,
    POST,
    DELETE,
    UPDATE,
}

impl FromStr for Method {
    type Err = MethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "UPDATE" => Ok(Self::UPDATE),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
