#!/bin/bash

# 定义函数来处理 file: 类型的 URL
handle_file_url() {
    local url=$1
    local destination=$2

    # 提取路径部分
    local path=${url#file:}

    # 如果路径末尾没有 /，则加上 /
    if [[ "$path" != */ ]]; then
        path="$path/"
    fi

    echo "Syncing files from $path to $destination..."

    # 使用 rsync 复制文件或目录内容
    rsync -avz "$path" "$destination"

    if [ $? -eq 0 ]; then
        echo "Sync completed successfully!"
    else
        echo "Sync failed. Please check the paths and try again."
        exit 1
    fi
}

# 定义函数来处理 git: 类型的 URL
handle_git_url() {
    local url=$1
    local destination=$2

    # 提取 Git 仓库地址
    local repo=${url#git:}

    echo "Cloning repository from $repo to $destination..."

    # 使用 git clone 克隆仓库
    git clone "$repo" "$destination"

    if [ $? -eq 0 ]; then
        echo "Clone completed successfully!"
    else
        echo "Clone failed. Please check the repository URL and try again."
        exit 1
    fi
}

# 主函数
main() {
    if [ $# -lt 2 ]; then
        echo "Usage: $0 <url> <destination>"
        exit 1
    fi

    local url=$1
    local destination=$2

    # 根据 URL 类型调用相应的处理函数
    if [[ $url == file:* ]]; then
        handle_file_url "$url" "$destination"
    elif [[ $url == git:* ]]; then
        handle_git_url "$url" "$destination"
    else
        echo "Unsupported URL type: $url"
        exit 1
    fi
}

# 调用主函数并传递参数
main "$@"

