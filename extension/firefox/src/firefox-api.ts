export interface BrowserRuntimeApi {
  onMessage: {
    addListener(listener: (message: Record<string, unknown>, sender: unknown) => unknown): void;
  };
  sendMessage(message: Record<string, unknown>): Promise<Record<string, unknown>>;
}

export interface BrowserWebRequestApi {
  onBeforeRequest: {
    addListener(
      listener: (details: { url: string; type: string }) => { cancel?: boolean } | void,
      filter: { urls: string[] },
      extraInfoSpec: string[],
    ): void;
  };
}

export interface BrowserStorageArea {
  get(keys: string[] | Record<string, unknown> | string | null): Promise<Record<string, unknown>>;
  set(items: Record<string, unknown>): Promise<void>;
}

export interface BrowserApi {
  runtime: BrowserRuntimeApi;
  webRequest: BrowserWebRequestApi;
  storage: { local: BrowserStorageArea };
}

declare const browser: BrowserApi;

export { browser };
