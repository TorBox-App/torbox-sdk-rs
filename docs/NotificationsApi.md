# \NotificationsApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_all_notifications**](NotificationsApi.md#clear_all_notifications) | **POST** /v1/api/notifications/clear | Clear All Notifications
[**clear_single_notification**](NotificationsApi.md#clear_single_notification) | **POST** /v1/api/notifications/clear/{id} | Clear Single Notification
[**get_notification_feed**](NotificationsApi.md#get_notification_feed) | **GET** /v1/api/notifications/mynotifications | Get Notification Feed
[**get_rss_notification_feed**](NotificationsApi.md#get_rss_notification_feed) | **GET** /v1/api/notifications/rss | Get RSS Notification Feed
[**send_test_notification**](NotificationsApi.md#send_test_notification) | **POST** /v1/api/notifications/test | Send Test Notification



## clear_all_notifications

> models::ClearAllNotificationsResponse clear_all_notifications()
Clear All Notifications

### Overview  Marks all of your notifications as read and deletes them permanently.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ClearAllNotificationsResponse**](ClearAllNotificationsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_single_notification

> models::ClearSingleNotificationResponse clear_single_notification(id)
Clear Single Notification

### Overview  Marks a single notification as read and permanently deletes it from your notifications. Requires a `notification_id` which can be found by getting your notification feed.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ClearSingleNotificationResponse**](ClearSingleNotificationResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_feed

> models::GetNotificationFeedResponse get_notification_feed(bypass_cache)
Get Notification Feed

### Overview  Gets your notifications in a JSON object that is easily parsable compared to the RSS Feed. Gives all the same data as the RSS Feed.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bypass_cache** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::GetNotificationFeedResponse**](GetNotificationFeedResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rss_notification_feed

> String get_rss_notification_feed(token)
Get RSS Notification Feed

### Overview  Gets your notifications in an RSS Feed which allows you to use them with RSS Feed readers or notification services that can take RSS Feeds and listen to updates. As soon as a notification goes to your account, it will be added to your feed.  ### Authorization  Requires an API key using as a query parameter using the `token` key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The user's API key. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_test_notification

> models::SendTestNotificationResponse send_test_notification()
Send Test Notification

### Overview  Sends a test notification to all enabled notification types. This can be useful for validating setups. No need for any body in this request.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SendTestNotificationResponse**](SendTestNotificationResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

