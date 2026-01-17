import { Button, Card, Input, Select, Space, Table, Tag, message } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useCallback, useEffect, useMemo, useState } from "react";
import SectionHeader from "../../components/SectionHeader";
import { apiGet, apiPost } from "../../lib/api";

interface UserRow {
  key: string;
  name: string;
  phone: string;
  role: string;
  status: string;
  city: string;
  updated: string;
  photographerId?: number;
  photographerStatus?: string;
}

interface AdminUserListItem {
  id: number;
  phone: string;
  status: string;
  role: string;
  nickname?: string | null;
  city_id?: number | null;
  updated_at: string;
  photographer_id?: number | null;
  photographer_status?: string | null;
}

interface PagedResponse<T> {
  items: T[];
  page: number;
  page_size: number;
  total: number;
}

const data: UserRow[] = [
  {
    key: "1",
    name: "苏清",
    phone: "138****8000",
    role: "用户",
    status: "正常",
    city: "上海",
    updated: "2026-01-16 09:20"
  },
  {
    key: "2",
    name: "陌上影像",
    phone: "139****1009",
    role: "摄影师",
    status: "待审核",
    city: "杭州",
    updated: "2026-01-16 08:50"
  },
  {
    key: "3",
    name: "蓉居瑜伽",
    phone: "137****2290",
    role: "商户",
    status: "已认证",
    city: "成都",
    updated: "2026-01-15 17:40"
  }
];

export default function UsersPage() {
  const [rows, setRows] = useState<UserRow[]>(data);
  const [keyword, setKeyword] = useState("");
  const [role, setRole] = useState("all");
  const [status, setStatus] = useState("all");
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(20);
  const [total, setTotal] = useState(0);

  const load = useCallback(async (params?: { keyword?: string; role?: string; status?: string; page?: number; pageSize?: number }) => {
    try {
      const search = new URLSearchParams({
        page: String(params?.page ?? 1),
        page_size: String(params?.pageSize ?? 20)
      });
      if (params?.keyword) {
        search.set("keyword", params.keyword);
      }
      if (params?.role && params.role !== "all") {
        search.set("role", params.role);
      }
      if (params?.status && params.status !== "all") {
        search.set("status", params.status);
      }

      const res = await apiGet<PagedResponse<AdminUserListItem>>(`/admin/users?${search.toString()}`);
      setTotal(res.total ?? 0);
      if (res.items?.length) {
        setRows(
          res.items.map((item) => ({
            key: String(item.id),
            name: item.nickname ?? item.phone,
            phone: item.phone,
            role: item.role === "photographer" ? "摄影师" : item.role === "merchant" ? "商户" : "用户",
            status: item.status === "active" ? "正常" : item.status,
            city: item.city_id ? `#${item.city_id}` : "-",
            updated: item.updated_at,
            photographerId: item.photographer_id ?? undefined,
            photographerStatus: item.photographer_status ?? undefined
          }))
        );
      } else {
        setRows([]);
      }
    } catch {
      // 忽略错误，使用演示数据
    }
  }, []);

  useEffect(() => {
    let cancelled = false;
    const run = async () => {
      if (!cancelled) {
        await load({ keyword, role, status, page, pageSize });
      }
    };

    run();

    return () => {
      cancelled = true;
    };
  }, [keyword, role, status, page, pageSize, load]);

  const handleReview = async (photographerId: number, nextStatus: "approved" | "rejected") => {
    try {
      await apiPost(`/admin/photographers/${photographerId}/review`, { status: nextStatus });
      message.success("审核已更新");
      await load({ keyword, role, status, page, pageSize });
    } catch {
      message.error("操作失败，请稍后重试");
    }
  };

  const columns: ColumnsType<UserRow> = useMemo(
    () => [
      { title: "姓名/机构", dataIndex: "name" },
      { title: "联系方式", dataIndex: "phone" },
      { title: "角色", dataIndex: "role" },
      {
        title: "状态",
        dataIndex: "status",
        render: (value: string) => {
          const color = value === "正常" || value === "已认证" ? "green" : "gold";
          return <Tag color={color}>{value}</Tag>;
        }
      },
      {
        title: "摄影师审核",
        dataIndex: "photographerStatus",
        render: (_: string | undefined, row: UserRow) => {
          if (row.role !== "摄影师") {
            return "-";
          }
          const statusLabel = row.photographerStatus ?? "-";
          const color = statusLabel === "approved" ? "green" : statusLabel === "rejected" ? "red" : "gold";
          return <Tag color={color}>{statusLabel}</Tag>;
        }
      },
      { title: "城市", dataIndex: "city" },
      { title: "更新时间", dataIndex: "updated" },
      {
        title: "操作",
        render: (_: unknown, row: UserRow) => {
          if (row.role !== "摄影师" || row.photographerStatus !== "pending" || !row.photographerId) {
            return "-";
          }
          return (
            <Space>
              <Button size="small" type="primary" onClick={() => handleReview(row.photographerId!, "approved")}>
                通过
              </Button>
              <Button size="small" danger onClick={() => handleReview(row.photographerId!, "rejected")}>
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
      <SectionHeader
        title="用户与摄影师"
        extra={<span className="tag-pill">支持实名与资质校验</span>}
      />

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <Space wrap size="middle" style={{ marginBottom: 16 }}>
          <Input.Search
            placeholder="搜索姓名/手机号"
            style={{ width: 220 }}
            onSearch={(value) => {
              setKeyword(value.trim());
              setPage(1);
            }}
            onChange={(event) => {
              if (!event.target.value) {
                setKeyword("");
                setPage(1);
              }
            }}
            allowClear
          />
          <Select
            placeholder="角色"
            style={{ width: 160 }}
            value={role}
            onChange={(value) => {
              setRole(value);
              setPage(1);
            }}
            options={[
              { label: "全部", value: "all" },
              { label: "用户", value: "user" },
              { label: "摄影师", value: "photographer" },
              { label: "商户", value: "merchant" }
            ]}
          />
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
              { label: "正常", value: "active" },
              { label: "待审核", value: "pending" },
              { label: "已认证", value: "verified" }
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
