use serde_json::Value;

use crate::router::libs::constant::{
    DATA_EMPTY,DATA_NOT_COLLECT_LENGTH,DATA_NOT_MATCHED,DATA_PASSED,
    MOBILE_LENGTH,OTP_LENGTH,ROLE_CUSTOMER,ROLE_MERCHANT,ROLE_RIDER,
    DATA_NOT_NUMERIC
};

fn is_digit(data : &String) -> bool {
    let digit = data.parse::<u64>();
    match digit {
        Ok(_ok) => true,
        Err(_e) => false
    }
}

pub fn validate_mobile_number(mobile_number : &String) -> &'static str {
    if mobile_number.is_empty() {
        return DATA_EMPTY;
    }

    if mobile_number.len() != MOBILE_LENGTH {
        return DATA_NOT_COLLECT_LENGTH;
    }

    if !is_digit(mobile_number){
        return DATA_NOT_NUMERIC;
    }
    
    return DATA_PASSED;
}

pub fn validate_otp_code(otp_code : &String) -> &'static str {
    if otp_code.is_empty() {
        return DATA_EMPTY;
    }

    if otp_code.len() != OTP_LENGTH {
        return DATA_NOT_COLLECT_LENGTH;
    }

    if !is_digit(otp_code) {
        return DATA_NOT_NUMERIC;
    }

    return DATA_PASSED;
}

pub fn validate_role(role : &String) -> &'static str {
    if role.is_empty() {
        return DATA_EMPTY;
    }

    match role.as_str() {
        ROLE_CUSTOMER => DATA_PASSED,
        ROLE_MERCHANT => DATA_PASSED,
        ROLE_RIDER => DATA_PASSED,
        _ => DATA_NOT_MATCHED
    }
}

pub fn validate_token(token : &String) -> &'static str {
    if token.is_empty() {
        return DATA_EMPTY;
    }

    return DATA_PASSED;
}