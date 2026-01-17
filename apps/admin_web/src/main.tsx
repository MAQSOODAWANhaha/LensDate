import { createRoot } from "react-dom/client";
import { ConfigProvider, App as AntApp } from "antd";
import zhCN from "antd/locale/zh_CN";
import "antd/dist/reset.css";
import App from "./App";
import { theme } from "./theme";
import "./styles/global.css";

const container = document.getElementById("root");
if (!container) {
  throw new Error("root not found");
}

createRoot(container).render(
  <ConfigProvider locale={zhCN} theme={theme}>
    <AntApp>
      <App />
    </AntApp>
  </ConfigProvider>
);
