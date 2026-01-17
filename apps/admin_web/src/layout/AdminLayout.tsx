import { Layout, Menu, Input, Button, Space, Avatar, Badge } from "antd";
import {
  BarChartOutlined,
  TeamOutlined,
  ShoppingCartOutlined,
  WarningOutlined,
  AuditOutlined,
  PictureOutlined,
  SettingOutlined,
  SearchOutlined
} from "@ant-design/icons";
import { Outlet, useLocation, useNavigate } from "react-router-dom";
import { clearSession, getSession, hasAnyRole } from "../lib/auth";

const { Header, Sider, Content } = Layout;

const menuItems = [
  { key: "/", icon: <BarChartOutlined />, label: "运营总览", roles: ["admin", "ops", "manager"] },
  { key: "/users", icon: <TeamOutlined />, label: "用户管理", roles: ["admin", "ops", "manager"] },
  { key: "/orders", icon: <ShoppingCartOutlined />, label: "订单管理", roles: ["admin", "ops", "manager"] },
  { key: "/disputes", icon: <WarningOutlined />, label: "纠纷处理", roles: ["admin", "ops"] },
  { key: "/content", icon: <PictureOutlined />, label: "内容审核", roles: ["admin", "ops"] },
  { key: "/audit", icon: <AuditOutlined />, label: "审计日志", roles: ["admin"] },
  { key: "/ops", icon: <SettingOutlined />, label: "配置中心", roles: ["admin"] }
];

export default function AdminLayout() {
  const location = useLocation();
  const navigate = useNavigate();

  return (
    <Layout className="app-shell">
      <Sider width={220} theme="dark" breakpoint="lg" collapsedWidth={72}>
        <div style={{ padding: 20, color: "#f8fafc" }}>
          <div style={{ fontFamily: "ZCOOL XiaoWei, serif", fontSize: 20 }}>
            约拍控制台
          </div>
          <div style={{ fontSize: 12, opacity: 0.7 }}>运营 · 审核 · 监控</div>
        </div>
        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={[location.pathname]}
          items={menuItems.filter((item) => hasAnyRole(item.roles))}
          onClick={(item) => navigate(item.key)}
        />
      </Sider>
      <Layout>
        <Header className="app-header">
          <div className="header-title">
            <span>运营中枢</span>
            <span>今日重点：商户审批、纠纷处理、交易回访</span>
          </div>
          <Space size="middle">
            <span style={{ color: "rgba(248, 250, 252, 0.7)", fontSize: 12 }}>
              {getSession()?.user?.phone ?? "未绑定账号"}
            </span>
            <Input
              prefix={<SearchOutlined />}
              placeholder="搜索订单 / 用户 / 商户"
              style={{ width: 240 }}
            />
            <Button type="primary">创建审计记录</Button>
            <Badge dot>
              <Avatar style={{ backgroundColor: "#2b6cb0" }}>管</Avatar>
            </Badge>
            <Button
              ghost
              onClick={() => {
                clearSession();
                navigate("/login", { replace: true });
              }}
            >
              退出
            </Button>
          </Space>
        </Header>
        <Content className="page-wrap">
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
}
