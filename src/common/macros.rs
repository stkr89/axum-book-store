#[macro_export]
macro_rules! validate_request {
    ($req:expr) => {
        match $req.validate() {
            Err(e) => {
                error!("Invalid request: {:?}", e);
                return Err(APIError::BadRequest(e.to_string()));
            },
            _ => {}
        };
    };
}

#[macro_export]
macro_rules! server_error_response {
    ($e:expr, $desc:expr) => {
        {
            error!("Error: {:?}", $e);
            Err(APIError::ServerError($desc))
        }
    };
}