const TOKEN_KEY = "admin_token";
const USER_KEY = "admin_user";

export interface LoginSession {
  token: string;
  user?: { id: number; phone: string; status: string };
  roles?: string[];
}

export function getToken(): string | null {
  return localStorage.getItem(TOKEN_KEY);
}

export function setSession(session: LoginSession) {
  localStorage.setItem(TOKEN_KEY, session.token);
  localStorage.setItem(USER_KEY, JSON.stringify(session));
}

export function getSession(): LoginSession | null {
  const raw = localStorage.getItem(USER_KEY);
  if (!raw) {
    return null;
  }
  try {
    return JSON.parse(raw) as LoginSession;
  } catch {
    return null;
  }
}

export function clearSession() {
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(USER_KEY);
}

export function isAuthed(): boolean {
  return Boolean(getToken());
}

export function getRoles(): string[] {
  return getSession()?.roles ?? [];
}

export function hasAnyRole(required: string[]): boolean {
  if (required.length === 0) {
    return true;
  }
  const roles = getRoles();
  return required.some((role) => roles.includes(role));
}
