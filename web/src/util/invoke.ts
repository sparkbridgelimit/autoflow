import { invoke as iv } from "@tauri-apps/api/core";
import { isTauri } from "./helper";

export const invoke = async (key: string, payload = {}) => {
  console.log(`before invoke: ${key}`, payload)
  if (isTauri()) {
    try {
      return await iv(key, payload);
    } catch (error) {
      console.error("invoke fail:", error);
    }
  }
}

export const list_browsers = async () => {
  return await invoke("list_browser", {
  });
}