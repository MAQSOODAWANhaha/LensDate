import { Button, Card, Space } from "antd";
import { useNavigate } from "react-router-dom";
import { clearSession } from "../lib/auth";

export default function AccessDeniedPage() {
  const navigate = useNavigate();

  return (
    <div style={{ minHeight: "100vh", display: "flex", alignItems: "center", justifyContent: "center" }}>
      <Card className="card-glass" style={{ width: 420, borderRadius: 18 }} bodyStyle={{ padding: 32 }}>
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <div>
            <h1 style={{ margin: 0 }}>权限不足</h1>
            <p style={{ margin: "8px 0 0", color: "rgba(15, 23, 42, 0.7)" }}>
              当前账号没有访问该模块的权限，请联系管理员分配角色。
            </p>
          </div>
          <Space>
            <Button
              type="primary"
              onClick={() => {
                clearSession();
                navigate("/login", { replace: true });
              }}
            >
              重新登录
            </Button>
            <Button onClick={() => navigate("/", { replace: true })}>返回首页</Button>
          </Space>
        </Space>
      </Card>
    </div>
  );
}
