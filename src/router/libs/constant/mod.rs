pub const HTTP_OK : u64 = 200;
pub const HTTP_CREATED : u64 = 201;
pub const HTTP_NO_CONTENT : u64 = 204;
pub const HTTP_BAD_REQUEST : u64 = 400;
pub const HTTP_NOT_FOUND : u64 = 404;

// Constant of log status
pub const SUCCESS : &str  = "SUCCESS";
pub const WARNING : &str = "WARNING";
pub const DEBUG : &str = "DEBUG";
pub const ERROR : &str = "ERROR";

// Log key
pub const LOG_KEY : &str = "AUTHENTICATION_LOG";

// Message validator
pub const DATA_EMPTY : &str = "DATA_EMPTY";
pub const DATA_NOT_COLLECT_LENGTH : &str = "DATA_NOT_COLLECT_LENGTH";
pub const DATA_NOT_MATCHED : &str = "DATA_NOT_MATCHED";
pub const DATA_NOT_NUMERIC : &str = "DATA_NOT_NUMERIC";
pub const DATA_PASSED : &str = "DATA_PASSED";

// Matching
pub const MOBILE_LENGTH : usize = 10;
pub const OTP_LENGTH : usize = 6;

// Role
pub const ROLE_CUSTOMER : &str = "customer";
pub const ROLE_MERCHANT : &str = "merchant";
pub const ROLE_RIDER : &str = "rider";

// Event
pub const EVENT_NEW_OTP : &str = "New_Otp";

// Jwt
pub const EXPIRE_ACCESS_TOKEN : i64 = 7; // 7 Days in millisecond
pub const EXPIRE_REFRESH_TOKEN : i64 = 90; // 90 Days in millisecond


