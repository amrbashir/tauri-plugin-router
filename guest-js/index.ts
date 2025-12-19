import { convertFileSrc } from "@tauri-apps/api/core";

// Taken from Tauri's IPC serialization logic
function processIpcArgs(...args: unknown[]) {
  if (
    (args.length === 1 && args instanceof ArrayBuffer) ||
    ArrayBuffer.isView(args) ||
    Array.isArray(args)
  ) {
    return {
      contentType: "application/octet-stream",
      data: args[0] as ArrayBuffer,
    };
  } else {
    const data = JSON.stringify(args, (_k, val) => {
      const SERIALIZE_TO_IPC_FN = "__TAURI_TO_IPC_KEY__";

      if (val instanceof Map) {
        return Object.fromEntries(val.entries());
      } else if (val instanceof Uint8Array) {
        return Array.from(val);
      } else if (val instanceof ArrayBuffer) {
        return Array.from(new Uint8Array(val));
      } else if (
        typeof val === "object" &&
        val !== null &&
        SERIALIZE_TO_IPC_FN in val
      ) {
        return val[SERIALIZE_TO_IPC_FN]();
      } else {
        return val;
      }
    });

    return {
      contentType: "application/json",
      data,
    };
  }
}

/**
 * Invokes a command on the Tauri router plugin.
 *
 * @example
 * ```ts
 * import { invoke } from "tauri-plugin-router";
 *
 * const result = await invoke("my_command", arg1, arg2);
 * console.log(result);
 * ```
 *
 * @param cmd The command to invoke.
 * @param args The arguments to pass to the command.
 * @returns A promise that resolves to the result of the command.
 */
export async function invoke<T>(
  cmd: string,
  ...args: unknown[]
): Promise<T | ArrayBuffer | string> {
  const url = convertFileSrc(cmd, "router");

  const { contentType, data } = processIpcArgs(...args);

  const response = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": contentType,
    },
    body: data,
  });

  // we need to split here because on Android the content-type gets duplicated
  switch ((response.headers.get("content-type") || "").split(",")[0]) {
    case "application/json":
      return response.json();
    case "text/plain":
      return response.text();
    default:
      return response.arrayBuffer();
  }
}
