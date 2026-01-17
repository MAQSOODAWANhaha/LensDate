import { createBrowserRouter } from "react-router-dom";
import AdminLayout from "./layout/AdminLayout";
import DashboardPage from "./modules/dashboard";
import UsersPage from "./modules/users";
import OrdersPage from "./modules/orders";
import OrderDetailPage from "./modules/orders/detail";
import DisputesPage from "./modules/disputes";
import DisputeDetailPage from "./modules/disputes/detail";
import AuditPage from "./modules/audit";
import ContentReviewPage from "./modules/content";
import OpsPage from "./modules/ops";
import LoginPage from "./pages/Login";
import RequireAuth from "./components/RequireAuth";
import RequireRole from "./components/RequireRole";

export const router = createBrowserRouter([
  {
    path: "/login",
    element: <LoginPage />
  },
  {
    path: "/",
    element: (
      <RequireAuth>
        <RequireRole roles={["admin", "ops", "manager"]}>
          <AdminLayout />
        </RequireRole>
      </RequireAuth>
    ),
    children: [
      { index: true, element: <DashboardPage /> },
      { path: "users", element: <UsersPage /> },
      { path: "orders", element: <OrdersPage /> },
      { path: "orders/:id", element: <OrderDetailPage /> },
      { path: "disputes", element: <DisputesPage /> },
      { path: "disputes/:id", element: <DisputeDetailPage /> },
      {
        path: "content",
        element: (
          <RequireRole roles={["admin", "ops"]}>
            <ContentReviewPage />
          </RequireRole>
        )
      },
      {
        path: "audit",
        element: (
          <RequireRole roles={["admin"]}>
            <AuditPage />
          </RequireRole>
        )
      },
      {
        path: "ops",
        element: (
          <RequireRole roles={["admin"]}>
            <OpsPage />
          </RequireRole>
        )
      }
    ]
  }
]);
