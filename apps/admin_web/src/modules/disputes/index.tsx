import { Button, Card, Select, Space, Table, Tag, message } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useEffect, useMemo, useState } from "react";
import { Link } from "react-router-dom";
import SectionHeader from "../../components/SectionHeader";
import { apiGet, apiPost } from "../../lib/api";

interface DisputeRow {
  key: string;
  id: number;
  orderNo: string;
  initiator: string;
  reason: string;
  status: string;
  updated: string;
}

interface AdminDisputeListItem {
  id: number;
  order_id: number;
  initiator_id: number;
  initiator_phone?: string | null;
  status: string;
  reason?: string | null;
  updated_at: string;
}

interface PagedResponse<T> {
  items: T[];
  page: number;
  page_size: number;
  total: number;
}

const data: DisputeRow[] = [
  {
    key: "1",
    id: 1,
    orderNo: "OP-20260115-0047",
    initiator: "蓉居瑜伽",
    reason: "交付延期",
    status: "处理中",
    updated: "2026-01-16 09:10"
  },
  {
    key: "2",
    id: 2,
    orderNo: "OP-20260114-0008",
    initiator: "陌上影像",
    reason: "付款争议",
    status: "待核实",
    updated: "2026-01-15 19:20"
  }
];

export default function DisputesPage() {
  const [rows, setRows] = useState<DisputeRow[]>(data);
  const [status, setStatus] = useState("all");
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
        const res = await apiGet<PagedResponse<AdminDisputeListItem>>(`/admin/disputes?${search.toString()}`);
        setTotal(res.total ?? 0);
        if (!cancelled && res.items?.length) {
          setRows(
            res.items.map((item) => ({
              key: String(item.id),
              id: item.id,
              orderNo: `OP-${item.order_id}`,
              initiator: item.initiator_phone ?? String(item.initiator_id),
              reason: item.reason ?? "-",
              status: item.status,
              updated: item.updated_at
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

  const handleResolve = async (id: number) => {
    const resolution = window.prompt("请输入处理说明", "已介入处理");
    if (!resolution) {
      return;
    }
    try {
      await apiPost(`/admin/disputes/${id}/resolve`, { resolution });
      message.success("已提交处理结果");
      setRefresh((prev) => prev + 1);
    } catch {
      message.error("处理失败，请稍后重试");
    }
  };

  const columns: ColumnsType<DisputeRow> = useMemo(
    () => [
      { title: "订单号", dataIndex: "orderNo" },
      { title: "发起方", dataIndex: "initiator" },
      { title: "原因", dataIndex: "reason" },
      {
        title: "状态",
        dataIndex: "status",
        render: (value: string) => {
          const color =
            value === "processing"
              ? "orange"
              : value === "resolved"
                ? "green"
                : value === "rejected"
                  ? "red"
                  : "gold";
          return <Tag color={color}>{value}</Tag>;
        }
      },
      { title: "更新时间", dataIndex: "updated" },
      {
        title: "操作",
        render: (_: unknown, row: DisputeRow) => (
          <Space>
            <Link to={`/disputes/${row.id}`}>查看</Link>
            <Button size="small" type="link" onClick={() => handleResolve(row.id)}>
              处理
            </Button>
          </Space>
        )
      }
    ],
    [handleResolve]
  );

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <SectionHeader
        title="纠纷处理"
        extra={<span className="tag-pill">需 2 小时内响应</span>}
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
              { label: "已提交", value: "submitted" },
              { label: "处理中", value: "processing" },
              { label: "已解决", value: "resolved" },
              { label: "已拒绝", value: "rejected" }
            ]}
          />
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
