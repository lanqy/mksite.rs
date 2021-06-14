---
title: Linux 下重新启动 Nginx web 服务器
created: 2018/05/21
description: Linux 下重新启动 Nginx web 服务器
author: lanqy
---
# Linux 下重新启动 Nginx web 服务器

译自 https://www.cyberciti.biz/faq/nginx-linux-restart/


如何使用命令行选项在 Linux 或 Unix 操作系统下重新启动 nginx Web 服务器？ 要重新启动 nginx Web 服务器，请以 root 用户身份使用以下任一命令。打开终端或使用 ssh 登录到远程服务器。

### Debian / Ubuntu / RHEL / CentOS Linux

使用以下命令：

```nginx
/etc/init.d/nginx restart
```

或者

```nginx
/etc/init.d/nginx reload
```

或者

```nginx
servive nginx restart
```

或者

```nginx
service nginx reload
```
或者如果您使用基于 systemd 的 Linux 发行版：

```nginx
sudo systemctl restart nginx
```

或者

```nginx
sudo systemctl reload nginx
```

查看状态

```nginx
service nginx status
```

或者

```nginx
sudo systemctl status nginx
```

但是，推荐方法如下。这应该适用于任何 Linux 发行版或类 Unix 操作系统：

```nginx
nginx -s reload
```

或者

```nginx
/path/to/full/nginx -s reload
```

#### 如果 nginx 编译并从源代码安装

如果 nginx 二进制文件安装在 /usr/local/nginx/sbin/nginx 中，请输入：

```nginx
/usr/local/nginx/sbin/nginx -s reload
```

