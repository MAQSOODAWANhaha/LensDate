import { Button, Card, DatePicker, Select, Space, Table, Tag, message } from "antd";
import type { ColumnsType } from "antd/es/table";
import type { Dayjs } from "dayjs";
import { useEffect, useMemo, useState } from "react";
import { Link } from "react-router-dom";
import SectionHeader from "../../components/SectionHeader";
import { apiGet, apiPost } from "../../lib/api";

interface OrderRow {
  key: string;
  id: number;
  orderNo: string;
  user: string;
  photographer: string;
  amount: string;
  payType: string;
  status: string;
  createdAt: string;
}

interface AdminOrderListItem {
  id: number;
  user_id: number;
  user_phone?: string | null;
  photographer_id?: number | null;
  photographer_phone?: string | null;
  status: string;
  pay_type: string;
  total_amount: number;
  created_at: string;
}

interface PagedResponse<T> {
  items: T[];
  page: number;
  page_size: number;
  total: number;
}

interface OrderReportResp {
  format: string;
  generated_at: string;
  total: number;
  csv?: string | null;
}

const data: OrderRow[] = [
  {
    key: "1",
    id: 1,
    orderNo: "OP-20260116-0031",
    user: "苏清",
    photographer: "陌上影像",
    amount: "¥ 1,880",
    payType: "定金",
    status: "待交付",
    createdAt: "2026-01-16 08:20"
  },
  {
    key: "2",
    id: 2,
    orderNo: "OP-20260115-0047",
    user: "蓉居瑜伽",
    photographer: "镜语团队",
    amount: "¥ 3,980",
    payType: "全款",
    status: "纠纷处理中",
    createdAt: "2026-01-15 15:40"
  }
];

export default function OrdersPage() {
  const [rows, setRows] = useState<OrderRow[]>(data);
  const [status, setStatus] = useState("all");
  const [range, setRange] = useState<[Dayjs | null, Dayjs | null] | null>(null);
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(20);
  const [total, setTotal] = useState(0);
  const [refresh, setRefresh] = useState(0);

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      try {
        const search = new URLSearchParams({
          page: String(page),
          page_size: String(pageSize)
        });
        if (status !== "all") {
          search.set("status", status);
        }
        const res = await apiGet<PagedResponse<AdminOrderListItem>>(`/admin/orders?${search.toString()}`);
        setTotal(res.total ?? 0);
        if (!cancelled && res.items?.length) {
          setRows(
            res.items.map((item) => ({
              key: String(item.id),
              id: item.id,
              orderNo: `OP-${item.id}`,
              user: item.user_phone ?? String(item.user_id),
              photographer: item.photographer_phone ?? (item.photographer_id ? String(item.photographer_id) : "-"),
              amount: `¥ ${item.total_amount.toFixed(2)}`,
              payType: item.pay_type,
              status: item.status,
              createdAt: item.created_at
            }))
          );
        } else if (!cancelled) {
          setRows([]);
        }
      } catch {
        // 忽略错误，使用演示数据
      }
    };

    load();

    return () => {
      cancelled = true;
    };
  }, [status, page, pageSize, refresh]);

  const handleFreeze = async (orderId: string) => {
    const confirmed = window.confirm("确认冻结该订单？");
    if (!confirmed) {
      return;
    }
    try {
      await apiPost(`/admin/orders/${orderId}/freeze`, {});
      message.success("订单已冻结");
      setRefresh((prev) => prev + 1);
    } catch {
      message.error("冻结失败，请稍后重试");
    }
  };

  const handleExport = async () => {
    try {
      const search = new URLSearchParams({ format: "csv", limit: "500" });
      if (status !== "all") {
        search.set("status", status);
      }
      if (range?.[0] && range?.[1]) {
        search.set("start_date", range[0].format("YYYY-MM-DD"));
        search.set("end_date", range[1].format("YYYY-MM-DD"));
      }
      const res = await apiGet<OrderReportResp>(`/admin/reports/orders?${search.toString()}`);
      if (!res.csv) {
        message.warning("暂无可导出的数据");
        return;
      }
      const blob = new Blob([res.csv], { type: "text/csv;charset=utf-8;" });
      const url = URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = `orders_report_${new Date().toISOString().slice(0, 10)}.csv`;
      link.click();
      URL.revokeObjectURL(url);
      message.success("报表已导出");
    } catch {
      message.error("导出失败，请稍后重试");
    }
  };

  const columns: ColumnsType<OrderRow> = useMemo(
    () => [
      { title: "订单号", dataIndex: "orderNo" },
      { title: "用户", dataIndex: "user" },
      { title: "摄影师/团队", dataIndex: "photographer" },
      { title: "金额", dataIndex: "amount" },
      { title: "付款方式", dataIndex: "payType" },
      {
        title: "状态",
        dataIndex: "status",
        render: (value: string) => {
          const color = value.includes("纠纷") || value === "frozen" ? "red" : value.includes("待") ? "gold" : "blue";
          return <Tag color={color}>{value}</Tag>;
        }
      },
      { title: "创建时间", dataIndex: "createdAt" },
      {
        title: "操作",
        render: (_: unknown, row: OrderRow) => {
          if (row.status === "frozen") {
            return "-";
          }
          return (
            <Space>
              <Link to={`/orders/${row.id}`}>查看</Link>
              <Button size="small" danger onClick={() => handleFreeze(String(row.id))}>
                冻结
              </Button>
            </Space>
          );
        }
      }
    ],
    [handleFreeze]
  );

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <SectionHeader
        title="订单管理"
        extra={<span className="tag-pill">支持退款与交付追踪</span>}
      />
      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <Space wrap size="middle" style={{ marginBottom: 16 }}>
          <Select
            placeholder="状态"
            style={{ width: 160 }}
            value={status}
            onChange={(value) => {
              setStatus(value);
              setPage(1);
            }}
            options={[
              { label: "全部", value: "all" },
              { label: "已确认", value: "confirmed" },
              { label: "已付款", value: "paid" },
              { label: "进行中", value: "ongoing" },
              { label: "已完成", value: "completed" },
              { label: "已评价", value: "reviewed" },
              { label: "已冻结", value: "frozen" }
            ]}
          />
          <DatePicker.RangePicker
            allowClear
            onChange={(values) => setRange(values)}
          />
          <Button type="primary" onClick={handleExport}>
            导出报表
          </Button>
        </Space>
        <Table
          columns={columns}
          dataSource={rows}
          pagination={{
            current: page,
            pageSize,
            total,
            showSizeChanger: true
          }}
          onChange={(pagination) => {
            if (pagination.current) {
              setPage(pagination.current);
            }
            if (pagination.pageSize) {
              setPageSize(pagination.pageSize);
            }
          }}
        />
      </Card>
    </Space>
  );
}
