# AddToDropbox

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | Id of the torrent, usenet download or web download. Still required by the route, but no longer used. | 
**file_id** | Option<**i64**> | Id of the file inside the download. No longer used. | [optional]
**zip** | Option<**bool**> | Whether to upload the whole download as one zip. No longer used. | [optional][default to false]
**r#type** | Option<**String**> | Which kind of download `id` refers to. One of `torrent`, `usenet` or `webdownload`. Optional, default `torrent`. Still checked before the route answers. | [optional][default to torrent]
**dropbox_token** | **String** | Dropbox OAuth access token. Still required by the route, but no longer used. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


