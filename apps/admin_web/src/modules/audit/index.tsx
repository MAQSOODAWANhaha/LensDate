import { Card, Input, Space, Table, Tag } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useEffect, useMemo, useState } from "react";
import SectionHeader from "../../components/SectionHeader";
import { apiGet } from "../../lib/api";

interface AuditRow {
  key: string;
  action: string;
  target: string;
  admin: string;
  createdAt: string;
  level: string;
}

interface AuditListItem {
  id: number;
  action: string;
  target_type?: string | null;
  target_id?: number | null;
  admin_id: number;
  admin_phone?: string | null;
  created_at: string;
}

interface PagedResponse<T> {
  items: T[];
  page: number;
  page_size: number;
  total: number;
}

const data: AuditRow[] = [];

export default function AuditPage() {
  const [rows, setRows] = useState<AuditRow[]>(data);
  const [keyword, setKeyword] = useState("");
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(20);
  const [total, setTotal] = useState(0);

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      try {
        const search = new URLSearchParams({
          page: String(page),
          page_size: String(pageSize)
        });
        if (keyword) {
          search.set("action", keyword);
        }
        const res = await apiGet<PagedResponse<AuditListItem>>(`/admin/audits?${search.toString()}`);
        if (!cancelled) {
          setTotal(res.total ?? 0);
          const mapped = res.items.map((item) => {
            const level = item.action.includes("freeze") || item.action.includes("reject") ? "WARN" : "INFO";
            const target = [item.target_type, item.target_id].filter(Boolean).join("#");
            return {
              key: String(item.id),
              action: item.action,
              target: target || "-",
              admin: item.admin_phone ?? String(item.admin_id),
              createdAt: item.created_at,
              level
            };
          });
          setRows(mapped);
        }
      } catch {
        // 忽略错误
      }
    };

    load();

    return () => {
      cancelled = true;
    };
  }, [keyword, page, pageSize]);

  const columns: ColumnsType<AuditRow> = useMemo(
    () => [
      { title: "操作", dataIndex: "action" },
      { title: "对象", dataIndex: "target" },
      { title: "执行人", dataIndex: "admin" },
      { title: "时间", dataIndex: "createdAt" },
      {
        title: "等级",
        dataIndex: "level",
        render: (value: string) => (
          <Tag color={value === "WARN" ? "red" : "blue"}>{value}</Tag>
        )
      }
    ],
    []
  );

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <SectionHeader
        title="审计日志"
        extra={<span className="tag-pill">关键动作留痕</span>}
      />
      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <Space wrap size="middle" style={{ marginBottom: 16 }}>
          <Input.Search
            placeholder="按动作筛选"
            style={{ width: 240 }}
            onSearch={(value) => {
              setKeyword(value.trim());
              setPage(1);
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
