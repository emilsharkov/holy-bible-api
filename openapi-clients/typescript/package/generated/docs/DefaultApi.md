# DefaultApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**getAudioBibleBooks**](DefaultApi.md#getaudiobiblebooks) | **GET** /audio_bibles/{audio_bible_id}/books |  |
| [**getAudioBibleChapters**](DefaultApi.md#getaudiobiblechapters) | **GET** /audio_bibles/{audio_bible_id}/books/{book_num}/chapters |  |
| [**getAudioBibles**](DefaultApi.md#getaudiobibles) | **GET** /audio_bibles |  |
| [**getAudioChapter**](DefaultApi.md#getaudiochapter) | **GET** /audio_bibles/{audio_bible_id}/books/{book_num}/chapters/{chapter_num} |  |
| [**getBibleBooks**](DefaultApi.md#getbiblebooks) | **GET** /bibles/{bible_id}/books |  |
| [**getBibleChapters**](DefaultApi.md#getbiblechapters) | **GET** /bibles/{bible_id}/books/{book_num}/chapters |  |
| [**getBibleVerseByNumber**](DefaultApi.md#getbibleversebynumber) | **GET** /bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses/{verse_num} |  |
| [**getBibleVerses**](DefaultApi.md#getbibleverses) | **GET** /bibles/{bible_id}/books/{book_num}/chapters/{chapter_num}/verses |  |
| [**getBibles**](DefaultApi.md#getbibles) | **GET** /bibles |  |
| [**getHealth**](DefaultApi.md#gethealth) | **GET** /health |  |
| [**getRandomBibleVerse**](DefaultApi.md#getrandombibleverse) | **GET** /bibles/{bible_id}/random |  |
| [**getVerseOfTheDay**](DefaultApi.md#getverseoftheday) | **GET** /bibles/{bible_id}/verse-of-the-day |  |



## getAudioBibleBooks

> BooksCountResponse getAudioBibleBooks(audioBibleId)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetAudioBibleBooksRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    audioBibleId: 56,
  } satisfies GetAudioBibleBooksRequest;

  try {
    const data = await api.getAudioBibleBooks(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **audioBibleId** | `number` |  | [Defaults to `undefined`] |

### Return type

[**BooksCountResponse**](BooksCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAudioBibleChapters

> ChaptersCountResponse getAudioBibleChapters(audioBibleId, bookNum)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetAudioBibleChaptersRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    audioBibleId: 56,
    // number
    bookNum: 56,
  } satisfies GetAudioBibleChaptersRequest;

  try {
    const data = await api.getAudioBibleChapters(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **audioBibleId** | `number` |  | [Defaults to `undefined`] |
| **bookNum** | `number` |  | [Defaults to `undefined`] |

### Return type

[**ChaptersCountResponse**](ChaptersCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAudioBibles

> Array&lt;AudioBible&gt; getAudioBibles(language, version)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetAudioBiblesRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // string (optional)
    language: language_example,
    // string (optional)
    version: version_example,
  } satisfies GetAudioBiblesRequest;

  try {
    const data = await api.getAudioBibles(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **language** | `string` |  | [Optional] [Defaults to `undefined`] |
| **version** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;AudioBible&gt;**](AudioBible.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getAudioChapter

> getAudioChapter(audioBibleId, bookNum, chapterNum)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetAudioChapterRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    audioBibleId: 56,
    // number
    bookNum: 56,
    // number
    chapterNum: 56,
  } satisfies GetAudioChapterRequest;

  try {
    const data = await api.getAudioChapter(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **audioBibleId** | `number` |  | [Defaults to `undefined`] |
| **bookNum** | `number` |  | [Defaults to `undefined`] |
| **chapterNum** | `number` |  | [Defaults to `undefined`] |

### Return type

`void` (Empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `audio/mpeg`, `text/plain`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Returns the audio chapter file |  -  |
| **404** | Audio Chapter not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getBibleBooks

> BooksCountResponse getBibleBooks(bibleId)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetBibleBooksRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    bibleId: 56,
  } satisfies GetBibleBooksRequest;

  try {
    const data = await api.getBibleBooks(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **bibleId** | `number` |  | [Defaults to `undefined`] |

### Return type

[**BooksCountResponse**](BooksCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getBibleChapters

> ChaptersCountResponse getBibleChapters(bibleId, bookNum)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetBibleChaptersRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    bibleId: 56,
    // number
    bookNum: 56,
  } satisfies GetBibleChaptersRequest;

  try {
    const data = await api.getBibleChapters(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **bibleId** | `number` |  | [Defaults to `undefined`] |
| **bookNum** | `number` |  | [Defaults to `undefined`] |

### Return type

[**ChaptersCountResponse**](ChaptersCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getBibleVerseByNumber

> BibleVerse getBibleVerseByNumber(bibleId, bookNum, chapterNum, verseNum)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetBibleVerseByNumberRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    bibleId: 56,
    // number
    bookNum: 56,
    // number
    chapterNum: 56,
    // number
    verseNum: 56,
  } satisfies GetBibleVerseByNumberRequest;

  try {
    const data = await api.getBibleVerseByNumber(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **bibleId** | `number` |  | [Defaults to `undefined`] |
| **bookNum** | `number` |  | [Defaults to `undefined`] |
| **chapterNum** | `number` |  | [Defaults to `undefined`] |
| **verseNum** | `number` |  | [Defaults to `undefined`] |

### Return type

[**BibleVerse**](BibleVerse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getBibleVerses

> Array&lt;BibleVerse&gt; getBibleVerses(bibleId, bookNum, chapterNum, start, end)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetBibleVersesRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    bibleId: 56,
    // number
    bookNum: 56,
    // number
    chapterNum: 56,
    // number (optional)
    start: 56,
    // number (optional)
    end: 56,
  } satisfies GetBibleVersesRequest;

  try {
    const data = await api.getBibleVerses(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **bibleId** | `number` |  | [Defaults to `undefined`] |
| **bookNum** | `number` |  | [Defaults to `undefined`] |
| **chapterNum** | `number` |  | [Defaults to `undefined`] |
| **start** | `number` |  | [Optional] [Defaults to `undefined`] |
| **end** | `number` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;BibleVerse&gt;**](BibleVerse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getBibles

> Array&lt;Bible&gt; getBibles(language, version)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetBiblesRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // string (optional)
    language: language_example,
    // string (optional)
    version: version_example,
  } satisfies GetBiblesRequest;

  try {
    const data = await api.getBibles(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **language** | `string` |  | [Optional] [Defaults to `undefined`] |
| **version** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;Bible&gt;**](Bible.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getHealth

> string getHealth()



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetHealthRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  try {
    const data = await api.getHealth();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `text/plain`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getRandomBibleVerse

> BibleVerse getRandomBibleVerse(bibleId)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetRandomBibleVerseRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    bibleId: 56,
  } satisfies GetRandomBibleVerseRequest;

  try {
    const data = await api.getRandomBibleVerse(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **bibleId** | `number` |  | [Defaults to `undefined`] |

### Return type

[**BibleVerse**](BibleVerse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## getVerseOfTheDay

> BibleVerse getVerseOfTheDay(bibleId)



### Example

```ts
import {
  Configuration,
  DefaultApi,
} from '';
import type { GetVerseOfTheDayRequest } from '';

async function example() {
  console.log("🚀 Testing  SDK...");
  const api = new DefaultApi();

  const body = {
    // number
    bibleId: 56,
  } satisfies GetVerseOfTheDayRequest;

  try {
    const data = await api.getVerseOfTheDay(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **bibleId** | `number` |  | [Defaults to `undefined`] |

### Return type

[**BibleVerse**](BibleVerse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

