module github.com/emilsharkov/holy-bible-api-demo

go 1.23

require github.com/emilsharkov/holy-bible-api/openapi-clients/go/package v0.0.0

require github.com/emilsharkov/holy-bible-api/openapi-clients/go/package/generated v0.0.0 // indirect

replace github.com/emilsharkov/holy-bible-api/openapi-clients/go/package/generated => ../package/generated
