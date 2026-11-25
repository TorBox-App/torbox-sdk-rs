# \VendorsApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_vendor_account**](VendorsApi.md#get_vendor_account) | **GET** /v1/api/vendors/account | Get Vendor Account
[**get_vendor_user**](VendorsApi.md#get_vendor_user) | **GET** /v1/api/vendors/getaccount | Get Vendor User
[**get_vendor_users**](VendorsApi.md#get_vendor_users) | **GET** /v1/api/vendors/getaccounts | Get Vendor Users
[**refresh_vendor_users**](VendorsApi.md#refresh_vendor_users) | **PATCH** /v1/api/vendors/refresh | Refresh Vendor Users
[**register_vendor**](VendorsApi.md#register_vendor) | **POST** /v1/api/vendors/register | Register Vendor
[**register_vendor_user**](VendorsApi.md#register_vendor_user) | **POST** /v1/api/vendors/registeruser | Register Vendor User
[**remove_vendor_user**](VendorsApi.md#remove_vendor_user) | **DELETE** /v1/api/vendors/removeuser | Remove Vendor User
[**update_vendor_account**](VendorsApi.md#update_vendor_account) | **PUT** /v1/api/vendors/updateaccount | Update Vendor Account



## get_vendor_account

> models::GetVendorAccountResponse get_vendor_account()
Get Vendor Account

### Overview  Gets the vendor account that is registered under your TorBox account.  ##### Response Keys  - \"vendor_id\" `the id of the vendor account. Users owned by the vendor have this value as their \"vendor_id\".` - \"vendor_token\" `the vendor token that was generated when the vendor account was registered.` - \"vendor_email\" `the email that was on the TorBox account when the vendor account was registered.` - \"vendor_status\" `the status of the vendor account. A vendor account is registered as \"pending\".` - \"users_allowed\" `the highest number of users the vendor account may have.` - \"current_users\" `the number of users currently counted against \"users_allowed\".` - \"can_register_new\" `whether the vendor account is allowed to register users.` - \"account_type\" `always \"individual\".` - \"plan\" `the plan that is given to the vendor's users.` - \"paid_until\" `the date the vendor account is paid until. It is given to the vendor's users as their \"premium_expires_at\".` - \"can_generate_vouchers\" `whether the vendor account is allowed to generate vouchers.`  ### Rate Limits  This endpoint is rate limited to **1 request per second** per API token.  ### Authorization  Requires an API key using the Authorization Bearer Header. Works with an API key from any plan, including the free plan. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetVendorAccountResponse**](GetVendorAccountResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vendor_user

> models::GetVendorUserResponse get_vendor_user(user_auth_id)
Get Vendor User

### Overview  Gets a single user account that is owned by your vendor account, using the user's `auth_id`. A user that is not owned by your vendor account is not found.  The response is the user's full account record, the same as an item of the Get Vendor Users endpoint, with one more key at the end:  - \"api_token\" `the user's API token. If the user has no API token, this is the text \"USER HAS NOT CONFIRMED EMAIL\".`  ### Rate Limits  This endpoint is rate limited to **1 request per second** per API token.  ### Authorization  Requires an API key using the Authorization Bearer Header. Works with an API key from any plan, including the free plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_auth_id** | **String** | Auth id of the user, from the Get Vendor Users endpoint. Required. | [required] |

### Return type

[**models::GetVendorUserResponse**](GetVendorUserResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vendor_users

> models::GetVendorUsersResponse get_vendor_users()
Get Vendor Users

### Overview  Gets every user account that is owned by your vendor account. Each item is the user's full account record, including their email, plan and `premium_expires_at`. There are no paging parameters, all of the users are returned in one response, ordered by their `id`. If the vendor account owns no users, `data` is an empty list.  The `auth_id` of a user is what the Get Vendor User and Remove Vendor User endpoints take as `user_auth_id`.  ### Rate Limits  This endpoint is rate limited to **1 request per second** per API token.  ### Authorization  Requires an API key using the Authorization Bearer Header. Works with an API key from any plan, including the free plan. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetVendorUsersResponse**](GetVendorUsersResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_vendor_users

> models::RefreshVendorUsersResponse refresh_vendor_users()
Refresh Vendor Users

### Overview  Refreshes every user that is owned by your vendor account. Each user's plan is set to your vendor account's `plan`, and each user's `premium_expires_at` is set to your vendor account's `paid_until`. Your vendor account's `current_users` is then recounted from the users it owns.  Your vendor account must be active: approved (its status is no longer \"pending\"), on a paid plan (its `plan` is not 0) and with a `paid_until` date that has not passed. Otherwise the request is rejected and no user is changed.  This endpoint takes no parameters and returns no data.  ### Authorization  Requires an API key using the Authorization Bearer Header. Works with an API key from any plan, including the free plan. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RefreshVendorUsersResponse**](RefreshVendorUsersResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_vendor

> models::RegisterVendorResponse register_vendor(vendor_name, vendor_url)
Register Vendor

### Overview  Registers a vendor account under your TorBox account. The vendor account uses your TorBox account's email, is created with the status \"pending\" and a newly generated vendor token, and your TorBox account is marked as a vendor. Once it is registered, a welcome email is sent to your account's email address and the TorBox team is notified of the signup.  A TorBox account can only have one vendor account, and an account that is itself owned by a vendor cannot register as a vendor.  The `vendor_url` must start with `http://` or `https://` and be a domain with an optional path. A URL with a port, a query string or a fragment is rejected.  To read the vendor account afterwards, including its id, user limit, plan and paid-until date, use the Get Vendor Account endpoint.  ### Rate Limits  This endpoint is rate limited to **15 requests per hour** per API token.  ### Authorization  Requires an API key using the Authorization Bearer Header. Works with an API key from any plan, including the free plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vendor_name** | **String** | Name of the vendor. Required. | [required] |
**vendor_url** | **String** | Website of the vendor. Must start with http:// or https:// and cannot contain a port, a query string or a fragment. Required. | [required] |

### Return type

[**models::RegisterVendorResponse**](RegisterVendorResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_vendor_user

> models::RegisterVendorUserResponse register_vendor_user(user_email)
Register Vendor User

### Overview  Registers a user under your vendor account. Your vendor account must be allowed to register users (`can_register_new`) and must be below its user limit (`current_users` lower than `users_allowed`). The email is trimmed and lowercased before it is used, and it must be a valid, deliverable address that is not from a disposable email provider.  What happens depends on whether the email already has a TorBox account:  - **New account** `the email has no TorBox account. A new TorBox account is created with its email already confirmed and a generated password. It is owned by your vendor account, on your vendor account's plan, with your vendor account's \"paid_until\" as its \"premium_expires_at\". The generated password is returned in the \"password\" key.` - **Existing account** `the email already has a TorBox account that is not owned by a vendor. The account is attached to your vendor account, and its plan and \"premium_expires_at\" are replaced with your vendor account's plan and \"paid_until\". Its password is not changed, so the \"password\" key is the text \"ALREADY_SET\". The user receives a notification that they were added to your vendor account, with a link to your vendor URL.`  An existing account is attached only when the email you send is the account's own email, or is that email without its dots and without anything from a `+` on in the part before the `@`. Any other spelling of an existing account's email attaches nothing and is rejected with the error `USER_EXISTS`: the account of `jane.doe@example.com` is not attached by `jane.doe+shop@example.com`.  An account that is already owned by a vendor cannot be registered, and that includes an account your own vendor account already owns. When two vendors register the same account at the same moment, one of them gets it and the other is rejected.  Each successful registration raises your vendor account's `current_users` by one. The `auth_id` in the response is what the Get Vendor User and Remove Vendor User endpoints take as `user_auth_id`.  ### Rate Limits  This endpoint is rate limited to **1 request per second** per API token.  ### Authorization  Requires an API key using the Authorization Bearer Header. Works with an API key from any plan, including the free plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_email** | **String** | Email address of the user to register. Required. | [required] |

### Return type

[**models::RegisterVendorUserResponse**](RegisterVendorUserResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_vendor_user

> models::RemoveVendorUserResponse remove_vendor_user(user_auth_id)
Remove Vendor User

### Overview  Removes a user from your vendor account. The user must be owned by your vendor account. The user's TorBox account is not deleted, it is only released from the vendor, even though the `detail` of the response reads \"User deleted successfully.\"  What happens to the user's plan depends on the plan they are on when they are removed:  - If the user is on the same plan as your vendor account, the user is moved to the free plan (plan 0) and their `premium_expires_at` is set to the current time. - If the user is on a different plan than your vendor account, their plan and their `premium_expires_at` are left as they are.  Your vendor account's `current_users` is lowered by one, and the user receives a notification that they were removed from the vendor.  ### Rate Limits  This endpoint is rate limited to **1 request per second** per API token.  ### Authorization  Requires an API key using the Authorization Bearer Header. Works with an API key from any plan, including the free plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_auth_id** | **String** | Auth id of the user to remove, from the Get Vendor Users endpoint. Required. | [required] |

### Return type

[**models::RemoveVendorUserResponse**](RemoveVendorUserResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vendor_account

> models::UpdateVendorAccountResponse update_vendor_account(vendor_name, vendor_url)
Update Vendor Account

### Overview  Updates the name, the URL, or both, of your vendor account. Send at least one of `vendor_name` and `vendor_url`. The one that is not sent is left as it is.  The `vendor_url` follows the same rule as on the Register Vendor endpoint: it must start with `http://` or `https://` and be a domain with an optional path. A URL with a port, a query string or a fragment is rejected, and the request then changes nothing.  The response is the updated vendor account, in the same shape and with the same `detail` as the Get Vendor Account endpoint.  ### Rate Limits  This endpoint is rate limited to **15 requests per hour** per API token.  ### Authorization  Requires an API key using the Authorization Bearer Header. Works with an API key from any plan, including the free plan. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vendor_name** | Option<**String**> | New name of the vendor. Optional, but at least one of vendor_name and vendor_url must be sent. |  |
**vendor_url** | Option<**String**> | New website of the vendor. Must start with http:// or https:// and cannot contain a port, a query string or a fragment. Optional, but at least one of vendor_name and vendor_url must be sent. |  |

### Return type

[**models::UpdateVendorAccountResponse**](UpdateVendorAccountResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

