import { Button, Card, Descriptions, Divider, Space, Table, Tag } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useEffect, useMemo, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import SectionHeader from "../../components/SectionHeader";
import { apiGet } from "../../lib/api";

interface OrderItem {
  name: string;
  price: number;
  quantity: number;
}

interface PaymentItem {
  id: number;
  amount: number;
  status: string;
  pay_channel: string;
  paid_at?: string | null;
  proof_url?: string | null;
}

interface RefundItem {
  id: number;
  amount: number;
  status: string;
  reason?: string | null;
  proof_url?: string | null;
  created_at: string;
}

interface DeliveryItem {
  id: number;
  file_url: string;
  version?: string | null;
  note?: string | null;
}

interface Delivery {
  id: number;
  status: string;
  submitted_at?: string | null;
  accepted_at?: string | null;
  items: DeliveryItem[];
}

interface Review {
  score: number;
  tags?: string[] | null;
  comment?: string | null;
  created_at: string;
}

interface OrderDetailResp {
  id: number;
  status: string;
  pay_type: string;
  total_amount: number;
  deposit_amount: number;
  service_fee: number;
  schedule_start?: string | null;
  schedule_end?: string | null;
  created_at: string;
  updated_at: string;
  user_id: number;
  user_phone?: string | null;
  photographer_id?: number | null;
  photographer_phone?: string | null;
  items: OrderItem[];
  payments: PaymentItem[];
  refunds: RefundItem[];
  deliveries: Delivery[];
  review?: Review | null;
}

export default function OrderDetailPage() {
  const navigate = useNavigate();
  const { id } = useParams();
  const [detail, setDetail] = useState<OrderDetailResp | null>(null);

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      if (!id) {
        return;
      }
      try {
        const res = await apiGet<OrderDetailResp>(`/admin/orders/${id}`);
        if (!cancelled) {
          setDetail(res);
        }
      } catch {
        // 忽略错误
      }
    };

    load();

    return () => {
      cancelled = true;
    };
  }, [id]);

  const itemColumns: ColumnsType<OrderItem> = useMemo(
    () => [
      { title: "项目", dataIndex: "name" },
      { title: "单价", dataIndex: "price", render: (v: number) => `¥ ${v.toFixed(2)}` },
      { title: "数量", dataIndex: "quantity" }
    ],
    []
  );

  const paymentColumns: ColumnsType<PaymentItem> = useMemo(
    () => [
      { title: "支付编号", dataIndex: "id" },
      { title: "渠道", dataIndex: "pay_channel" },
      { title: "金额", dataIndex: "amount", render: (v: number) => `¥ ${v.toFixed(2)}` },
      { title: "状态", dataIndex: "status" },
      { title: "完成时间", dataIndex: "paid_at" },
      {
        title: "凭证",
        dataIndex: "proof_url",
        render: (value?: string | null) =>
          value ? (
            <a href={value} target="_blank" rel="noreferrer">
              查看
            </a>
          ) : (
            "-"
          )
      }
    ],
    []
  );

  const refundColumns: ColumnsType<RefundItem> = useMemo(
    () => [
      { title: "退款编号", dataIndex: "id" },
      { title: "金额", dataIndex: "amount", render: (v: number) => `¥ ${v.toFixed(2)}` },
      { title: "状态", dataIndex: "status" },
      { title: "原因", dataIndex: "reason" },
      {
        title: "凭证",
        dataIndex: "proof_url",
        render: (value?: string | null) =>
          value ? (
            <a href={value} target="_blank" rel="noreferrer">
              查看
            </a>
          ) : (
            "-"
          )
      },
      { title: "申请时间", dataIndex: "created_at" }
    ],
    []
  );

  const deliveryColumns: ColumnsType<DeliveryItem> = useMemo(
    () => [
      {
        title: "文件",
        dataIndex: "file_url",
        render: (value: string) => (
          <a href={value} target="_blank" rel="noreferrer">
            查看
          </a>
        )
      },
      { title: "版本", dataIndex: "version" },
      { title: "备注", dataIndex: "note" }
    ],
    []
  );

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <SectionHeader
        title={`订单详情 #${id ?? "-"}`}
        extra={
          <Button onClick={() => navigate(-1)} type="default">
            返回列表
          </Button>
        }
      />

      <Card className="card-glass" bodyStyle={{ padding: 20 }}>
        <Descriptions title="订单概览" column={2} bordered>
          <Descriptions.Item label="订单状态">
            <Tag color={detail?.status === "frozen" ? "red" : "blue"}>{detail?.status ?? "-"}</Tag>
          </Descriptions.Item>
          <Descriptions.Item label="支付方式">{detail?.pay_type ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="用户">{detail?.user_phone ?? detail?.user_id ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="摄影师">
            {detail?.photographer_phone ?? detail?.photographer_id ?? "-"}
          </Descriptions.Item>
          <Descriptions.Item label="总金额">
            {detail ? `¥ ${detail.total_amount.toFixed(2)}` : "-"}
          </Descriptions.Item>
          <Descriptions.Item label="定金">
            {detail ? `¥ ${detail.deposit_amount.toFixed(2)}` : "-"}
          </Descriptions.Item>
          <Descriptions.Item label="平台服务费">
            {detail ? `¥ ${detail.service_fee.toFixed(2)}` : "-"}
          </Descriptions.Item>
          <Descriptions.Item label="服务时间">
            {detail?.schedule_start ?? "-"} ~ {detail?.schedule_end ?? "-"}
          </Descriptions.Item>
          <Descriptions.Item label="创建时间">{detail?.created_at ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="更新时间">{detail?.updated_at ?? "-"}</Descriptions.Item>
        </Descriptions>
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <SectionHeader title="订单条目" />
        <Table columns={itemColumns} dataSource={detail?.items ?? []} pagination={false} />
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <SectionHeader title="支付记录" />
        <Table columns={paymentColumns} dataSource={detail?.payments ?? []} pagination={false} />
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <SectionHeader title="退款记录" />
        <Table columns={refundColumns} dataSource={detail?.refunds ?? []} pagination={false} />
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <SectionHeader title="交付记录" />
        <Space direction="vertical" size="middle" style={{ width: "100%" }}>
          {(detail?.deliveries ?? []).map((delivery) => (
            <Card key={delivery.id} type="inner" title={`交付 #${delivery.id}`}>
              <Space direction="vertical" size="small" style={{ width: "100%" }}>
                <Space>
                  <Tag color="blue">{delivery.status}</Tag>
                  <span>提交：{delivery.submitted_at ?? "-"}</span>
                  <span>验收：{delivery.accepted_at ?? "-"}</span>
                </Space>
                <Table columns={deliveryColumns} dataSource={delivery.items} pagination={false} />
              </Space>
            </Card>
          ))}
          {detail?.deliveries?.length === 0 && <div>暂无交付记录</div>}
        </Space>
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <SectionHeader title="评价记录" />
        {detail?.review ? (
          <Space direction="vertical" size="small">
            <div>评分：{detail.review.score}</div>
            <div>标签：{detail.review.tags?.join("，") ?? "-"}</div>
            <div>评价：{detail.review.comment ?? "-"}</div>
            <div>时间：{detail.review.created_at}</div>
          </Space>
        ) : (
          <div>暂无评价</div>
        )}
      </Card>
      <Divider />
    </Space>
  );
}
