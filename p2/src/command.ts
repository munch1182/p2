import { invoke } from "@tauri-apps/api/core";

/**
 * 将命令和网络接口统一转为此类的方法
 */
export class Command {
  static save_window_loc(): Promise<boolean> {
    return invoke("save_window_loc");
  }

  static reset_window(): Promise<boolean> {
    return invoke("reset_window");
  }

  static log(str: string): Promise<void> {
    return invoke("log", { str });
  }

  static getDefaultsCommand(): Promise<
    {
      id: number;
      name: string;
      value: string;
    }[]
  > {
    return invoke("get_defaults_command");
  }
}
