# \TagsApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**control_tag**](TagsApi.md#control_tag) | **POST** /v1/api/tags/controltag | Control Tag
[**edit_tag**](TagsApi.md#edit_tag) | **PUT** /v1/api/tags/edittag | Edit Tag
[**get_tags**](TagsApi.md#get_tags) | **GET** /v1/api/tags/mytags | Get User Tags



## control_tag

> models::ControlTagResponse control_tag(control_tag)
Control Tag

### Overview  Controls a tag. Currently, the only operation is \"delete\", which deletes the tag and removes it from every download using it; the downloads stay in your account and are otherwise unmodified. The ID is the tag's ID from `/mytags` endpoint.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**control_tag** | [**ControlTag**](ControlTag.md) |  | [required] |

### Return type

[**models::ControlTagResponse**](ControlTagResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_tag

> models::EditTagResponse edit_tag(edit_tag)
Edit Tag

### Overview  Renames a tag. This new name is case sensitive. The ID is the tag's ID from the `/mytags`. Fails with `DUPLICATE_ITEM` when another of the user's tags already has the same name, in any casing.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**edit_tag** | [**EditTag**](EditTag.md) |  | [required] |

### Return type

[**models::EditTagResponse**](EditTagResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> models::GetTagsResponse get_tags()
Get User Tags

### Overview  Gets the tags attached to a user account, with the downloads using each tag, their count and total size.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetTagsResponse**](GetTagsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

