use crate::core::str::StaticString;

pub const PORT: u16 = 8080;
pub const SPEC_ROUTE: StaticString = "/api-docs/openapi.json";
pub const SWAGGER_ROUTE: StaticString = "/swagger-ui/{_:.*}";
