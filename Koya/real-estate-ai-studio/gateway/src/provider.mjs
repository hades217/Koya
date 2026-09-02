export function createOpenAiImageProvider({ apiKey, endpoint = 'https://api.openai.com/v1/images/generations', fetchImpl = fetch }) {
  if (!apiKey) return { async generate() { throw new Error('OPENAI_API_KEY is unavailable.'); } };
  return {
    async generate({ model, prompt, parameters, outputCount }) {
      if (arguments[0]?.inputs?.length) throw new Error('The configured OpenAI generation endpoint does not accept reference inputs; configure an audited input-capable adapter before submission.');
      const response = await fetchImpl(endpoint, { method: 'POST', headers: { Authorization: `Bearer ${apiKey}`, 'Content-Type': 'application/json' }, body: JSON.stringify({ model, prompt, n: outputCount, ...parameters }) });
      if (!response.ok) throw new Error(`OpenAI image request failed with HTTP ${response.status}.`);
      const payload = await response.json();
      const outputs = (payload.data ?? []).map((item) => ({ b64Json: item.b64_json, revisedPrompt: item.revised_prompt ?? null }));
      if (outputs.length !== outputCount || outputs.some((item) => !item.b64Json)) throw new Error('OpenAI returned an incomplete image result.');
      return { providerRequestId: response.headers.get('x-request-id'), outputs, usage: payload.usage ?? null };
    }
  };
}
