//struct for handling request;
pub struct Request {
    pub httpversion: String,
    pub host: String,
    pub route: String,
    pub method: String,
    pub body_data: String,
    pub content_type: String,
    pub params_data: String,
}

impl Request {
    pub fn new(
        httpversion: String,
        host: String,
        route: String,
        method: String,
        body_data: String,
        content_type: String,
        params_data: String,
    ) -> Self {
        Self {
            httpversion,
            host,
            route,
            method,
            body_data,
            content_type,
            params_data,
        }
    }
}
