# holy-bible-api.com
## About
This is an Open-Source API for serving the Holy Bible written in Rust. The API serves text Bibles in 100+ languages with 800+ versions. It also serves audio Bibles in 40+ languages. This API is also OpenAPI compliant, meaning that the goal is to create [auto-generated](https://openapi-generator.tech/docs/generators) SDKs in the future.

## OpenAPI Docs
https://holy-bible-api.com/docs

## OpenAPI Clients
### TypeScript SDK
```sh
npm install holy-bible-api
```
https://www.npmjs.com/package/holy-bible-api

### Python SDK
```sh
pip install holy_bible_api
```
https://pypi.org/project/holy_bible_api/

### Rust SDK
```sh
cargo add holy_bible_api
```
https://crates.io/crates/holy_bible_api

## Self Hosting via Docker Compose
### Requirements
- S3 Bucket
- Virutal Machine (preferably running amd64 linux otherwise you will need to rebuild the api image)

### In the VM execute the following:
1. Download Docker and Clone Repo
    ```bash
    curl -fsSL https://get.docker.com | sh
    cd ~
    git clone https://github.com/emilsharkov/holy-bible-api
    ```

2. Update Configs
    ```bash
    cd holy-bible-api/docker
    cp .env.example .env # Fill it out everything except beszel env vars
    nano Caddyfile # Replace holy-bible-api.com with your Host/IP
    docker compose up -d
    ```

3. Setup Beszel for Resource Monitoring
    1. Go to https://yourhost/beszel
    2. Create your account
    3. Click on add system in the top right corner
    4. Fill in `Name` with a name of your choosing
    5. Fill in `Host/IP` with your HOST/IP
    6. Copy the value in `Public Key` and `Token` into your `.env` in `BESZEL_PUBLIC_KEY` and `BESZEL_TOKEN`

5. Reset Docker Compose
    ```bash
    docker compose down
    docker compose up -d
    ```


## Data Source Disclaimer
This project is open-source meaning that each user that wants to run this project needs to provide their own data to the API. The data I am using has been source from the [Word Project](https://www.wordproject.org/bibles/audio/index.htm) for the audio and [Holy-Bible-JSON](https://github.com/emilsharkov/Holy-Bible-JSON) and [Holy-Bible-XML](https://github.com/Beblia/Holy-Bible-XML-Format) for the text.

Please double check this data with the real thing before using it. This data has been first parsed from another data source then again reparsed by me. This could cause there to be mistakes so please use the API with caution. If any are open to helping contribute to the verification of the data sources, it would be much appreciated.
