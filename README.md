# Jimiru
Jimiru は、固定されていない IP アドレス環境のリモートデスクトップと、 LAN 外からの Wake on LAN をサポートするプロジェクトです。

## jimiru_server
固定 IP アドレスが提供されている環境に配置し、コントロールパネルとなる web ページを表示します。

## jimiru_jitaku
jimiru_server に接続し、利用可能なマシン一覧を送信します。
jimiru_server からマシン起動命令を受け取ると、マジックパケットを送出します。

# ビルド
通常のビルドは cargo で行うことができます。

jimiru_jitaku を ARM64 向けにビルドする場合は `gcc-aarch64-linux-gnu` パッケージが必要です。
