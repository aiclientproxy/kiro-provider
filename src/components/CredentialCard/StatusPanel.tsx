/**
 * StatusPanel 组件
 * 显示 Kiro 详细状态和健康分数
 */

import {
  Activity,
  ChevronUp,
  Timer,
  BarChart3,
  RefreshCw,
  Power,
  cn,
} from "@proxycast/plugin-components";
import type { StatusPanelProps } from "./types";

export function StatusPanel({
  expanded,
  loading,
  healthScore,
  credential,
  kiroRefreshing,
  checkingHealth,
  onClose,
  onToggle,
  onQuickRefresh,
  onCheckHealth,
}: StatusPanelProps) {
  if (!expanded) return null;

  return (
    <div className="mx-4 mb-4 p-4 rounded-lg bg-emerald-50 dark:bg-emerald-950/30 border border-emerald-200 dark:border-emerald-800">
      <div className="flex items-center justify-between mb-3">
        <span className="text-sm font-medium text-emerald-700 dark:text-emerald-300 flex items-center gap-2">
          <Activity className="h-4 w-4" />
          Kiro 详细状态
        </span>
        <button
          onClick={onClose}
          className="text-emerald-500 hover:text-emerald-700 dark:hover:text-emerald-300"
        >
          <ChevronUp className="h-4 w-4" />
        </button>
      </div>

      {loading ? (
        <div className="flex items-center gap-2 text-sm text-emerald-600 dark:text-emerald-400">
          <div className="animate-spin h-4 w-4 border-2 border-current border-t-transparent rounded-full" />
          加载中...
        </div>
      ) : healthScore !== null ? (
        <div className="space-y-4">
          {/* 健康分数详情 */}
          <div className="bg-white dark:bg-emerald-950/50 rounded-lg p-3 border border-emerald-200 dark:border-emerald-800">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-emerald-700 dark:text-emerald-300">
                健康分数
              </span>
              <div
                className={cn(
                  "px-3 py-1 rounded-full text-sm font-bold",
                  healthScore >= 80
                    ? "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400"
                    : healthScore >= 60
                      ? "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400"
                      : healthScore >= 40
                        ? "bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400"
                        : "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400"
                )}
              >
                {Math.round(healthScore)} / 100
              </div>
            </div>

            {/* 健康分数条 */}
            <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
              <div
                className={cn(
                  "h-2 rounded-full transition-all duration-300",
                  healthScore >= 80
                    ? "bg-green-500"
                    : healthScore >= 60
                      ? "bg-yellow-500"
                      : healthScore >= 40
                        ? "bg-orange-500"
                        : "bg-red-500"
                )}
                style={{
                  width: `${Math.max(0, Math.min(100, healthScore))}%`,
                }}
              />
            </div>

            {/* 健康状态描述 */}
            <div className="mt-2 text-xs text-muted-foreground">
              {credential.is_disabled
                ? "凭证已被自动禁用，需手动重新启用"
                : healthScore >= 80
                  ? "凭证状态良好，可正常使用"
                  : healthScore >= 60
                    ? "凭证状态一般，建议注意监控"
                    : healthScore >= 40
                      ? "凭证状态较差，可能有风险"
                      : "凭证状态异常，需要立即处理"}
            </div>
          </div>

          {/* 状态指标 */}
          <div className="grid grid-cols-2 gap-3">
            <div className="bg-white dark:bg-emerald-950/50 rounded-lg p-3 border border-emerald-200 dark:border-emerald-800">
              <div className="flex items-center gap-2 mb-1">
                <Timer className="h-4 w-4 text-emerald-600" />
                <span className="text-sm font-medium text-emerald-700 dark:text-emerald-300">
                  冷却时间
                </span>
              </div>
              <div className="text-sm text-muted-foreground">
                根据使用频率计算的建议等待时间
              </div>
            </div>

            <div className="bg-white dark:bg-emerald-950/50 rounded-lg p-3 border border-emerald-200 dark:border-emerald-800">
              <div className="flex items-center gap-2 mb-1">
                <BarChart3 className="h-4 w-4 text-emerald-600" />
                <span className="text-sm font-medium text-emerald-700 dark:text-emerald-300">
                  使用权重
                </span>
              </div>
              <div className="text-sm text-muted-foreground">
                在轮询池中的权重分配
              </div>
            </div>
          </div>

          {/* 快速操作 */}
          <div className="flex gap-2 pt-2 border-t border-emerald-200 dark:border-emerald-800">
            {credential.is_disabled ? (
              <button
                onClick={onToggle}
                className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-orange-600 text-white rounded-md hover:bg-orange-700 transition-colors text-sm"
              >
                <Power className="h-4 w-4" />
                重新启用
              </button>
            ) : (
              <>
                <button
                  onClick={onQuickRefresh}
                  disabled={kiroRefreshing}
                  className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-emerald-600 text-white rounded-md hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-sm"
                >
                  <RefreshCw
                    className={cn("h-4 w-4", kiroRefreshing && "animate-spin")}
                  />
                  {kiroRefreshing ? "刷新中..." : "立即刷新"}
                </button>
                <button
                  onClick={onCheckHealth}
                  disabled={checkingHealth}
                  className="flex-1 flex items-center justify-center gap-2 px-3 py-2 bg-white dark:bg-emerald-950/50 border border-emerald-300 dark:border-emerald-700 text-emerald-700 dark:text-emerald-300 rounded-md hover:bg-emerald-50 dark:hover:bg-emerald-900/30 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-sm"
                >
                  <Activity
                    className={cn("h-4 w-4", checkingHealth && "animate-pulse")}
                  />
                  {checkingHealth ? "检查中..." : "重新检查"}
                </button>
              </>
            )}
          </div>
        </div>
      ) : (
        <div className="text-sm text-muted-foreground">
          无法获取状态信息，请重试
        </div>
      )}
    </div>
  );
}
