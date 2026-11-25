# \StreamApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_stream**](StreamApi.md#create_stream) | **GET** /v1/api/stream/createstream | Create Stream
[**get_stream_data**](StreamApi.md#get_stream_data) | **GET** /v1/api/stream/getstreamdata | Get Stream Data



## create_stream

> models::CreateStreamResponse create_stream(id, file_id, r#type, chosen_subtitle_index, chosen_audio_index, chosen_resolution_index, scrobbling_enabled)
Create Stream

### Overview  Creates a new stream based off of the data provided.  #### Selecting Subtitles and Audios  We use indexes to determine which audio and subtitle you would like to select. To view all available indexes, you must request this endpoint intially with \"null\" or 0, for the chosen indexes. This will return a metadata dictionary in the response, which will contain the list of audios and subtitles available for this stream. When choosing a subtitle or audio stream, use the **relative index**, starting from 0, rather than the \"index\" value. This is important for selecting the correct index. Once your user gets a selection of audios and subtitles available, you can submit this to the endpoint again to get the chosen stream.  - **null** - Selecting no subtitle. Not available for audio as one must always be selected.  - **0** - Selecting first subtitle or audio   #### Selecting Resolution  Similar to the subtitles and audios, we use indexes to determine the resolution chosen. Unlike the above, these are predetermined. We do not upscale video, so if for example, you pass a 1080p video, and request it with resolution, 7, it will stay at 1080p. Please see the below for options.  - **null** - Original video resolution. No downscaling will take place.  - **0** - 144p  - **1** - 240p  - **2** - 360p  - **3** - 480p  - **4** - 720p  - **5** - 1080p  - **6** - 1440p  - **7** - 2160p   #### Types  This endpoint allows you to handle torrents, usenet and web downloads all in one. This is why the \"type\" parameter is available. It defaults to \"torrent\".  - **torrent** - Torrents  - **usenet** - Usenet downloads  - **webdownload** - Web Downloads   #### Important Notes  - Only text based subtitles are supported. If a non-text based subtitle is selected, it will be ignored.  - If a subtitle or audio index does not exist on the stream, it will be ignored.  - There are a lot of edge-case videos, if something doesn't work, don't hesitate to report the download hash and file_id of the failing video file.  - This is a **Pro** only feature.   ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the type of download. | [required] |
**file_id** | Option<**i64**> | The file ID inside of the download you would like to stream. |  |[default to 0]
**r#type** | Option<**String**> | The type of download that this is. If choosing the incorrect one, the ID will be incorrect. |  |[default to torrent]
**chosen_subtitle_index** | Option<**String**> | The relative index of the subtitle you would like to stream. This is default null for none. 0 is the first subtitle found. |  |
**chosen_audio_index** | Option<**i64**> | The relative index of the subtitle you would like to stream. Defaults to 0 for the first audio stream found inside the file. Cannot be null. |  |[default to 0]
**chosen_resolution_index** | Option<**String**> | The index of the resolution you would like to stream. This is default null for original. |  |
**scrobbling_enabled** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::CreateStreamResponse**](CreateStreamResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stream_data

> models::GetStreamDataResponse get_stream_data(token, presigned_token, chosen_subtitle_index, chosen_audio_index, chosen_resolution_index)
Get Stream Data

### Overview  Gets stream data from an already created stream. Requires the presigned token and user API token from the original request.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | The user's API token, can also be used from the original, /createstream request. | [required] |
**presigned_token** | **String** | The presigned token given from the original, /createstream request. | [required] |
**chosen_subtitle_index** | Option<**String**> | The chosen subtitle index that you are requesting information about. |  |
**chosen_audio_index** | Option<**i64**> | The chosen audio index that you are requesting information about. |  |[default to 0]
**chosen_resolution_index** | Option<**String**> | The index of the resolution you would like to stream. This is default null for original. |  |

### Return type

[**models::GetStreamDataResponse**](GetStreamDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

