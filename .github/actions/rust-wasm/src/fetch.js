const fetch = await import("node-fetch");
global.fetch = fetch.default;
global.Headers = fetch.Headers;
global.Request = fetch.Request;
global.Response = fetch.Response;
