# 使用官方Rust镜像作为构建环境
FROM rust:1.75-slim as builder

# 创建新的空项目目录
WORKDIR /usr/src/app

# 复制项目文件
COPY . .

# 构建项目
RUN cargo build --release

# 使用更小的基础镜像运行程序
FROM debian:bookworm-slim

# 复制构建好的二进制文件
COPY --from=builder /usr/src/app/target/release/tee-verify /usr/local/bin/tee-verify

# 暴露3000端口
EXPOSE 5000

# 运行程序
CMD ["tee-verify"]