import { Card, Progress, Space, Table, Tag } from "antd";
import type { ColumnsType } from "antd/es/table";
import { RiseOutlined } from "@ant-design/icons";
import { useEffect, useMemo, useState } from "react";
import SectionHeader from "../../components/SectionHeader";
import StatCard from "../../components/StatCard";
import { apiGet } from "../../lib/api";

interface FocusOrder {
  key: string;
  orderNo: string;
  user: string;
  photographer: string;
  amount: string;
  status: string;
}

interface MetricsResp {
  period_days: number;
  users_total: number;
  orders_total: number;
  orders_period: number;
  orders_today: number;
  disputes_open: number;
  disputes_period: number;
  pending_photographers: number;
  pending_merchant_approvals: number;
  revenue_today: number;
  revenue_period: number;
}

interface TrendPoint {
  date: string;
  orders: number;
  disputes: number;
  revenue: number;
}

interface TrendResp {
  days: number;
  items: TrendPoint[];
}

interface Paged<T> {
  items: T[];
  page: number;
  page_size: number;
  total: number;
}

interface AdminOrderListItem {
  id: number;
  status: string;
  user_phone?: string | null;
  photographer_phone?: string | null;
  total_amount: number;
}

const columns: ColumnsType<FocusOrder> = [
  { title: "订单号", dataIndex: "orderNo" },
  { title: "需求方", dataIndex: "user" },
  { title: "摄影师", dataIndex: "photographer" },
  { title: "金额", dataIndex: "amount" },
  {
    title: "状态",
    dataIndex: "status",
    render: (value: string) => {
      const color = value.includes("纠纷") ? "red" : value.includes("待") ? "gold" : "blue";
      return <Tag color={color}>{value}</Tag>;
    }
  }
];

export default function DashboardPage() {
  const [orders, setOrders] = useState<FocusOrder[]>([]);
  const [trends, setTrends] = useState<TrendPoint[]>([]);
  const [lastUpdated, setLastUpdated] = useState<string>("--");
  const [stats, setStats] = useState({
    periodDays: 7,
    pendingPhotographers: 0,
    disputes: 0,
    merchantApprovals: 0,
    totalAmount: 0,
    ordersToday: 0,
    revenuePeriod: 0
  });

  useEffect(() => {
    let cancelled = false;

    const load = async () => {
      try {
        const metrics = await apiGet<MetricsResp>("/admin/metrics?days=7");
        if (!cancelled && metrics) {
          setStats({
            periodDays: metrics.period_days,
            pendingPhotographers: metrics.pending_photographers,
            disputes: metrics.disputes_open,
            merchantApprovals: metrics.pending_merchant_approvals,
            totalAmount: metrics.revenue_today,
            ordersToday: metrics.orders_today,
            revenuePeriod: metrics.revenue_period
          });
        }
      } catch {
        // 忽略错误
      }

      try {
        const trendResp = await apiGet<TrendResp>("/admin/metrics/trends?days=7");
        if (!cancelled && trendResp?.items) {
          setTrends(trendResp.items);
        }
      } catch {
        // 忽略错误
      }

      try {
        const orderResp = await apiGet<Paged<AdminOrderListItem>>(
          "/admin/orders?page=1&page_size=8"
        );
        if (!cancelled) {
          const items = orderResp?.items ?? [];
          setOrders(
            items.map((item) => ({
              key: String(item.id),
              orderNo: `OP-${item.id}`,
              user: item.user_phone ?? "未知用户",
              photographer: item.photographer_phone ?? "未指派",
              amount: `¥ ${item.total_amount.toFixed(2)}`,
              status: item.status
            }))
          );
        }
      } catch {
        // 忽略错误
      }

      if (!cancelled) {
        setLastUpdated(
          new Date().toLocaleTimeString("zh-CN", { hour: "2-digit", minute: "2-digit" })
        );
      }
    };

    load();

    return () => {
      cancelled = true;
    };
  }, []);

  const formattedTotal = useMemo(
    () => `¥ ${stats.totalAmount.toLocaleString("zh-CN")}`,
    [stats.totalAmount]
  );

  const trendColumns: ColumnsType<TrendPoint> = [
    { title: "日期", dataIndex: "date" },
    { title: "订单数", dataIndex: "orders" },
    {
      title: "成交额",
      dataIndex: "revenue",
      render: (value: number) => `¥ ${value.toLocaleString("zh-CN")}`
    },
    { title: "纠纷数", dataIndex: "disputes" }
  ];

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <SectionHeader
        title="今日运营脉搏"
        extra={<span className="tag-pill">数据刷新于 {lastUpdated}</span>}
      />
      <div className="hero-grid">
        <StatCard
          title="待审核摄影师"
          value={String(stats.pendingPhotographers)}
          metaLeft={`近${stats.periodDays}日`}
          metaRight={`共 ${stats.pendingPhotographers}`}
          icon={<RiseOutlined />}
        />
        <StatCard
          title="待处理纠纷"
          value={String(stats.disputes)}
          metaLeft="处理中"
          metaRight={`近${stats.periodDays}日 ${stats.disputes}`}
          icon={<RiseOutlined />}
        />
        <StatCard
          title="商户审批"
          value={String(stats.merchantApprovals)}
          metaLeft="待处理"
          metaRight={`共 ${stats.merchantApprovals}`}
          icon={<RiseOutlined />}
        />
        <StatCard
          title="今日成交额"
          value={formattedTotal}
          metaLeft={`今日订单 ${stats.ordersToday}`}
          metaRight={`近${stats.periodDays}日 ¥ ${stats.revenuePeriod.toLocaleString("zh-CN")}`}
          icon={<RiseOutlined />}
        />
      </div>

      <Card className="card-glass" bodyStyle={{ padding: 12 }}>
        <SectionHeader
          title={`近 ${stats.periodDays} 日趋势`}
          extra={<span className="tag-pill">订单 / 成交 / 纠纷</span>}
        />
        <div className="table-wrap">
          <Table
            rowKey="date"
            columns={trendColumns}
            dataSource={trends}
            pagination={false}
            size="small"
          />
        </div>
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 20 }}>
        <SectionHeader
          title="关键链路健康度"
          extra={<span className="tag-pill">SLA 目标 98%</span>}
        />
        <Space direction="vertical" size="middle" style={{ width: "100%" }}>
          <div>
            <Space style={{ width: "100%", justifyContent: "space-between" }}>
              <span>报价响应</span>
              <span>96%</span>
            </Space>
            <Progress percent={96} strokeColor="#2b6cb0" />
          </div>
          <div>
            <Space style={{ width: "100%", justifyContent: "space-between" }}>
              <span>交付准时率</span>
              <span>93%</span>
            </Space>
            <Progress percent={93} strokeColor="#14b8a6" />
          </div>
          <div>
            <Space style={{ width: "100%", justifyContent: "space-between" }}>
              <span>退款响应</span>
              <span>98%</span>
            </Space>
            <Progress percent={98} strokeColor="#f59e0b" />
          </div>
        </Space>
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 12 }}>
        <SectionHeader title="重点订单监控" extra={<span className="tag-pill">需要人工跟进</span>} />
        <div className="table-wrap">
          <Table columns={columns} dataSource={orders} pagination={false} />
        </div>
      </Card>
    </Space>
  );
}
