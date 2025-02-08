# bible-api
## About
This is an Open-Source API for serving the Holy Bible written in Rust. The API serves text Bibles in 100+ languages with 800+ versions. It also serves audio Bibles in 50+ languages. This API is also OpenAPI compliant, meaning that the goal is to create [auto-generated](https://openapi-generator.tech/docs/generators) SDKs in Java, Python, TypeScript, JavaScript, Go, Ruby, PHP, Kotlin, C#/.NET

## Data Sources 
### Disclaimer
This project is open-source meaning that each user that wants to run this project needs to provide their own data to the API. The data I am using has been source from the [Word Project](https://www.wordproject.org/bibles/audio/index.htm) for the audio and [Holy-Bible-JSON](https://github.com/emilsharkov/Holy-Bible-JSON) and [Holy-Bible-XML](https://github.com/Beblia/Holy-Bible-XML-Format) for the text.

Please double check this data with the real thing before using it. This data has been first parsed by me then parsed before me an unknown amount of times. This could cause there to be mistakes so please use the APIs and SDKs with caution. If any are open to helping contribute to the verification of the data sources, it would be much appreciated.

## Usage


## To Do List
- [x] Set up project structure
- [x] Get Data into Data Sources
- [x] Implement API
- [x] Test API
- [ ] Infrastructure
- [ ] Generate SDKs
- [ ] CICD
- [ ] Frontend Docs?
