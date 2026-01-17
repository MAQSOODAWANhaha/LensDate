import { useState } from "react";
import { Button, Card, Form, Input, message, Space } from "antd";
import { useLocation, useNavigate } from "react-router-dom";
import { apiPost } from "../lib/api";
import { setSession } from "../lib/auth";

interface SendCodeReq {
  phone: string;
}

interface SendCodeResp {
  expired_at: number;
}

interface LoginReq {
  phone: string;
  code: string;
}

interface LoginResp {
  token: string;
  user: { id: number; phone: string; status: string };
  roles: string[];
}

export default function LoginPage() {
  const navigate = useNavigate();
  const location = useLocation();
  const [loading, setLoading] = useState(false);

  const handleSendCode = async (phone?: string) => {
    if (!phone) {
      message.warning("请输入手机号");
      return;
    }
    setLoading(true);
    try {
      await apiPost<SendCodeReq, SendCodeResp>("/auth/code", { phone });
      message.success("验证码已发送");
    } catch {
      message.error("发送失败，请稍后再试");
    } finally {
      setLoading(false);
    }
  };

  const handleFinish = async (values: LoginReq) => {
    setLoading(true);
    try {
      const data = await apiPost<LoginReq, LoginResp>("/auth/login", values);
      setSession({ token: data.token, user: data.user, roles: data.roles });
      message.success("登录成功");
      const redirect = (location.state as { from?: { pathname?: string } } | null)?.from?.pathname ?? "/";
      navigate(redirect, { replace: true });
    } catch {
      message.error("登录失败，请检查验证码");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ minHeight: "100vh", display: "flex", alignItems: "center", justifyContent: "center" }}>
      <Card
        className="card-glass"
        style={{ width: 420, borderRadius: 18 }}
        bodyStyle={{ padding: 32 }}
      >
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <div>
            <h1 style={{ margin: 0 }}>约拍管理后台</h1>
            <p style={{ margin: "8px 0 0", color: "rgba(15, 23, 42, 0.7)" }}>
              请输入手机号与验证码登录
            </p>
          </div>
          <Form layout="vertical" onFinish={handleFinish}>
            <Form.Item
              label="手机号"
              name="phone"
              rules={[{ required: true, message: "请输入手机号" }]}
            >
              <Input placeholder="例如 13800138000" />
            </Form.Item>
            <Form.Item label="验证码" required>
              <Input.Group compact>
                <Form.Item
                  name="code"
                  noStyle
                  rules={[{ required: true, message: "请输入验证码" }]}
                >
                  <Input style={{ width: "60%" }} placeholder="6 位验证码" />
                </Form.Item>
                <Form.Item shouldUpdate noStyle>
                  {({ getFieldValue }) => (
                    <Button
                      style={{ width: "40%" }}
                      onClick={() => handleSendCode(getFieldValue("phone"))}
                      loading={loading}
                    >
                      发送验证码
                    </Button>
                  )}
                </Form.Item>
              </Input.Group>
            </Form.Item>
            <Button type="primary" htmlType="submit" block loading={loading}>
              登录
            </Button>
          </Form>
        </Space>
      </Card>
    </div>
  );
}
