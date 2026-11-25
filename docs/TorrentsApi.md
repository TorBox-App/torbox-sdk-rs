# \TorrentsApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**async_create_torrent**](TorrentsApi.md#async_create_torrent) | **POST** /v1/api/torrents/asynccreatetorrent | Async Create Torrent
[**control_queued_torrents**](TorrentsApi.md#control_queued_torrents) | **POST** /v1/api/torrents/controlqueued | Control Queued Torrents
[**control_torrent**](TorrentsApi.md#control_torrent) | **POST** /v1/api/torrents/controltorrent | Control Torrent
[**create_torrent**](TorrentsApi.md#create_torrent) | **POST** /v1/api/torrents/createtorrent | Create Torrent
[**edit_torrent_item**](TorrentsApi.md#edit_torrent_item) | **PUT** /v1/api/torrents/edittorrent | Edit Torrent Item
[**export_torrent_data**](TorrentsApi.md#export_torrent_data) | **GET** /v1/api/torrents/exportdata | Export Torrent Data
[**get_queued_torrents**](TorrentsApi.md#get_queued_torrents) | **GET** /v1/api/torrents/getqueued | Get Queued Torrents
[**get_torrent_cached_availability**](TorrentsApi.md#get_torrent_cached_availability) | **GET** /v1/api/torrents/checkcached | Get Torrent Cached Availability
[**get_torrent_cached_availability_batch**](TorrentsApi.md#get_torrent_cached_availability_batch) | **POST** /v1/api/torrents/checkcached | Get Torrent Cached Availability Batch
[**get_torrent_info**](TorrentsApi.md#get_torrent_info) | **GET** /v1/api/torrents/torrentinfo | Get Torrent Info
[**get_torrent_info_from_file**](TorrentsApi.md#get_torrent_info_from_file) | **POST** /v1/api/torrents/torrentinfo | Get Torrent Info
[**get_torrent_list**](TorrentsApi.md#get_torrent_list) | **GET** /v1/api/torrents/mylist | Get Torrent List
[**magnet_to_file**](TorrentsApi.md#magnet_to_file) | **POST** /v1/api/torrents/magnettofile | Magnet To File
[**request_torrent_download_link**](TorrentsApi.md#request_torrent_download_link) | **GET** /v1/api/torrents/requestdl | Request Download Link
[**request_torrent_download_links_get**](TorrentsApi.md#request_torrent_download_links_get) | **GET** /v1/api/torrents/requestdl/batch | Request Download Links - GET
[**request_torrent_download_links_post**](TorrentsApi.md#request_torrent_download_links_post) | **POST** /v1/api/torrents/requestdl/batch | Request Download Links - POST



## async_create_torrent

> models::AsyncCreateTorrentResponse async_create_torrent(file, magnet, seed, allow_zip, name, as_queued, add_only_if_cached)
Async Create Torrent

### Overview  Creates a torrent asynchrously under your account. Simply send **either** a magnet link, or a torrent file. Once they have been checked, they will begin downloading assuming your account has available active download slots, and they aren't too large.  The request will return instantly, not returning any data other than a success message. This is the same for failures, as they will return the error in the background, rather than immediately.  ### Rate Limits  This endpoint has the same limits as the Create Torrent endpoint: **300 requests per minute**, and **60 uncached torrents per hour**. The hourly limit is shared between the two endpoints, so an uncached torrent added through either one counts against the same 60. As on that endpoint, a request sent with `add_only_if_cached` and without `as_queued` is not counted against the hourly limit.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> | The torrent's torrent file. Optional. |  |
**magnet** | Option<**String**> | The torrent's magnet link. Optional. |  |
**seed** | Option<**i64**> | Tells TorBox your preference for seeding this torrent. 1 is auto. 2 is seed. 3 is don't seed. Optional. Default is 1, or whatever the user has in their settings. Overwrites option in settings. |  |
**allow_zip** | Option<**bool**> | Tells TorBox if you want to allow this torrent to be zipped or not. TorBox only zips if the torrent is 100 files or larger. |  |[default to true]
**name** | Option<**String**> | The name you want the torrent to be. Optional. |  |
**as_queued** | Option<**bool**> | Tells TorBox you want this torrent instantly queued. This is bypassed if user is on free plan, and will process the request as normal in this case. Optional. |  |[default to false]
**add_only_if_cached** | Option<**bool**> | Only adds the download if it is cached on TorBox. If not cached, it won't be added. |  |[default to false]

### Return type

[**models::AsyncCreateTorrentResponse**](AsyncCreateTorrentResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## control_queued_torrents

> control_queued_torrents()
Control Queued Torrents

### Overview  This endpoint is deprecated and always answers with a `DEPRECATED` error. Use the Control Queued Downloads endpoint (`/v1/api/queued/controlqueued`) instead.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## control_torrent

> models::ControlTorrentResponse control_torrent(control_torrent)
Control Torrent

### Overview  Controls a torrent. By sending the torrent's ID and the type of operation you want to perform, it will send that request to the torrent client.  Operations are either:  - **Reannounce** `reannounces the torrent to get new peers`  - **Delete** `deletes the torrent from the client and your account permanently`  - **Resume** `resumes a paused torrent`   ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**control_torrent** | [**ControlTorrent**](ControlTorrent.md) |  | [required] |

### Return type

[**models::ControlTorrentResponse**](ControlTorrentResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_torrent

> models::CreateTorrentResponse create_torrent(file, magnet, seed, allow_zip, name, as_queued, add_only_if_cached)
Create Torrent

### Overview  Creates a torrent under your account. Simply send **either** a magnet link, or a torrent file. Once they have been checked, they will begin downloading assuming your account has available active download slots, and they aren't too large.  ### Rate Limits  This endpoint has two limits that apply at the same time:  - **300 requests per minute:** a shared ceiling that _every_ request counts against, whether the torrent is cached or uncached.  - **60 uncached torrents per hour:** an additional, stricter limit that only uncached torrents count against. A request sent with `add_only_if_cached` is not counted, because it adds nothing when the torrent is not cached. It is counted when it is also sent with `as_queued`, because a queued torrent is added whether it is cached or not.   The important thing is that cached and uncached requests both draw from the same 300/min pool, but uncached requests _also_ draw from their own separate 60/hour pool. An uncached request therefore counts twice: once against the per-minute limit, and once against the per-hour uncached limit.  **Example:** In a single minute you add 250 cached + 50 uncached torrents. That's 300 total, so you've hit the per-minute limit for that minute. But those 50 uncached torrents also came out of your 60/hour uncached budget, leaving only **10 uncached torrents** for the remaining 59 minutes of that hour. (You could still add plenty more _cached_ torrents during that hour, since cached requests aren't capped hourly: only by the 300/min ceiling.)  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> | The torrent's torrent file. Optional. |  |
**magnet** | Option<**String**> | The torrent's magnet link. Optional. |  |
**seed** | Option<**i64**> | Tells TorBox your preference for seeding this torrent. 1 is auto. 2 is seed. 3 is don't seed. Optional. Default is 1, or whatever the user has in their settings. Overwrites option in settings. |  |
**allow_zip** | Option<**bool**> | Tells TorBox if you want to allow this torrent to be zipped or not. TorBox only zips if the torrent is 100 files or larger. |  |[default to true]
**name** | Option<**String**> | The name you want the torrent to be. Optional. |  |
**as_queued** | Option<**bool**> | Tells TorBox you want this torrent instantly queued. This is bypassed if user is on free plan, and will process the request as normal in this case. Optional. |  |[default to false]
**add_only_if_cached** | Option<**bool**> | Only adds the download if it is cached on TorBox. If not cached, it won't be added. |  |[default to false]

### Return type

[**models::CreateTorrentResponse**](CreateTorrentResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_torrent_item

> models::EditTorrentItemResponse edit_torrent_item(edit_torrent)
Edit Torrent Item

### Overview  Updates a download item based on the item's ID. Sending a PUT request with valid items overwrites the previous items entirely, so use this endpoint with caution. Please make sure to read the restrictions below as they are important.  ### Restrictions  - Does not affect the cached database (or any other user's item).  - Name must pass validation:      - Above 1 character long.      - Below 200 characters long.      - Cannot contain any non-url or non filesystem safe characters. (This is removed automatically).      - Cannot contain any leading or trailing whitespace. You can have spaces in the middle. (This is removed automatically).  - Tags follow the same rules as the name.  - Alternative hashes must be of MD5, SHA1, or SHA256.   ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edit_torrent** | [**EditTorrent**](EditTorrent.md) |  | [required] |

### Return type

[**models::EditTorrentItemResponse**](EditTorrentItemResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_torrent_data

> models::ExportTorrentDataResponse export_torrent_data(torrent_id, r#type)
Export Torrent Data

### Overview  Exports the magnet or torrent file. Requires a type to be passed. If type is **magnet**, it will return a JSON response with the magnet as a string in the _data_ key. If type is **file**, it will return a bittorrent file as a download. Not compatible with cached downloads.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**torrent_id** | **i64** | The torrent's ID. | [required] |
**r#type** | **String** | Either \"magnet\" or \"file\". Tells how the API what to get, and what to respond as. If magnet, it returns a JSON response with the magnet as a string in the data key. If file, it responds with a torrent file download. | [required] |

### Return type

[**models::ExportTorrentDataResponse**](ExportTorrentDataResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-bittorrent

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queued_torrents

> get_queued_torrents()
Get Queued Torrents

### Overview  This endpoint is deprecated and always answers with a `DEPRECATED` error. Use the Get Queued Downloads endpoint (`/v1/api/queued/getqueued`) instead.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_torrent_cached_availability

> models::GetTorrentCachedAvailabilityResponse get_torrent_cached_availability(hash, format, list_files, check_cached)
Get Torrent Cached Availability

### Overview  Takes in a list of comma separated torrent hashes and checks if the torrent is cached. This endpoint only gets a max of around 100 at a time, due to http limits in queries. If you want to do more, you can simply do more hash queries. Such as: `?hash=XXXX&hash=XXXX&hash=XXXX`  or `?hash=XXXX,XXXX&hash=XXXX&hash=XXXX,XXXX` and this will work too. Performance is very fast. Less than 1 second per 100. Time is approximately O(log n) time for those interested in taking it to its max. That is without caching as well. This endpoint stores a cache for an hour.  You may also pass a `format` parameter with the format you want the data in. Options are either `object` or `list`. You can view examples of both below.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | Option<[**Vec<String>**](String.md)> | The list of torrent hashes you want to check. Comma seperated. |  |
**format** | Option<**String**> | Format you want the data in. Acceptable is either \"object\" or \"list\". List is the most performant option as it doesn't require modification of the list. |  |[default to object]
**list_files** | Option<**bool**> | Allows you to list the files found inside the cached data. |  |[default to false]
**check_cached** | Option<[**CheckCached**](CheckCached.md)> |  |  |

### Return type

[**models::GetTorrentCachedAvailabilityResponse**](GetTorrentCachedAvailabilityResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_torrent_cached_availability_batch

> models::GetTorrentCachedAvailabilityBatchResponse get_torrent_cached_availability_batch(hash, format, list_files, check_cached)
Get Torrent Cached Availability Batch

### Overview  Takes in a list of comma separated torrent hashes and checks if the torrent is cached. With the POST endpoint you can submit a nearly unlimited hashes compared to the GET endpoint which is limited by url parameters.   Performance is very fast. Less than 1 second per 100. Time is approximately O(log n) time for those interested in taking it to its max. That is without caching as well. This endpoint stores a cache for an hour.  You may also pass a `format` parameter with the format you want the data in. Options are either `object` or `list`. You can view examples of both below.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | Option<[**Vec<String>**](String.md)> |  |  |
**format** | Option<**String**> |  |  |[default to object]
**list_files** | Option<**bool**> |  |  |[default to false]
**check_cached** | Option<[**CheckCached**](CheckCached.md)> |  |  |

### Return type

[**models::GetTorrentCachedAvailabilityBatchResponse**](GetTorrentCachedAvailabilityBatchResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_torrent_info

> models::GetTorrentInfoResponse get_torrent_info(hash, timeout, use_cache_lookup)
Get Torrent Info

### Overview  A general route that allows you to give a hash, and TorBox will return data about the torrent. This data is retrieved from the Bittorrent network, so expect it to take some time. If the request goes longer than 10 seconds, TorBox will cancel it. You can adjust this if you like, but the default is 10 seconds. This route is cached as well, so subsequent requests will be instant.  **It is not recommended to use this for checking TorBox's cache. Please use the GET /checkcache endpoint instead.**  ### Authorization  None required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | Hash of the torrent you want to get info for. This is required. | [required] |
**timeout** | Option<**i64**> | The amount of time you want TorBox to search for the torrent on the Bittorrent network. If the number of seeders is low or even zero, this value may be helpful to move up. Default is 10. Optional. |  |[default to 30]
**use_cache_lookup** | Option<**bool**> | If you want to use a pre-cached request if sent before. This can save a lot of time. Default false. |  |[default to false]

### Return type

[**models::GetTorrentInfoResponse**](GetTorrentInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_torrent_info_from_file

> models::GetTorrentInfoFromFileResponse get_torrent_info_from_file(timeout, hash, magnet, file, use_cache_lookup, peers_only)
Get Torrent Info

### Overview  Same as the GET route, but allows posting data such as magnet, and torrent files.  Hashes will have precedence over magnets, and magnets will have precedence over torrent files.  Only proper torrent files are accepted.  At least one of hash, magnet, or torrent file is required.  A general route that allows you to give a hash, and TorBox will return data about the torrent. This data is retrieved from the Bittorrent network, so expect it to take some time. If the request goes longer than 10 seconds, TorBox will cancel it. You can adjust this if you like, but the default is 10 seconds. This route is cached as well, so subsequent requests will be instant.  **It is not recommended to use this for checking TorBox's cache. Please use the GET /checkcache endpoint instead.**  ### Authorization  None required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timeout** | Option<**i64**> | The amount of time you want TorBox to search for the torrent on the Bittorrent network. If the number of seeders is low or even zero, this value may be helpful to move up. Default is 10. Optional. |  |[default to 10]
**hash** | Option<**String**> | A torrent hash for requesting information about the torrent. Optional. |  |
**magnet** | Option<**String**> | A magnet link for requesting information abouit the torrent. Optional. |  |
**file** | Option<**std::path::PathBuf**> | A torrent file for requesting information about the torrent. Optional. |  |
**use_cache_lookup** | Option<**bool**> | If you want to use a pre-cached request if sent before. This can save a lot of time. Default false. |  |[default to false]
**peers_only** | Option<**bool**> | Uses a quick lookup method using the trackers contained within the magnet or file. Requires the submitted magnet or torrent file to contain trackers of UDP or HTTP(S) to work. This will not get metadata such as name, size, or files. Best used for simply and quickly gathering the current seeds and peers according to the trackers. Uses the highest number of all the trackers as the final value. Default off. |  |[default to false]

### Return type

[**models::GetTorrentInfoFromFileResponse**](GetTorrentInfoFromFileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_torrent_list

> models::GetTorrentListResponse get_torrent_list(id, bypass_cache, offset, limit, files, active)
Get Torrent List

### Overview  Gets the user's torrent list. This gives you the needed information to perform other torrent actions. This information only gets updated every 600 seconds, or when the _Request Update On Torrent_ request is sent to the _relay API_.  #### Download States:  - \"downloading\" -> The torrent is currently downloading.  - \"uploading\" -> The torrent is currently seeding.  - \"stalled (no seeds)\" -> The torrent is trying to download, but there are no seeds connected to download from.  - \"paused\" -> The torrent is paused.  - \"completed\" -> The torrent is completely downloaded. Do not use this for download completion status.  - \"cached\" -> The torrent is cached from the server.  - \"metaDL\" -> The torrent is downloading metadata from the hoard.  - \"checkingResumeData\" -> The torrent is checking resumable data.   All other statuses are basic qBittorrent states. [Check out here for the full list](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1)#torrent-management).  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**i64**> | Determines the torrent requested, will return an object rather than list. Optional. |  |
**bypass_cache** | Option<**bool**> | Allows you to bypass the cached data, and always get fresh information. Useful if constantly querying for fresh download stats. Otherwise, we request that you save our database a few calls. |  |[default to false]
**offset** | Option<**i64**> | Determines the offset of items to get from the database. Default is 0. Optional. |  |[default to 0]
**limit** | Option<**i64**> | Determines the number of items to recieve per request. Default is 1000. Optional. |  |[default to 1000]
**files** | Option<**bool**> | Exclude files from response. Optional |  |[default to true]
**active** | Option<**bool**> | Shows only active downloads. Optional |  |

### Return type

[**models::GetTorrentListResponse**](GetTorrentListResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## magnet_to_file

> String magnet_to_file(magnet_to_torrent)
Magnet To File

### Overview  Converts any magnet to a torrent file. Does not require the magnet to be on your TorBox account or in the TorBox cache. Free endpoint for public use.  ### Authorization  None required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**magnet_to_torrent** | [**MagnetToTorrent**](MagnetToTorrent.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/x-bittorrent, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_torrent_download_link

> models::RequestTorrentDownloadLinkResponse request_torrent_download_link(token, torrent_id, file_id, zip_link, user_ip, redirect, append_name)
Request Download Link

### Overview  Requests the download link from the server. Because downloads are metered, TorBox cannot afford to allow free access to the links directly. This endpoint opens the link for 3 hours for downloads. Once a download is started, the user has nearly unlilimited time to download the file. The 1 hour time limit is simply for starting downloads. This prevents long term link sharing.  ### Permalinks  Instead of generating many CDN urls by requesting this endpoint, you can instead create a permalink such as: `https://api.torbox.app/v1/api/torrents/requestdl?token=APIKEY&torrent_id=NUMBER&file_id=NUMBER&redirect=true` and when a user clicks on it, it will automatically redirect them to the CDN link. This saves requests and doesn't abuse the API. Use this method rather than saving CDN links as they are not permanent. To invalidate these permalinks, simply reset your API token or delete the item from your dashboard.  ### Authorization  Requires an API key as a parameter for the `token` parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Your API Key | [required] |
**torrent_id** | **i64** | The torrent's ID that you want to download | [required] |
**file_id** | Option<**i64**> | The files's ID that you want to download |  |[default to 0]
**zip_link** | Option<**bool**> | If you want a zip link. Required if no file_id. Takes precedence over file_id if both are given. |  |[default to false]
**user_ip** | Option<**String**> | The user's IP to determine the closest CDN. Optional. |  |
**redirect** | Option<**bool**> | If you want to redirect the user to the CDN link. This is useful for creating permalinks so that you can just make this request URL the link. |  |[default to false]
**append_name** | Option<**bool**> | If you want to add the \"filename\" parameter to the returned CDN link. Optional. Default false. |  |[default to false]

### Return type

[**models::RequestTorrentDownloadLinkResponse**](RequestTorrentDownloadLinkResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_torrent_download_links_get

> models::RequestTorrentDownloadLinksGetResponse request_torrent_download_links_get(token, torrent_id, file_ids, user_ip, append_name, request_download_batch)
Request Download Links - GET

### Overview  Requests multiple download links from the server. Because downloads are metered, TorBox cannot afford to allow free access to the links directly. This endpoint opens the link for 3 hours for downloads. Once a download is started, the user has nearly unlilimited time to download the file. The 1 hour time limit is simply for starting downloads. This prevents long term link sharing.  ### Authorization  Requires an API key as a parameter for the `token` parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Your API Key | [required] |
**torrent_id** | **i64** | The torrent's ID that you want to download | [required] |
**file_ids** | Option<[**Vec<i64>**](I64.md)> | The files's ID that you want to download |  |
**user_ip** | Option<**String**> | The user's IP to determine the closest CDN. Optional. |  |
**append_name** | Option<**bool**> | If you want to add the \"filename\" parameter to the returned CDN link. Optional. Default false. |  |[default to false]
**request_download_batch** | Option<[**RequestDownloadBatch**](RequestDownloadBatch.md)> |  |  |

### Return type

[**models::RequestTorrentDownloadLinksGetResponse**](RequestTorrentDownloadLinksGetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_torrent_download_links_post

> models::RequestTorrentDownloadLinksPostResponse request_torrent_download_links_post(token, torrent_id, file_ids, user_ip, append_name, request_download_batch)
Request Download Links - POST

### Overview  Requests multiple download links from the server. Because downloads are metered, TorBox cannot afford to allow free access to the links directly. This endpoint opens the link for 3 hours for downloads. Once a download is started, the user has nearly unlilimited time to download the file. The 1 hour time limit is simply for starting downloads. This prevents long term link sharing.  ### Authorization  Requires an API key as a parameter for the `token` parameter. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Your API Key | [required] |
**torrent_id** | **i64** | The torrent's ID that you want to download | [required] |
**file_ids** | Option<[**Vec<i64>**](I64.md)> |  |  |
**user_ip** | Option<**String**> | The user's IP to determine the closest CDN. Optional. |  |
**append_name** | Option<**bool**> | If you want to add the \"filename\" parameter to the returned CDN link. Optional. Default false. |  |[default to false]
**request_download_batch** | Option<[**RequestDownloadBatch**](RequestDownloadBatch.md)> |  |  |

### Return type

[**models::RequestTorrentDownloadLinksPostResponse**](RequestTorrentDownloadLinksPostResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

