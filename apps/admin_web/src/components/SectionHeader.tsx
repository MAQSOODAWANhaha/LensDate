import { ReactNode } from "react";

interface SectionHeaderProps {
  title: string;
  extra?: ReactNode;
}

export default function SectionHeader({ title, extra }: SectionHeaderProps) {
  return (
    <div className="section-title">
      <h2>{title}</h2>
      {extra}
    </div>
  );
}
