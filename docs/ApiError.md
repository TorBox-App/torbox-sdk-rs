# ApiError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** | False when the request failed. A list with nothing to return answers `ITEM_NOT_FOUND` with `success` true: the request worked, there is no data. | 
**error** | **String** | The error code, such as `BAD_TOKEN`. Listed in the introduction. | 
**detail** | **String** | What went wrong, in words. | 
**data** | Option<**serde_json::Value**> | Extra data for some errors, usually null. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


