import { ReactElement } from "react";
import { hasAnyRole } from "../lib/auth";
import AccessDeniedPage from "../pages/AccessDenied";

interface RequireRoleProps {
  roles: string[];
  children: ReactElement;
}

export default function RequireRole({ roles, children }: RequireRoleProps) {
  if (!hasAnyRole(roles)) {
    return <AccessDeniedPage />;
  }
  return children;
}
