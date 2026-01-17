import { Button, Card, Form, Input, Select, Space, Table, Tag, message } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useCallback, useEffect, useMemo, useState } from "react";
import SectionHeader from "../../components/SectionHeader";
import { apiGet, apiPost, apiPut } from "../../lib/api";

interface MerchantApproval {
  key: string;
  id: number;
  merchant: string;
  demandId: number;
  status: string;
  comment?: string | null;
  createdAt: string;
}

interface MerchantApprovalItem {
  id: number;
  merchant_id: number;
  merchant_name: string;
  demand_id: number;
  status: string;
  comment?: string | null;
  created_at: string;
}

interface MerchantTemplateItem {
  name: string;
  quantity: number;
  price: number;
}

interface MerchantTemplateItemResp {
  id: number;
  merchant_id: number;
  merchant_name: string;
  name: string;
  description?: string | null;
  created_at: string;
  items: MerchantTemplateItem[];
}

interface PagedResponse<T> {
  items: T[];
  page: number;
  page_size: number;
  total: number;
}

const approvals: MerchantApproval[] = [];

export default function OpsPage() {
  const [form] = Form.useForm();
  const [rows, setRows] = useState<MerchantApproval[]>(approvals);
  const [status, setStatus] = useState("all");
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(20);
  const [total, setTotal] = useState(0);
  const [refresh, setRefresh] = useState(0);
  const [templateRows, setTemplateRows] = useState<MerchantTemplateItemResp[]>([]);
  const [templatePage, setTemplatePage] = useState(1);
  const [templatePageSize, setTemplatePageSize] = useState(10);
  const [templateTotal, setTemplateTotal] = useState(0);

  const handleSubmit = async (values: {
    autoCancel?: string;
    refundPenalty?: string;
    disputePriority?: string;
    demandTags?: string;
    photographerTags?: string;
    recommendSlots?: string;
    activityBanners?: string;
  }) => {
    const parseTags = (value?: string) =>
      (value ?? "")
        .split(",")
        .map((item) => item.trim())
        .filter((item) => item.length > 0);

    const parseJson = (value?: string) => {
      if (!value || value.trim().length === 0) {
        return [];
      }
      return JSON.parse(value);
    };

    let recommendSlots: unknown = [];
    let activityBanners: unknown = [];
    try {
      recommendSlots = parseJson(values.recommendSlots);
    } catch {
      message.error("推荐位配置必须是合法 JSON");
      return;
    }
    try {
      activityBanners = parseJson(values.activityBanners);
    } catch {
      message.error("活动配置必须是合法 JSON");
      return;
    }

    try {
      const payloads = [
        { key: "order_auto_cancel_hours", value: Number(values.autoCancel) || values.autoCancel || null },
        { key: "refund_penalty_rate", value: Number(values.refundPenalty) || values.refundPenalty || null },
        { key: "dispute_priority", value: values.disputePriority || null },
        { key: "demand_tags", value: parseTags(values.demandTags) },
        { key: "photographer_tags", value: parseTags(values.photographerTags) },
        { key: "recommend_slots", value: recommendSlots },
        { key: "activity_banners", value: activityBanners }
      ];

      await Promise.all(
        payloads.map((item) => apiPut(`/admin/configs/${item.key}`, { value: item.value }))
      );
      message.success("配置已保存");
    } catch {
      message.error("保存失败，请稍后重试");
    }
  };

  useEffect(() => {
    let cancelled = false;
    const loadConfigs = async () => {
      const keys = [
        "order_auto_cancel_hours",
        "refund_penalty_rate",
        "dispute_priority",
        "demand_tags",
        "photographer_tags",
        "recommend_slots",
        "activity_banners"
      ];
      const values: Record<string, unknown> = {};
      await Promise.all(
        keys.map(async (key) => {
          try {
            const res = await apiGet<{ value: unknown }>(`/admin/configs/${key}`);
            values[key] = res.value;
          } catch {
            // 忽略不存在的配置
          }
        })
      );
      if (!cancelled) {
        form.setFieldsValue({
          autoCancel: values.order_auto_cancel_hours ?? "",
          refundPenalty: values.refund_penalty_rate ?? "",
          disputePriority: values.dispute_priority ?? undefined,
          demandTags: Array.isArray(values.demand_tags) ? values.demand_tags.join(",") : "",
          photographerTags: Array.isArray(values.photographer_tags) ? values.photographer_tags.join(",") : "",
          recommendSlots: values.recommend_slots ? JSON.stringify(values.recommend_slots, null, 2) : "",
          activityBanners: values.activity_banners ? JSON.stringify(values.activity_banners, null, 2) : ""
        });
      }
    };

    loadConfigs();

    return () => {
      cancelled = true;
    };
  }, [form]);

  useEffect(() => {
    let cancelled = false;
    const loadApprovals = async () => {
      try {
        const search = new URLSearchParams({
          page: String(page),
          page_size: String(pageSize)
        });
        if (status !== "all") {
          search.set("status", status);
        }
        const res = await apiGet<PagedResponse<MerchantApprovalItem>>(
          `/admin/merchant-approvals?${search.toString()}`
        );
        if (!cancelled) {
          setTotal(res.total ?? 0);
          setRows(
            res.items.map((item) => ({
              key: String(item.id),
              id: item.id,
              merchant: item.merchant_name,
              demandId: item.demand_id,
              status: item.status,
              comment: item.comment,
              createdAt: item.created_at
            }))
          );
        }
      } catch {
        // 忽略错误
      }
    };

    loadApprovals();

    return () => {
      cancelled = true;
    };
  }, [status, page, pageSize, refresh]);

  useEffect(() => {
    let cancelled = false;
    const loadTemplates = async () => {
      try {
        const search = new URLSearchParams({
          page: String(templatePage),
          page_size: String(templatePageSize)
        });
        const res = await apiGet<PagedResponse<MerchantTemplateItemResp>>(
          `/admin/merchant-templates?${search.toString()}`
        );
        if (!cancelled) {
          setTemplateTotal(res.total ?? 0);
          setTemplateRows(res.items ?? []);
        }
      } catch {
        // 忽略错误
      }
    };

    loadTemplates();

    return () => {
      cancelled = true;
    };
  }, [templatePage, templatePageSize, refresh]);

  const handleReview = useCallback(async (approvalId: number, nextStatus: "approved" | "rejected") => {
    const comment = window.prompt("审批备注（可选）", "");
    try {
      await apiPost(`/admin/merchant-approvals/${approvalId}/review`, {
        status: nextStatus,
        comment: comment || null
      });
      message.success("审批已更新");
      setRefresh((prev) => prev + 1);
    } catch {
      message.error("操作失败，请稍后重试");
    }
  }, []);

  const columns: ColumnsType<MerchantApproval> = useMemo(
    () => [
      { title: "商户", dataIndex: "merchant" },
      { title: "需求", dataIndex: "demandId", render: (value: number) => `#${value}` },
      {
        title: "状态",
        dataIndex: "status",
        render: (value: string) => {
          const color = value === "approved" ? "green" : value === "rejected" ? "red" : "gold";
          return <Tag color={color}>{value}</Tag>;
        }
      },
      { title: "备注", dataIndex: "comment", render: (value: string | null | undefined) => value || "-" },
      { title: "创建时间", dataIndex: "createdAt" },
      {
        title: "操作",
        render: (_: unknown, row: MerchantApproval) => {
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

  const templateColumns: ColumnsType<MerchantTemplateItemResp> = useMemo(
    () => [
      { title: "模板名称", dataIndex: "name" },
      { title: "商户", dataIndex: "merchant_name" },
      { title: "描述", dataIndex: "description", render: (value: string | null) => value || "-" },
      { title: "创建时间", dataIndex: "created_at" }
    ],
    []
  );

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <SectionHeader
        title="配置与商户审批"
        extra={<span className="tag-pill">策略发布需二次确认</span>}
      />

      <Card className="card-glass" bodyStyle={{ padding: 20 }}>
        <Form layout="vertical" form={form} onFinish={handleSubmit}>
          <Space wrap size="large">
            <Form.Item label="订单自动取消时长" name="autoCancel" style={{ width: 260 }}>
              <Input placeholder="例如 24" />
            </Form.Item>
            <Form.Item label="退款违约比例" name="refundPenalty" style={{ width: 260 }}>
              <Input placeholder="例如 10" />
            </Form.Item>
            <Form.Item label="争议优先级" name="disputePriority" style={{ width: 220 }}>
              <Select
                placeholder="请选择"
                options={[
                  { label: "低", value: "low" },
                  { label: "中", value: "medium" },
                  { label: "高", value: "high" }
                ]}
              />
            </Form.Item>
            <Form.Item label="">
              <Button type="primary" htmlType="submit">保存配置</Button>
            </Form.Item>
          </Space>
          <Space wrap size="large" style={{ marginTop: 12 }}>
            <Form.Item label="需求标签（逗号分隔）" name="demandTags" style={{ width: 360 }}>
              <Input placeholder="例如 写真,活动,课程" />
            </Form.Item>
            <Form.Item label="摄影师标签（逗号分隔）" name="photographerTags" style={{ width: 360 }}>
              <Input placeholder="例如 纪实,棚拍,新手友好" />
            </Form.Item>
          </Space>
          <Space wrap size="large" style={{ marginTop: 12 }}>
            <Form.Item label="推荐位配置（JSON）" name="recommendSlots" style={{ width: 480 }}>
              <Input.TextArea rows={4} placeholder='[{"key":"home_top","title":"首页推荐"}]' />
            </Form.Item>
            <Form.Item label="活动配置（JSON）" name="activityBanners" style={{ width: 480 }}>
              <Input.TextArea rows={4} placeholder='[{"title":"开春活动","image_url":"","link":""}]' />
            </Form.Item>
          </Space>
        </Form>
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <Space wrap size="middle" style={{ marginBottom: 16 }}>
          <Select
            style={{ width: 160 }}
            value={status}
            onChange={(value) => {
              setStatus(value);
              setPage(1);
            }}
            options={[
              { label: "全部", value: "all" },
              { label: "待审批", value: "pending" },
              { label: "已通过", value: "approved" },
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

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <SectionHeader title="商户模板" />
        <Table
          columns={templateColumns}
          dataSource={templateRows}
          rowKey="id"
          expandable={{
            expandedRowRender: (record) => (
              <div>
                {record.items.length === 0 ? (
                  <span>暂无模板条目</span>
                ) : (
                  <ul style={{ margin: 0, paddingLeft: 16 }}>
                    {record.items.map((item, index) => (
                      <li key={`${record.id}-${index}`}>
                        {item.name} × {item.quantity}（¥ {item.price.toFixed(2)}）
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            )
          }}
          pagination={{
            current: templatePage,
            pageSize: templatePageSize,
            total: templateTotal,
            showSizeChanger: true
          }}
          onChange={(pagination) => {
            if (pagination.current) {
              setTemplatePage(pagination.current);
            }
            if (pagination.pageSize) {
              setTemplatePageSize(pagination.pageSize);
            }
          }}
        />
      </Card>
    </Space>
  );
}
