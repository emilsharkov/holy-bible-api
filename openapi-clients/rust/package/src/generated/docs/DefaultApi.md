# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_audio_bible_books**](DefaultApi.md#get_audio_bible_books) | **GET** /audio_bibles/{audio_bible_id}/books | 
[**get_audio_bible_chapters**](DefaultApi.md#get_audio_bible_chapters) | **GET** /audio_bibles/{audio_bible_id}/books/{book_num}/chapters | 
[**get_audio_bibles**](DefaultApi.md#get_audio_bibles) | **GET** /audio_bibles | 
[**get_audio_chapter**](DefaultApi.md#get_audio_chapter) | **GET** /audio_bibles/{audio_bible_id}/books/{book_num}/chapters/{chapter_num} | 
[**get_bible_books**](DefaultApi.md#get_bible_books) | **GET** /bibles/{bible_id}/books | 
[**get_bible_chapters**](DefaultApi.md#get_bible_chapters) | **GET** /bibles/{bible_id}/books/{book_num}/chapters | 
[**get_bible_verse_by_number**](DefaultApi.md#get_bible_verse_by_number) | **GET** /bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses/{verse_num} | 
[**get_bible_verses**](DefaultApi.md#get_bible_verses) | **GET** /bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses | 
[**get_bibles**](DefaultApi.md#get_bibles) | **GET** /bibles | 
[**get_health**](DefaultApi.md#get_health) | **GET** /health | 



## get_audio_bible_books

> models::GetAudioBooksRes get_audio_bible_books(audio_bible_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audio_bible_id** | **i32** |  | [required] |

### Return type

[**models::GetAudioBooksRes**](GetAudioBooksRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_bible_chapters

> models::GetAudioChaptersRes get_audio_bible_chapters(audio_bible_id, book_num)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audio_bible_id** | **i32** |  | [required] |
**book_num** | **i32** |  | [required] |

### Return type

[**models::GetAudioChaptersRes**](GetAudioChaptersRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_bibles

> models::GetAudioBiblesRes get_audio_bibles(language, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | Option<**String**> |  |  |
**version** | Option<**String**> |  |  |

### Return type

[**models::GetAudioBiblesRes**](GetAudioBiblesRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audio_chapter

> get_audio_chapter(audio_bible_id, book_num, chapter_num)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audio_bible_id** | **i32** |  | [required] |
**book_num** | **i32** |  | [required] |
**chapter_num** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/mpeg, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bible_books

> models::GetBibleBooksRes get_bible_books(bible_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bible_id** | **i32** |  | [required] |

### Return type

[**models::GetBibleBooksRes**](GetBibleBooksRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bible_chapters

> models::GetBibleChaptersRes get_bible_chapters(bible_id, book_num)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bible_id** | **i32** |  | [required] |
**book_num** | **i32** |  | [required] |

### Return type

[**models::GetBibleChaptersRes**](GetBibleChaptersRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bible_verse_by_number

> models::BibleVerse get_bible_verse_by_number(bible_id, book_num, chapter_num, verse_num)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bible_id** | **i32** |  | [required] |
**book_num** | **i32** |  | [required] |
**chapter_num** | **i32** |  | [required] |
**verse_num** | **i32** |  | [required] |

### Return type

[**models::BibleVerse**](BibleVerse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bible_verses

> models::GetBibleVersesRes get_bible_verses(bible_id, book_num, chapter_num, start, end)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bible_id** | **i32** |  | [required] |
**book_num** | **i32** |  | [required] |
**chapter_num** | **i32** |  | [required] |
**start** | Option<**i32**> |  |  |
**end** | Option<**i32**> |  |  |

### Return type

[**models::GetBibleVersesRes**](GetBibleVersesRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bibles

> models::GetBiblesRes get_bibles(language, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | Option<**String**> |  |  |
**version** | Option<**String**> |  |  |

### Return type

[**models::GetBiblesRes**](GetBiblesRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_health

> String get_health()


### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

