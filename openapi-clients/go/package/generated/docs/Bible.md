# Bible

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BibleId** | **int32** |  | 
**Language** | **string** |  | 
**Version** | Pointer to **NullableString** |  | [optional] 

## Methods

### NewBible

`func NewBible(bibleId int32, language string, ) *Bible`

NewBible instantiates a new Bible object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBibleWithDefaults

`func NewBibleWithDefaults() *Bible`

NewBibleWithDefaults instantiates a new Bible object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBibleId

`func (o *Bible) GetBibleId() int32`

GetBibleId returns the BibleId field if non-nil, zero value otherwise.

### GetBibleIdOk

`func (o *Bible) GetBibleIdOk() (*int32, bool)`

GetBibleIdOk returns a tuple with the BibleId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBibleId

`func (o *Bible) SetBibleId(v int32)`

SetBibleId sets BibleId field to given value.


### GetLanguage

`func (o *Bible) GetLanguage() string`

GetLanguage returns the Language field if non-nil, zero value otherwise.

### GetLanguageOk

`func (o *Bible) GetLanguageOk() (*string, bool)`

GetLanguageOk returns a tuple with the Language field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLanguage

`func (o *Bible) SetLanguage(v string)`

SetLanguage sets Language field to given value.


### GetVersion

`func (o *Bible) GetVersion() string`

GetVersion returns the Version field if non-nil, zero value otherwise.

### GetVersionOk

`func (o *Bible) GetVersionOk() (*string, bool)`

GetVersionOk returns a tuple with the Version field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVersion

`func (o *Bible) SetVersion(v string)`

SetVersion sets Version field to given value.

### HasVersion

`func (o *Bible) HasVersion() bool`

HasVersion returns a boolean if a field has been set.

### SetVersionNil

`func (o *Bible) SetVersionNil(b bool)`

 SetVersionNil sets the value for Version to be an explicit nil

### UnsetVersion
`func (o *Bible) UnsetVersion()`

UnsetVersion ensures that no value is present for Version, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


