const BASE = "/api";

async function request<T>(path: string): Promise<T> {
  const res = await fetch(`${BASE}${path}`);
  if (!res.ok) throw new Error(`${res.status} ${res.statusText}`);
  return res.json() as Promise<T>;
}

interface HealthResponse {
  status: string;
}

interface HelloResponse {
  message: string;
}

export async function checkHealth(): Promise<boolean> {
  try {
    const data = await request<HealthResponse>("/health");
    return data.status === "ok";
  } catch {
    return false;
  }
}

export async function hello(name?: string): Promise<string> {
  const query = name ? `?name=${encodeURIComponent(name)}` : "";
  const data = await request<HelloResponse>(`/hello${query}`);
  return data.message;
}
