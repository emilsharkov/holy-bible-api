
# BibleVerse


## Properties

Name | Type
------------ | -------------
`bibleId` | number
`book` | number
`chapter` | number
`text` | string
`verse` | number

## Example

```typescript
import type { BibleVerse } from ''

// TODO: Update the object below with actual values
const example = {
  "bibleId": null,
  "book": null,
  "chapter": null,
  "text": null,
  "verse": null,
} satisfies BibleVerse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as BibleVerse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


