import { clearSession, getToken } from "./auth";

const API_BASE = "/api/v1";

interface ApiResponse<T> {
  code: number;
  message: string;
  data?: T | null;
}

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
  const token = getToken();
  const headers: HeadersInit = {
    "Content-Type": "application/json",
    ...(options.headers || {})
  };
  if (token) {
    headers.Authorization = `Bearer ${token}`;
  }

  const response = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers
  });

  if (response.status === 401) {
    clearSession();
    throw new Error("unauthorized");
  }

  const payload = (await response.json()) as ApiResponse<T>;
  if (payload.code !== 0) {
    throw new Error(payload.message || "request_failed");
  }

  return payload.data as T;
}

export function apiGet<T>(path: string): Promise<T> {
  return request<T>(path, { method: "GET" });
}

export function apiPost<T, R>(path: string, body: T): Promise<R> {
  return request<R>(path, {
    method: "POST",
    body: JSON.stringify(body)
  });
}

export function apiPut<T, R>(path: string, body: T): Promise<R> {
  return request<R>(path, {
    method: "PUT",
    body: JSON.stringify(body)
  });
}
