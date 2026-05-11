export interface StorageArea {
  get(keys: string[] | Record<string, unknown> | string | null): Promise<Record<string, unknown>>;
  set(items: Record<string, unknown>): Promise<void>;
}

export interface RuntimeMessageSender {
  tab?: { id?: number };
}

export interface RuntimeApi {
  onInstalled: {
    addListener(listener: (details: { reason: string }) => void): void;
  };
  onMessage: {
    addListener(
      listener: (
        message: Record<string, unknown>,
        sender: RuntimeMessageSender,
        sendResponse: (response: Record<string, unknown>) => void,
      ) => boolean | void,
    ): void;
  };
  sendMessage(message: Record<string, unknown>): Promise<Record<string, unknown>>;
}

export interface RuleAction {
  type: string;
}

export interface RuleCondition {
  urlFilter?: string;
  resourceTypes?: string[];
}

export interface DynamicRule {
  id: number;
  priority: number;
  action: RuleAction;
  condition: RuleCondition;
}

export interface DeclarativeNetRequestApi {
  getDynamicRules(): Promise<DynamicRule[]>;
  updateDynamicRules(details: { addRules?: DynamicRule[]; removeRuleIds?: number[] }): Promise<void>;
}

export interface ChromeApi {
  runtime: RuntimeApi;
  storage: { local: StorageArea };
  declarativeNetRequest: DeclarativeNetRequestApi;
}

declare const chrome: ChromeApi;

export { chrome };
