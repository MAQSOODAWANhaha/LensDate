import { Button, Card, Descriptions, Input, Select, Space, Table, Tag, message } from "antd";
import type { ColumnsType } from "antd/es/table";
import { useEffect, useMemo, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import SectionHeader from "../../components/SectionHeader";
import { apiGet, apiPost } from "../../lib/api";

interface EvidenceItem {
  id: number;
  file_url: string;
  note?: string | null;
  created_at: string;
}

interface DisputeDetailResp {
  id: number;
  order_id: number;
  order_status?: string | null;
  initiator_id: number;
  initiator_phone?: string | null;
  status: string;
  reason?: string | null;
  resolution?: string | null;
  created_at: string;
  updated_at: string;
  evidence: EvidenceItem[];
}

export default function DisputeDetailPage() {
  const navigate = useNavigate();
  const { id } = useParams();
  const [detail, setDetail] = useState<DisputeDetailResp | null>(null);
  const [resolution, setResolution] = useState("");
  const [status, setStatus] = useState("resolved");

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      if (!id) {
        return;
      }
      try {
        const res = await apiGet<DisputeDetailResp>(`/admin/disputes/${id}`);
        if (!cancelled) {
          setDetail(res);
          setResolution(res.resolution ?? "");
          setStatus(res.status ?? "resolved");
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

  const evidenceColumns: ColumnsType<EvidenceItem> = useMemo(
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
      { title: "备注", dataIndex: "note" },
      { title: "上传时间", dataIndex: "created_at" }
    ],
    []
  );

  const handleResolve = async () => {
    if (!id) {
      return;
    }
    if (!resolution.trim()) {
      message.warning("请填写处理说明");
      return;
    }
    try {
      await apiPost(`/admin/disputes/${id}/resolve`, { resolution, status });
      message.success("处理结果已提交");
    } catch {
      message.error("提交失败，请稍后重试");
    }
  };

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <SectionHeader
        title={`纠纷详情 #${id ?? "-"}`}
        extra={
          <Button onClick={() => navigate(-1)} type="default">
            返回列表
          </Button>
        }
      />

      <Card className="card-glass" bodyStyle={{ padding: 20 }}>
        <Descriptions title="纠纷概览" column={2} bordered>
          <Descriptions.Item label="纠纷状态">
            <Tag color={detail?.status === "resolved" ? "green" : detail?.status === "rejected" ? "red" : "gold"}>
              {detail?.status ?? "-"}
            </Tag>
          </Descriptions.Item>
          <Descriptions.Item label="订单状态">{detail?.order_status ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="订单编号">{detail?.order_id ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="发起人">{detail?.initiator_phone ?? detail?.initiator_id ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="原因">{detail?.reason ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="创建时间">{detail?.created_at ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="更新时间">{detail?.updated_at ?? "-"}</Descriptions.Item>
          <Descriptions.Item label="处理说明">{detail?.resolution ?? "-"}</Descriptions.Item>
        </Descriptions>
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <SectionHeader title="证据材料" />
        <Table columns={evidenceColumns} dataSource={detail?.evidence ?? []} pagination={false} />
      </Card>

      <Card className="card-glass" bodyStyle={{ padding: 16 }}>
        <SectionHeader title="处理结论" />
        <Space direction="vertical" size="middle" style={{ width: "100%" }}>
          <Select
            value={status}
            style={{ width: 200 }}
            onChange={(value) => setStatus(value)}
            options={[
              { label: "处理完成", value: "resolved" },
              { label: "处理中", value: "processing" },
              { label: "已拒绝", value: "rejected" }
            ]}
          />
          <Input.TextArea
            rows={4}
            value={resolution}
            onChange={(event) => setResolution(event.target.value)}
            placeholder="填写处理说明"
          />
          <Button type="primary" onClick={handleResolve}>
            提交处理
          </Button>
        </Space>
      </Card>
    </Space>
  );
}
