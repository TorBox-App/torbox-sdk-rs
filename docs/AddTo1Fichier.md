# AddTo1Fichier

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | Id of the torrent, usenet download or web download to upload. It has to be one of your own downloads. Required. | 
**file_id** | Option<**i64**> | Id of the file to upload, from the download's files. Leave out when `zip` is true. | [optional]
**zip** | Option<**bool**> | Upload the whole download as one zip instead of a single file. Takes precedence over `file_id`. Optional, default false. | [optional][default to false]
**r#type** | Option<**String**> | Which kind of download `id` refers to. One of `torrent`, `usenet` or `webdownload`. Optional, default `torrent`. | [optional][default to torrent]
**onefichier_token** | Option<**String**> | 1Fichier API token of the account to upload to. It is issued by 1Fichier, not by TorBox. Optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


