import { Button, Card, Input, Select, Space, Table, Tag, message } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useCallback, useEffect, useMemo, useState } from "react";
import SectionHeader from "../../components/SectionHeader";
import { apiGet, apiPost } from "../../lib/api";

interface PortfolioRow {
  key: string;
  id: number;
  title: string;
  photographer: string;
  status: string;
  createdAt: string;
  updatedAt: string;
}

interface AdminPortfolioListItem {
  id: number;
  photographer_id: number;
  photographer_phone?: string | null;
  title: string;
  status: string;
  created_at: string;
  updated_at: string;
}

interface PagedResponse<T> {
  items: T[];
  page: number;
  page_size: number;
  total: number;
}

export default function ContentReviewPage() {
  const [rows, setRows] = useState<PortfolioRow[]>([]);
  const [status, setStatus] = useState("all");
  const [photographerId, setPhotographerId] = useState("");
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(20);
  const [total, setTotal] = useState(0);
  const [refresh, setRefresh] = useState(0);

  const load = useCallback(
    async (params?: { status?: string; photographerId?: string; page?: number; pageSize?: number }) => {
      try {
        const search = new URLSearchParams({
          page: String(params?.page ?? 1),
          page_size: String(params?.pageSize ?? 20)
        });
        if (params?.status && params.status !== "all") {
          search.set("status", params.status);
        }
        if (params?.photographerId) {
          search.set("photographer_id", params.photographerId);
        }

        const res = await apiGet<PagedResponse<AdminPortfolioListItem>>(`/admin/portfolios?${search.toString()}`);
        setTotal(res.total ?? 0);
        setRows(
          res.items.map((item) => ({
            key: String(item.id),
            id: item.id,
            title: item.title,
            photographer: item.photographer_phone ?? String(item.photographer_id),
            status: item.status,
            createdAt: item.created_at,
            updatedAt: item.updated_at
          }))
        );
      } catch {
        setRows([]);
      }
    },
    []
  );

  useEffect(() => {
    let cancelled = false;
    const run = async () => {
      if (!cancelled) {
        await load({ status, photographerId, page, pageSize });
      }
    };
    run();
    return () => {
      cancelled = true;
    };
  }, [status, photographerId, page, pageSize, refresh, load]);

  const handleReview = useCallback(
    async (portfolioId: number, nextStatus: "approved" | "rejected") => {
      const comment = window.prompt("审核备注（可选）", "");
      try {
        await apiPost(`/admin/portfolios/${portfolioId}/review`, {
          status: nextStatus,
          comment: comment || null
        });
        message.success("审核已更新");
        setRefresh((prev) => prev + 1);
      } catch {
        message.error("操作失败，请稍后重试");
      }
    },
    []
  );

  const columns: ColumnsType<PortfolioRow> = useMemo(
    () => [
      { title: "作品集ID", dataIndex: "id" },
      { title: "标题", dataIndex: "title" },
      { title: "摄影师", dataIndex: "photographer" },
      {
        title: "状态",
        dataIndex: "status",
        render: (value: string) => {
          const color =
            value === "approved" ? "green" : value === "rejected" ? "red" : value === "pending" ? "gold" : "default";
          return <Tag color={color}>{value}</Tag>;
        }
      },
      { title: "创建时间", dataIndex: "createdAt" },
      { title: "更新时间", dataIndex: "updatedAt" },
      {
        title: "操作",
        render: (_: unknown, row: PortfolioRow) => {
          if (row.status !== "pending") {
            return "-";
          }
          return (
            <Space>
              <Button size="small" type="primary" onClick={() => handleReview(row.id, "approved")}>
                通过
              </Button>
              <Button size="small" danger onClick={() => handleReview(row.id, "rejected")}>
                拒绝
              </Button>
            </Space>
          );
        }
      }
    ],
    [handleReview]
  );

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <SectionHeader title="内容审核" extra={<span className="tag-pill">作品集审核队列</span>} />
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
              { label: "待审核", value: "pending" },
              { label: "已通过", value: "approved" },
              { label: "已拒绝", value: "rejected" }
            ]}
          />
          <Input.Search
            placeholder="摄影师ID"
            style={{ width: 200 }}
            onSearch={(value) => {
              setPhotographerId(value.trim());
              setPage(1);
            }}
            onChange={(event) => {
              if (!event.target.value) {
                setPhotographerId("");
                setPage(1);
              }
            }}
            allowClear
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
