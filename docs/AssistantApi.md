# \AssistantApi

All URIs are relative to *https://api.torbox.app*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_with_assistant**](AssistantApi.md#chat_with_assistant) | **POST** /v1/api/assistant/chat | Chat With Assistant
[**delete_assistant_conversation**](AssistantApi.md#delete_assistant_conversation) | **DELETE** /v1/api/assistant/conversations/{session_id} | Delete Assistant Conversation
[**delete_assistant_conversations**](AssistantApi.md#delete_assistant_conversations) | **DELETE** /v1/api/assistant/conversations | Delete All Assistant Conversations
[**get_assistant_usage**](AssistantApi.md#get_assistant_usage) | **GET** /v1/api/assistant/usage | Get Assistant Usage
[**list_assistant_conversations**](AssistantApi.md#list_assistant_conversations) | **GET** /v1/api/assistant/conversations | Get Assistant Conversations
[**read_assistant_conversation**](AssistantApi.md#read_assistant_conversation) | **GET** /v1/api/assistant/conversations/{session_id} | Get Assistant Conversation



## chat_with_assistant

> String chat_with_assistant(chat_request)
Chat With Assistant

### Overview  Send a message to the AI Assistant. Optionally include a `session_id` to resume a previous conversation. If no `session_id` is given, the AI will have no previous context and will create a new conversation.  Also included is a way to upload `files` in base64 allowing you to attach a file that the AI Assistant can use.  We also allow you to send `blocks` which is a list of strings that state what additional features your chat interface includes. If no `blocks` are sent, responses are sent in plain text and markdown (for things like tables or bolding text).  The answer is streamed as plain text while it is written. Send the header `Accept: text/event-stream` to get it as server-sent events instead: `tool` with the `name` of each tool the assistant uses, `text` with each piece of the answer, `support` with the `id` of the support conversation when the chat is handed to the support team, `usage` with the share of the day's allowance that is `used` and when it `resets_at`, `error` with the error if the answer fails, and `done` at the end.  ``` event: tool data: {\"name\": \"get_torrent_list\"}  event: text data: {\"text\": \"You have 2 torrents, \"}  event: text data: {\"text\": \"and both have finished downloading.\"}  event: usage data: {\"used\": 0.0123, \"resets_at\": \"2026-01-01T00:00:00Z\"}  event: done data: {} ```  The `X-Session-ID` response header holds the `session_id` to send with the next message.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_request** | [**ChatRequest**](ChatRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_assistant_conversation

> models::DeleteAssistantConversationResponse delete_assistant_conversation(session_id)
Delete Assistant Conversation

### Overview  Deletes a specific conversation with the AI assistant. Usage persists from the conversation.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session you want to retrieve. | [required] |

### Return type

[**models::DeleteAssistantConversationResponse**](DeleteAssistantConversationResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_assistant_conversations

> models::DeleteAssistantConversationsResponse delete_assistant_conversations()
Delete All Assistant Conversations

### Overview  Deletes all conversations for a user's account. Usage persists from the conversations.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DeleteAssistantConversationsResponse**](DeleteAssistantConversationsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assistant_usage

> models::GetAssistantUsageResponse get_assistant_usage()
Get Assistant Usage

### Overview  Gets the current user's AI assistant usage. Users are limited to a certain number of tokens daily, and this endpoint tells you exactly how much is left as a percentage. This makes it easy for developers so they don't have to spend time counting tokens, which is usually inaccurate.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAssistantUsageResponse**](GetAssistantUsageResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_assistant_conversations

> models::ListAssistantConversationsResponse list_assistant_conversations()
Get Assistant Conversations

### Overview  Gets the user's open conversations. Conversations are sorted newest first. Each conversation is kept for 30 days after its last message.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListAssistantConversationsResponse**](ListAssistantConversationsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_assistant_conversation

> models::ReadAssistantConversationResponse read_assistant_conversation(session_id, since)
Get Assistant Conversation

### Overview  Get a specific user conversation with the AI assistant.  You can pass the `since` parameter with a Unix timestamp, and will show any revisions since that time, this way, you don't waste bandwidth retrieving and updating the entire conversation. If no update has been made since, the data return will be null.  ### Authorization  Requires an API key using the Authorization Bearer Header. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | The session you want to retrieve. | [required] |
**since** | Option<**i64**> | Unix timestamp of the conversation you want to get since this time. |  |

### Return type

[**models::ReadAssistantConversationResponse**](ReadAssistantConversationResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

