/**
 * SwitchResultPanel 组件
 * 显示切换到本地的结果
 */

import {
  Check,
  AlertTriangle,
} from "@proxycast/plugin-components";
import type { SwitchResultPanelProps } from "./types";

export function SwitchResultPanel({ result }: SwitchResultPanelProps) {
  if (!result) return null;

  return (
    <div
      className={`mx-4 mb-3 rounded-lg p-3 text-sm ${
        result.success
          ? "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300"
          : "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-300"
      }`}
    >
      <div className="flex items-center gap-2">
        {result.success ? (
          <Check className="h-4 w-4 shrink-0" />
        ) : (
          <AlertTriangle className="h-4 w-4 shrink-0" />
        )}
        <span>{result.message}</span>
      </div>
      {result.success && result.requires_kiro_restart && (
        <div className="mt-2 text-xs opacity-80">
          请重启 Kiro IDE 使配置生效
        </div>
      )}
    </div>
  );
}
