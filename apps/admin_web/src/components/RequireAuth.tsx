import { ReactElement } from "react";
import { Navigate, useLocation } from "react-router-dom";
import { isAuthed } from "../lib/auth";

interface RequireAuthProps {
  children: ReactElement;
}

export default function RequireAuth({ children }: RequireAuthProps) {
  const location = useLocation();
  if (!isAuthed()) {
    return <Navigate to="/login" state={{ from: location }} replace />;
  }
  return children;
}
