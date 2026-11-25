# \UserApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_referral_to_account**](UserApi.md#add_referral_to_account) | **POST** /v1/api/user/addreferral | Add Referral To Account
[**delete_account**](UserApi.md#delete_account) | **DELETE** /v1/api/user/deleteme | Delete Account
[**edit_user_settings**](UserApi.md#edit_user_settings) | **PUT** /v1/api/user/settings/editsettings | Edit User Settings
[**get_confirmation_code**](UserApi.md#get_confirmation_code) | **GET** /v1/api/user/getconfirmation | Get Confirmation Code
[**get_referral_data**](UserApi.md#get_referral_data) | **GET** /v1/api/user/referraldata | Get Referral Data
[**get_subscriptions**](UserApi.md#get_subscriptions) | **GET** /v1/api/user/subscriptions | Get Subscriptions
[**get_token_from_device_code_process**](UserApi.md#get_token_from_device_code_process) | **POST** /v1/api/user/auth/device/token | Get Token From Device Code Process
[**get_transaction_pdf**](UserApi.md#get_transaction_pdf) | **GET** /v1/api/user/transaction/pdf | Get Transaction PDF
[**get_transactions**](UserApi.md#get_transactions) | **GET** /v1/api/user/transactions | Get Transactions
[**get_user_data**](UserApi.md#get_user_data) | **GET** /v1/api/user/me | Get User Data
[**get_user_stats**](UserApi.md#get_user_stats) | **GET** /v1/api/user/stats | Get User Stats
[**refresh_api_token**](UserApi.md#refresh_api_token) | **POST** /v1/api/user/refreshtoken | Refresh API Token
[**start_device_code_authorization_process**](UserApi.md#start_device_code_authorization_process) | **GET** /v1/api/user/auth/device/start | Start Device Code Authorization Process



## add_referral_to_account

> models::AddReferralToAccountResponse add_referral_to_account(referral)
Add Referral To Account

### Overview  Automatically adds a referral code to the user's account. This can be used for developers to automatically add their referral to user's accounts who use their service.  This will not override any referral a user already has. If they already have one, then it cannot be changed using this endpoint. It can only be done by the user on the [website](https://torbox.app/subscription).  ### Authorization  Requires an API key using the Authorization Bearer Header. Use the user's API key, not your own. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**referral** | **String** | A referral code. Must be UUID. | [required] |

### Return type

[**models::AddReferralToAccountResponse**](AddReferralToAccountResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_account

> models::DeleteAccountResponse delete_account(delete_account_schema)
Delete Account

### Overview  Permanently deletes the user's TorBox account. This cannot be undone.  Because of how dangerous this action is, it needs two things on top of the API key: the user's session token from the website, and a confirmation code, which is sent to the user's email by the Get Confirmation Code endpoint.  ### Authorization  Requires an API key using the Authorization Bearer Header, as well as the session token and the confirmation code in the body. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_account_schema** | [**DeleteAccountSchema**](DeleteAccountSchema.md) |  | [required] |

### Return type

[**models::DeleteAccountResponse**](DeleteAccountResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_user_settings

> models::EditUserSettingsResponse edit_user_settings(base_settings_model)
Edit User Settings

### Overview  Edits a user's TorBox settings. These values take effect immediately. Please view the body to read about allowed options and parameters.  To get the user settings, simply use the \"[Get User Data](https://api.torbox.app/docs#/User/get_user_data)\" endpoint, while sending the \"?settings=true\" parameter  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**base_settings_model** | [**BaseSettingsModel**](BaseSettingsModel.md) |  | [required] |

### Return type

[**models::EditUserSettingsResponse**](EditUserSettingsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_confirmation_code

> models::GetConfirmationCodeResponse get_confirmation_code()
Get Confirmation Code

### Overview  Requests a 6 digit code to be sent to the user's email for verification. Used to verify a user actually wants to perform a potentially dangerous action.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetConfirmationCodeResponse**](GetConfirmationCodeResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_referral_data

> models::GetReferralDataResponse get_referral_data()
Get Referral Data

### Overview  Use this endpoint to get the user's referral data. This includes information such as how many referrals they have made, and their referral code.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetReferralDataResponse**](GetReferralDataResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions

> models::GetSubscriptionsResponse get_subscriptions()
Get Subscriptions

### Overview  Gets a users subscriptions. This includes active and inactive subscriptions for all payment processors.  It should be noted that \"plan_code\" is not normalized to the standard plan codes such as 1, 2, or 3. These \"plan_code\"s are payment processor specific and used internally. If you want to display these subscriptions, use the \"plan_name\".  For displaying information to users, use the \"status\", \"gateway\", and \"plan_name\". No other information is relevant to the user, and the rest is used for TorBox internally.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSubscriptionsResponse**](GetSubscriptionsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_token_from_device_code_process

> models::GetTokenFromDeviceCodeProcessResponse get_token_from_device_code_process(device_token_schema)
Get Token From Device Code Process

### Overview  Gets the user's API token from the device code authorization process. This endpoint is expected to be polled in interval until it returns success, or until the expiry has passed. All data you need for this request is in the [original request](https://api.torbox.app/docs#/User/start_device_code_authorization_process), which includes the interval to be polled by, the expiry time to stop at, and `device_code` to be sent to the API to get the correct data.  ### Authorization  None required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_token_schema** | [**DeviceTokenSchema**](DeviceTokenSchema.md) |  | [required] |

### Return type

[**models::GetTokenFromDeviceCodeProcessResponse**](GetTokenFromDeviceCodeProcessResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_pdf

> String get_transaction_pdf(transaction_id)
Get Transaction PDF

### Overview  Generates a PDF of the transaction information. Requires the transaction ID from the \"Get Transactions\" request.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | The transaction ID that you want to generate the PDF for. | [required] |

### Return type

**String**

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pdf, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions

> models::GetTransactionsResponse get_transactions()
Get Transactions

### Overview  Use this endpoint to get the user's list of transactions. This data isn't exactly relevant to a third party developer, unless you want to build full parity with TorBox's dashboard, or simply want to show this information.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetTransactionsResponse**](GetTransactionsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_data

> models::GetUserDataResponse get_user_data(settings, download_stats, referral)
Get User Data

### Overview  Gets a users account data and information.  ### Plans  `0` is Free plan  `1` is Essential plan (_$3 plan_)  `2` is Pro plan (_$10 plan_)  `3` is Standard plan (_$5 plan_)  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings** | Option<**bool**> | Allows you to retrieve user settings. |  |[default to false]
**download_stats** | Option<**bool**> |  |  |[default to false]
**referral** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::GetUserDataResponse**](GetUserDataResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_stats

> models::GetUserStatsResponse get_user_stats(general, bandwidth, bandwidth_grouping)
Get User Stats

### Overview  Gets a users stats. This data is currently split into two sections, \"general\" and \"bandwidth\". General shows general data, such as \"Total Data Downloaded\", \"Total Data Egress\", \"Overall Ratio\", and \"Total Downloaded\". Bandwidth shows the amount of bandwidth a user has downloaded within the past 30 days. We don't store this data for longer than 30 days, and it can be made more or less granular. Makes cool graphs!  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**general** | Option<**bool**> | Retrieve general stats. |  |[default to true]
**bandwidth** | Option<**bool**> | Retrieve bandwidth stats. |  |[default to false]
**bandwidth_grouping** | Option<**String**> | If the \"bandwidth\" query parameter is sent, this value is used. Allowed is \"hour\", \"day\", \"week\", and \"month\", this narrates how bandwidth is grouped. More granular shows more information. |  |[default to day]

### Return type

[**models::GetUserStatsResponse**](GetUserStatsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_api_token

> models::RefreshApiTokenResponse refresh_api_token(refresh_token_schema)
Refresh API Token

### Overview  If you want a new API token, or your old one has been compromised, you may request a new one. If you happen to forget the token, go the [TorBox settings page ](https://torbox.app/settings) and copy the current one. If this still doesn't work, you may contact us at our support email for a new one.  ### Authorization  Requires an API key using the Authorization Bearer Header as well as passing the `session_token` from the website to be passed in the body. You can find the `session_token` in the localStorage of your browser on any TorBox.app page under the key `torbox_session_token`. This is a temporary token that only lasts for 1 hour, which is why it is used here to verify the identity of a user as well as their API token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_token_schema** | [**RefreshTokenSchema**](RefreshTokenSchema.md) |  | [required] |

### Return type

[**models::RefreshApiTokenResponse**](RefreshApiTokenResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_device_code_authorization_process

> models::StartDeviceCodeAuthorizationProcessResponse start_device_code_authorization_process(app)
Start Device Code Authorization Process

### Overview  Starts the device code authorization process. Returns all the necessary information to complete the rest of the flow. This includes the following:  - `device_code` - The code your app will use for consecutive requests for getting the user token.  - `code` - This is the 6 digit code that you show to the user to be entered at the following verification_url.  - `verification_url` - This url which you will show as a QR code.  - `friendly_verification_url` - The friendly short url which you can show as plain text to user if they cannot use the QR code method.   You may also submit the `app` parameter to this request to show your app name on the `verification_url` page. This concretes user trust in authorizing your application to use their TorBox account. This is not required, but is a good idea. Default is \"Third Party App\". This name is also only allowed to be alpha-numeric characters (_A-Z,0-9_). Anything else will be filtered our when being shown to the user. The `friendly_verification_url` will always show \"Third Party App\" to keep the URL short.  The device_code is only valid for 10 minutes. Make sure the user is aware of this time limit so they submit and login before this time limit is over.  ### Authorization  None required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app** | Option<**String**> | The app name that the user will see when verifying the code. This should be the name of your app. |  |[default to Third Party App]

### Return type

[**models::StartDeviceCodeAuthorizationProcessResponse**](StartDeviceCodeAuthorizationProcessResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

