package holybibleapi

import (
	openapi "github.com/emilsharkov/holy-bible-api/openapi-clients/go/package/generated"
)

func CreateBibleApi(basePath ...string) *openapi.APIClient {
	cfg := openapi.NewConfiguration()

	url := ""
	if len(basePath) > 0 {
		url = basePath[0]
	}

	cfg.Servers = openapi.ServerConfigurations{
		{
			URL: url,
		},
	}
	return openapi.NewAPIClient(cfg)
}
