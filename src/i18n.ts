// Minimal i18n shim - only provides Chinese strings
// This is a placeholder for the full i18n module

export type Lang = 'zh' | 'en';

export const LANG_OPTIONS: { code: Lang; label: string }[] = [
  { code: 'zh', label: '简体中文' },
];

const translations: Record<string, string> = {
  // General
  'app.loading': '查询中…',
  'app.error': '查询失败',
  'app.unconfigured': '未配置',
  'app.unconfigured_token': '未配置 Token',
  'app.unavailable': '不可用',
  'app.no_data': '暂无数据',
  'app.tokens': 'tokens',

  // Balance
  'balance.title': '账户余额',
  'balance.available': '可用',
  'balance.insufficient': '余额不足',
  'balance.today': '当日消耗',
  'balance.monthly': '本月消费',

  // Settings
  'settings.title': '设置',
  'settings.api_key': 'API Key',
  'settings.api_key_desc': '用于调用 API 获取余额和用量数据。',
  'settings.save': '验证并保存',
  'settings.clear': '清除 Key',
  'settings.verified': '已配置',
  'settings.not_configured': '未配置',
  'settings.general': '通用',
  'settings.autostart': '开机自启',
  'settings.autostart_desc': '开启后，每次登录 Windows 时自动启动应用。',
  'settings.auto_refresh': '自动刷新',
  'settings.auto_refresh_desc': '开启后，按设定周期自动拉取最新数据。',
  'settings.window_size': '窗口大小',
  'settings.window_desc': '选择预设窗口尺寸，或拖拽窗口边缘自由调整。',
  'settings.compact': '紧凑',
  'settings.standard': '标准',
  'settings.wide': '宽屏',
  'settings.large': '大屏',
  'settings.about': '关于',
  'settings.version': '当前版本',
  'settings.check_update': '检查更新',
  'settings.checking': '检查中…',
  'settings.latest': '已是最新版本',
  'settings.update_found': '发现新版本',
  'settings.download_update': '下载更新',
  'settings.downloading_update': '正在下载更新…',
  'settings.update_installed': '更新已下载，即将安装',
  'settings.cat_account': '账户',
  'settings.cat_general': '通用',
  'settings.cat_display': '显示',
  'settings.cat_data': '数据',
  'settings.cat_about': '关于',
  'settings.theme': '主题',
  'settings.theme_desc': '选择深色、浅色或跟随系统主题。',
  'settings.theme_light': '浅色',
  'settings.theme_dark': '深色',
  'settings.theme_system': '跟随系统',
  'settings.currency': '货币单位',
  'settings.currency_desc': '选择显示金额的货币类型。',
  'settings.efficiency': '效率单位',
  'settings.efficiency_desc': '选择显示效率指标的方向。',
  'settings.token_per_currency': 'MT/¥',
  'settings.currency_per_token': '¥/MT',
  'settings.language': '语言',
  'settings.language_desc': '选择界面显示语言。',
  'settings.default_provider': '默认平台',
  'settings.currency_cny': '人民币 (¥)',
  'settings.currency_usd': '美元 ($)',
  'settings.clear_cache': '清除缓存',
  'settings.clear_cache_desc': '清除本地缓存的使用数据，下次启动时重新获取。',
  'settings.notify_cooldown': '通知冷却时间',
  'settings.notify_cooldown_desc': '两次余额不足通知之间的最小间隔。',

  // Notifications
  'notify.title': '通知',
  'notify.toggle': '余额不足时发送 Windows 通知',
  'notify.desc': '当 API 余额低于设定阈值时，通过 Windows 通知提醒。',
  'notify.threshold': '阈值',

  // Usage
  'usage.title': '用量同步 Token',
  'usage.desc': '用于同步 Token 用量、消费和趋势图。需网页登录 token。',
  'usage.auto_sync': '网页登录自动同步',
  'usage.waiting': '等待登录',
  'usage.manual': '方式二：手动粘贴 token',
  'usage.manual_collapse': '收起手动粘贴',
  'usage.save_token': '保存 Token',
  'usage.clear_token': '清除 Token',

  // MiMo
  'mimo.login': 'MiMo 登录',
  'mimo.login_desc': '通过小米账号登录 MiMo 平台，登录成功后即可查看余额和用量数据。',
  'mimo.login_btn': '打开 MiMo 登录',
  'mimo.opening': '正在打开…',
  'mimo.no_key': 'MiMo 平台通过小米账号登录认证，无需 API Key。',
  'mimo.not_logged_in': 'MiMo 未登录，请在设置中重新登录小米账号',

  // Charts
  'chart.cache_hit': '缓存命中明细',
  'chart.hit': '命中',
  'chart.miss': '未命中',
  'chart.output': '输出',
  'chart.hit_rate': '命中率',
  'chart.total': '合计',
  'chart.this_week': '本周',
  'chart.last_week': '上周',
  'chart.weeks_ago': '周前',
  'chart.input_hit': '输入（命中缓存）',
  'chart.input_miss': '输入（未命中缓存）',

  // Model Detail
  'detail.requests': 'API 请求次数',
  'detail.daily': '按日 Token 消耗',
  'detail.back': '返回主面板',

  // Navigation
  'nav.refresh': '刷新',
  'nav.settings': '设置',
  'nav.close': '关闭',
};

let currentLang: Lang = 'zh';

export function setLang(lang: Lang) {
  currentLang = lang;
  try { localStorage.setItem('dsm-lang', lang); } catch {}
}

export function getLang(): Lang {
  return currentLang;
}

export function initLang() {
  try {
    const saved = localStorage.getItem('dsm-lang');
    if (saved === 'en' || saved === 'zh') currentLang = saved as Lang;
  } catch {}
}

export function t(key: string): string {
  return translations[key] || key;
}
