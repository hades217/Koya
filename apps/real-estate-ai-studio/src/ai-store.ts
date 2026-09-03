import { invoke } from '@tauri-apps/api/core';
import type { AiProjectUpdateDraft } from './types';

export type AiStatus = {
  provider: 'codex';
  installed: boolean;
  authenticated: boolean;
  available: boolean;
  version?: string;
  accountLabel?: string;
  bindingProtocol?: string;
  authMode?: string;
  planType?: string;
  canLaunchLogin: boolean;
  detail: string;
};

export type AiHistoryMessage = { role: 'assistant' | 'user'; text: string };
export type AiChatResponse = { provider: 'codex'; content: string; projectUpdateDraft?: AiProjectUpdateDraft };

function isTauri() {
  return Boolean(window.__TAURI_INTERNALS__);
}

export async function checkAiStatus(): Promise<AiStatus> {
  if (!isTauri()) {
    return { provider: 'codex', installed: false, authenticated: false, available: false, canLaunchLogin: false, detail: 'AI binding is available in the installed desktop app.' };
  }
  return invoke<AiStatus>('check_ai_status');
}

export async function openCodexLogin(): Promise<void> {
  if (!isTauri()) throw new Error('Open Estate Studio desktop to bind Codex.');
  await invoke('open_codex_login');
}

export async function chatWithCodex(projectId: string | undefined, message: string, history: AiHistoryMessage[]) {
  if (!isTauri()) throw new Error('Property AI chat is available in the installed desktop app.');
  return invoke<AiChatResponse>('chat_with_codex', { input: { projectId, message, history } });
}
