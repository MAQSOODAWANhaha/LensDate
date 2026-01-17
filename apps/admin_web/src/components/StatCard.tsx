import { ReactNode } from "react";

interface StatCardProps {
  title: string;
  value: string;
  metaLeft?: string;
  metaRight?: string;
  icon?: ReactNode;
}

export default function StatCard({ title, value, metaLeft, metaRight, icon }: StatCardProps) {
  return (
    <div className="stat-card">
      <h3>{title}</h3>
      <strong>{value}</strong>
      <div className="stat-meta">
        <span>{metaLeft}</span>
        <span>{icon}</span>
        <span>{metaRight}</span>
      </div>
    </div>
  );
}
