---
title: Alist API
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
toc_footers: []
includes: []
search: true
code_clipboard: true
highlight_theme: darkula
headingLevel: 2
generator: "@tarslib/widdershins v4.0.30"

---

# 默认模块

Base URLs:

# Authentication

# auth

## POST token获取

POST /api/auth/login

获取某个用户的临时JWt token，默认48小时过期

> Body 请求参数

```json
{
    "username": "{{alist_username}}",
    "password": "{{alist_password}}"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» username|body|string| 是 | 用户名|用户名|
|» password|body|string| 是 | 密码|密码|
|» otp_code|body|string| 否 | 二步验证码|二步验证码|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "token": "abcd"
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||状态码|
|» message|string|true|none||信息|
|» data|object|true|none||data|
|»» token|string|true|none||token|

## POST 用户注册

POST /api/auth/register

> Body 请求参数

```json
{
    "username": "string",
    "password": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» username|body|string| 是 ||none|
|» password|body|string| 是 ||none|

> 返回示例

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

```json
{
    "code": 403,
    "message": "registration is disabled",
    "data": null
}
```

> 403 Response

```json
{
  "code": 0,
  "message": "string",
  "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|403|[Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|null|true|none||none|

状态码 **403**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|null|true|none||none|

## POST token获取hash

POST /api/auth/login/hash

获取某个用户的临时JWt token，传入的密码需要在添加-https://github.com/alist-org/alist后缀后再进行sha256

> Body 请求参数

```json
{
    "username": "{{alist_username}}",
    "password": "{{alist_password}}"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 否 ||none|
|» username|body|string| 是 | 用户名|用户名|
|» password|body|string| 是 | 密码|hash后密码，获取方式为`sha256(密码-https://github.com/alist-org/alist)`|
|» otp_code|body|string| 否 | 二步验证码|二步验证码|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "token": "abcd"
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||状态码|
|» message|string|true|none||信息|
|» data|object|true|none||data|
|»» token|string|true|none||token|

## POST 生成2FA密钥

POST /api/auth/2fa/generate

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "qr": "data:image/png;base64,iVBORw0KGgoAAAANSUhE",
        "secret": "RPQZG4MDS3"
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none|数据|none|
|»» qr|string|true|none|二维码|二维码图片的data url|
|»» secret|string|true|none|密钥|none|

## POST 验证2FA code

POST /api/auth/2fa/verify

> Body 请求参数

```json
{
  "code": "string",
  "secret": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» code|body|string| 是 | 2FA验证码|none|
|» secret|body|string| 是 | 2FA密钥|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## GET 获取当前用户信息

GET /api/me

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "id": 2,
        "username": "admin",
        "password": "",
        "base_path": "/",
        "role": [
            2
        ],
        "disabled": false,
        "permission": 65535,
        "sso_id": "",
        "otp": false,
        "role_names": [
            "admin"
        ],
        "permissions": [
            {
                "path": "/",
                "permission": 65535
            }
        ]
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none|数据|none|
|»» id|integer|true|none|id|none|
|»» username|string|true|none|用户名|none|
|»» password|string|true|none|密码|none|
|»» base_path|string|true|none|根目录|none|
|»» role|[integer]|true|none||none|
|»» disabled|boolean|true|none|是否禁用|none|
|»» permission|integer|true|none|权限（无用，未来移除）|none|
|»» sso_id|string|true|none|sso id|none|
|»» otp|boolean|true|none|是否开启二步验证|none|
|»» role_names|[string]|true|none||none|
|»» permissions|[object]|true|none||none|
|»»» path|string|false|none||none|
|»»» permission|integer|false|none||none|

# fs

## POST 列出文件目录

POST /api/fs/list

> Body 请求参数

```json
{
  "path": "string",
  "password": "string",
  "page": 0,
  "per_page": 0,
  "refresh": true
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» path|body|string| 是 | 路径|none|
|» password|body|string| 是 | 密码|none|
|» page|body|integer| 是 | 页数|none|
|» per_page|body|integer| 是 | 每页数目|none|
|» refresh|body|boolean| 是 | 是否强制刷新|none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "content": [
      {
        "id": "string",
        "path": "string",
        "name": "string",
        "size": 0,
        "is_dir": true,
        "modified": "string",
        "created": "string",
        "sign": "string",
        "thumb": "string",
        "type": 0,
        "hashinfo": "string",
        "hash_info": {
          "md5": "string"
        },
        "label_list": [
          {
            "id": null,
            "type": null,
            "name": null,
            "create_time": null
          }
        ]
      }
    ],
    "total": 0,
    "readme": "string",
    "header": "string",
    "write": true,
    "provider": "string"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» content|[object]|true|none|内容|none|
|»»» id|string|true|none||none|
|»»» path|string|true|none||none|
|»»» name|string|true|none||none|
|»»» size|integer|true|none||none|
|»»» is_dir|boolean|true|none||none|
|»»» modified|string|true|none||none|
|»»» created|string|true|none||none|
|»»» sign|string|true|none||none|
|»»» thumb|string|true|none||none|
|»»» type|integer|true|none||none|
|»»» hashinfo|string|true|none||none|
|»»» hash_info|object|true|none||none|
|»»»» md5|string|true|none||none|
|»»» label_list|[object]¦null|true|none||none|
|»»»» id|integer|true|none||none|
|»»»» type|integer|true|none||none|
|»»»» name|string|true|none||none|
|»»»» create_time|string|true|none||none|
|»» total|integer|true|none|总数|none|
|»» readme|string|true|none|说明|none|
|»» header|string|true|none||none|
|»» write|boolean|true|none|是否可写入|none|
|»» provider|string|true|none||none|

## POST 获取某个文件/目录信息

POST /api/fs/get

> Body 请求参数

```json
{
    "path": "/t",
    "password": "",
    "page": 1,
    "per_page": 0,
    "refresh": false
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» path|body|string| 是 | 路径|none|
|» password|body|string| 是 | 密码|none|
|» page|body|integer| 否 ||none|
|» per_page|body|integer| 否 ||none|
|» refresh|body|boolean| 否 | 强制 刷新|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "name": "Alist V3.md",
        "size": 2618,
        "is_dir": false,
        "modified": "2024-05-17T16:05:36.4651534+08:00",
        "created": "2024-05-17T16:05:29.2001008+08:00",
        "sign": "",
        "thumb": "",
        "type": 4,
        "hashinfo": "null",
        "hash_info": null,
        "raw_url": "http://127.0.0.1:5244/p/local/Alist%20V3.md",
        "readme": "",
        "header": "",
        "provider": "Local",
        "related": null
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» name|string|true|none|文件名|none|
|»» size|integer|true|none|大小|none|
|»» is_dir|boolean|true|none|是否是文件夹|none|
|»» modified|string|true|none|修改时间|none|
|»» sign|string|true|none|签名|none|
|»» thumb|string|true|none|缩略图|none|
|»» type|integer|true|none|类型|none|
|»» raw_url|string|true|none|原始url|none|
|»» readme|string|true|none|说明|none|
|»» provider|string|true|none||none|
|»» related|null|true|none||none|
|»» created|string|true|none|创建时间|none|
|»» hashinfo|string|true|none||none|
|»» hash_info|null|true|none||none|
|»» header|string|true|none||none|

## POST 获取目录

POST /api/fs/dirs

> Body 请求参数

```json
{
    "path": "/t",
    "password": "",
    "force_root": false
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» path|body|string| 否 | 路径|none|
|» password|body|string| 否 | 密码|none|
|» force_root|body|boolean| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": [
        {
            "name": "a",
            "modified": "2023-07-19T09:48:13.695585868+08:00"
        }
    ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|[object]|true|none||none|
|»» name|string|true|none|文件夹名|none|
|»» modified|string|true|none|修改时间|none|

## POST 搜索文件或文件夹

POST /api/fs/search

> Body 请求参数

```json
{
    "parent": "/local",
    "keywords": "test",
    "scope": 0,
    "page": 1,
    "per_page": 1,
    "password": ""
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» parent|body|string| 是 | 搜索目录|none|
|» keywords|body|string| 是 | 关键词|none|
|» scope|body|integer| 是 | 搜索类型|0-全部 1-文件夹 2-文件|
|» page|body|integer| 是 | 页数|none|
|» per_page|body|integer| 是 | 每页数目|none|
|» password|body|string| 是 | 密码|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "content": [
            {
                "parent": "/m",
                "name": "4305da1e",
                "is_dir": false,
                "size": 393090,
                "type": 0
            }
        ],
        "total": 1
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» content|[object]|true|none||none|
|»»» parent|string|true|none|路径|none|
|»»» name|string|true|none|文件名|none|
|»»» is_dir|boolean|true|none|是否是文件夹|none|
|»»» size|integer|true|none|大小|none|
|»»» type|integer|true|none|类型|none|
|»» total|integer|true|none|总数|none|

## POST 新建文件夹

POST /api/fs/mkdir

> Body 请求参数

```json
{
    "path": "/tt"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|
|Content-Type|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» path|body|string| 是 | 新目录路径|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 重命名文件

POST /api/fs/rename

> Body 请求参数

```json
{
    "name": "test3",
    "path": "/阿里云盘/test2"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|
|Content-Type|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» name|body|string| 是 | 目标文件名，不支持'/'|none|
|» path|body|string| 是 | 源文件名|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 批量重命名

POST /api/fs/batch_rename

> Body 请求参数

```json
{
    "src_dir": "/m2",
    "rename_objects": [
        {
            "src_name": "test.txt",
            "new_name": "aaas2.txt"
        }
    ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|
|Content-Type|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» src_dir|body|string| 是 | 源目录|none|
|» rename_objects|body|[object]| 是 ||none|
|»» src_name|body|string| 否 | 原文件名|none|
|»» new_name|body|string| 否 | 新文件名|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|null|true|none||none|

## POST 正则重命名

POST /api/fs/regex_rename

> Body 请求参数

```json
{
    "src_dir": "/m2",
    "rename_objects": [
        {
            "src_name": "test.txt",
            "new_name": "aaas2.txt"
        }
    ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|
|Content-Type|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» src_dir|body|string| 是 | 源目录|none|
|» src_name_regex|body|string| 是 | 源文件匹配正则|none|
|» new_name_regex|body|string| 是 | 新文件名正则|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|null|true|none||none|

## POST 移动文件

POST /api/fs/move

> Body 请求参数

```json
{
  "src_dir": "string",
  "dst_dir": "string",
  "names": [
    "string"
  ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» src_dir|body|string| 是 | 源文件夹|none|
|» dst_dir|body|string| 是 | 目标文件夹|none|
|» names|body|[string]| 是 | 文件名|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 聚合移动

POST /api/fs/recursive_move

> Body 请求参数

```json
{
  "src_dir": "string",
  "dst_dir": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» src_dir|body|string| 是 | 源文件夹|none|
|» dst_dir|body|string| 是 | 目标文件夹|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 复制文件

POST /api/fs/copy

> Body 请求参数

```json
{
  "src_dir": "string",
  "dst_dir": "string",
  "names": [
    "string"
  ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» src_dir|body|string| 是 | 源文件夹|none|
|» dst_dir|body|string| 是 | 目标文件夹|none|
|» names|body|[string]| 是 | 文件名|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 删除文件或文件夹

POST /api/fs/remove

> Body 请求参数

```json
{
  "names": [
    "string"
  ],
  "dir": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» names|body|[string]| 是 | 文件名|none|
|» dir|body|string| 是 | 目录|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 删除空文件夹

POST /api/fs/remove_empty_directory

> Body 请求参数

```json
{
  "src_dir": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» src_dir|body|string| 是 | 目录|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 添加离线下载

POST /api/fs/add_offline_download

> Body 请求参数

```json
{
    "path": "/local",
    "urls": [
        "https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png"
    ],
    "tool": "SimpleHttp",
    "delete_policy": "delete_on_upload_succeed"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» urls|body|[string]| 是 | url|none|
|» path|body|string| 是 | 目标路径|none|
|» tool|body|string| 是 | 工具|可选`aria2`,`SimpleHttp`和`qBittorrent`|
|» delete_policy|body|string| 是 | 删除策略|可选`delete_on_upload_succeed`,`delete_on_upload_failed`,`delete_never`,`delete_always`|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "tasks": [
            {
                "id": "jwy7BrfZRzbI2xWg7-y",
                "name": "download https://www.baidu.com/img/20d6cf.png to (/local)",
                "state": 0,
                "status": "",
                "progress": 0,
                "error": ""
            }
        ]
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» tasks|[object]|true|none||none|
|»»» id|string|false|none||none|
|»»» name|string|false|none||none|
|»»» state|integer|false|none||none|
|»»» status|string|false|none||none|
|»»» progress|integer|false|none||none|
|»»» error|string|false|none||none|

## PUT 表单上传文件

PUT /api/fs/form

> Body 请求参数

```yaml
file: []

```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|
|Content-Type|header|string| 是 ||需要是multipart/form-data;|
|Content-Length|header|string| 是 ||文件大小|
|File-Path|header|string| 是 ||经过URL编码的完整文件路径|
|As-Task|header|string| 否 ||是否添加为任务|
|body|body|object| 否 ||none|
|» file|body|string(binary)| 是 ||文件|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "task": {
            "id": "sdH2LbjyWRk",
            "name": "upload animated_zoom.gif to [/data](/alist)",
            "state": 0,
            "status": "uploading",
            "progress": 0,
            "error": ""
        }
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» task|object|true|none||none|
|»»» id|string|true|none||none|
|»»» name|string|true|none||none|
|»»» state|integer|true|none||none|
|»»» status|string|true|none||none|
|»»» progress|integer|true|none||none|
|»»» error|string|true|none||none|

## PUT 流式上传文件

PUT /api/fs/put

> Body 请求参数

```yaml
string

```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|File-Path|header|string| 是 ||经过URL编码的完整目标文件路径|
|As-Task|header|string| 否 ||是否添加为任务|
|Content-Type|header|string| 是 ||none|
|Content-Length|header|string| 是 ||none|
|body|body|string(binary)| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "task": {
            "id": "sdH2LbjyWRk",
            "name": "upload animated_zoom.gif to [/data](/alist)",
            "state": 0,
            "status": "uploading",
            "progress": 0,
            "error": ""
        }
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» task|object|true|none||none|
|»»» id|string|true|none||none|
|»»» name|string|true|none||none|
|»»» state|integer|true|none||none|
|»»» status|string|true|none||none|
|»»» progress|integer|true|none||none|
|»»» error|string|true|none||none|

# public

## GET 获取站点设置

GET /api/public/settings

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "allow_indexed": "false",
        "allow_mounted": "false",
        "announcement": "",
        "audio_autoplay": "true",
        "audio_cover": "https://jsd.nn.ci/gh/alist-org/logo@main/logo.svg",
        "auto_update_index": "false",
        "default_page_size": "30",
        "external_previews": "{}",
        "favicon": "https://cdn.jsdelivr.net/gh/alist-org/logo@main/logo.svg",
        "filename_char_mapping": "{\"/\": \"|\"}",
        "forward_direct_link_params": "false",
        "hide_files": "/\\/README.md/i",
        "home_container": "hope_container",
        "home_icon": "🏠",
        "iframe_previews": "{\n\t\"doc,docx,xls,xlsx,ppt,pptx\": {\n\t\t\"Microsoft\":\"https://view.officeapps.live.com/op/view.aspx?src=$e_url\",\n\t\t\"Google\":\"https://docs.google.com/gview?url=$e_url&embedded=true\"\n\t},\n\t\"pdf\": {\n\t\t\"PDF.js\":\"https://alist-org.github.io/pdf.js/web/viewer.html?file=$e_url\"\n\t},\n\t\"epub\": {\n\t\t\"EPUB.js\":\"https://alist-org.github.io/static/epub.js/viewer.html?url=$e_url\"\n\t}\n}",
        "logo": "https://cdn.jsdelivr.net/gh/alist-org/logo@main/logo.svg",
        "main_color": "#1890ff",
        "ocr_api": "https://api.nn.ci/ocr/file/json",
        "package_download": "true",
        "pagination_type": "all",
        "robots_txt": "User-agent: *\nAllow: /",
        "search_index": "none",
        "settings_layout": "responsive",
        "site_title": "AList",
        "sso_login_enabled": "false",
        "sso_login_platform": "",
        "version": "v3.25.1",
        "video_autoplay": "true"
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none|数据|none|
|»» allow_indexed|string|true|none|允许索引|none|
|»» allow_mounted|string|true|none|允许挂载|none|
|»» announcement|string|true|none|公告|none|
|»» audio_autoplay|string|true|none|自动播放音频|none|
|»» audio_cover|string|true|none|音频封面|none|
|»» auto_update_index|string|true|none|自动更新索引|none|
|»» default_page_size|string|true|none|默认分页数|none|
|»» external_previews|string|true|none|外部预览|none|
|»» favicon|string|true|none|网站图标|none|
|»» filename_char_mapping|string|true|none||none|
|»» forward_direct_link_params|string|true|none||none|
|»» hide_files|string|true|none|隐藏文件|none|
|»» home_container|string|true|none|主页容器|none|
|»» home_icon|string|true|none|主页图标|none|
|»» iframe_previews|string|true|none|iframe预览设置|none|
|»» logo|string|true|none|logo|none|
|»» main_color|string|true|none|主题颜色|none|
|»» ocr_api|string|true|none|pcr接口|none|
|»» package_download|string|true|none|打包下载|none|
|»» pagination_type|string|true|none||none|
|»» robots_txt|string|true|none|robots文件|none|
|»» search_index|string|true|none||none|
|»» settings_layout|string|true|none||none|
|»» site_title|string|true|none|站点标题|none|
|»» sso_login_enabled|string|true|none|启用sso登录|none|
|»» sso_login_platform|string|true|none|sso登录平台|none|
|»» version|string|true|none|版本|none|
|»» video_autoplay|string|true|none|视频自动播放|none|

## GET ping检测

GET /ping

连通性ping检测

> 返回示例

> 200 Response

```json
pong
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

# admin/meta

## GET 列出元信息

GET /api/admin/meta/list

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 否 ||页数|
|per_page|query|string| 否 ||每页个数|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "content": [
            {
                "id": 1,
                "path": "/a",
                "password": "i",
                "p_sub": false,
                "write": false,
                "w_sub": false,
                "hide": "",
                "h_sub": false,
                "readme": "",
                "r_sub": false
            }
        ],
        "total": 1
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none|数据|none|
|»» content|[object]|true|none|内容|none|
|»»» id|integer|false|none|id|none|
|»»» path|string|false|none|路径|none|
|»»» password|string|false|none|密码|none|
|»»» p_sub|boolean|false|none|密码是否应用到子文件夹|none|
|»»» write|boolean|false|none|是否允许写入|none|
|»»» w_sub|boolean|false|none|是否允许写入引用到子文件夹|none|
|»»» hide|string|false|none|隐藏|none|
|»»» h_sub|boolean|false|none|隐藏是否应用到子文件夹|none|
|»»» readme|string|false|none|说明|none|
|»»» r_sub|boolean|false|none|说明是否应用到子文件夹|none|
|»» total|integer|true|none|总数|none|

## GET 获取元信息

GET /api/admin/meta/get

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 是 ||元信息id|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "id": 1,
        "path": "/a",
        "password": "c",
        "p_sub": false,
        "write": false,
        "w_sub": false,
        "hide": "",
        "h_sub": false,
        "readme": "",
        "r_sub": false
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» id|integer|true|none|id|none|
|»» path|string|true|none|路径|none|
|»» password|string|true|none|密码|none|
|»» p_sub|boolean|true|none|密码是否应用到子文件夹|none|
|»» write|boolean|true|none|开启写入|none|
|»» w_sub|boolean|true|none|开启写入是否应用到子文件夹|none|
|»» hide|string|true|none|隐藏|none|
|»» h_sub|boolean|true|none|隐藏是否应用到子文件夹|none|
|»» readme|string|true|none|说明|none|
|»» r_sub|boolean|true|none|说明是否应用到子文件夹|none|

## POST 新增元信息

POST /api/admin/meta/create

> Body 请求参数

```json
{
    "id": 0,
    "path": "/a",
    "password": "c",
    "p_sub": false,
    "write": false,
    "w_sub": false,
    "hide": "",
    "h_sub": false,
    "readme": "",
    "r_sub": false
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» id|body|integer| 是 | id|none|
|» path|body|string| 是 | 路径|none|
|» password|body|string| 是 | 密码|none|
|» p_sub|body|boolean| 是 | 密码是否应用到子文件夹|none|
|» write|body|boolean| 是 | 开启写入|none|
|» w_sub|body|boolean| 是 | 开启写入是否应用到子文件夹|none|
|» hide|body|string| 是 | 隐藏|none|
|» h_sub|body|boolean| 是 | 隐藏是否应用到子文件夹|none|
|» readme|body|string| 是 | 说明|none|
|» r_sub|body|boolean| 是 | 说明是否应用到子文件夹|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|null|true|none||none|

## POST 更新元信息

POST /api/admin/meta/update

> Body 请求参数

```json
{
    "id": 0,
    "path": "/a",
    "password": "c",
    "p_sub": false,
    "write": false,
    "w_sub": false,
    "hide": "",
    "h_sub": false,
    "readme": "",
    "r_sub": false
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» id|body|integer| 是 | id|none|
|» path|body|string| 是 | 路径|none|
|» password|body|string| 是 | 密码|none|
|» p_sub|body|boolean| 是 | 密码是否应用到子文件夹|none|
|» write|body|boolean| 是 | 开启写入|none|
|» w_sub|body|boolean| 是 | 开启写入是否应用到子文件夹|none|
|» hide|body|string| 是 | 隐藏|none|
|» h_sub|body|boolean| 是 | 隐藏是否应用到子文件夹|none|
|» readme|body|string| 是 | 说明|none|
|» r_sub|body|boolean| 是 | 说明是否应用到子文件夹|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|null|true|none||none|

## POST 删除元信息

POST /api/admin/meta/delete

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 是 ||none|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

# admin/user

## GET 列出所有用户

GET /api/admin/user/list

列出所有用户的信息

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "content": [
            {
                "id": 1,
                "username": "admin",
                "password": "",
                "base_path": "/",
                "role": 2,
                "disabled": false,
                "permission": 0,
                "sso_id": ""
            },
            {
                "id": 2,
                "username": "guest",
                "password": "",
                "base_path": "/",
                "role": 1,
                "disabled": true,
                "permission": 0,
                "sso_id": ""
            },
            {
                "id": 3,
                "username": "N",
                "password": "",
                "base_path": "/",
                "role": 0,
                "disabled": false,
                "permission": 256,
                "sso_id": ""
            }
        ],
        "total": 3
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» content|[object]|true|none||none|
|»»» id|integer|true|none|id|none|
|»»» username|string|true|none|用户名|none|
|»»» password|string|true|none|密码|none|
|»»» base_path|string|true|none|基本路径|none|
|»»» role|integer|true|none|角色|none|
|»»» disabled|boolean|true|none|是否禁用|none|
|»»» permission|integer|true|none|权限|none|
|»»» sso_id|string|true|none|sso id|none|
|»» total|integer|true|none|总数|none|

## GET 列出某个用户

GET /api/admin/user/get

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 是 ||none|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "id": 1,
        "username": "admin",
        "password": "",
        "base_path": "/",
        "role": 2,
        "disabled": false,
        "permission": 0,
        "sso_id": ""
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» id|integer|true|none|id|none|
|»» username|string|true|none|用户名|none|
|»» password|string|true|none|密码|none|
|»» base_path|string|true|none|基本路径|none|
|»» role|integer|true|none|角色|none|
|»» disabled|boolean|true|none|是否禁用|none|
|»» permission|integer|true|none|权限|none|
|»» sso_id|string|true|none|sso id|none|

## POST 新建用户

POST /api/admin/user/create

> Body 请求参数

```json
{
    "id": 0,
    "username": "a",
    "password": "123456",
    "base_path": "/",
    "role": 0,
    "permission": 60,
    "disabled": false,
    "sso_id": ""
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» id|body|integer| 否 | id|none|
|» username|body|string| 是 | 用户名|none|
|» password|body|string| 否 | 密码|none|
|» base_path|body|string| 否 | 基本路径|none|
|» role|body|integer| 否 | 角色|none|
|» permission|body|integer| 否 | 权限|none|
|» disabled|body|boolean| 否 | 是否禁用|none|
|» sso_id|body|string| 否 | sso id|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 更新用户信息

POST /api/admin/user/update

> Body 请求参数

```json
{
    "id": 0,
    "username": "a",
    "password": "123456",
    "base_path": "/",
    "role": 0,
    "permission": 60,
    "disabled": false,
    "sso_id": ""
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» id|body|integer| 是 | id|none|
|» username|body|string| 是 | 用户名|none|
|» password|body|string| 否 | 密码|none|
|» base_path|body|string| 否 | 基本路径|none|
|» role|body|integer| 否 | 角色|none|
|» permission|body|integer| 否 | 权限|none|
|» disabled|body|boolean| 否 | 是否禁用|none|
|» sso_id|body|string| 否 | sso id|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 取消某个用户的两步验证

POST /api/admin/user/cancel_2fa

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 是 ||none|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 删除用户

POST /api/admin/user/delete

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 是 ||none|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 删除用户缓存

POST /api/admin/user/del_cache

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|username|query|string| 是 ||none|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

# admin/storage

## POST 创建存储

POST /api/admin/storage/create

> Body 请求参数

```json
{
    "mount_path": "/lll",
    "order": 0,
    "remark": "",
    "cache_expiration": 30,
    "web_proxy": false,
    "webdav_policy": "native_proxy",
    "down_proxy_url": "",
    "extract_folder": "front",
    "enable_sign": false,
    "driver": "Local",
    "order_by": "name",
    "order_direction": "asc",
    "addition": "{\"root_folder_path\":\"/\",\"thumbnail\":false,\"thumb_cache_folder\":\"\",\"show_hidden\":true,\"mkdir_perm\":\"777\"}"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|
|body|body|object| 否 ||none|
|» id|body|string| 否 | ID|none|
|» mount_path|body|string| 是 | 挂载路径|none|
|» order|body|integer| 否 | 排序|none|
|» driver|body|string| 是 | 驱动|none|
|» remark|body|string| 否 | 备注名|none|
|» cache_expiration|body|integer| 否 | 缓存过期时间|none|
|» status|body|string| 是 ||none|
|» web_proxy|body|boolean| 是 | web代理|none|
|» webdav_policy|body|string| 否 | webdav策略|none|
|» down_proxy_url|body|string| 否 | 下载代理|none|
|» order_by|body|string| 是 | 排序方式|none|
|» extract_folder|body|string| 是 | 提取目录|none|
|» order_direction|body|string| 是 | 排序方向|none|
|» addition|body|string| 是 | 额外信息|none|
|» enable_sign|body|string| 是 | 启用签名|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "id": 7
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|object|true|none|data|data|
|»» id|integer|true|none||none|

## POST 更新存储

POST /api/admin/storage/update

> Body 请求参数

```json
{
    "mount_path": "/lll",
    "order": 0,
    "remark": "",
    "cache_expiration": 30,
    "web_proxy": false,
    "webdav_policy": "native_proxy",
    "down_proxy_url": "",
    "extract_folder": "front",
    "enable_sign": false,
    "driver": "Local",
    "order_by": "name",
    "order_direction": "asc",
    "addition": "{\"root_folder_path\":\"/\",\"thumbnail\":false,\"thumb_cache_folder\":\"\",\"show_hidden\":true,\"mkdir_perm\":\"777\"}"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|
|body|body|object| 否 ||none|
|» id|body|string| 否 | ID|none|
|» mount_path|body|string| 是 | 挂载路径|none|
|» order|body|integer| 否 | 排序|none|
|» driver|body|string| 是 | 驱动|none|
|» remark|body|string| 否 | 备注名|none|
|» cache_expiration|body|integer| 否 | 缓存过期时间|none|
|» status|body|string| 是 ||none|
|» web_proxy|body|boolean| 是 | web代理|none|
|» webdav_policy|body|string| 否 | webdav策略|none|
|» down_proxy_url|body|string| 否 | 下载代理|none|
|» order_by|body|string| 是 | 排序方式|none|
|» extract_folder|body|string| 是 | 提取目录|none|
|» order_direction|body|string| 是 | 排序方向|none|
|» addition|body|string| 是 | 额外信息|none|
|» enable_sign|body|string| 是 | 启用签名|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "id": 7
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|object|true|none|data|data|
|»» id|integer|true|none||none|

## GET 列出存储列表

GET /api/admin/storage/list

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|string| 否 ||页数|
|per_page|query|string| 否 ||每页数目|
|Authorization|header|string| 是 ||token|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "content": [
            {
                "id": 1,
                "mount_path": "/lll",
                "order": 0,
                "driver": "Local",
                "cache_expiration": 0,
                "status": "work",
                "addition": "{\"root_folder_path\":\"/root/www\",\"thumbnail\":false,\"thumb_cache_folder\":\"\",\"show_hidden\":true,\"mkdir_perm\":\"777\"}",
                "remark": "",
                "modified": "2023-07-19T09:46:38.868739912+08:00",
                "disabled": false,
                "enable_sign": false,
                "order_by": "name",
                "order_direction": "asc",
                "extract_folder": "front",
                "web_proxy": false,
                "webdav_policy": "native_proxy",
                "down_proxy_url": ""
            }
        ],
        "total": 5
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|object|true|none||none|
|»» content|[object]|true|none||none|
|»»» id|integer|false|none|id|id|
|»»» mount_path|string|false|none|挂载路径|挂载路径|
|»»» order|integer|false|none|排序|顺序|
|»»» driver|string|false|none|驱动|驱动类型|
|»»» cache_expiration|integer|false|none|缓存过期时间|缓存时间|
|»»» status|string|false|none|状态|状态|
|»»» addition|string|false|none|额外信息|额外信息|
|»»» remark|string|false|none|备注|备注名|
|»»» modified|string|false|none|修改时间|修改时间|
|»»» disabled|boolean|false|none|禁用|是否被禁用|
|»»» enable_sign|boolean|false|none|启用签名|none|
|»»» order_by|string|false|none|排序|排序方式|
|»»» order_direction|string|false|none|排序方向|排序方向|
|»»» extract_folder|string|false|none|提取文件夹|提取目录顺序|
|»»» web_proxy|boolean|false|none|web代理|http代理|
|»»» webdav_policy|string|false|none|webdav代理|webdav策略|
|»»» down_proxy_url|string|false|none|下载代理url|下载代理url|
|»» total|integer|true|none|总数|none|

## POST 启用存储

POST /api/admin/storage/enable

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|integer| 是 ||存储id|
|Authorization|header|string| 是 ||token|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|null|true|none|data|data|

## POST 禁用存储

POST /api/admin/storage/disable

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 是 ||存储id|
|Authorization|header|string| 是 ||token|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|null|true|none|data|data|

## GET 查询指定存储信息

GET /api/admin/storage/get

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 是 ||存储id|
|Authorization|header|string| 是 ||token|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "id": 2,
        "mount_path": "/aa",
        "order": 1,
        "driver": "Aliyundrive",
        "cache_expiration": 30,
        "status": "work",
        "addition": "{\"root_folder_id\":\"\",\"refresh_token\":\"\",\"order_by\":\"size\",\"order_direction\":\"ASC\",\"rapid_upload\":false}",
        "remark": "",
        "modified": "2022-11-26T21:50:44.142348853+08:00",
        "disabled": false,
        "order_by": "",
        "order_direction": "",
        "extract_folder": "front",
        "web_proxy": false,
        "webdav_policy": "302_redirect",
        "down_proxy_url": ""
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» id|integer|true|none|id|none|
|»» mount_path|string|true|none|挂载路径|none|
|»» order|integer|true|none|排序|none|
|»» driver|string|true|none|驱动|none|
|»» cache_expiration|integer|true|none|缓存过期时间|none|
|»» status|string|true|none|状态|none|
|»» addition|string|true|none|额外信息|none|
|»» remark|string|true|none|备注|none|
|»» modified|string|true|none|修改时间|none|
|»» disabled|boolean|true|none|是否被禁用|none|
|»» order_by|string|true|none|排序方式|none|
|»» order_direction|string|true|none|排序方向|none|
|»» extract_folder|string|true|none|提取目录|none|
|»» web_proxy|boolean|true|none|web代理|none|
|»» webdav_policy|string|true|none|webdav策略|none|
|»» down_proxy_url|string|true|none|下载代理|none|

## POST 删除指定存储

POST /api/admin/storage/delete

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 否 ||存储id|
|Authorization|header|string| 是 ||token|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|null|true|none|data|data|

## POST 重新加载所有存储

POST /api/admin/storage/load_all

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

# admin/driver

## GET 查询所有驱动配置模板列表

GET /api/admin/driver/list

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "115 Cloud": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "cookie",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "one of QR code token and cookie required"
                },
                {
                    "name": "qrcode_token",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "one of QR code token and cookie required"
                },
                {
                    "name": "qrcode_source",
                    "type": "select",
                    "default": "linux",
                    "options": "web,android,ios,linux,mac,windows,tv",
                    "required": false,
                    "help": "select the QR code device, default linux"
                },
                {
                    "name": "page_size",
                    "type": "number",
                    "default": "56",
                    "options": "",
                    "required": false,
                    "help": "list api per page size of 115 driver"
                },
                {
                    "name": "limit_rate",
                    "type": "number",
                    "default": "2",
                    "options": "",
                    "required": false,
                    "help": "limit all api request rate (1r/[limit_rate]s)"
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "115 Cloud",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "115 Share": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "cookie",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "one of QR code token and cookie required"
                },
                {
                    "name": "qrcode_token",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "one of QR code token and cookie required"
                },
                {
                    "name": "qrcode_source",
                    "type": "select",
                    "default": "linux",
                    "options": "web,android,ios,linux,mac,windows,tv",
                    "required": false,
                    "help": "select the QR code device, default linux"
                },
                {
                    "name": "page_size",
                    "type": "number",
                    "default": "20",
                    "options": "",
                    "required": false,
                    "help": "list api per page size of 115 driver"
                },
                {
                    "name": "limit_rate",
                    "type": "number",
                    "default": "2",
                    "options": "",
                    "required": false,
                    "help": "limit all api request rate (1r/[limit_rate]s)"
                },
                {
                    "name": "share_code",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "share code of 115 share link"
                },
                {
                    "name": "receive_code",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "receive code of 115 share link"
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "115 Share",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": true,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "123Pan": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "file_name",
                    "options": "file_name,size,update_at",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "123Pan",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "123PanLink": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "origin_urls",
                    "type": "text",
                    "default": "https://vip.123pan.com/29/folder/file.mp3",
                    "options": "",
                    "required": true,
                    "help": "structure:FolderName:\n  [FileSize:][Modified:]Url"
                },
                {
                    "name": "private_key",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "uid",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "valid_duration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": false,
                    "help": "minutes"
                }
            ],
            "config": {
                "name": "123PanLink",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "123PanShare": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "sharekey",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "sharepassword",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "file_name",
                    "options": "file_name,size,update_at",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "accesstoken",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "123PanShare",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": true,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "139Yun": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "authorization",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "type",
                    "type": "select",
                    "default": "personal",
                    "options": "personal,family,personal_new",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cloud_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "139Yun",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "189Cloud": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Fill in the cookie if need captcha"
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "-11",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "189Cloud",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "-11",
                "alert": "info|You can try to use 189PC driver if this driver does not work."
            }
        },
        "189CloudPC": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "validate_code",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "-11",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "filename",
                    "options": "filename,filesize,lastOpTime",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "type",
                    "type": "select",
                    "default": "personal",
                    "options": "personal,family",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "family_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "upload_method",
                    "type": "select",
                    "default": "stream",
                    "options": "stream,rapid,old",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "upload_thread",
                    "type": "string",
                    "default": "3",
                    "options": "",
                    "required": false,
                    "help": "1<=thread<=32"
                },
                {
                    "name": "family_transfer",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "rapid_upload",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "no_use_ocr",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "189CloudPC",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "-11",
                "alert": ""
            }
        },
        "AList V2": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "url",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "access_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "AList V2",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": true,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "AList V3": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "url",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "meta_password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "pass_ua_to_upsteam",
                    "type": "bool",
                    "default": "true",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "AList V3",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "Alias": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "paths",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "Alias",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": true,
                "no_upload": true,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "Aliyundrive": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "root",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,updated_at,created_at",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "ASC,DESC",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "rapid_upload",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "internal_upload",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "Aliyundrive",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "root",
                "alert": "warning|There may be an infinite loop bug in this driver.\nDeprecated, no longer maintained and will be removed in a future version.\nWe recommend using the official driver AliyundriveOpen."
            }
        },
        "AliyundriveOpen": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "drive_type",
                    "type": "select",
                    "default": "default",
                    "options": "default,resource,backup",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "root",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,updated_at,created_at",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "ASC,DESC",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "oauth_token_url",
                    "type": "string",
                    "default": "https://api.nn.ci/alist/ali_open/token",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Keep it empty if you don't have one"
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Keep it empty if you don't have one"
                },
                {
                    "name": "remove_way",
                    "type": "select",
                    "default": "",
                    "options": "trash,delete",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "rapid_upload",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "If you enable this option, the file will be uploaded to the server first, so the progress will be incorrect"
                },
                {
                    "name": "internal_upload",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "If you are using Aliyun ECS is located in Beijing, you can turn it on to boost the upload speed"
                },
                {
                    "name": "livp_download_format",
                    "type": "select",
                    "default": "jpeg",
                    "options": "jpeg,mov",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "AliyundriveOpen",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "root",
                "alert": ""
            }
        },
        "AliyundriveShare": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "share_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "share_pwd",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "root",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,updated_at,created_at",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "ASC,DESC",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "AliyundriveShare",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": true,
                "need_ms": false,
                "default_root": "root",
                "alert": ""
            }
        },
        "BaiduNetdisk": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "name",
                    "options": "name,time,size",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "download_api",
                    "type": "select",
                    "default": "official",
                    "options": "official,crack",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "iYCeC9g08h5vuP9UqvPHKKSVrKFXGa1v",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "jXiFMOPVPCWlO2M5CwWQzffpNPaGTRBG",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "custom_crack_ua",
                    "type": "string",
                    "default": "netdisk",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "upload_thread",
                    "type": "string",
                    "default": "3",
                    "options": "",
                    "required": false,
                    "help": "1<=thread<=32"
                },
                {
                    "name": "upload_api",
                    "type": "string",
                    "default": "https://d.pcs.baidu.com",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "custom_upload_part_size",
                    "type": "number",
                    "default": "0",
                    "options": "",
                    "required": false,
                    "help": "0 for auto"
                }
            ],
            "config": {
                "name": "BaiduNetdisk",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "BaiduPhoto": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "show_type",
                    "type": "select",
                    "default": "root",
                    "options": "root,root_only_album,root_only_file",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "album_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "delete_origin",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "iYCeC9g08h5vuP9UqvPHKKSVrKFXGa1v",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "jXiFMOPVPCWlO2M5CwWQzffpNPaGTRBG",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "upload_thread",
                    "type": "string",
                    "default": "3",
                    "options": "",
                    "required": false,
                    "help": "1<=thread<=32"
                }
            ],
            "config": {
                "name": "BaiduPhoto",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "BaiduShare": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "surl",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "pwd",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "BDUSS",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "BaiduShare",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": true,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "ChaoXingGroupDrive": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "user_name",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "bbsid",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "-1",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "ChaoXingGroupDrive",
                "local_sort": false,
                "only_local": false,
                "only_proxy": true,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "-1",
                "alert": ""
            }
        },
        "Cloudreve": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "address",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "custom_ua",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_thumb_and_folder_size",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "Cloudreve",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "Crypt": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "filename_encryption",
                    "type": "select",
                    "default": "off",
                    "options": "off,standard,obfuscate",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "directory_name_encryption",
                    "type": "select",
                    "default": "false",
                    "options": "false,true",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "remote_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "This is where the encrypted data stores"
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "the main password"
                },
                {
                    "name": "salt",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "If you don't know what is salt, treat it as a second password. Optional but recommended"
                },
                {
                    "name": "encrypted_suffix",
                    "type": "string",
                    "default": ".bin",
                    "options": "",
                    "required": true,
                    "help": "for advanced user only! encrypted files will have this suffix"
                },
                {
                    "name": "filename_encoding",
                    "type": "select",
                    "default": "base64",
                    "options": "base64,base32,base32768",
                    "required": true,
                    "help": "for advanced user only!"
                },
                {
                    "name": "thumbnail",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": "enable thumbnail which pre-generated under .thumbnails folder"
                },
                {
                    "name": "show_hidden",
                    "type": "bool",
                    "default": "true",
                    "options": "",
                    "required": false,
                    "help": "show hidden directories and files"
                }
            ],
            "config": {
                "name": "Crypt",
                "local_sort": true,
                "only_local": false,
                "only_proxy": true,
                "no_cache": true,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "Doge": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "bucket",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "endpoint",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "region",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "access_key_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "secret_access_key",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "session_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "custom_host",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "sign_url_expire",
                    "type": "number",
                    "default": "4",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "placeholder",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "force_path_style",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "list_object_version",
                    "type": "select",
                    "default": "v1",
                    "options": "v1,v2",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "remove_bucket",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Remove bucket name from path when using custom host."
                },
                {
                    "name": "add_filename_to_disposition",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Add filename to Content-Disposition header."
                }
            ],
            "config": {
                "name": "Doge",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "Dropbox": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "oauth_token_url",
                    "type": "string",
                    "default": "https://api.xhofe.top/alist/dropbox/token",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Keep it empty if you don't have one"
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Keep it empty if you don't have one"
                }
            ],
            "config": {
                "name": "Dropbox",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "FTP": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "address",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "FTP",
                "local_sort": true,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "FeijiPan": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "FeijiPan",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "GoogleDrive": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "root",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "such as: folder,name,modifiedTime"
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "202264815644.apps.googleusercontent.com",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "X4Z3ca8xfWDb1Voo-F9a7ZxJ",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "chunk_size",
                    "type": "number",
                    "default": "5",
                    "options": "",
                    "required": false,
                    "help": "chunk size while uploading (unit: MB)"
                }
            ],
            "config": {
                "name": "GoogleDrive",
                "local_sort": false,
                "only_local": false,
                "only_proxy": true,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "root",
                "alert": ""
            }
        },
        "GooglePhoto": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "root",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "202264815644.apps.googleusercontent.com",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "X4Z3ca8xfWDb1Voo-F9a7ZxJ",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "show_archive",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "GooglePhoto",
                "local_sort": true,
                "only_local": false,
                "only_proxy": true,
                "no_cache": false,
                "no_upload": true,
                "need_ms": false,
                "default_root": "root",
                "alert": ""
            }
        },
        "ILanZou": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "ILanZou",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "IPFS API": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "endpoint",
                    "type": "string",
                    "default": "http://127.0.0.1:5001",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "gateway",
                    "type": "string",
                    "default": "https://ipfs.io",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "IPFS API",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "Lanzou": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "type",
                    "type": "select",
                    "default": "cookie",
                    "options": "account,cookie,url",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "account",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "about 15 days valid, ignore if shareUrl is used"
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "-1",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "share_password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "baseUrl",
                    "type": "string",
                    "default": "https://pc.woozooo.com",
                    "options": "",
                    "required": true,
                    "help": "basic URL for file operation"
                },
                {
                    "name": "shareUrl",
                    "type": "string",
                    "default": "https://pan.lanzouo.com",
                    "options": "",
                    "required": true,
                    "help": "used to get the sharing page"
                },
                {
                    "name": "repair_file_info",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "To use webdav, you need to enable it"
                }
            ],
            "config": {
                "name": "Lanzou",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "-1",
                "alert": ""
            }
        },
        "Local": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "thumbnail",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "enable thumbnail"
                },
                {
                    "name": "thumb_cache_folder",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "show_hidden",
                    "type": "bool",
                    "default": "true",
                    "options": "",
                    "required": false,
                    "help": "show hidden directories and files"
                },
                {
                    "name": "mkdir_perm",
                    "type": "string",
                    "default": "777",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "recycle_bin_path",
                    "type": "string",
                    "default": "delete permanently",
                    "options": "",
                    "required": false,
                    "help": "path to recycle bin, delete permanently if empty or keep 'delete permanently'"
                }
            ],
            "config": {
                "name": "Local",
                "local_sort": true,
                "only_local": true,
                "only_proxy": false,
                "no_cache": true,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "MediaTrack": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "access_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "project_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "title",
                    "options": "updated_at,title,size",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_desc",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "MediaTrack",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "Mega_nz": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "email",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "Mega_nz",
                "local_sort": true,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "MoPan": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "phone",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "sms_code",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "input 'send' send sms "
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cloud_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "filename",
                    "options": "filename,filesize,lastOpTime",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "device_info",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "upload_thread",
                    "type": "string",
                    "default": "3",
                    "options": "",
                    "required": false,
                    "help": "1<=thread<=32"
                }
            ],
            "config": {
                "name": "MoPan",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": "warning|This network disk may store your password in clear text. Please set your password carefully"
            }
        },
        "NeteaseMusic": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "cookie",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "song_limit",
                    "type": "number",
                    "default": "200",
                    "options": "",
                    "required": false,
                    "help": "only get 200 songs by default"
                }
            ],
            "config": {
                "name": "NeteaseMusic",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "Onedrive": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "region",
                    "type": "select",
                    "default": "global",
                    "options": "global,cn,us,de",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "is_sharepoint",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "redirect_uri",
                    "type": "string",
                    "default": "https://alist.nn.ci/tool/onedrive/callback",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "site_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "chunk_size",
                    "type": "number",
                    "default": "5",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "custom_host",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Custom host for onedrive download link"
                }
            ],
            "config": {
                "name": "Onedrive",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "OnedriveAPP": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "region",
                    "type": "select",
                    "default": "global",
                    "options": "global,cn,us,de",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "tenant_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "email",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "chunk_size",
                    "type": "number",
                    "default": "5",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "custom_host",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Custom host for onedrive download link"
                }
            ],
            "config": {
                "name": "OnedriveAPP",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "PikPak": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "disable_media_link",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "PikPak",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "PikPakShare": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "share_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "share_pwd",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "PikPakShare",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": true,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "Quark": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "none",
                    "options": "none,file_type,file_name,updated_at",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "Quark",
                "local_sort": false,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "Quqi": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "phone",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Cookie can be used on multiple clients at the same time"
                },
                {
                    "name": "cdn",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "If you enable this option, the download speed can be increased, but there will be some performance loss"
                }
            ],
            "config": {
                "name": "Quqi",
                "local_sort": true,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "S3": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "bucket",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "endpoint",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "region",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "access_key_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "secret_access_key",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "session_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "custom_host",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "sign_url_expire",
                    "type": "number",
                    "default": "4",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "placeholder",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "force_path_style",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "list_object_version",
                    "type": "select",
                    "default": "v1",
                    "options": "v1,v2",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "remove_bucket",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Remove bucket name from path when using custom host."
                },
                {
                    "name": "add_filename_to_disposition",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Add filename to Content-Disposition header."
                }
            ],
            "config": {
                "name": "S3",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "SFTP": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "address",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "private_key",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "ignore_symlink_error",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "SFTP",
                "local_sort": true,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "SMB": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": ".",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "address",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "share_name",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "SMB",
                "local_sort": true,
                "only_local": true,
                "only_proxy": false,
                "no_cache": true,
                "no_upload": false,
                "need_ms": false,
                "default_root": ".",
                "alert": ""
            }
        },
        "Seafile": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "address",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "repoId",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "repoPwd",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "Seafile",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "Teambition": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "region",
                    "type": "select",
                    "default": "",
                    "options": "china,international",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "project_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "fileName",
                    "options": "fileName,fileSize,updated,created",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "Asc",
                    "options": "Asc,Desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "use_s3_upload_method",
                    "type": "bool",
                    "default": "true",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "Teambition",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "Terabox": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "download_api",
                    "type": "select",
                    "default": "official",
                    "options": "official,crack",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "name",
                    "options": "name,time,size",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "Terabox",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "Thunder": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "captcha_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "Thunder",
                "local_sort": true,
                "only_local": false,
                "only_proxy": true,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "ThunderExpert": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "login_type",
                    "type": "select",
                    "default": "user",
                    "options": "user,refresh_token",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "sign_type",
                    "type": "select",
                    "default": "algorithms",
                    "options": "algorithms,captcha_sign",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "login type is user,this is required"
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "login type is user,this is required"
                },
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "login type is refresh_token,this is required"
                },
                {
                    "name": "algorithms",
                    "type": "string",
                    "default": "HPxr4BVygTQVtQkIMwQH33ywbgYG5l4JoR,GzhNkZ8pOBsCY+7,v+l0ImTpG7c7/,e5ztohgVXNP,t,EbXUWyVVqQbQX39Mbjn2geok3/0WEkAVxeqhtx857++kjJiRheP8l77gO,o7dvYgbRMOpHXxCs,6MW8TD8DphmakaxCqVrfv7NReRRN7ck3KLnXBculD58MvxjFRqT+,kmo0HxCKVfmxoZswLB4bVA/dwqbVAYghSb,j,4scKJNdd7F27Hv7tbt",
                    "options": "",
                    "required": true,
                    "help": "sign type is algorithms,this is required"
                },
                {
                    "name": "captcha_sign",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "sign type is captcha_sign,this is required"
                },
                {
                    "name": "timestamp",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "sign type is captcha_sign,this is required"
                },
                {
                    "name": "captcha_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "device_id",
                    "type": "string",
                    "default": "9aa5c268e7bcfc197a9ad88e2fb330e5",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "Xp6vsxz_7IYVw2BB",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "Xp6vsy4tN9toTVdMSpomVdXpRmES",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_version",
                    "type": "string",
                    "default": "7.51.0.8196",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "package_name",
                    "type": "string",
                    "default": "com.xunlei.downloadprovider",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "user_agent",
                    "type": "string",
                    "default": "ANDROID-com.xunlei.downloadprovider/7.51.0.8196 netWorkType/4G appid/40 deviceName/Xiaomi_M2004j7ac deviceModel/M2004J7AC OSVersion/12 protocolVersion/301 platformVersion/10 sdkVersion/220200 Oauth2Client/0.9 (Linux 4_14_186-perf-gdcf98eab238b) (JAVA 0)",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "download_user_agent",
                    "type": "string",
                    "default": "Dalvik/2.1.0 (Linux; U; Android 12; M2004J7AC Build/SP1A.210812.016)",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "use_video_url",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "ThunderExpert",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "Trainbit": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0_000",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "AUSHELLPORTAL",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "apikey",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "Trainbit",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0_000",
                "alert": ""
            }
        },
        "UC": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "none",
                    "options": "none,file_type,file_name,updated_at",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "UC",
                "local_sort": false,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "USS": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "bucket",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "endpoint",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "operator_name",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "operator_password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "anti_theft_chain_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "sign_url_expire",
                    "type": "number",
                    "default": "4",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "USS",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "UrlTree": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "url_structure",
                    "type": "text",
                    "default": "https://jsd.nn.ci/gh/alist-org/alist/README.md\nhttps://jsd.nn.ci/gh/alist-org/alist/README_cn.md\nfolder:\n  CONTRIBUTING.md:1635:https://jsd.nn.ci/gh/alist-org/alist/CONTRIBUTING.md\n  CODE_OF_CONDUCT.md:2093:https://jsd.nn.ci/gh/alist-org/alist/CODE_OF_CONDUCT.md",
                    "options": "",
                    "required": true,
                    "help": "structure:FolderName:\n  [FileName:][FileSize:][Modified:]Url"
                },
                {
                    "name": "head_size",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": false,
                    "help": "Use head method to get file size, but it may be failed."
                }
            ],
            "config": {
                "name": "UrlTree",
                "local_sort": true,
                "only_local": false,
                "only_proxy": false,
                "no_cache": true,
                "no_upload": true,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "VTencent": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "9",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "cookie",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "tf_uid",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "Name,Size,UpdateTime,CreatTime",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "Asc,Desc",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "VTencent",
                "local_sort": false,
                "only_local": false,
                "only_proxy": true,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "9",
                "alert": ""
            }
        },
        "Virtual": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "num_file",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "num_folder",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "max_file_size",
                    "type": "number",
                    "default": "1073741824",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "min_file_size",
                    "type": "number",
                    "default": "1048576",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "Virtual",
                "local_sort": true,
                "only_local": true,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": true,
                "default_root": "",
                "alert": ""
            }
        },
        "WebDav": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "",
                    "options": "name,size,modified",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "vendor",
                    "type": "select",
                    "default": "other",
                    "options": "sharepoint,other",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "address",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "username",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "password",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "tls_insecure_skip_verify",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "WebDav",
                "local_sort": true,
                "only_local": false,
                "only_proxy": true,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        },
        "WeiYun": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "native_proxy",
                    "options": "use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cookies",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "name",
                    "options": "name,size,updated_at",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "upload_thread",
                    "type": "string",
                    "default": "4",
                    "options": "",
                    "required": false,
                    "help": "4<=thread<=32"
                }
            ],
            "config": {
                "name": "WeiYun",
                "local_sort": false,
                "only_local": false,
                "only_proxy": true,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "",
                "alert": ""
            }
        },
        "WoPan": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "root_folder_id",
                    "type": "string",
                    "default": "0",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "family_id",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "Keep it empty if you want to use your personal drive"
                },
                {
                    "name": "sort_rule",
                    "type": "select",
                    "default": "name_asc",
                    "options": "name_asc,name_desc,time_asc,time_desc,size_asc,size_desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "access_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                }
            ],
            "config": {
                "name": "WoPan",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "0",
                "alert": ""
            }
        },
        "YandexDisk": {
            "common": [
                {
                    "name": "mount_path",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": "The path you want to mount to, it is unique and cannot be repeated"
                },
                {
                    "name": "order",
                    "type": "number",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": "use to sort"
                },
                {
                    "name": "remark",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "cache_expiration",
                    "type": "number",
                    "default": "30",
                    "options": "",
                    "required": true,
                    "help": "The cache expiration time for this storage"
                },
                {
                    "name": "web_proxy",
                    "type": "bool",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "webdav_policy",
                    "type": "select",
                    "default": "302_redirect",
                    "options": "302_redirect,use_proxy_url,native_proxy",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "down_proxy_url",
                    "type": "text",
                    "default": "",
                    "options": "",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "extract_folder",
                    "type": "select",
                    "default": "",
                    "options": "front,back",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "enable_sign",
                    "type": "bool",
                    "default": "false",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "additional": [
                {
                    "name": "refresh_token",
                    "type": "string",
                    "default": "",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "order_by",
                    "type": "select",
                    "default": "name",
                    "options": "name,path,created,modified,size",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "order_direction",
                    "type": "select",
                    "default": "asc",
                    "options": "asc,desc",
                    "required": false,
                    "help": ""
                },
                {
                    "name": "root_folder_path",
                    "type": "string",
                    "default": "/",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_id",
                    "type": "string",
                    "default": "a78d5a69054042fa936f6c77f9a0ae8b",
                    "options": "",
                    "required": true,
                    "help": ""
                },
                {
                    "name": "client_secret",
                    "type": "string",
                    "default": "9c119bbb04b346d2a52aa64401936b2b",
                    "options": "",
                    "required": true,
                    "help": ""
                }
            ],
            "config": {
                "name": "YandexDisk",
                "local_sort": false,
                "only_local": false,
                "only_proxy": false,
                "no_cache": false,
                "no_upload": false,
                "need_ms": false,
                "default_root": "/",
                "alert": ""
            }
        }
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» 115 Cloud|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» 115 Share|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» 123Pan|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» 123PanLink|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» 123PanShare|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» 139Yun|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» 189Cloud|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» 189CloudPC|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» AList V2|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» AList V3|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Alias|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|false|none||none|
|»»»» type|string|false|none||none|
|»»»» default|string|false|none||none|
|»»»» options|string|false|none||none|
|»»»» required|boolean|false|none||none|
|»»»» help|string|false|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Aliyundrive|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» AliyundriveOpen|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» AliyundriveShare|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» BaiduNetdisk|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» BaiduPhoto|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» BaiduShare|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» ChaoXingGroupDrive|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Cloudreve|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Crypt|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Doge|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Dropbox|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» FTP|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» FeijiPan|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» GoogleDrive|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» GooglePhoto|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» ILanZou|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» IPFS API|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Lanzou|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Local|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» MediaTrack|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Mega_nz|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» MoPan|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» NeteaseMusic|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Onedrive|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» OnedriveAPP|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» PikPak|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» PikPakShare|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Quark|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Quqi|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» S3|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» SFTP|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» SMB|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Seafile|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Teambition|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Terabox|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Thunder|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» ThunderExpert|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Trainbit|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» UC|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» USS|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» UrlTree|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» VTencent|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» Virtual|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» WebDav|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» WeiYun|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» WoPan|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|
|»» YandexDisk|object|true|none||none|
|»»» common|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» additional|[object]|true|none||none|
|»»»» name|string|true|none||none|
|»»»» type|string|true|none||none|
|»»»» default|string|true|none||none|
|»»»» options|string|true|none||none|
|»»»» required|boolean|true|none||none|
|»»»» help|string|true|none||none|
|»»» config|object|true|none||none|
|»»»» name|string|true|none||none|
|»»»» local_sort|boolean|true|none||none|
|»»»» only_local|boolean|true|none||none|
|»»»» only_proxy|boolean|true|none||none|
|»»»» no_cache|boolean|true|none||none|
|»»»» no_upload|boolean|true|none||none|
|»»»» need_ms|boolean|true|none||none|
|»»»» default_root|string|true|none||none|
|»»»» alert|string|true|none||none|

## GET 列出驱动名列表

GET /api/admin/driver/names

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||token|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": [
        "IPFS API",
        "BaiduPhoto",
        "Onedrive",
        "PikPakShare",
        "Seafile",
        "AList V3",
        "Alias",
        "Crypt",
        "123PanLink",
        "ChaoXingGroupDrive",
        "OnedriveAPP",
        "Teambition",
        "WoPan",
        "ThunderExpert",
        "Mega_nz",
        "123PanShare",
        "Cloudreve",
        "UC",
        "115 Share",
        "MoPan",
        "AliyundriveOpen",
        "GooglePhoto",
        "UrlTree",
        "ILanZou",
        "189Cloud",
        "Local",
        "139Yun",
        "BaiduNetdisk",
        "Quqi",
        "115 Cloud",
        "FeijiPan",
        "S3",
        "Thunder",
        "PikPak",
        "BaiduShare",
        "Aliyundrive",
        "Dropbox",
        "NeteaseMusic",
        "WeiYun",
        "123Pan",
        "Doge",
        "189CloudPC",
        "GoogleDrive",
        "Quark",
        "YandexDisk",
        "MediaTrack",
        "AList V2",
        "Virtual",
        "USS",
        "AliyundriveShare",
        "Lanzou",
        "WebDav",
        "VTencent",
        "Trainbit",
        "FTP",
        "SFTP",
        "SMB",
        "Terabox"
    ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|[string]|true|none||none|

## GET 列出特定驱动信息

GET /api/admin/driver/info

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|driver|query|string| 是 ||none|
|Authorization|header|string| 是 ||token|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "common": [
            {
                "name": "mount_path",
                "type": "string",
                "default": "",
                "options": "",
                "required": true,
                "help": "The path you want to mount to, it is unique and cannot be repeated"
            },
            {
                "name": "order",
                "type": "number",
                "default": "",
                "options": "",
                "required": false,
                "help": "use to sort"
            },
            {
                "name": "remark",
                "type": "text",
                "default": "",
                "options": "",
                "required": false,
                "help": ""
            },
            {
                "name": "cache_expiration",
                "type": "number",
                "default": "30",
                "options": "",
                "required": true,
                "help": "The cache expiration time for this storage"
            },
            {
                "name": "webdav_policy",
                "type": "select",
                "default": "native_proxy",
                "options": "use_proxy_url,native_proxy",
                "required": true,
                "help": ""
            },
            {
                "name": "down_proxy_url",
                "type": "text",
                "default": "",
                "options": "",
                "required": false,
                "help": ""
            },
            {
                "name": "extract_folder",
                "type": "select",
                "default": "",
                "options": "front,back",
                "required": false,
                "help": ""
            },
            {
                "name": "enable_sign",
                "type": "bool",
                "default": "false",
                "options": "",
                "required": true,
                "help": ""
            }
        ],
        "additional": [
            {
                "name": "cookie",
                "type": "string",
                "default": "",
                "options": "",
                "required": true,
                "help": ""
            },
            {
                "name": "root_folder_id",
                "type": "string",
                "default": "0",
                "options": "",
                "required": true,
                "help": ""
            },
            {
                "name": "order_by",
                "type": "select",
                "default": "none",
                "options": "none,file_type,file_name,updated_at",
                "required": false,
                "help": ""
            },
            {
                "name": "order_direction",
                "type": "select",
                "default": "asc",
                "options": "asc,desc",
                "required": false,
                "help": ""
            }
        ],
        "config": {
            "name": "UC",
            "local_sort": false,
            "only_local": true,
            "only_proxy": false,
            "no_cache": false,
            "no_upload": false,
            "need_ms": false,
            "default_root": "0",
            "alert": ""
        }
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|状态码|
|» message|string|true|none|信息|信息|
|» data|object|true|none||none|
|»» common|[object]|true|none|通用配置|none|
|»»» name|string|true|none|配置名|none|
|»»» type|string|true|none|类型|none|
|»»» default|string|true|none|默认值|none|
|»»» options|string|true|none|选项|none|
|»»» required|boolean|true|none|是否必须|none|
|»»» help|string|true|none|帮助信息|none|
|»» additional|[object]|true|none|额外配置|none|
|»»» name|string|true|none|配置名|none|
|»»» type|string|true|none|类型|none|
|»»» default|string|true|none|默认值|none|
|»»» options|string|true|none|选项|none|
|»»» required|boolean|true|none|是否必须|none|
|»»» help|string|true|none|帮助信息|none|
|»» config|object|true|none|配置|none|
|»»» name|string|true|none|配置名|none|
|»»» local_sort|boolean|true|none|本地排序|none|
|»»» only_local|boolean|true|none|仅本地|none|
|»»» only_proxy|boolean|true|none|仅代理|none|
|»»» no_cache|boolean|true|none|无缓存|none|
|»»» no_upload|boolean|true|none|无上传|none|
|»»» need_ms|boolean|true|none||none|
|»»» default_root|string|true|none|默认基本路径|none|
|»»» alert|string|true|none|警告信息|none|

# admin/setting

## GET 列出设置

GET /api/admin/setting/list

包括永久令牌

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|groups|query|string| 否 ||5,0-其它设置，包括aria2和令牌等|
|group|query|string| 否 ||1-站点；2-样式；3-预览；4-全局；7-单点登录|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": [
        {
            "key": "aria2_uri",
            "value": "http://localhost:6800/jsonrpc",
            "help": "",
            "type": "string",
            "options": "",
            "group": 5,
            "flag": 1
        },
        {
            "key": "aria2_secret",
            "value": "",
            "help": "",
            "type": "string",
            "options": "",
            "group": 5,
            "flag": 1
        },
        {
            "key": "token",
            "value": "alist-2a",
            "help": "",
            "type": "string",
            "options": "",
            "group": 0,
            "flag": 1
        },
        {
            "key": "index_progress",
            "value": "{\"obj_count\":0,\"is_done\":true,\"last_done_time\":null,\"error\":\"\"}",
            "help": "",
            "type": "text",
            "options": "",
            "group": 0,
            "flag": 1
        },
        {
            "key": "qbittorrent_url",
            "value": "http://a:an@localhost:8080/",
            "help": "",
            "type": "string",
            "options": "",
            "group": 0,
            "flag": 1
        },
        {
            "key": "qbittorrent_seedtime",
            "value": "0",
            "help": "",
            "type": "number",
            "options": "",
            "group": 0,
            "flag": 1
        }
    ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|[object]|true|none||none|
|»» key|string|true|none|键|none|
|»» value|string|true|none|值|none|
|»» help|string|true|none|帮助信息|none|
|»» type|string|true|none|类型|string, number, bool, select|
|»» options|string|true|none|选项|none|
|»» group|integer|true|none|分组|用于前端分组|
|»» flag|integer|true|none|标志|0 = public, 1 = private, 2 = readonly, 3 = deprecated|

## GET 获取某项设置

GET /api/admin/setting/get

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|keys|query|string| 否 ||none|
|key|query|string| 否 ||none|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "key": "hide_files",
        "value": "/\\/README.md/i",
        "help": "",
        "type": "text",
        "options": "",
        "group": 4,
        "flag": 0
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|object|true|none||none|
|»» key|string|true|none|键|none|
|»» value|string|true|none|值|none|
|»» help|string|true|none|帮助信息|none|
|»» type|string|true|none|类型|string, number, bool, select|
|»» options|string|true|none|选项|none|
|»» group|integer|true|none|分组|none|
|»» flag|integer|true|none|标志|0 = public, 1 = private, 2 = readonly, 3 = deprecated|

## POST 保存设置

POST /api/admin/setting/save

> Body 请求参数

```json
[
    {
        "key": "version",
        "value": "v3.25.1",
        "help": "",
        "type": "string",
        "options": "",
        "group": 1,
        "flag": 2
    },
    {
        "key": "site_title",
        "value": "AList",
        "help": "",
        "type": "string",
        "options": "",
        "group": 1,
        "flag": 0
    },
    {
        "key": "announcement",
        "value": "",
        "help": "",
        "type": "text",
        "options": "",
        "group": 1,
        "flag": 0
    },
    {
        "key": "pagination_type",
        "value": "all",
        "help": "",
        "type": "select",
        "options": "all,pagination,load_more,auto_load_more",
        "group": 1,
        "flag": 0
    },
    {
        "key": "default_page_size",
        "value": "30",
        "help": "",
        "type": "number",
        "options": "",
        "group": 1,
        "flag": 0
    },
    {
        "key": "allow_indexed",
        "value": "false",
        "help": "",
        "type": "bool",
        "options": "",
        "group": 1,
        "flag": 0
    },
    {
        "key": "allow_mounted",
        "value": "false",
        "help": "",
        "type": "bool",
        "options": "",
        "group": 1,
        "flag": 0
    },
    {
        "key": "robots_txt",
        "value": "User-agent: *\nAllow: /",
        "help": "",
        "type": "text",
        "options": "",
        "group": 1,
        "flag": 0
    }
]
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|array[object]| 否 | 数组|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 删除设置

POST /api/admin/setting/delete

仅用于弃用的设置

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|key|query|string| 是 ||none|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

## POST 重置令牌

POST /api/admin/setting/reset_token

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": "alist-9d"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|string|true|none|新令牌|none|

## POST 设置aria2

POST /api/admin/setting/set_aria2

> Body 请求参数

```json
{
  "uri": "string",
  "secret": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» uri|body|string| 是 | aria2地址|none|
|» secret|body|string| 是 | aria2密钥|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": "1.36.0"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|string|true|none|aria2版本|none|

## POST 设置qBittorrent

POST /api/admin/setting/set_qbit

> Body 请求参数

```json
{
  "url": "string",
  "seedtime": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» url|body|string| 是 | qBittorrent链接|none|
|» seedtime|body|string| 是 | 做种时间|none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": "1.36.0"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|string|true|none||none|

# admin/task/upload

## POST 获取任务信息

POST /api/admin/task/upload/info

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|tid|query|string| 否 ||任务id|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": [
        {
            "id": "1",
            "name": "upload 1.png to [/s](/test)",
            "state": "succeeded",
            "status": "",
            "progress": 100,
            "error": ""
        }
    ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|[object]|true|none||none|
|»» id|string|false|none|id|none|
|»» name|string|false|none|任务名|none|
|»» state|string|false|none|任务完成状态|none|
|»» status|string|false|none||none|
|»» progress|integer|false|none|进度|none|
|»» error|string|false|none|错误信息|none|

## GET 获取已完成任务

GET /api/admin/task/upload/done

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": [
        {
            "id": "1",
            "name": "upload 1.png to [/s](/test)",
            "state": "succeeded",
            "status": "",
            "progress": 100,
            "error": ""
        }
    ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|[object]|true|none||none|
|»» id|string|false|none|id|none|
|»» name|string|false|none|任务名|none|
|»» state|string|false|none|任务完成状态|none|
|»» status|string|false|none||none|
|»» progress|integer|false|none|进度|none|
|»» error|string|false|none|错误信息|none|

## GET 获取未完成任务

GET /api/admin/task/upload/undone

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": [
        {
            "id": "1",
            "name": "upload 1.png to [/s](/test)",
            "state": "succeeded",
            "status": "",
            "progress": 100,
            "error": ""
        }
    ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|[object]|true|none||none|
|»» id|string|false|none|id|none|
|»» name|string|false|none|任务名|none|
|»» state|string|false|none|任务完成状态|none|
|»» status|string|false|none||none|
|»» progress|integer|false|none|进度|none|
|»» error|string|false|none|错误信息|none|

## POST 删除任务

POST /api/admin/task/upload/delete

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|tid|query|string| 是 ||任务id|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 取消任务

POST /api/admin/task/upload/cancel

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|tid|query|string| 是 ||任务id|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 重试任务

POST /api/admin/task/upload/retry

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|tid|query|string| 是 ||任务id|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 清除已完成任务

POST /api/admin/task/upload/clear_done

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

## POST 清除已成功任务

POST /api/admin/task/upload/clear_succeeded

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|状态码|none|
|» message|string|true|none|信息|none|
|» data|null|true|none||none|

# admin/tag

## GET 标签列表

GET /api/admin/label/list

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|page|query|integer| 否 ||页数|
|per_page|query|integer| 否 ||每页数量|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "content": [
      {
        "id": 0,
        "type": 0,
        "name": "string",
        "description": "string",
        "bg_color": "string",
        "create_time": "string"
      }
    ],
    "total": 0
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» content|[object]|true|none||none|
|»»» id|integer|true|none||none|
|»»» type|integer|true|none||标签类型，预留字段|
|»»» name|string|true|none||标签名称|
|»»» description|string|true|none||标签描述|
|»»» bg_color|string|true|none||标签颜色|
|»»» create_time|string|true|none||创建时间|
|»» total|integer|true|none||总数量|

## POST 创建标签

POST /api/admin/label/create

> Body 请求参数

```json
{
  "name": "string",
  "type": 0,
  "description": "string",
  "bg_color": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» name|body|string| 是 ||标签名称|
|» type|body|integer| 否 ||预留字段，标签类型，暂时不必传|
|» description|body|string| 否 ||标签描述|
|» bg_color|body|string| 否 ||标签颜色|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "id": 0
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» id|integer|true|none||标签id|

## POST 修改标签

POST /api/admin/label/update

> Body 请求参数

```json
{
  "id": 0,
  "name": "string",
  "type": "string",
  "description": "string",
  "bg_color": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» id|body|integer| 是 ||标签id|
|» name|body|string| 是 ||标签名称|
|» type|body|string| 否 ||预留字段，标签类型，暂时不必传|
|» description|body|string| 否 ||标签描述|
|» bg_color|body|string| 是 ||标签颜色|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "id": 0,
    "type": 0,
    "name": "string",
    "create_time": "string"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» id|integer|true|none||none|
|»» type|integer|true|none||none|
|»» name|string|true|none||none|
|»» create_time|string|true|none||none|

## GET 标签详情

GET /api/admin/label/get

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 否 ||标签id|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "id": 0,
    "type": 0,
    "name": "string",
    "create_time": "string"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» id|integer|true|none||none|
|»» type|integer|true|none||none|
|»» name|string|true|none||none|
|»» create_time|string|true|none||none|

## POST 删除标签

POST /api/admin/label/delete

> Body 请求参数

```json
{
  "id": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» id|body|string| 是 ||标签Id|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|null|true|none||none|

## POST 文件绑定标签

POST /api/admin/label_file_binding/create

> Body 请求参数

```json
{
  "label_ids": "string",
  "name": "string",
  "id": "string",
  "path": "string",
  "size": 0,
  "is_dir": true,
  "modified": "string",
  "created": "string",
  "sign": "string",
  "thumb": "string",
  "type": 0,
  "hashinfo": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» label_ids|body|string| 是 ||标签id，多个以英文逗号分隔|
|» name|body|string| 是 ||文件名称|
|» id|body|string| 是 ||none|
|» path|body|string| 是 ||none|
|» size|body|integer| 是 ||none|
|» is_dir|body|boolean| 是 ||none|
|» modified|body|string| 是 ||none|
|» created|body|string| 是 ||none|
|» sign|body|string| 是 ||none|
|» thumb|body|string| 是 ||none|
|» type|body|integer| 是 ||none|
|» hashinfo|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "msg": "string"
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» msg|string|true|none||none|

## GET 根据文件名称获取对应标签

GET /api/admin/label_file_binding/get

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|file_name|query|string| 否 ||文件名称|
|Authorization|header|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": [
    {
      "id": 0,
      "type": 0,
      "name": "string",
      "create_time": "string"
    }
  ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|[object]|true|none||none|
|»» id|integer|true|none||none|
|»» type|integer|true|none||none|
|»» name|string|true|none||none|
|»» create_time|string|true|none||none|

## POST 解除文件标签绑定关系

POST /api/admin/label_file_binding/delete

> Body 请求参数

```json
{
  "label_id": "string",
  "file_name": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» label_id|body|string| 是 ||标签id|
|» file_name|body|string| 是 ||文件名称|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|null|true|none||none|

## GET 根据标签获取文件信息

GET /api/admin/label_file_binding/get_file_by_label

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|label_id|query|string| 否 ||多个逗号分隔|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": [
        {
            "id": "254070892733601",
            "path": "/redis-desktop-manager-0.8.3-2550.dmg",
            "name": "redis-desktop-manager-0.8.3-2550.dmg",
            "size": 18295365,
            "is_dir": false,
            "modified": "2020-09-04T14:53:51+08:00",
            "created": "2018-06-28T09:50:15+08:00",
            "sign": "qbfCzvy-Fxi1SzUP8bpugeZHJNOHyE2Q5cq5NiYUup8=:0",
            "thumb": "",
            "type": 0,
            "hashinfo": "{\"md5\":\"b72197247f28f34aab543d7eebd94c05\"}",
            "label_list": [
                {
                    "id": 3,
                    "type": 1,
                    "name": "测试标签1",
                    "description": "",
                    "bg_color": "",
                    "create_time": "2025-07-03T11:57:32.601095+08:00"
                }
            ]
        },
        {
            "id": "103131660682597",
            "path": "/Navicat15安装和破解.zip",
            "name": "Navicat15安装和破解.zip",
            "size": 73800750,
            "is_dir": false,
            "modified": "2022-06-27T13:21:43+08:00",
            "created": "2020-04-30T13:53:16+08:00",
            "sign": "SRqEXqJlqZadQ_9_Nd3KyYp8AmwT-oIpQfO_gGtK46A=:0",
            "thumb": "",
            "type": 0,
            "hashinfo": "{\"md5\":\"06d2abbf9386c6cefe71ca59d56401b3\"}",
            "label_list": [
                {
                    "id": 1,
                    "type": 0,
                    "name": "测试标签",
                    "description": "",
                    "bg_color": "",
                    "create_time": "2025-07-03T15:48:03.994034+08:00"
                }
            ]
        }
    ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|[object]|true|none||none|
|»» id|string|true|none||none|
|»» path|string|true|none||none|
|»» name|string|true|none||none|
|»» size|integer|true|none||none|
|»» is_dir|boolean|true|none||none|
|»» modified|string|true|none||none|
|»» created|string|true|none||none|
|»» sign|string|true|none||none|
|»» thumb|string|true|none||none|
|»» type|integer|true|none||none|
|»» hashinfo|string|true|none||none|
|»» label_list|[object]|true|none||none|
|»»» id|integer|true|none||none|
|»»» type|integer|true|none||none|
|»»» name|string|true|none||none|
|»»» description|string|true|none||none|
|»»» bg_color|string|true|none||none|
|»»» create_time|string|true|none||none|

## POST 批量设置文件标签

POST /api/admin/label_file_binding/create_batch

> Body 请求参数

```json
{
    "items": [
        {
            "path": "/123",
            "name": "20221226_210943.jpg",
            "isDir": false,
            "labelIdList": [
                1,
                2
            ],
            "size": 0,
            "type": 0,
            "modified": "2025-08-15T00:00:00Z",
            "created": "2025-08-15T00:00:00Z",
            "sign": "",
            "thumb": "",
            "hashInfoStr": ""
        },
        {
            "path": "/123",
            "name": "-517cde7417777ca9.jpg",
            "isDir": false,
            "labelIdList": [
                1,
                2
            ],
            "size": 0,
            "type": 0,
            "modified": "2025-08-15T00:00:00Z",
            "created": "2025-08-15T00:00:00Z",
            "sign": "",
            "thumb": "",
            "hashInfoStr": ""
        }
    ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» items|body|[object]| 是 ||none|
|»» path|body|string| 是 ||none|
|»» name|body|string| 是 ||none|
|»» isDir|body|boolean| 是 ||none|
|»» labelIdList|body|[integer]| 是 ||none|
|»» size|body|integer| 是 ||none|
|»» type|body|integer| 是 ||none|
|»» modified|body|string| 是 ||none|
|»» created|body|string| 是 ||none|
|»» sign|body|string| 是 ||none|
|»» thumb|body|string| 是 ||none|
|»» hashInfoStr|body|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "failed": 0,
        "results": [
            {
                "name": "20221226_210943.jpg",
                "ok": true
            },
            {
                "name": "-517cde7417777ca9.jpg",
                "ok": true
            }
        ],
        "succeed": 2,
        "total": 2
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» failed|integer|true|none||none|
|»» results|[object]|true|none||none|
|»»» name|string|true|none||none|
|»»» ok|boolean|true|none||none|
|»» succeed|integer|true|none||none|
|»» total|integer|true|none||none|

# admin/role

## GET 获取角色列表

GET /api/admin/role/list

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "content": [
            {
                "id": 1,
                "name": "guest",
                "description": "Guest",
                "permission_scopes": [
                    {
                        "path": "/",
                        "permission": 0
                    }
                ]
            },
            {
                "id": 2,
                "name": "admin",
                "description": "Administrator",
                "permission_scopes": [
                    {
                        "path": "/",
                        "permission": 12543
                    }
                ]
            }
        ],
        "total": 2
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Success|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» content|[object]|true|none||none|
|»»» id|integer|true|none||none|
|»»» name|string|true|none||none|
|»»» description|string|true|none||none|
|»»» permission_scopes|[object]|true|none||none|
|»»»» path|string|true|none||none|
|»»»» permission|integer|true|none||none|
|»» total|integer|true|none||none|

## GET 获取角色详情

GET /api/admin/role/get

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|integer| 是 ||none|
|Authorization|header|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "id": 1,
        "name": "guest",
        "description": "Guest",
        "permission_scopes": [
            {
                "path": "/",
                "permission": 0
            }
        ]
    }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Success|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» id|integer|true|none||none|
|»» name|string|true|none||none|
|»» description|string|true|none||none|
|»» permission_scopes|[object]|true|none||none|
|»»» path|string|false|none||none|
|»»» permission|integer|false|none||none|

## POST 创建角色

POST /api/admin/role/create

> Body 请求参数

```json
{
    "id": 1,
    "name": "admin",
    "description": "Administrator role",
    "permission_scopes": [
        {
            "path": "/admin",
            "permission": 7
        }
    ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» id|body|integer| 是 ||none|
|» name|body|string| 是 ||none|
|» description|body|string| 是 ||none|
|» permission_scopes|body|[object]| 是 ||none|
|»» path|body|string| 否 ||none|
|»» permission|body|integer| 否 ||none|
|» raw_permission|body|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Created|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|null|true|none||none|

## POST 更新角色

POST /api/admin/role/update

> Body 请求参数

```json
{
    "id": 1,
    "name": "admin",
    "description": "Administrator role",
    "permission_scopes": [
        {
            "path": "/admin",
            "permission": 7
        },
        {
            "path": "/admin",
            "permission": 7
        }
    ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|Authorization|header|string| 否 ||none|
|body|body|object| 否 ||none|
|» id|body|integer| 是 ||none|
|» name|body|string| 是 ||none|
|» description|body|string| 是 ||none|
|» permission_scopes|body|[object]| 是 ||none|
|»» path|body|string| 是 ||none|
|»» permission|body|integer| 是 ||none|

> 返回示例

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|{
    "code": 200,
    "message": "success",
    "data": null
}|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|null|true|none||none|

## POST 删除角色

POST /api/admin/role/delete

> Body 请求参数

```json
{
  "id": 0
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|id|query|string| 是 ||none|
|Authorization|header|string| 是 ||none|
|body|body|object| 否 ||none|
|» id|body|integer| 是 ||none|

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|{
    "code": 200,
    "message": "success",
    "data": null
}|None|

# 数据模型

<h2 id="tocS_PermissionEntry">PermissionEntry</h2>

<a id="schemapermissionentry"></a>
<a id="schema_PermissionEntry"></a>
<a id="tocSpermissionentry"></a>
<a id="tocspermissionentry"></a>

```json
{
  "path": "/admin",
  "permission": 7
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|path|string|false|none||none|
|permission|integer|false|none||none|

<h2 id="tocS_Role">Role</h2>

<a id="schemarole"></a>
<a id="schema_Role"></a>
<a id="tocSrole"></a>
<a id="tocsrole"></a>

```json
{
  "id": 1,
  "name": "admin",
  "description": "Administrator role",
  "permission_scopes": [
    {
      "path": "/admin",
      "permission": 7
    }
  ],
  "raw_permission": "[{\"path\":\"/admin\",\"permission\":7}]"
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|id|integer|false|none||none|
|name|string|true|none||none|
|description|string|false|none||none|
|permission_scopes|[[PermissionEntry](#schemapermissionentry)]|false|none||none|
|raw_permission|string|false|none||none|

