import os
import json
import subprocess
from typing import Dict, Any

def run_all(folder: str) -> None:
    """递归遍历并运行所有.ant文件，捕获并解析Rust的JSON输出"""
    for root, _, files in os.walk(folder):
        for file in files:
            if not file.endswith(".ant"):
                continue
                
            file_path = os.path.join(root, file)
            relative_path = os.path.relpath(file_path, folder)
            print(f"Running {relative_path}:")
            
            # 构建命令并抑制警告
            cmd = [
                "cargo", "run", "--quiet",
                "--message-format", "json", 
                "--", "--file", file_path, 
            ]
            
            try:
                # 捕获输出并解析JSON
                result = subprocess.run(
                    cmd,
                    capture_output=True,
                    text=True,
                    check=True,
                    env={**os.environ, "RUSTFLAGS": "-A warnings"}  # 全局抑制警告
                )
                
                # 解析JSON输出
                output_data: Dict[str, Any] = json.loads(result.stdout)
                print(f"✅ Success | Result: {output_data}")
                
            except subprocess.CalledProcessError as e:
                print(f"❌ Rust Error | Exit Code: {e.returncode}\nError: {e.stderr}")
            except json.JSONDecodeError:
                print(f"Result: {result.stdout}")
if __name__ == "__main__":
    current_dir = os.path.dirname(os.path.abspath(__file__))
    run_all(current_dir)