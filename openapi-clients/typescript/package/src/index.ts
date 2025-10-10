
import { DefaultApi, Configuration } from "../generated/index.js";

export function createBibleApi(basePath: string = "https://holy-bible-api.com") {
  return new DefaultApi(new Configuration({ basePath }));
}

export * from "../generated/index.js";
