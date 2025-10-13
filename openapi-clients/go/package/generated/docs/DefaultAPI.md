# \DefaultAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetAudioBibleBooks**](DefaultAPI.md#GetAudioBibleBooks) | **Get** /audio_bibles/{audio_bible_id}/books | 
[**GetAudioBibleChapters**](DefaultAPI.md#GetAudioBibleChapters) | **Get** /audio_bibles/{audio_bible_id}/books/{book_num}/chapters | 
[**GetAudioBibles**](DefaultAPI.md#GetAudioBibles) | **Get** /audio_bibles | 
[**GetAudioChapter**](DefaultAPI.md#GetAudioChapter) | **Get** /audio_bibles/{audio_bible_id}/books/{book_num}/chapters/{chapter_num} | 
[**GetBibleBooks**](DefaultAPI.md#GetBibleBooks) | **Get** /bibles/{bible_id}/books | 
[**GetBibleChapters**](DefaultAPI.md#GetBibleChapters) | **Get** /bibles/{bible_id}/books/{book_num}/chapters | 
[**GetBibleVerseByNumber**](DefaultAPI.md#GetBibleVerseByNumber) | **Get** /bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses/{verse_num} | 
[**GetBibleVerses**](DefaultAPI.md#GetBibleVerses) | **Get** /bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses | 
[**GetBibles**](DefaultAPI.md#GetBibles) | **Get** /bibles | 
[**GetHealth**](DefaultAPI.md#GetHealth) | **Get** /health | 



## GetAudioBibleBooks

> GetAudioBooksRes GetAudioBibleBooks(ctx, audioBibleId).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	audioBibleId := int32(56) // int32 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetAudioBibleBooks(context.Background(), audioBibleId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAudioBibleBooks``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAudioBibleBooks`: GetAudioBooksRes
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAudioBibleBooks`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**audioBibleId** | **int32** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetAudioBibleBooksRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**GetAudioBooksRes**](GetAudioBooksRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetAudioBibleChapters

> GetAudioChaptersRes GetAudioBibleChapters(ctx, audioBibleId, bookNum).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	audioBibleId := int32(56) // int32 | 
	bookNum := int32(56) // int32 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetAudioBibleChapters(context.Background(), audioBibleId, bookNum).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAudioBibleChapters``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAudioBibleChapters`: GetAudioChaptersRes
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAudioBibleChapters`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**audioBibleId** | **int32** |  | 
**bookNum** | **int32** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetAudioBibleChaptersRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



### Return type

[**GetAudioChaptersRes**](GetAudioChaptersRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetAudioBibles

> GetAudioBiblesRes GetAudioBibles(ctx).Language(language).Version(version).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	language := "language_example" // string |  (optional)
	version := "version_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetAudioBibles(context.Background()).Language(language).Version(version).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAudioBibles``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAudioBibles`: GetAudioBiblesRes
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetAudioBibles`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiGetAudioBiblesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **string** |  | 
 **version** | **string** |  | 

### Return type

[**GetAudioBiblesRes**](GetAudioBiblesRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetAudioChapter

> GetAudioChapter(ctx, audioBibleId, bookNum, chapterNum).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	audioBibleId := int32(56) // int32 | 
	bookNum := int32(56) // int32 | 
	chapterNum := int32(56) // int32 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.DefaultAPI.GetAudioChapter(context.Background(), audioBibleId, bookNum, chapterNum).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetAudioChapter``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**audioBibleId** | **int32** |  | 
**bookNum** | **int32** |  | 
**chapterNum** | **int32** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetAudioChapterRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------




### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: audio/mpeg, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetBibleBooks

> GetBibleBooksRes GetBibleBooks(ctx, bibleId).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	bibleId := int32(56) // int32 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetBibleBooks(context.Background(), bibleId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetBibleBooks``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetBibleBooks`: GetBibleBooksRes
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetBibleBooks`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**bibleId** | **int32** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetBibleBooksRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**GetBibleBooksRes**](GetBibleBooksRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetBibleChapters

> GetBibleChaptersRes GetBibleChapters(ctx, bibleId, bookNum).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	bibleId := int32(56) // int32 | 
	bookNum := int32(56) // int32 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetBibleChapters(context.Background(), bibleId, bookNum).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetBibleChapters``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetBibleChapters`: GetBibleChaptersRes
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetBibleChapters`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**bibleId** | **int32** |  | 
**bookNum** | **int32** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetBibleChaptersRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



### Return type

[**GetBibleChaptersRes**](GetBibleChaptersRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetBibleVerseByNumber

> BibleVerse GetBibleVerseByNumber(ctx, bibleId, bookNum, chapterNum, verseNum).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	bibleId := int32(56) // int32 | 
	bookNum := int32(56) // int32 | 
	chapterNum := int32(56) // int32 | 
	verseNum := int32(56) // int32 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetBibleVerseByNumber(context.Background(), bibleId, bookNum, chapterNum, verseNum).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetBibleVerseByNumber``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetBibleVerseByNumber`: BibleVerse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetBibleVerseByNumber`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**bibleId** | **int32** |  | 
**bookNum** | **int32** |  | 
**chapterNum** | **int32** |  | 
**verseNum** | **int32** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetBibleVerseByNumberRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------





### Return type

[**BibleVerse**](BibleVerse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetBibleVerses

> GetBibleVersesRes GetBibleVerses(ctx, bibleId, bookNum, chapterNum).Start(start).End(end).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	bibleId := int32(56) // int32 | 
	bookNum := int32(56) // int32 | 
	chapterNum := int32(56) // int32 | 
	start := int32(56) // int32 |  (optional)
	end := int32(56) // int32 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetBibleVerses(context.Background(), bibleId, bookNum, chapterNum).Start(start).End(end).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetBibleVerses``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetBibleVerses`: GetBibleVersesRes
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetBibleVerses`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**bibleId** | **int32** |  | 
**bookNum** | **int32** |  | 
**chapterNum** | **int32** |  | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetBibleVersesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **start** | **int32** |  | 
 **end** | **int32** |  | 

### Return type

[**GetBibleVersesRes**](GetBibleVersesRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetBibles

> GetBiblesRes GetBibles(ctx).Language(language).Version(version).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	language := "language_example" // string |  (optional)
	version := "version_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetBibles(context.Background()).Language(language).Version(version).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetBibles``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetBibles`: GetBiblesRes
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetBibles`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiGetBiblesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **language** | **string** |  | 
 **version** | **string** |  | 

### Return type

[**GetBiblesRes**](GetBiblesRes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetHealth

> string GetHealth(ctx).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetHealth(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetHealth``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetHealth`: string
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetHealth`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiGetHealthRequest struct via the builder pattern


### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

