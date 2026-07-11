export const fmtInt = (n: number) => Math.round(n).toLocaleString("en-US");

export const fmtTokensShort = (n: number) => {
  if (n >= 1e8) return (n / 1e6).toFixed(0) + "M";
  if (n >= 1e6) return (n / 1e6).toFixed(1) + "M";
  if (n >= 1e3) return (n / 1e3).toFixed(1) + "K";
  return String(Math.round(n));
};

export const fmtMoney = (n: number, currency?: string, rate?: number) => {
  if (currency === "usd" && rate && rate > 0) {
    return "$" + (n * rate).toFixed(2);
  }
  return "¥" + n.toFixed(2);
};

export const mmdd = (date: string) => {
  const parts = date.split("-");
  return parts.length === 3 ? `${Number(parts[1])}/${Number(parts[2])}` : date;
};

export const todayStr = () => {
  const now = new Date();
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}-${String(now.getDate()).padStart(2, "0")}`;
};

export const dateKey = (date: Date) =>
  `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")}`;

export const addDays = (date: Date, offset: number) => {
  const next = new Date(date);
  next.setDate(next.getDate() + offset);
  return next;
};

export const previousMonth = (date: Date) => {
  const previous = new Date(date.getFullYear(), date.getMonth() - 1, 1);
  return { month: previous.getMonth() + 1, year: previous.getFullYear() };
};

export const modelDisplayName = (key: string): string => {
  const map: Record<string, string> = {
    "mimo-v2.5": "V2.5",
    "mimo-v2.5-pro": "V2.5 Pro",
  };
  return map[key] ?? key;
};

export const modelIcon = (key: string): "flash" | "pro" | "mimo" => {
  if (key.includes("pro")) return "pro";
  if (key.startsWith("mimo-")) return "mimo";
  return "flash";
};

/** 通用缓存工具：成功写入 localStorage，失败时回退到缓存 */
export async function fetchWithCache<T>(key: string, fetcher: () => Promise<T>): Promise<T> {
  try {
    const data = await fetcher();
    try { localStorage.setItem(key, JSON.stringify(data)); } catch {}
    return data;
  } catch (error) {
    try {
      const cached = localStorage.getItem(key);
      if (cached) return JSON.parse(cached) as T;
    } catch {}
    throw error;
  }
}

