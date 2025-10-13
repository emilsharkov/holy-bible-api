# AudioBible

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AudioBibleId** | **int32** |  | 
**Language** | **string** |  | 
**Version** | Pointer to **NullableString** |  | [optional] 

## Methods

### NewAudioBible

`func NewAudioBible(audioBibleId int32, language string, ) *AudioBible`

NewAudioBible instantiates a new AudioBible object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAudioBibleWithDefaults

`func NewAudioBibleWithDefaults() *AudioBible`

NewAudioBibleWithDefaults instantiates a new AudioBible object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAudioBibleId

`func (o *AudioBible) GetAudioBibleId() int32`

GetAudioBibleId returns the AudioBibleId field if non-nil, zero value otherwise.

### GetAudioBibleIdOk

`func (o *AudioBible) GetAudioBibleIdOk() (*int32, bool)`

GetAudioBibleIdOk returns a tuple with the AudioBibleId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAudioBibleId

`func (o *AudioBible) SetAudioBibleId(v int32)`

SetAudioBibleId sets AudioBibleId field to given value.


### GetLanguage

`func (o *AudioBible) GetLanguage() string`

GetLanguage returns the Language field if non-nil, zero value otherwise.

### GetLanguageOk

`func (o *AudioBible) GetLanguageOk() (*string, bool)`

GetLanguageOk returns a tuple with the Language field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLanguage

`func (o *AudioBible) SetLanguage(v string)`

SetLanguage sets Language field to given value.


### GetVersion

`func (o *AudioBible) GetVersion() string`

GetVersion returns the Version field if non-nil, zero value otherwise.

### GetVersionOk

`func (o *AudioBible) GetVersionOk() (*string, bool)`

GetVersionOk returns a tuple with the Version field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVersion

`func (o *AudioBible) SetVersion(v string)`

SetVersion sets Version field to given value.

### HasVersion

`func (o *AudioBible) HasVersion() bool`

HasVersion returns a boolean if a field has been set.

### SetVersionNil

`func (o *AudioBible) SetVersionNil(b bool)`

 SetVersionNil sets the value for Version to be an explicit nil

### UnsetVersion
`func (o *AudioBible) UnsetVersion()`

UnsetVersion ensures that no value is present for Version, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


