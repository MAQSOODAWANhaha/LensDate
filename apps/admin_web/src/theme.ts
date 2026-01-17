import type { ThemeConfig } from "antd";

export const theme: ThemeConfig = {
  token: {
    colorPrimary: "#2b6cb0",
    colorInfo: "#2b6cb0",
    colorSuccess: "#1e8e6e",
    colorWarning: "#c05621",
    colorError: "#c53030",
    borderRadius: 10,
    fontFamily: "\"Work Sans\", \"PingFang SC\", \"Microsoft YaHei\", sans-serif"
  },
  components: {
    Layout: {
      headerBg: "#0f172a",
      bodyBg: "#eef2f7",
      siderBg: "#0f172a"
    },
    Menu: {
      darkItemBg: "#0f172a",
      darkItemSelectedBg: "#1d4ed8",
      darkItemSelectedColor: "#f8fafc"
    }
  }
};
