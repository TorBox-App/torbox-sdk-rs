# \QueuedApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**control_queued_downloads**](QueuedApi.md#control_queued_downloads) | **POST** /v1/api/queued/controlqueued | Control Queued Downloads
[**get_queued_downloads**](QueuedApi.md#get_queued_downloads) | **GET** /v1/api/queued/getqueued | Get Queued Downloads



## control_queued_downloads

> models::ControlQueuedDownloadsResponse control_queued_downloads(control_queued_download)
Control Queued Downloads

### Overview  Controls a queued torrent. By sending the queued torrent's ID and the type of operation you want to perform, it will perform that action on the queued torrent.  Operations are either:  - **Delete** `deletes the queued download from your account`  - **Start** `starts a queued download, cannot be used with the \"all\" parameter`   ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**control_queued_download** | [**ControlQueuedDownload**](ControlQueuedDownload.md) |  | [required] |

### Return type

[**models::ControlQueuedDownloadsResponse**](ControlQueuedDownloadsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queued_downloads

> models::GetQueuedDownloadsResponse get_queued_downloads(id, r#type, bypass_cache, offset, limit)
Get Queued Downloads

### Overview  Retrieves all of a user's queued downloads by type. If you want to get all 3 types, \"torrent\", \"usenet\" and \"webdl\" then you will need to run this request 3 times, each with the different type.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i64**> | Determines the queued download requested, will return an object rather than list. Optional. |  |
**r#type** | Option<**String**> | The type of the queued download you want to retrieve. Can be \"torrent\", \"usenet\" or \"webdl\". Optional. Default is \"torrent\". |  |[default to torrent]
**bypass_cache** | Option<**bool**> | Allows you to bypass the cached data, and always get fresh information. Useful if constantly querying for fresh download stats. Otherwise, we request that you save our database a few calls. |  |[default to false]
**offset** | Option<**i64**> | Determines the offset of items to get from the database. Default is 0. Optional. |  |[default to 0]
**limit** | Option<**i64**> | Determines the number of items to recieve per request. Default is 1000. Optional. |  |[default to 1000]

### Return type

[**models::GetQueuedDownloadsResponse**](GetQueuedDownloadsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

