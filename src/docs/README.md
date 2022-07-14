## [1] - Find or create new login user (mobile number)

```http
POST /api/v1/authentication/
```

## Request
```json
{
    "mobile_number" : "0881829915"
}
```

## Responses

```json
{
    "code": 201,
    "success": true,
    "payload": {
        "message": "Create new authentication successfully",
        "data": [
            {
                "id": "62bc4f27eaceff44edf66e61",
                "mobile": "0881829915",
                "expired": 1656508499148
            }
        ]
    }
}
```

## [2] - Resend new OTP

```http
PATCH /api/v1/authentication/{id}/
```

## Request
```json
{
    "mobile_number" : "0881029915"
}
```

## Responses

```json
{
    "code": 200,
    "success": true,
    "payload": {
        "message": "Resend OTP successfully",
        "data": [
            {
                "id": "62bc4f27eaceff44edf66e61",
                "mobile": "0881829915",
                "expired": 1656509604684
            }
        ]
    }
}
```

## [3] - Login user (mobile number)

```http
PUT /api/v1/authentication/{id}/
```

## Request
```json
{
    "mobile_number" : "0881829915",
    "otp_code" : "298460",
    "role": "customer"
}
```

## Responses

```json
{
    "code": 200,
    "success": true,
    "payload": {
        "message": "Login successfully",
        "data": [
            {
                "id": "62bc4f27eaceff44edf66e61",
                "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2MmJjNGYyN2VhY2VmZjQ0ZWRmNjZlNjEiLCJyb2xlIjoiY3VzdG9tZXIiLCJpYXQiOjE2NTY1MDk0MjIsImV4cCI6MTY1NzExNDIyMn0.--BkN6l43Yc52DdZiDJe2dymjpS7LE0CY9weKRxVFMY"
            }
        ]
    }
}
```

## [4] - Renew access token

```http
PATCH /api/v1/authorization/{id}/
```

## Request
```json
{
    "mobile_number" : "0881829915",
    "access_token" : "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2MmJjNGYyN2VhY2VmZjQ0ZWRmNjZlNjEiLCJyb2xlIjoiY3VzdG9tZXIiLCJpYXQiOjE2NTY1MDgyNzIsImV4cCI6MTY1NzExMzA3Mn0.BcgQ0cMOMB5njBiW5w9ptvgmjq02NFKKGtTlqwfnD7M"
}
```

## Responses

```json
{
    "code": 200,
    "success": true,
    "payload": {
        "message": "Verify authorization successfully",
        "data": [
            {
                "id": "62bc4f27eaceff44edf66e61",
                "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2MmJjNGYyN2VhY2VmZjQ0ZWRmNjZlNjEiLCJyb2xlIjoiY3VzdG9tZXIiLCJpYXQiOjE2NTY1MDk0NjIsImV4cCI6MTY1NzExNDI2Mn0.3A7ceQNF13sPI_QX0myeIZkhJqCTROUaPyAp2RJCoK0"
            }
        ]
    }
}
```

## Status Codes

| Status Code | Description |
| :--- | :--- |
| 200 | `OK` |
| 201 | `CREATED` |
| 400 | `BAD REQUEST` |
| 404 | `NOT FOUND` |
| 500 | `INTERNAL SERVER ERROR` |