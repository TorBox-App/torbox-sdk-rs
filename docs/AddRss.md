# AddRss

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | URL of the rss feed you want to add. | 
**name** | **String** | Display name of the rss feed you want to add. | 
**do_regex** | Option<**String**> | Regex of items you want to make sure are added. Optional. | [optional]
**dont_regex** | Option<**String**> | Regex of items you dont want added. Optional. | [optional]
**dont_older_than** | Option<**i64**> | Time in days that tells the parser to skip if the torrent is older than that many days. Optional. | [optional]
**pass_check** | Option<**bool**> | Boolean which tells to allow multiple of the same url if it already exists on your account | [optional][default to false]
**scan_interval** | Option<**i64**> | Time in minutes with the minimum being 10 which you want the api to scan in interval. Optional. Default is 60. | [optional][default to 60]
**rss_type** | Option<**String**> | The type of downloads inside of this RSS feed. Optional. Default is torrent. Options are \"torrent\", \"usenet\", \"webdl\" | [optional][default to torrent]
**torrent_seeding** | Option<**i64**> | Determines if torrents inside of this RSS feed will seed or not. Optional. Default is 1, for auto. Options are 1 for auto, 2 for always, and 3 for never. | [optional][default to 1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


