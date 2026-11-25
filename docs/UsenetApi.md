# \UsenetApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**async_create_usenet_download**](UsenetApi.md#async_create_usenet_download) | **POST** /v1/api/usenet/asynccreateusenetdownload | Async Create Usenet Download
[**control_usenet_download**](UsenetApi.md#control_usenet_download) | **POST** /v1/api/usenet/controlusenetdownload | Control Usenet Download
[**create_usenet_download**](UsenetApi.md#create_usenet_download) | **POST** /v1/api/usenet/createusenetdownload | Create Usenet Download
[**edit_usenet_item**](UsenetApi.md#edit_usenet_item) | **PUT** /v1/api/usenet/editusenetdownload | Edit Usenet Item
[**get_news_server_account**](UsenetApi.md#get_news_server_account) | **GET** /v1/api/usenet/provider/account | Get News Server Account
[**get_news_server_connection_information**](UsenetApi.md#get_news_server_connection_information) | **GET** /v1/api/usenet/provider/connection | Get News Server Connection Information
[**get_usenet_cached_availability**](UsenetApi.md#get_usenet_cached_availability) | **GET** /v1/api/usenet/checkcached | Get Usenet Cached Availability
[**get_usenet_cached_availability_batch**](UsenetApi.md#get_usenet_cached_availability_batch) | **POST** /v1/api/usenet/checkcached | Get Usenet Cached Availability Batch
[**get_usenet_list**](UsenetApi.md#get_usenet_list) | **GET** /v1/api/usenet/mylist | Get Usenet List
[**request_usenet_download_link**](UsenetApi.md#request_usenet_download_link) | **GET** /v1/api/usenet/requestdl | Request Download Link
[**request_usenet_download_links_get**](UsenetApi.md#request_usenet_download_links_get) | **GET** /v1/api/usenet/requestdl/batch | Request Download Links - GET
[**request_usenet_download_links_post**](UsenetApi.md#request_usenet_download_links_post) | **POST** /v1/api/usenet/requestdl/batch | Request Download Links - POST
[**reset_news_server_account_password**](UsenetApi.md#reset_news_server_account_password) | **POST** /v1/api/usenet/provider/account/resetpw | Reset News Server Account Password



## async_create_usenet_download

> models::AsyncCreateUsenetDownloadResponse async_create_usenet_download(link, file, name, password, post_processing, as_queued, add_only_if_cached)
Async Create Usenet Download

### Overview  Creates a usenet download asynchronously under your account. Simply send **either** a link, or an nzb file. Once they have been checked, they will begin downloading assuming your account has available active download slots, and they aren't too large.  The request will return instantly, not returning any data other than a success message. This is the same for failures, as they will return the error in the background as a notification, rather than immediately.  The post processing options are the same as for the Create Usenet Download endpoint.  ### Rate Limits  This endpoint is rate limited to **60 requests per hour** per API token.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link** | Option<**String**> | An accessible link to an NZB file. Cannot be a redirection. Optional. |  |
**file** | Option<**std::path::PathBuf**> | An NZB File. Optional. |  |
**name** | Option<**String**> | The name you want the usenet download to be. Optional. |  |
**password** | Option<**String**> | The password to send to the Usenet client. This is used for extracting the RAR at the end. Optional. |  |
**post_processing** | Option<**i64**> | The post processing option to use. Read the overview of the Create Usenet Download endpoint for the options. Default is -1. Optional. |  |[default to -1]
**as_queued** | Option<**bool**> | Tells TorBox you want this usenet download instantly queued. Optional. |  |[default to false]
**add_only_if_cached** | Option<**bool**> | Only adds the download if it is cached on TorBox. If not cached, it won't be added. |  |[default to false]

### Return type

[**models::AsyncCreateUsenetDownloadResponse**](AsyncCreateUsenetDownloadResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## control_usenet_download

> models::ControlUsenetDownloadResponse control_usenet_download(control_usenet_download)
Control Usenet Download

### Overview  Controls a usenet download. By sending the usenet download's ID and the type of operation you want to perform, it will send that request to the usenet client.  Operations are either:  - **Delete** `deletes the nzb from the client and your account permanently`   ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**control_usenet_download** | [**ControlUsenetDownload**](ControlUsenetDownload.md) |  | [required] |

### Return type

[**models::ControlUsenetDownloadResponse**](ControlUsenetDownloadResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_usenet_download

> models::CreateUsenetDownloadResponse create_usenet_download(link, file, name, password, post_processing, as_queued, add_only_if_cached)
Create Usenet Download

### Overview  Creates a usenet download under your account. Simply send **either** a link, or an nzb file. Once they have been checked, they will begin downloading assuming your account has available active download slots, and they aren't too large.  #### Post Processing Options:  All post processing options that the Usenet client will perform before TorBox's own processing to make the files available. It is recommended you either don't send this parameter, or keep it at `-1` for default, which will give only the wanted files.  - `-1`, Default. This runs repairs, and extractions as well as deletes the source files leaving only the wanted downloaded files.  - `0`, None. No post-processing at all. Just download all the files (including all PAR2). TorBox will still do its normal processing to make the download available, but no repairs, or extraction will take place.  - `1`, Repair. Download files and do a PAR2 verification. If the verification fails, download more PAR2 files and attempt to repair the files.  - `2`, Repair and Unpack. Download all files, do a PAR2 verification and unpack the files. The final folder will also include the RAR and ZIP files.  - `3`, Repair, Unpack and Delete. Download all files, do a PAR2 verification, unpack the files to the final folder and delete the source files.   ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link** | Option<**String**> | An accessible link to an NZB file. Cannot be a redirection. Optional. |  |
**file** | Option<**std::path::PathBuf**> | An NZB File. Optional. |  |
**name** | Option<**String**> | The name you want the usenet download to be. Optional. |  |
**password** | Option<**String**> | The password to send to the Usenet client. This is used for extracting the RAR at the end. Optional. |  |
**post_processing** | Option<**i64**> | The post processing option to use. Read the overview for the options. Default is -1. Optional. |  |[default to -1]
**as_queued** | Option<**bool**> | Tells TorBox you want this usenet download.instantly queued. Optional. |  |[default to false]
**add_only_if_cached** | Option<**bool**> | Only adds the download if it is cached on TorBox. If not cached, it won't be added. |  |[default to false]

### Return type

[**models::CreateUsenetDownloadResponse**](CreateUsenetDownloadResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_usenet_item

> models::EditUsenetItemResponse edit_usenet_item(edit_usenet_download)
Edit Usenet Item

### Overview  Updates a download item based on the item's ID. Sending a PUT request with valid items overwrites the previous items entirely, so use this endpoint with caution. Please make sure to read the restrictions below as they are important.  ### Restrictions  - Does not affect the cached database (or any other user's item).  - Name must pass validation:      - Above 1 character long.      - Below 200 characters long.      - Cannot contain any non-url or non filesystem safe characters. (This is removed automatically).      - Cannot contain any leading or trailing whitespace. You can have spaces in the middle. (This is removed automatically).  - Tags follow the same rules as the name.  - Alternative hashes must be of MD5, SHA1, or SHA256.   ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edit_usenet_download** | [**EditUsenetDownload**](EditUsenetDownload.md) |  | [required] |

### Return type

[**models::EditUsenetItemResponse**](EditUsenetItemResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_news_server_account

> models::GetNewsServerAccountResponse get_news_server_account()
Get News Server Account

### Overview  Gets the user's TorBox News Server Account with authentication details. Does not show password if not creating the account. Account creation happens on the first request. If you need to get the password, or reset the password, use the `/usenet/provider/account/resetpw` `POST` endpoint.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetNewsServerAccountResponse**](GetNewsServerAccountResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_news_server_connection_information

> models::GetNewsServerConnectionInformationResponse get_news_server_connection_information()
Get News Server Connection Information

### Overview  Gets the TorBox News Server Connection information. This is a static path, and is not dynamic.  ### Authorization  No authentication required. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetNewsServerConnectionInformationResponse**](GetNewsServerConnectionInformationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usenet_cached_availability

> models::GetUsenetCachedAvailabilityResponse get_usenet_cached_availability(hash, format, list_files, check_cached)
Get Usenet Cached Availability

### Overview  Takes in a list of comma separated usenet hashes and checks if the usenet download is cached. This endpoint only gets a max of around 100 at a time, due to http limits in queries. If you want to do more, you can simply do more hash queries. Such as: `?hash=XXXX&hash=XXXX&hash=XXXX`  or `?hash=XXXX,XXXX&hash=XXXX&hash=XXXX,XXXX` and this will work too. Performance is very fast. Less than 1 second per 100. Time is approximately O(log n) time for those interested in taking it to its max. That is without caching as well. This endpoint stores a cache for an hour.  You may also pass a `format` parameter with the format you want the data in. Options are either `object` or `list`. You can view examples of both below.  To get the hash of a usenet download, you can read [this document](https://support.torbox.app/en/articles/13681109-technical-getting-hashes-for-searches) for all of the strategies.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | Option<[**Vec<String>**](String.md)> | The list of usenet hashes you want to check. Comma seperated. To find the hash, md5 the link of the usenet link or file. |  |
**format** | Option<**String**> | Format you want the data in. Acceptable is either \"object\" or \"list\". List is the most performant option as it doesn't require modification of the list. |  |[default to object]
**list_files** | Option<**bool**> | Allows you to list the files found inside the cached data. Optional. |  |[default to false]
**check_cached** | Option<[**CheckCached**](CheckCached.md)> |  |  |

### Return type

[**models::GetUsenetCachedAvailabilityResponse**](GetUsenetCachedAvailabilityResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usenet_cached_availability_batch

> models::GetUsenetCachedAvailabilityBatchResponse get_usenet_cached_availability_batch(hash, format, list_files, check_cached)
Get Usenet Cached Availability Batch

### Overview  Takes in a list of comma separated hashes and checks if the torrent is cached. With the POST endpoint you can submit a nearly unlimited hashes compared to the GET endpoint which is limited by url parameters.  Performance is very fast. Less than 1 second per 100. Time is approximately O(log n) time for those interested in taking it to its max. That is without caching as well. This endpoint stores a cache for an hour.  You may also pass a `format` parameter with the format you want the data in. Options are either `object` or `list`. You can view examples of both below.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | Option<[**Vec<String>**](String.md)> |  |  |
**format** | Option<**String**> |  |  |[default to object]
**list_files** | Option<**bool**> |  |  |[default to false]
**check_cached** | Option<[**CheckCached**](CheckCached.md)> |  |  |

### Return type

[**models::GetUsenetCachedAvailabilityBatchResponse**](GetUsenetCachedAvailabilityBatchResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usenet_list

> models::GetUsenetListResponse get_usenet_list(id, bypass_cache, offset, limit, files, active)
Get Usenet List

### Overview  Gets the user's usenet download list. This gives you the needed information to perform other usenet actions. Unlike Torrents, this information is updated on its own every 5 seconds for live usenet downloads.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i64**> | Determines the usenet download requested, will return an object rather than list. Optional. |  |
**bypass_cache** | Option<**bool**> | Allows you to bypass the cached data, and always get fresh information. Useful if constantly querying for fresh download stats. Otherwise, we request that you save our database a few calls. |  |[default to false]
**offset** | Option<**i64**> | Determines the offset of items to get from the database. Default is 0. Optional. |  |[default to 0]
**limit** | Option<**i64**> | Determines the number of items to recieve per request. Default is 1000. Optional. |  |[default to 1000]
**files** | Option<**bool**> | Exclude files from response. Optional |  |[default to true]
**active** | Option<**bool**> | Shows only active downloads. Optional |  |

### Return type

[**models::GetUsenetListResponse**](GetUsenetListResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_usenet_download_link

> models::RequestUsenetDownloadLinkResponse request_usenet_download_link(token, usenet_id, file_id, zip_link, user_ip, redirect, append_name)
Request Download Link

### Overview  Requests the download link from the server. Because downloads are metered, TorBox cannot afford to allow free access to the links directly. This endpoint opens the link for 3 hours for downloads. Once a download is started, the user has nearly unlilimited time to download the file. The 1 hour time limit is simply for starting downloads. This prevents long term link sharing.  ### Permalinks  Instead of generating many CDN urls by requesting this endpoint, you can instead create a permalink such as: `https://api.torbox.app/v1/api/torrents/requestdl?token=APIKEY&torrent_id=NUMBER&file_id=NUMBER&redirect=true` and when a user clicks on it, it will automatically redirect them to the CDN link. This saves requests and doesn't abuse the API. Use this method rather than saving CDN links as they are not permanent. To invalidate these permalinks, simply reset your API token or delete the item from your dashboard.  ### Authorization  Requires an API key as a parameter for the `token` parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Your API Key | [required] |
**usenet_id** | **i64** | The usenet download's ID that you want to download | [required] |
**file_id** | Option<**i64**> | The files's ID that you want to download. Optional if using \"zip_link\". |  |[default to 0]
**zip_link** | Option<**bool**> | If you want a zip link. Required if no file_id. Takes precedence over file_id if both are given. |  |[default to false]
**user_ip** | Option<**String**> | The user's IPv4 to determine the closest CDN. Optional. |  |
**redirect** | Option<**bool**> | If you want to redirect the user to the CDN link. This is useful for creating permalinks so that you can just make this request URL the link. |  |[default to false]
**append_name** | Option<**bool**> | If you want to add the \"filename\" parameter to the returned CDN link. Optional. Default false. |  |[default to false]

### Return type

[**models::RequestUsenetDownloadLinkResponse**](RequestUsenetDownloadLinkResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_usenet_download_links_get

> models::RequestUsenetDownloadLinksGetResponse request_usenet_download_links_get(token, usenet_id, file_ids, user_ip, append_name, request_download_batch)
Request Download Links - GET

### Overview  Requests multiple download links from the server. Because downloads are metered, TorBox cannot afford to allow free access to the links directly. This endpoint opens the link for 3 hours for downloads. Once a download is started, the user has nearly unlilimited time to download the file. The 1 hour time limit is simply for starting downloads. This prevents long term link sharing.  ### Authorization  Requires an API key as a parameter for the `token` parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Your API Key | [required] |
**usenet_id** | **i64** | The usenet download's ID that you want to download | [required] |
**file_ids** | Option<[**Vec<i64>**](I64.md)> | The files's ID that you want to download |  |
**user_ip** | Option<**String**> | The user's IP to determine the closest CDN. Optional. |  |
**append_name** | Option<**bool**> | If you want to add the \"filename\" parameter to the returned CDN link. Optional. Default false. |  |[default to false]
**request_download_batch** | Option<[**RequestDownloadBatch**](RequestDownloadBatch.md)> |  |  |

### Return type

[**models::RequestUsenetDownloadLinksGetResponse**](RequestUsenetDownloadLinksGetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_usenet_download_links_post

> models::RequestUsenetDownloadLinksPostResponse request_usenet_download_links_post(token, usenet_id, file_ids, user_ip, append_name, request_download_batch)
Request Download Links - POST

### Overview  Requests multiple download links from the server. Because downloads are metered, TorBox cannot afford to allow free access to the links directly. This endpoint opens the link for 3 hours for downloads. Once a download is started, the user has nearly unlilimited time to download the file. The 1 hour time limit is simply for starting downloads. This prevents long term link sharing.  ### Authorization  Requires an API key as a parameter for the `token` parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Your API Key | [required] |
**usenet_id** | **i64** | The usenet downloads' ID that you want to download | [required] |
**file_ids** | Option<[**Vec<i64>**](I64.md)> |  |  |
**user_ip** | Option<**String**> | The user's IP to determine the closest CDN. Optional. |  |
**append_name** | Option<**bool**> | If you want to add the \"filename\" parameter to the returned CDN link. Optional. Default false. |  |[default to false]
**request_download_batch** | Option<[**RequestDownloadBatch**](RequestDownloadBatch.md)> |  |  |

### Return type

[**models::RequestUsenetDownloadLinksPostResponse**](RequestUsenetDownloadLinksPostResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_news_server_account_password

> models::ResetNewsServerAccountPasswordResponse reset_news_server_account_password()
Reset News Server Account Password

### Overview  Resets the user's TorBox News Server account password. This password is automatically generated, and is not saved, it is only shown once.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ResetNewsServerAccountPasswordResponse**](ResetNewsServerAccountPasswordResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

