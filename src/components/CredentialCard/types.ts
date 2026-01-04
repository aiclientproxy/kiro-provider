/**
 * CredentialCard 组件类型定义
 */

import type {
  CredentialDisplay,
  KiroFingerprintInfo,
  UsageInfo,
} from "@proxycast/plugin-components";

/**
 * 凭证状态配置
 */
export interface StatusConfig {
  color: string;
  text: string;
  borderColor: string;
}

/**
 * 凭证来源配置
 */
export interface SourceConfig {
  text: string;
  color: string;
}

/**
 * 切换到本地结果
 */
export interface SwitchToLocalResult {
  success: boolean;
  message: string;
  requires_action: boolean;
  requires_kiro_restart: boolean;
}

/**
 * CardHeader Props
 */
export interface CardHeaderProps {
  credential: CredentialDisplay;
  isHealthy: boolean;
  isLocalActive?: boolean;
}

/**
 * CardActions Props
 */
export interface CardActionsProps {
  credential: CredentialDisplay;
  isOAuth: boolean;
  // 操作状态
  checkingHealth: boolean;
  refreshingToken: boolean;
  fingerprintLoading: boolean;
  fingerprintExpanded: boolean;
  usageLoading: boolean;
  usageExpanded: boolean;
  kiroStatusLoading: boolean;
  kiroStatusExpanded: boolean;
  kiroRefreshing: boolean;
  switchingToLocal: boolean;
  deleting: boolean;
  // 回调
  onToggle: () => void;
  onEdit: () => void;
  onCheckHealth: () => void;
  onRefreshToken: () => void;
  onCheckFingerprint: () => void;
  onCheckUsage: () => void;
  onCheckKiroStatus: () => void;
  onQuickRefresh: () => void;
  onSwitchToLocal: () => void;
  onReset: () => void;
  onDelete: () => void;
}

/**
 * CardStats Props
 */
export interface CardStatsProps {
  credential: CredentialDisplay;
  isOAuth: boolean;
  kiroHealthScore: number | null;
}

/**
 * FingerprintPanel Props
 */
export interface FingerprintPanelProps {
  expanded: boolean;
  loading: boolean;
  info: KiroFingerprintInfo | null;
  onClose: () => void;
  onCopyMachineId: () => void;
  copied: boolean;
}

/**
 * UsagePanel Props
 */
export interface UsagePanelProps {
  expanded: boolean;
  loading: boolean;
  info: UsageInfo | null;
  error: string | null;
  onClose: () => void;
}

/**
 * StatusPanel Props
 */
export interface StatusPanelProps {
  expanded: boolean;
  loading: boolean;
  healthScore: number | null;
  credential: CredentialDisplay;
  kiroRefreshing: boolean;
  checkingHealth: boolean;
  onClose: () => void;
  onToggle: () => void;
  onQuickRefresh: () => void;
  onCheckHealth: () => void;
}

/**
 * SwitchResultPanel Props
 */
export interface SwitchResultPanelProps {
  result: SwitchToLocalResult | null;
}

/**
 * CredentialCard Props
 */
export interface CredentialCardProps {
  credential: CredentialDisplay;
  onToggle: () => void;
  onDelete: () => void;
  onReset: () => void;
  onCheckHealth: () => void;
  onRefreshToken: () => void;
  onEdit: () => void;
  deleting: boolean;
  checkingHealth: boolean;
  refreshingToken: boolean;
  isLocalActive?: boolean;
  onSwitchToLocal?: () => void;
}
