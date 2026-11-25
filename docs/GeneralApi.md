# \GeneralApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get30_day_stats**](GeneralApi.md#get30_day_stats) | **GET** /v1/api/stats/30days | Get 30 Day Stats
[**get_changelogs_json**](GeneralApi.md#get_changelogs_json) | **GET** /v1/api/changelogs/json | Get Changelogs JSON
[**get_changelogs_rss_feed**](GeneralApi.md#get_changelogs_rss_feed) | **GET** /v1/api/changelogs/rss | Get Changelogs RSS Feed
[**get_speedtest_files**](GeneralApi.md#get_speedtest_files) | **GET** /v1/api/speedtest | Get Speedtest Files
[**get_stats**](GeneralApi.md#get_stats) | **GET** /v1/api/stats | Get Stats
[**get_up_status**](GeneralApi.md#get_up_status) | **GET** / | Get Up Status



## get30_day_stats

> models::Get30DayStatsResponse get30_day_stats()
Get 30 Day Stats

### Overview  Gets the general stats of the TorBox service for the last 30 days, one entry per day. Each entry is a snapshot of the totals at that time.  ### Authorization  None needed. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Get30DayStatsResponse**](Get30DayStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_changelogs_json

> models::GetChangelogsJsonResponse get_changelogs_json(offset, limit)
Get Changelogs JSON

### Overview  Gets most recent changelogs from [https://feedback.torbox.app/changelog](https://feedback.torbox.app/changelog). This is useful for developers who want to integrate the changelog into their apps for their users to see. Includes content in HTML for developers to easily render the text in a fancy yet simple way.  ### Authorization  None needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 10]

### Return type

[**models::GetChangelogsJsonResponse**](GetChangelogsJsonResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_changelogs_rss_feed

> String get_changelogs_rss_feed(offset, limit)
Get Changelogs RSS Feed

### Overview  Gets most recent 100 changelogs from [https://feedback.torbox.app/changelog](https://feedback.torbox.app/changelog). This is useful for people who want updates on the TorBox changelog. Includes all the necessary items to make this compatible with RSS feed viewers for notifications, and proper HTML viewing.  ### Authorization  None needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i64**> |  |  |[default to 0]
**limit** | Option<**i64**> |  |  |[default to 10]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speedtest_files

> models::GetSpeedtestFilesResponse get_speedtest_files(user_ip, region, test_length)
Get Speedtest Files

### Overview  Gets CDN speedtest files. This can be used for speedtesting TorBox for users or other usages, such as checking download speeds for verification. Provides all necessary data such as region, server name, and even coordinates. Uses the requesting IP to determine if the server is the closest to the user.  You also have the ability to choose between long tests or short tests via the \"test_length\" parameter. You may also force the region selection by passing the \"region\" as a specific region.  ### Authorization  None needed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_ip** | Option<**String**> | Pass a user's IP to get the closest server to them, otherwise will use the calling IP for location. |  |
**region** | Option<**String**> | Determines what cdns are returned. May be any region that TorBox is located in. To get this value, send a request without this value to determine all of the available regions that are available. |  |[default to all]
**test_length** | Option<**String**> | Determines the size of the file used for the speedtest. May be \"long\" or \"short\". Optional. |  |[default to short]

### Return type

[**models::GetSpeedtestFilesResponse**](GetSpeedtestFilesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stats

> models::GetStatsResponse get_stats()
Get Stats

### Overview  Simply gets general stats about the TorBox service.  ### Authorization  None needed. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetStatsResponse**](GetStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_up_status

> models::GetUpStatusResponse get_up_status()
Get Up Status

### Overview  Simply gets if the TorBox API is available for use or not. Also might include information about downtimes.  ### Authorization  None needed. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetUpStatusResponse**](GetUpStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

