---
title: é»èź€æšĄć
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

# é»èź€æšĄć

Base URLs:

# Authentication

# auth

## POST tokenè·ć

POST /api/auth/login

è·ćæäžȘçšæ·çäžŽæ¶JWt tokenïŒé»èź€48ć°æ¶èżæ

> Body èŻ·æ±ćæ°

```json
{
    "username": "akimio",
    "password": "JuXQMCe4m6LstB"
}
```

### èŻ·æ±ćæ°

| ćç§°      | äœçœź | ç±»ć | ćżé | äž­æć       | èŻŽæ          |
| ----------- | ------ | ------ | ------ | --------------- | --------------- |
| body        | body   | object | ćŠ    |                 | none            |
| Â» username | body   | string | æŻ    | çšæ·ć       | çšæ·ć       |
| Â» password | body   | string | æŻ    | ćŻç           | ćŻç           |
| Â» otp_code | body   | string | ćŠ    | äșæ­„éȘèŻç  | äșæ­„éȘèŻç  |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   |           | ç¶æç  |
| Â» message | string  | true   | none   |           | äżĄæŻ    |
| Â» data    | object  | true   | none   |           | data      |
| Â»Â» token | string  | true   | none   |           | token     |

## POST tokenè·ćhash

POST /api/auth/login/hash

è·ćæäžȘçšæ·çäžŽæ¶JWt tokenïŒäŒ ć„çćŻç éèŠćšæ·»ć -https://github.com/alist-org/alistćçŒććèżèĄsha256

> Body èŻ·æ±ćæ°

```json
{
    "username": "{{alist_username}}",
    "password": "{{alist_password}}"
}
```

### èŻ·æ±ćæ°

| ćç§°      | äœçœź | ç±»ć | ćżé | äž­æć       | èŻŽæ                                                                             |
| ----------- | ------ | ------ | ------ | --------------- | ---------------------------------------------------------------------------------- |
| body        | body   | object | ćŠ    |                 | none                                                                               |
| Â» username | body   | string | æŻ    | çšæ·ć       | çšæ·ć                                                                          |
| Â» password | body   | string | æŻ    | ćŻç           | hashććŻç ïŒè·ćæčćŒäžș`sha256(ćŻç -https://github.com/alist-org/alist)` |
| Â» otp_code | body   | string | ćŠ    | äșæ­„éȘèŻç  | äșæ­„éȘèŻç                                                                     |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   |           | ç¶æç  |
| Â» message | string  | true   | none   |           | äżĄæŻ    |
| Â» data    | object  | true   | none   |           | data      |
| Â»Â» token | string  | true   | none   |           | token     |

## POST çæ2FAćŻé„

POST /api/auth/2fa/generate

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°      | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ                     |
| ----------- | ------- | ------ | ------ | --------- | -------------------------- |
| Â» code     | integer | true   | none   | ç¶æç  | none                       |
| Â» message  | string  | true   | none   | äżĄæŻ    | none                       |
| Â» data     | object  | true   | none   | æ°æź    | none                       |
| Â»Â» qr     | string  | true   | none   | äșç»Žç  | äșç»Žç ćŸççdata url |
| Â»Â» secret | string  | true   | none   | ćŻé„    | none                       |

## POST éȘèŻ2FA code

POST /api/auth/2fa/verify

> Body èŻ·æ±ćæ°

```json
{
  "code": "string",
  "secret": "string"
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć    | èŻŽæ |
| ------------- | ------ | ------ | ------ | ------------ | ------ |
| Authorization | header | string | æŻ    |              | none   |
| body          | body   | object | ćŠ    |              | none   |
| Â» code       | body   | string | æŻ    | 2FAéȘèŻç  | none   |
| Â» secret     | body   | string | æŻ    | 2FAćŻé„    | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## GET è·ććœćçšæ·äżĄæŻ

GET /api/me

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | ćŠ    |           | none   |

> èżćç€șäŸ

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
        "sso_id": "",
        "otp": true
    }
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°          | ç±»ć  | ćżé | çșŠæ | äž­æć                | èŻŽæ |
| --------------- | ------- | ------ | ------ | ------------------------ | ------ |
| Â» code         | integer | true   | none   | ç¶æç                 | none   |
| Â» message      | string  | true   | none   | äżĄæŻ                   | none   |
| Â» data         | object  | true   | none   | æ°æź                   | none   |
| Â»Â» id         | integer | true   | none   | id                       | none   |
| Â»Â» username   | string  | true   | none   | çšæ·ć                | none   |
| Â»Â» password   | string  | true   | none   | ćŻç                    | none   |
| Â»Â» base_path  | string  | true   | none   | æ čçźćœ                | none   |
| Â»Â» role       | integer | true   | none   | è§èČ                   | none   |
| Â»Â» disabled   | boolean | true   | none   | æŻćŠçŠçš             | none   |
| Â»Â» permission | integer | true   | none   | æé                   | none   |
| Â»Â» sso_id     | string  | true   | none   | sso id                   | none   |
| Â»Â» otp        | boolean | true   | none   | æŻćŠćŒćŻäșæ­„éȘèŻ | none   |

# fs

## POST ććșæä»¶çźćœ

POST /api/fs/list

> Body èŻ·æ±ćæ°

```json
{
    "path": "/t",
    "password": "",
    "page": 1,
    "per_page": 0,
    "refresh": false
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć          | èŻŽæ |
| ------------- | ------ | ------- | ------ | ------------------ | ------ |
| Authorization | header | string  | æŻ    |                    | none   |
| body          | body   | object  | ćŠ    |                    | none   |
| Â» path       | body   | string  | ćŠ    | è·ŻćŸ             | none   |
| Â» password   | body   | string  | ćŠ    | ćŻç              | none   |
| Â» page       | body   | integer | ćŠ    | éĄ”æ°             | none   |
| Â» per_page   | body   | integer | ćŠ    | æŻéĄ”æ°çź       | none   |
| Â» refresh    | body   | boolean | ćŠ    | æŻćŠćŒșć¶ć·æ° | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": {
        "content": [
            {
                "name": "Alist V3.md",
                "size": 1592,
                "is_dir": false,
                "modified": "2024-05-17T13:47:55.4174917+08:00",
                "created": "2024-05-17T13:47:47.5725906+08:00",
                "sign": "",
                "thumb": "",
                "type": 4,
                "hashinfo": "null",
                "hash_info": null
            }
        ],
        "total": 1,
        "readme": "",
        "header": "",
        "write": true,
        "provider": "Local"
    }
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°           | ç±»ć   | ćżé | çșŠæ | äž­æć          | èŻŽæ |
| ---------------- | -------- | ------ | ------ | ------------------ | ------ |
| Â» code          | integer  | true   | none   | ç¶æç           | none   |
| Â» message       | string   | true   | none   | äżĄæŻ             | none   |
| Â» data          | object   | true   | none   |                    | none   |
| Â»Â» content     | [object] | true   | none   | ććźč             | none   |
| Â»Â»Â» name      | string   | true   | none   | æä»¶ć          | none   |
| Â»Â»Â» size      | integer  | true   | none   | ć€§ć°             | none   |
| Â»Â»Â» is_dir    | boolean  | true   | none   | æŻćŠæŻæä»¶ć€č | none   |
| Â»Â»Â» modified  | string   | true   | none   | äżźæčæ¶éŽ       | none   |
| Â»Â»Â» sign      | string   | true   | none   | ç­Ÿć             | none   |
| Â»Â»Â» thumb     | string   | true   | none   | çŒ©ç„ćŸ          | none   |
| Â»Â»Â» type      | integer  | true   | none   | ç±»ć             | none   |
| Â»Â»Â» created   | string   | false  | none   | ćć»șæ¶éŽ       | none   |
| Â»Â»Â» hashinfo  | string   | false  | none   |                    | none   |
| Â»Â»Â» hash_info | null     | false  | none   |                    | none   |
| Â»Â» total       | integer  | true   | none   | æ»æ°             | none   |
| Â»Â» readme      | string   | true   | none   | èŻŽæ             | none   |
| Â»Â» write       | boolean  | true   | none   | æŻćŠćŻćć„    | none   |
| Â»Â» provider    | string   | true   | none   |                    | none   |
| Â»Â» header      | string   | true   | none   |                    | none   |

## POST è·ćæäžȘæä»¶/çźćœäżĄæŻ

POST /api/fs/get

> Body èŻ·æ±ćæ°

```json
{
    "path": "/t",
    "password": "",
    "page": 1,
    "per_page": 0,
    "refresh": false
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć     | èŻŽæ |
| ------------- | ------ | ------- | ------ | ------------- | ------ |
| Authorization | header | string  | æŻ    |               | none   |
| body          | body   | object  | ćŠ    |               | none   |
| Â» path       | body   | string  | æŻ    | è·ŻćŸ        | none   |
| Â» password   | body   | string  | æŻ    | ćŻç         | none   |
| Â» page       | body   | integer | ćŠ    |               | none   |
| Â» per_page   | body   | integer | ćŠ    |               | none   |
| Â» refresh    | body   | boolean | ćŠ    | ćŒșć¶ ć·æ° | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°         | ç±»ć  | ćżé | çșŠæ | äž­æć          | èŻŽæ |
| -------------- | ------- | ------ | ------ | ------------------ | ------ |
| Â» code        | integer | true   | none   | ç¶æç           | none   |
| Â» message     | string  | true   | none   | äżĄæŻ             | none   |
| Â» data        | object  | true   | none   |                    | none   |
| Â»Â» name      | string  | true   | none   | æä»¶ć          | none   |
| Â»Â» size      | integer | true   | none   | ć€§ć°             | none   |
| Â»Â» is_dir    | boolean | true   | none   | æŻćŠæŻæä»¶ć€č | none   |
| Â»Â» modified  | string  | true   | none   | äżźæčæ¶éŽ       | none   |
| Â»Â» sign      | string  | true   | none   | ç­Ÿć             | none   |
| Â»Â» thumb     | string  | true   | none   | çŒ©ç„ćŸ          | none   |
| Â»Â» type      | integer | true   | none   | ç±»ć             | none   |
| Â»Â» raw_url   | string  | true   | none   | ćć§url          | none   |
| Â»Â» readme    | string  | true   | none   | èŻŽæ             | none   |
| Â»Â» provider  | string  | true   | none   |                    | none   |
| Â»Â» related   | null    | true   | none   |                    | none   |
| Â»Â» created   | string  | true   | none   | ćć»șæ¶éŽ       | none   |
| Â»Â» hashinfo  | string  | true   | none   |                    | none   |
| Â»Â» hash_info | null    | true   | none   |                    | none   |
| Â»Â» header    | string  | true   | none   |                    | none   |

## POST è·ćçźćœ

POST /api/fs/dirs

> Body èŻ·æ±ćæ°

```json
{
    "path": "/t",
    "password": "",
    "force_root": false
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------- | ------ | --------- | ------ |
| Authorization | header | string  | æŻ    |           | none   |
| body          | body   | object  | ćŠ    |           | none   |
| Â» path       | body   | string  | ćŠ    | è·ŻćŸ    | none   |
| Â» password   | body   | string  | ćŠ    | ćŻç     | none   |
| Â» force_root | body   | boolean | ćŠ    |           | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°        | ç±»ć   | ćżé | çșŠæ | äž­æć    | èŻŽæ |
| ------------- | -------- | ------ | ------ | ------------ | ------ |
| Â» code       | integer  | true   | none   | ç¶æç     | none   |
| Â» message    | string   | true   | none   | äżĄæŻ       | none   |
| Â» data       | [object] | true   | none   |              | none   |
| Â»Â» name     | string   | true   | none   | æä»¶ć€čć | none   |
| Â»Â» modified | string   | true   | none   | äżźæčæ¶éŽ | none   |

## POST æçŽąæä»¶ææä»¶ć€č

POST /api/fs/search

> Body èŻ·æ±ćæ°

```json
{
    "parent": "/local",
    "keywords": "test",
    "scope": 0,
    "page": 1,
    "per_page":1 ,
    "password": ""
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć    | èŻŽæ                        |
| ------------- | ------ | ------- | ------ | ------------ | ----------------------------- |
| Authorization | header | string  | æŻ    |              | none                          |
| body          | body   | object  | ćŠ    |              | none                          |
| Â» parent     | body   | string  | æŻ    | æçŽąçźćœ | none                          |
| Â» keywords   | body   | string  | æŻ    | ćłéźèŻ    | none                          |
| Â» scope      | body   | integer | æŻ    | æçŽąç±»ć | 0-ćšéš 1-æä»¶ć€č 2-æä»¶ |
| Â» page       | body   | integer | æŻ    | éĄ”æ°       | none                          |
| Â» per_page   | body   | integer | æŻ    | æŻéĄ”æ°çź | none                          |
| Â» password   | body   | string  | æŻ    | ćŻç        | none                          |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°        | ç±»ć   | ćżé | çșŠæ | äž­æć          | èŻŽæ |
| ------------- | -------- | ------ | ------ | ------------------ | ------ |
| Â» code       | integer  | true   | none   | ç¶æç           | none   |
| Â» message    | string   | true   | none   | äżĄæŻ             | none   |
| Â» data       | object   | true   | none   |                    | none   |
| Â»Â» content  | [object] | true   | none   |                    | none   |
| Â»Â»Â» parent | string   | true   | none   | è·ŻćŸ             | none   |
| Â»Â»Â» name   | string   | true   | none   | æä»¶ć          | none   |
| Â»Â»Â» is_dir | boolean  | true   | none   | æŻćŠæŻæä»¶ć€č | none   |
| Â»Â»Â» size   | integer  | true   | none   | ć€§ć°             | none   |
| Â»Â»Â» type   | integer  | true   | none   | ç±»ć             | none   |
| Â»Â» total    | integer  | true   | none   | æ»æ°             | none   |

## POST æ°ć»șæä»¶ć€č

POST /api/fs/mkdir

> Body èŻ·æ±ćæ°

```json
{
    "path": "/tt"
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć       | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------------- | ------ |
| Authorization | header | string | æŻ    |                 | token  |
| Content-Type  | header | string | ćŠ    |                 | none   |
| body          | body   | object | ćŠ    |                 | none   |
| Â» path       | body   | string | æŻ    | æ°çźćœè·ŻćŸ | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST éćœćæä»¶

POST /api/fs/rename

> Body èŻ·æ±ćæ°

```json
{
    "name": "test3",
    "path": "/éżéäșç/test2"
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć                      | èŻŽæ |
| ------------- | ------ | ------ | ------ | ------------------------------ | ------ |
| Authorization | header | string | æŻ    |                                | token  |
| Content-Type  | header | string | ćŠ    |                                | none   |
| body          | body   | object | ćŠ    |                                | none   |
| Â» name       | body   | string | æŻ    | çźæ æä»¶ćïŒäžæŻæ'/' | none   |
| Â» path       | body   | string | æŻ    | æșæä»¶ć                   | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST æčééćœć

POST /api/fs/batch_rename

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°            | äœçœź | ç±»ć   | ćżé | äž­æć    | èŻŽæ |
| ----------------- | ------ | -------- | ------ | ------------ | ------ |
| Authorization     | header | string   | æŻ    |              | token  |
| Content-Type      | header | string   | ćŠ    |              | none   |
| body              | body   | object   | ćŠ    |              | none   |
| Â» src_dir        | body   | string   | æŻ    | æșçźćœ    | none   |
| Â» rename_objects | body   | [object] | æŻ    |              | none   |
| Â»Â» src_name     | body   | string   | ćŠ    | ćæä»¶ć | none   |
| Â»Â» new_name     | body   | string   | ćŠ    | æ°æä»¶ć | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   | ç¶æç  | ç¶æç  |
| Â» message | string  | true   | none   | äżĄæŻ    | äżĄæŻ    |
| Â» data    | null    | true   | none   |           | none      |

## POST æ­Łćéćœć

POST /api/fs/regex_rename

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°            | äœçœź | ç±»ć | ćżé | äž­æć             | èŻŽæ |
| ----------------- | ------ | ------ | ------ | --------------------- | ------ |
| Authorization     | header | string | æŻ    |                       | token  |
| Content-Type      | header | string | ćŠ    |                       | none   |
| body              | body   | object | ćŠ    |                       | none   |
| Â» src_dir        | body   | string | æŻ    | æșçźćœ             | none   |
| Â» src_name_regex | body   | string | æŻ    | æșæä»¶ćčéæ­Łć | none   |
| Â» new_name_regex | body   | string | æŻ    | æ°æä»¶ćæ­Łć    | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   | ç¶æç  | ç¶æç  |
| Â» message | string  | true   | none   | äżĄæŻ    | äżĄæŻ    |
| Â» data    | null    | true   | none   |           | none      |

## PUT èĄšćäžäŒ æä»¶

PUT /api/fs/form

> Body èŻ·æ±ćæ°

```yaml
file: []

```

### èŻ·æ±ćæ°

| ćç§°         | äœçœź | ç±»ć         | ćżé | äž­æć | èŻŽæ                               |
| -------------- | ------ | -------------- | ------ | --------- | ------------------------------------ |
| Authorization  | header | string         | æŻ    |           | token                                |
| Content-Type   | header | string         | æŻ    |           | éèŠæŻmultipart/form-data;        |
| Content-Length | header | string         | æŻ    |           | æä»¶ć€§ć°                         |
| File-Path      | header | string         | æŻ    |           | ç»èżURLçŒç çćźæŽæä»¶è·ŻćŸ |
| As-Task        | header | string         | ćŠ    |           | æŻćŠæ·»ć äžșä»»ćĄ                |
| body           | body   | object         | ćŠ    |           | none                                 |
| Â» file        | body   | string(binary) | æŻ    |           | æä»¶                               |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°          | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| --------------- | ------- | ------ | ------ | --------- | ------ |
| Â» code         | integer | true   | none   | ç¶æç  | none   |
| Â» message      | string  | true   | none   | äżĄæŻ    | none   |
| Â» data         | object  | true   | none   |           | none   |
| Â»Â» task       | object  | true   | none   |           | none   |
| Â»Â»Â» id       | string  | true   | none   |           | none   |
| Â»Â»Â» name     | string  | true   | none   |           | none   |
| Â»Â»Â» state    | integer | true   | none   |           | none   |
| Â»Â»Â» status   | string  | true   | none   |           | none   |
| Â»Â»Â» progress | integer | true   | none   |           | none   |
| Â»Â»Â» error    | string  | true   | none   |           | none   |

## POST ç§»ćšæä»¶

POST /api/fs/move

> Body èŻ·æ±ćæ°

```json
{
  "src_dir": "string",
  "dst_dir": "string",
  "names": [
    "string"
  ]
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć   | ćżé | äž­æć       | èŻŽæ |
| ------------- | ------ | -------- | ------ | --------------- | ------ |
| Authorization | header | string   | æŻ    |                 | none   |
| body          | body   | object   | ćŠ    |                 | none   |
| Â» src_dir    | body   | string   | æŻ    | æșæä»¶ć€č    | none   |
| Â» dst_dir    | body   | string   | æŻ    | çźæ æä»¶ć€č | none   |
| Â» names      | body   | [string] | æŻ    | æä»¶ć       | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ć€ć¶æä»¶

POST /api/fs/copy

> Body èŻ·æ±ćæ°

```json
{
  "src_dir": "string",
  "dst_dir": "string",
  "names": [
    "string"
  ]
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć   | ćżé | äž­æć       | èŻŽæ |
| ------------- | ------ | -------- | ------ | --------------- | ------ |
| Authorization | header | string   | æŻ    |                 | none   |
| body          | body   | object   | ćŠ    |                 | none   |
| Â» src_dir    | body   | string   | æŻ    | æșæä»¶ć€č    | none   |
| Â» dst_dir    | body   | string   | æŻ    | çźæ æä»¶ć€č | none   |
| Â» names      | body   | [string] | æŻ    | æä»¶ć       | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ć é€æä»¶ææä»¶ć€č

POST /api/fs/remove

> Body èŻ·æ±ćæ°

```json
{
  "names": [
    "string"
  ],
  "dir": "string"
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć   | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | -------- | ------ | --------- | ------ |
| Authorization | header | string   | æŻ    |           | none   |
| body          | body   | object   | ćŠ    |           | none   |
| Â» names      | body   | [string] | æŻ    | æä»¶ć | none   |
| Â» dir        | body   | string   | æŻ    | çźćœ    | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ć é€ç©șæä»¶ć€č

POST /api/fs/remove_empty_directory

> Body èŻ·æ±ćæ°

```json
{
  "src_dir": "string"
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | none   |
| body          | body   | object | ćŠ    |           | none   |
| Â» src_dir    | body   | string | æŻ    | çźćœ    | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST èćç§»ćš

POST /api/fs/recursive_move

> Body èŻ·æ±ćæ°

```json
{
  "src_dir": "string",
  "dst_dir": "string"
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć       | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------------- | ------ |
| Authorization | header | string | æŻ    |                 | none   |
| body          | body   | object | ćŠ    |                 | none   |
| Â» src_dir    | body   | string | æŻ    | æșæä»¶ć€č    | none   |
| Â» dst_dir    | body   | string | æŻ    | çźæ æä»¶ć€č | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## PUT æ”ćŒäžäŒ æä»¶

PUT /api/fs/put

> Body èŻ·æ±ćæ°

```yaml
string

```

### èŻ·æ±ćæ°

| ćç§°         | äœçœź | ç±»ć         | ćżé | äž­æć | èŻŽæ                                     |
| -------------- | ------ | -------------- | ------ | --------- | ------------------------------------------ |
| Authorization  | header | string         | æŻ    |           | none                                       |
| File-Path      | header | string         | æŻ    |           | ç»èżURLçŒç çćźæŽçźæ æä»¶è·ŻćŸ |
| As-Task        | header | string         | ćŠ    |           | æŻćŠæ·»ć äžșä»»ćĄ                      |
| Content-Type   | header | string         | æŻ    |           | none                                       |
| Content-Length | header | string         | æŻ    |           | none                                       |
| body           | body   | string(binary) | ćŠ    |           | none                                       |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°          | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| --------------- | ------- | ------ | ------ | --------- | ------ |
| Â» code         | integer | true   | none   | ç¶æç  | none   |
| Â» message      | string  | true   | none   | äżĄæŻ    | none   |
| Â» data         | object  | true   | none   |           | none   |
| Â»Â» task       | object  | true   | none   |           | none   |
| Â»Â»Â» id       | string  | true   | none   |           | none   |
| Â»Â»Â» name     | string  | true   | none   |           | none   |
| Â»Â»Â» state    | integer | true   | none   |           | none   |
| Â»Â»Â» status   | string  | true   | none   |           | none   |
| Â»Â»Â» progress | integer | true   | none   |           | none   |
| Â»Â»Â» error    | string  | true   | none   |           | none   |

## POST æ·»ć çŠ»çșżäžèœœ

POST /api/fs/add_offline_download

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°           | äœçœź | ç±»ć   | ćżé | äž­æć    | èŻŽæ                                                                                    |
| ---------------- | ------ | -------- | ------ | ------------ | ----------------------------------------------------------------------------------------- |
| Authorization    | header | string   | æŻ    |              | none                                                                                      |
| body             | body   | object   | ćŠ    |              | none                                                                                      |
| Â» urls          | body   | [string] | æŻ    | url          | none                                                                                      |
| Â» path          | body   | string   | æŻ    | çźæ è·ŻćŸ | none                                                                                      |
| Â» tool          | body   | string   | æŻ    | ć·„ć·       | ćŻé`aria2`,`SimpleHttp`ć`qBittorrent`                                                |
| Â» delete_policy | body   | string   | æŻ    | ć é€ç­ç„ | ćŻé`delete_on_upload_succeed`,`delete_on_upload_failed`,`delete_never`,`delete_always` |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°          | ç±»ć   | ćżé | çșŠæ | äž­æć | èŻŽæ |
| --------------- | -------- | ------ | ------ | --------- | ------ |
| Â» code         | integer  | true   | none   | ç¶æç  | none   |
| Â» message      | string   | true   | none   | äżĄæŻ    | none   |
| Â» data         | object   | true   | none   |           | none   |
| Â»Â» tasks      | [object] | true   | none   |           | none   |
| Â»Â»Â» id       | string   | false  | none   |           | none   |
| Â»Â»Â» name     | string   | false  | none   |           | none   |
| Â»Â»Â» state    | integer  | false  | none   |           | none   |
| Â»Â»Â» status   | string   | false  | none   |           | none   |
| Â»Â»Â» progress | integer  | false  | none   |           | none   |
| Â»Â»Â» error    | string   | false  | none   |           | none   |

# public

## GET è·ćç«çčèźŸçœź

GET /api/public/settings

> èżćç€șäŸ

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
        "home_icon": "đ ",
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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°                          | ç±»ć  | ćżé | çșŠæ | äž­æć          | èŻŽæ |
| ------------------------------- | ------- | ------ | ------ | ------------------ | ------ |
| Â» code                         | integer | true   | none   | ç¶æç           | none   |
| Â» message                      | string  | true   | none   | äżĄæŻ             | none   |
| Â» data                         | object  | true   | none   | æ°æź             | none   |
| Â»Â» allow_indexed              | string  | true   | none   | ćèźžçŽąćŒ       | none   |
| Â»Â» allow_mounted              | string  | true   | none   | ćèźžæèœœ       | none   |
| Â»Â» announcement               | string  | true   | none   | ćŹć             | none   |
| Â»Â» audio_autoplay             | string  | true   | none   | èȘćšæ­æŸéłéą | none   |
| Â»Â» audio_cover                | string  | true   | none   | éłéąć°éą       | none   |
| Â»Â» auto_update_index          | string  | true   | none   | èȘćšæŽæ°çŽąćŒ | none   |
| Â»Â» default_page_size          | string  | true   | none   | é»èź€ćéĄ”æ°    | none   |
| Â»Â» external_previews          | string  | true   | none   | ć€éšéąè§       | none   |
| Â»Â» favicon                    | string  | true   | none   | çœç«ćŸæ        | none   |
| Â»Â» filename_char_mapping      | string  | true   | none   |                    | none   |
| Â»Â» forward_direct_link_params | string  | true   | none   |                    | none   |
| Â»Â» hide_files                 | string  | true   | none   | éèæä»¶       | none   |
| Â»Â» home_container             | string  | true   | none   | äž»éĄ”ćźčćš       | none   |
| Â»Â» home_icon                  | string  | true   | none   | äž»éĄ”ćŸæ        | none   |
| Â»Â» iframe_previews            | string  | true   | none   | iframeéąè§èźŸçœź | none   |
| Â»Â» logo                       | string  | true   | none   | logo               | none   |
| Â»Â» main_color                 | string  | true   | none   | äž»éąéąèČ       | none   |
| Â»Â» ocr_api                    | string  | true   | none   | pcræ„ćŁ          | none   |
| Â»Â» package_download           | string  | true   | none   | æćäžèœœ       | none   |
| Â»Â» pagination_type            | string  | true   | none   |                    | none   |
| Â»Â» robots_txt                 | string  | true   | none   | robotsæä»¶       | none   |
| Â»Â» search_index               | string  | true   | none   |                    | none   |
| Â»Â» settings_layout            | string  | true   | none   |                    | none   |
| Â»Â» site_title                 | string  | true   | none   | ç«çčæ éą       | none   |
| Â»Â» sso_login_enabled          | string  | true   | none   | ćŻçšssoç»ćœ    | none   |
| Â»Â» sso_login_platform         | string  | true   | none   | ssoç»ćœćčłć°    | none   |
| Â»Â» version                    | string  | true   | none   | çæŹ             | none   |
| Â»Â» video_autoplay             | string  | true   | none   | è§éąèȘćšæ­æŸ | none   |

## GET pingæŁæ”

GET /ping

èżéæ§pingæŁæ”

> èżćç€șäŸ

> 200 Response

```json
pong
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

# admin/meta

## GET ććșćäżĄæŻ

GET /api/admin/meta/list

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ       |
| ------------- | ------ | ------ | ------ | --------- | ------------ |
| page          | query  | string | ćŠ    |           | éĄ”æ°       |
| per_page      | query  | string | ćŠ    |           | æŻéĄ”äžȘæ° |
| Authorization | header | string | æŻ    |           | none         |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°          | ç±»ć   | ćżé | çșŠæ | äž­æć                               | èŻŽæ |
| --------------- | -------- | ------ | ------ | --------------------------------------- | ------ |
| Â» code         | integer  | true   | none   | ç¶æç                                | none   |
| Â» message      | string   | true   | none   | äżĄæŻ                                  | none   |
| Â» data         | object   | true   | none   | æ°æź                                  | none   |
| Â»Â» content    | [object] | true   | none   | ććźč                                  | none   |
| Â»Â»Â» id       | integer  | false  | none   | id                                      | none   |
| Â»Â»Â» path     | string   | false  | none   | è·ŻćŸ                                  | none   |
| Â»Â»Â» password | string   | false  | none   | ćŻç                                   | none   |
| Â»Â»Â» p_sub    | boolean  | false  | none   | ćŻç æŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â»Â»Â» write    | boolean  | false  | none   | æŻćŠćèźžćć„                      | none   |
| Â»Â»Â» w_sub    | boolean  | false  | none   | æŻćŠćèźžćć„ćŒçšć°ć­æä»¶ć€č | none   |
| Â»Â»Â» hide     | string   | false  | none   | éè                                  | none   |
| Â»Â»Â» h_sub    | boolean  | false  | none   | éèæŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â»Â»Â» readme   | string   | false  | none   | èŻŽæ                                  | none   |
| Â»Â»Â» r_sub    | boolean  | false  | none   | èŻŽææŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â»Â» total      | integer  | true   | none   | æ»æ°                                  | none   |

## GET è·ććäżĄæŻ

GET /api/admin/meta/get

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ      |
| ------------- | ------ | ------ | ------ | --------- | ----------- |
| id            | query  | string | æŻ    |           | ćäżĄæŻid |
| Authorization | header | string | æŻ    |           | none        |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°        | ç±»ć  | ćżé | çșŠæ | äž­æć                               | èŻŽæ |
| ------------- | ------- | ------ | ------ | --------------------------------------- | ------ |
| Â» code       | integer | true   | none   | ç¶æç                                | none   |
| Â» message    | string  | true   | none   | äżĄæŻ                                  | none   |
| Â» data       | object  | true   | none   |                                         | none   |
| Â»Â» id       | integer | true   | none   | id                                      | none   |
| Â»Â» path     | string  | true   | none   | è·ŻćŸ                                  | none   |
| Â»Â» password | string  | true   | none   | ćŻç                                   | none   |
| Â»Â» p_sub    | boolean | true   | none   | ćŻç æŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â»Â» write    | boolean | true   | none   | ćŒćŻćć„                            | none   |
| Â»Â» w_sub    | boolean | true   | none   | ćŒćŻćć„æŻćŠćșçšć°ć­æä»¶ć€č | none   |
| Â»Â» hide     | string  | true   | none   | éè                                  | none   |
| Â»Â» h_sub    | boolean | true   | none   | éèæŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â»Â» readme   | string  | true   | none   | èŻŽæ                                  | none   |
| Â»Â» r_sub    | boolean | true   | none   | èŻŽææŻćŠćșçšć°ć­æä»¶ć€č       | none   |

## POST æ°ćąćäżĄæŻ

POST /api/admin/meta/create

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć                               | èŻŽæ |
| ------------- | ------ | ------- | ------ | --------------------------------------- | ------ |
| Authorization | header | string  | æŻ    |                                         | none   |
| body          | body   | object  | ćŠ    |                                         | none   |
| Â» id         | body   | integer | æŻ    | id                                      | none   |
| Â» path       | body   | string  | æŻ    | è·ŻćŸ                                  | none   |
| Â» password   | body   | string  | æŻ    | ćŻç                                   | none   |
| Â» p_sub      | body   | boolean | æŻ    | ćŻç æŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â» write      | body   | boolean | æŻ    | ćŒćŻćć„                            | none   |
| Â» w_sub      | body   | boolean | æŻ    | ćŒćŻćć„æŻćŠćșçšć°ć­æä»¶ć€č | none   |
| Â» hide       | body   | string  | æŻ    | éè                                  | none   |
| Â» h_sub      | body   | boolean | æŻ    | éèæŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â» readme     | body   | string  | æŻ    | èŻŽæ                                  | none   |
| Â» r_sub      | body   | boolean | æŻ    | èŻŽææŻćŠćșçšć°ć­æä»¶ć€č       | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   |           | none   |
| Â» message | string  | true   | none   |           | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST æŽæ°ćäżĄæŻ

POST /api/admin/meta/update

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć                               | èŻŽæ |
| ------------- | ------ | ------- | ------ | --------------------------------------- | ------ |
| Authorization | header | string  | æŻ    |                                         | none   |
| body          | body   | object  | ćŠ    |                                         | none   |
| Â» id         | body   | integer | æŻ    | id                                      | none   |
| Â» path       | body   | string  | æŻ    | è·ŻćŸ                                  | none   |
| Â» password   | body   | string  | æŻ    | ćŻç                                   | none   |
| Â» p_sub      | body   | boolean | æŻ    | ćŻç æŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â» write      | body   | boolean | æŻ    | ćŒćŻćć„                            | none   |
| Â» w_sub      | body   | boolean | æŻ    | ćŒćŻćć„æŻćŠćșçšć°ć­æä»¶ć€č | none   |
| Â» hide       | body   | string  | æŻ    | éè                                  | none   |
| Â» h_sub      | body   | boolean | æŻ    | éèæŻćŠćșçšć°ć­æä»¶ć€č       | none   |
| Â» readme     | body   | string  | æŻ    | èŻŽæ                                  | none   |
| Â» r_sub      | body   | boolean | æŻ    | èŻŽææŻćŠćșçšć°ć­æä»¶ć€č       | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   |           | none   |
| Â» message | string  | true   | none   |           | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ć é€ćäżĄæŻ

POST /api/admin/meta/delete

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| id            | query  | string | æŻ    |           | none   |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

# admin/user

## GET ććșææçšæ·

GET /api/admin/user/list

ććșææçšæ·çäżĄæŻ

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°            | ç±»ć   | ćżé | çșŠæ | äž­æć    | èŻŽæ |
| ----------------- | -------- | ------ | ------ | ------------ | ------ |
| Â» code           | integer  | true   | none   | ç¶æç     | none   |
| Â» message        | string   | true   | none   | äżĄæŻ       | none   |
| Â» data           | object   | true   | none   |              | none   |
| Â»Â» content      | [object] | true   | none   |              | none   |
| Â»Â»Â» id         | integer  | true   | none   | id           | none   |
| Â»Â»Â» username   | string   | true   | none   | çšæ·ć    | none   |
| Â»Â»Â» password   | string   | true   | none   | ćŻç        | none   |
| Â»Â»Â» base_path  | string   | true   | none   | ćșæŹè·ŻćŸ | none   |
| Â»Â»Â» role       | integer  | true   | none   | è§èČ       | none   |
| Â»Â»Â» disabled   | boolean  | true   | none   | æŻćŠçŠçš | none   |
| Â»Â»Â» permission | integer  | true   | none   | æé       | none   |
| Â»Â»Â» sso_id     | string   | true   | none   | sso id       | none   |
| Â»Â» total        | integer  | true   | none   | æ»æ°       | none   |

## GET ććșæäžȘçšæ·

GET /api/admin/user/get

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| id            | query  | string | æŻ    |           | none   |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°          | ç±»ć  | ćżé | çșŠæ | äž­æć    | èŻŽæ |
| --------------- | ------- | ------ | ------ | ------------ | ------ |
| Â» code         | integer | true   | none   |              | none   |
| Â» message      | string  | true   | none   |              | none   |
| Â» data         | object  | true   | none   |              | none   |
| Â»Â» id         | integer | true   | none   | id           | none   |
| Â»Â» username   | string  | true   | none   | çšæ·ć    | none   |
| Â»Â» password   | string  | true   | none   | ćŻç        | none   |
| Â»Â» base_path  | string  | true   | none   | ćșæŹè·ŻćŸ | none   |
| Â»Â» role       | integer | true   | none   | è§èČ       | none   |
| Â»Â» disabled   | boolean | true   | none   | æŻćŠçŠçš | none   |
| Â»Â» permission | integer | true   | none   | æé       | none   |
| Â»Â» sso_id     | string  | true   | none   | sso id       | none   |

## POST æ°ć»șçšæ·

POST /api/admin/user/create

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć    | èŻŽæ |
| ------------- | ------ | ------- | ------ | ------------ | ------ |
| Authorization | header | string  | æŻ    |              | none   |
| body          | body   | object  | ćŠ    |              | none   |
| Â» id         | body   | integer | ćŠ    | id           | none   |
| Â» username   | body   | string  | æŻ    | çšæ·ć    | none   |
| Â» password   | body   | string  | ćŠ    | ćŻç        | none   |
| Â» base_path  | body   | string  | ćŠ    | ćșæŹè·ŻćŸ | none   |
| Â» role       | body   | integer | ćŠ    | è§èČ       | none   |
| Â» permission | body   | integer | ćŠ    | æé       | none   |
| Â» disabled   | body   | boolean | ćŠ    | æŻćŠçŠçš | none   |
| Â» sso_id     | body   | string  | ćŠ    | sso id       | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST æŽæ°çšæ·äżĄæŻ

POST /api/admin/user/update

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć    | èŻŽæ |
| ------------- | ------ | ------- | ------ | ------------ | ------ |
| Authorization | header | string  | æŻ    |              | none   |
| body          | body   | object  | ćŠ    |              | none   |
| Â» id         | body   | integer | æŻ    | id           | none   |
| Â» username   | body   | string  | æŻ    | çšæ·ć    | none   |
| Â» password   | body   | string  | ćŠ    | ćŻç        | none   |
| Â» base_path  | body   | string  | ćŠ    | ćșæŹè·ŻćŸ | none   |
| Â» role       | body   | integer | ćŠ    | è§èČ       | none   |
| Â» permission | body   | integer | ćŠ    | æé       | none   |
| Â» disabled   | body   | boolean | ćŠ    | æŻćŠçŠçš | none   |
| Â» sso_id     | body   | string  | ćŠ    | sso id       | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ćæ¶æäžȘçšæ·çäž€æ­„éȘèŻ

POST /api/admin/user/cancel_2fa

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| id            | query  | string | æŻ    |           | none   |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ć é€çšæ·

POST /api/admin/user/delete

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| id            | query  | string | æŻ    |           | none   |
| Authorization | header | string | ćŠ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ć é€çšæ·çŒć­

POST /api/admin/user/del_cache

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| username      | query  | string | æŻ    |           | none   |
| Authorization | header | string | ćŠ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

# admin/storage

## GET ććșć­ćšćèĄš

GET /api/admin/storage/list

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ       |
| ------------- | ------ | ------ | ------ | --------- | ------------ |
| page          | query  | string | ćŠ    |           | éĄ”æ°       |
| per_page      | query  | string | ćŠ    |           | æŻéĄ”æ°çź |
| Authorization | header | string | æŻ    |           | token        |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°                  | ç±»ć   | ćżé | çșŠæ | äž­æć          | èŻŽæ             |
| ----------------------- | -------- | ------ | ------ | ------------------ | ------------------ |
| Â» code                 | integer  | true   | none   | ç¶æç           | ç¶æç           |
| Â» message              | string   | true   | none   | äżĄæŻ             | äżĄæŻ             |
| Â» data                 | object   | true   | none   |                    | none               |
| Â»Â» content            | [object] | true   | none   |                    | none               |
| Â»Â»Â» id               | integer  | false  | none   | id                 | id                 |
| Â»Â»Â» mount_path       | string   | false  | none   | æèœœè·ŻćŸ       | æèœœè·ŻćŸ       |
| Â»Â»Â» order            | integer  | false  | none   | æćș             | éĄșćș             |
| Â»Â»Â» driver           | string   | false  | none   | é©±ćš             | é©±ćšç±»ć       |
| Â»Â»Â» cache_expiration | integer  | false  | none   | çŒć­èżææ¶éŽ | çŒć­æ¶éŽ       |
| Â»Â»Â» status           | string   | false  | none   | ç¶æ             | ç¶æ             |
| Â»Â»Â» addition         | string   | false  | none   | éąć€äżĄæŻ       | éąć€äżĄæŻ       |
| Â»Â»Â» remark           | string   | false  | none   | ć€æłš             | ć€æłšć          |
| Â»Â»Â» modified         | string   | false  | none   | äżźæčæ¶éŽ       | äżźæčæ¶éŽ       |
| Â»Â»Â» disabled         | boolean  | false  | none   | çŠçš             | æŻćŠèą«çŠçš    |
| Â»Â»Â» enable_sign      | boolean  | false  | none   | ćŻçšç­Ÿć       | none               |
| Â»Â»Â» order_by         | string   | false  | none   | æćș             | æćșæčćŒ       |
| Â»Â»Â» order_direction  | string   | false  | none   | æćșæčć       | æćșæčć       |
| Â»Â»Â» extract_folder   | string   | false  | none   | æćæä»¶ć€č    | æćçźćœéĄșćș |
| Â»Â»Â» web_proxy        | boolean  | false  | none   | webä»Łç          | httpä»Łç         |
| Â»Â»Â» webdav_policy    | string   | false  | none   | webdavä»Łç       | webdavç­ç„       |
| Â»Â»Â» down_proxy_url   | string   | false  | none   | äžèœœä»Łçurl    | äžèœœä»Łçurl    |
| Â»Â» total              | integer  | true   | none   | æ»æ°             | none               |

## POST ćŻçšć­ćš

POST /api/admin/storage/enable

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć  | ćżé | äž­æć | èŻŽæ   |
| ------------- | ------ | ------- | ------ | --------- | -------- |
| id            | query  | integer | æŻ    |           | ć­ćšid |
| Authorization | header | string  | æŻ    |           | token    |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   | ç¶æç  | ç¶æç  |
| Â» message | string  | true   | none   | äżĄæŻ    | äżĄæŻ    |
| Â» data    | null    | true   | none   | data      | data      |

## POST çŠçšć­ćš

POST /api/admin/storage/disable

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ   |
| ------------- | ------ | ------ | ------ | --------- | -------- |
| id            | query  | string | æŻ    |           | ć­ćšid |
| Authorization | header | string | æŻ    |           | token    |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   | ç¶æç  | ç¶æç  |
| Â» message | string  | true   | none   | äżĄæŻ    | äżĄæŻ    |
| Â» data    | null    | true   | none   | data      | data      |

## POST ćć»șć­ćš

POST /api/admin/storage/create

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°              | äœçœź | ç±»ć  | ćżé | äž­æć          | èŻŽæ |
| ------------------- | ------ | ------- | ------ | ------------------ | ------ |
| Authorization       | header | string  | æŻ    |                    | token  |
| body                | body   | object  | ćŠ    |                    | none   |
| Â» id               | body   | string  | ćŠ    | ID                 | none   |
| Â» mount_path       | body   | string  | æŻ    | æèœœè·ŻćŸ       | none   |
| Â» order            | body   | integer | ćŠ    | æćș             | none   |
| Â» driver           | body   | string  | æŻ    | é©±ćš             | none   |
| Â» remark           | body   | string  | ćŠ    | ć€æłšć          | none   |
| Â» cache_expiration | body   | integer | ćŠ    | çŒć­èżææ¶éŽ | none   |
| Â» status           | body   | string  | æŻ    |                    | none   |
| Â» web_proxy        | body   | boolean | æŻ    | webä»Łç          | none   |
| Â» webdav_policy    | body   | string  | ćŠ    | webdavç­ç„       | none   |
| Â» down_proxy_url   | body   | string  | ćŠ    | äžèœœä»Łç       | none   |
| Â» order_by         | body   | string  | æŻ    | æćșæčćŒ       | none   |
| Â» extract_folder   | body   | string  | æŻ    | æćçźćœ       | none   |
| Â» order_direction  | body   | string  | æŻ    | æćșæčć       | none   |
| Â» addition         | body   | string  | æŻ    | éąć€äżĄæŻ       | none   |
| Â» enable_sign      | body   | string  | æŻ    | ćŻçšç­Ÿć       | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   | ç¶æç  | ç¶æç  |
| Â» message | string  | true   | none   | äżĄæŻ    | äżĄæŻ    |
| Â» data    | object  | true   | none   | data      | data      |
| Â»Â» id    | integer | true   | none   |           | none      |

## POST æŽæ°ć­ćš

POST /api/admin/storage/update

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°              | äœçœź | ç±»ć  | ćżé | äž­æć          | èŻŽæ |
| ------------------- | ------ | ------- | ------ | ------------------ | ------ |
| Authorization       | header | string  | æŻ    |                    | token  |
| body                | body   | object  | ćŠ    |                    | none   |
| Â» id               | body   | string  | ćŠ    | ID                 | none   |
| Â» mount_path       | body   | string  | æŻ    | æèœœè·ŻćŸ       | none   |
| Â» order            | body   | integer | ćŠ    | æćș             | none   |
| Â» driver           | body   | string  | æŻ    | é©±ćš             | none   |
| Â» remark           | body   | string  | ćŠ    | ć€æłšć          | none   |
| Â» cache_expiration | body   | integer | ćŠ    | çŒć­èżææ¶éŽ | none   |
| Â» status           | body   | string  | æŻ    |                    | none   |
| Â» web_proxy        | body   | boolean | æŻ    | webä»Łç          | none   |
| Â» webdav_policy    | body   | string  | ćŠ    | webdavç­ç„       | none   |
| Â» down_proxy_url   | body   | string  | ćŠ    | äžèœœä»Łç       | none   |
| Â» order_by         | body   | string  | æŻ    | æćșæčćŒ       | none   |
| Â» extract_folder   | body   | string  | æŻ    | æćçźćœ       | none   |
| Â» order_direction  | body   | string  | æŻ    | æćșæčć       | none   |
| Â» addition         | body   | string  | æŻ    | éąć€äżĄæŻ       | none   |
| Â» enable_sign      | body   | string  | æŻ    | ćŻçšç­Ÿć       | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   | ç¶æç  | ç¶æç  |
| Â» message | string  | true   | none   | äżĄæŻ    | äżĄæŻ    |
| Â» data    | object  | true   | none   | data      | data      |
| Â»Â» id    | integer | true   | none   |           | none      |

## GET æ„èŻąæćźć­ćšäżĄæŻ

GET /api/admin/storage/get

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ   |
| ------------- | ------ | ------ | ------ | --------- | -------- |
| id            | query  | string | æŻ    |           | ć­ćšid |
| Authorization | header | string | æŻ    |           | token    |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°                | ç±»ć  | ćżé | çșŠæ | äž­æć          | èŻŽæ |
| --------------------- | ------- | ------ | ------ | ------------------ | ------ |
| Â» code               | integer | true   | none   | ç¶æç           | none   |
| Â» message            | string  | true   | none   | äżĄæŻ             | none   |
| Â» data               | object  | true   | none   |                    | none   |
| Â»Â» id               | integer | true   | none   | id                 | none   |
| Â»Â» mount_path       | string  | true   | none   | æèœœè·ŻćŸ       | none   |
| Â»Â» order            | integer | true   | none   | æćș             | none   |
| Â»Â» driver           | string  | true   | none   | é©±ćš             | none   |
| Â»Â» cache_expiration | integer | true   | none   | çŒć­èżææ¶éŽ | none   |
| Â»Â» status           | string  | true   | none   | ç¶æ             | none   |
| Â»Â» addition         | string  | true   | none   | éąć€äżĄæŻ       | none   |
| Â»Â» remark           | string  | true   | none   | ć€æłš             | none   |
| Â»Â» modified         | string  | true   | none   | äżźæčæ¶éŽ       | none   |
| Â»Â» disabled         | boolean | true   | none   | æŻćŠèą«çŠçš    | none   |
| Â»Â» order_by         | string  | true   | none   | æćșæčćŒ       | none   |
| Â»Â» order_direction  | string  | true   | none   | æćșæčć       | none   |
| Â»Â» extract_folder   | string  | true   | none   | æćçźćœ       | none   |
| Â»Â» web_proxy        | boolean | true   | none   | webä»Łç          | none   |
| Â»Â» webdav_policy    | string  | true   | none   | webdavç­ç„       | none   |
| Â»Â» down_proxy_url   | string  | true   | none   | äžèœœä»Łç       | none   |

## POST ć é€æćźć­ćš

POST /api/admin/storage/delete

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ   |
| ------------- | ------ | ------ | ------ | --------- | -------- |
| id            | query  | string | ćŠ    |           | ć­ćšid |
| Authorization | header | string | æŻ    |           | token    |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | ------- | ------ | ------ | --------- | --------- |
| Â» code    | integer | true   | none   | ç¶æç  | ç¶æç  |
| Â» message | string  | true   | none   | äżĄæŻ    | äżĄæŻ    |
| Â» data    | null    | true   | none   | data      | data      |

## POST éæ°ć èœœææć­ćš

POST /api/admin/storage/load_all

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

# admin/driver

## GET æ„èŻąææé©±ćšéçœźæšĄæżćèĄš

GET /api/admin/driver/list

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | token  |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°                  | ç±»ć   | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ----------------------- | -------- | ------ | ------ | --------- | ------ |
| Â» code                 | integer  | true   | none   |           | none   |
| Â» message              | string   | true   | none   |           | none   |
| Â» data                 | object   | true   | none   |           | none   |
| Â»Â» 115 Cloud          | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» 115 Share          | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» 123Pan             | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» 123PanLink         | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» 123PanShare        | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» 139Yun             | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» 189Cloud           | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» 189CloudPC         | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» AList V2           | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» AList V3           | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Alias              | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | false  | none   |           | none   |
| Â»Â»Â»Â» type           | string   | false  | none   |           | none   |
| Â»Â»Â»Â» default        | string   | false  | none   |           | none   |
| Â»Â»Â»Â» options        | string   | false  | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | false  | none   |           | none   |
| Â»Â»Â»Â» help           | string   | false  | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Aliyundrive        | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» AliyundriveOpen    | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» AliyundriveShare   | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» BaiduNetdisk       | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» BaiduPhoto         | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» BaiduShare         | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» ChaoXingGroupDrive | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Cloudreve          | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Crypt              | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Doge               | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Dropbox            | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» FTP                | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» FeijiPan           | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» GoogleDrive        | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» GooglePhoto        | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» ILanZou            | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» IPFS API           | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Lanzou             | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Local              | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» MediaTrack         | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Mega_nz            | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» MoPan              | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» NeteaseMusic       | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Onedrive           | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» OnedriveAPP        | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» PikPak             | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» PikPakShare        | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Quark              | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Quqi               | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» S3                 | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» SFTP               | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» SMB                | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Seafile            | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Teambition         | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Terabox            | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Thunder            | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» ThunderExpert      | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Trainbit           | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» UC                 | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» USS                | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» UrlTree            | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» VTencent           | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» Virtual            | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» WebDav             | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» WeiYun             | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» WoPan              | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |
| Â»Â» YandexDisk         | object   | true   | none   |           | none   |
| Â»Â»Â» common           | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» additional       | [object] | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» type           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» default        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» options        | string   | true   | none   |           | none   |
| Â»Â»Â»Â» required       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» help           | string   | true   | none   |           | none   |
| Â»Â»Â» config           | object   | true   | none   |           | none   |
| Â»Â»Â»Â» name           | string   | true   | none   |           | none   |
| Â»Â»Â»Â» local_sort     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_local     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» only_proxy     | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_cache       | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» no_upload      | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» need_ms        | boolean  | true   | none   |           | none   |
| Â»Â»Â»Â» default_root   | string   | true   | none   |           | none   |
| Â»Â»Â»Â» alert          | string   | true   | none   |           | none   |

## GET ććșé©±ćšććèĄš

GET /api/admin/driver/names

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | token  |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć   | ćżé | çșŠæ | äž­æć | èŻŽæ    |
| ---------- | -------- | ------ | ------ | --------- | --------- |
| Â» code    | integer  | true   | none   | ç¶æç  | ç¶æç  |
| Â» message | string   | true   | none   | äżĄæŻ    | äżĄæŻ    |
| Â» data    | [string] | true   | none   |           | none      |

## GET ććșçčćźé©±ćšäżĄæŻ

GET /api/admin/driver/info

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| driver        | query  | string | æŻ    |           | none   |
| Authorization | header | string | æŻ    |           | token  |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°              | ç±»ć   | ćżé | çșŠæ | äž­æć          | èŻŽæ    |
| ------------------- | -------- | ------ | ------ | ------------------ | --------- |
| Â» code             | integer  | true   | none   | ç¶æç           | ç¶æç  |
| Â» message          | string   | true   | none   | äżĄæŻ             | äżĄæŻ    |
| Â» data             | object   | true   | none   |                    | none      |
| Â»Â» common         | [object] | true   | none   | éçšéçœź       | none      |
| Â»Â»Â» name         | string   | true   | none   | éçœźć          | none      |
| Â»Â»Â» type         | string   | true   | none   | ç±»ć             | none      |
| Â»Â»Â» default      | string   | true   | none   | é»èź€ćŒ          | none      |
| Â»Â»Â» options      | string   | true   | none   | ééĄč             | none      |
| Â»Â»Â» required     | boolean  | true   | none   | æŻćŠćżéĄ»       | none      |
| Â»Â»Â» help         | string   | true   | none   | ćžźć©äżĄæŻ       | none      |
| Â»Â» additional     | [object] | true   | none   | éąć€éçœź       | none      |
| Â»Â»Â» name         | string   | true   | none   | éçœźć          | none      |
| Â»Â»Â» type         | string   | true   | none   | ç±»ć             | none      |
| Â»Â»Â» default      | string   | true   | none   | é»èź€ćŒ          | none      |
| Â»Â»Â» options      | string   | true   | none   | ééĄč             | none      |
| Â»Â»Â» required     | boolean  | true   | none   | æŻćŠćżéĄ»       | none      |
| Â»Â»Â» help         | string   | true   | none   | ćžźć©äżĄæŻ       | none      |
| Â»Â» config         | object   | true   | none   | éçœź             | none      |
| Â»Â»Â» name         | string   | true   | none   | éçœźć          | none      |
| Â»Â»Â» local_sort   | boolean  | true   | none   | æŹć°æćș       | none      |
| Â»Â»Â» only_local   | boolean  | true   | none   | ä»æŹć°          | none      |
| Â»Â»Â» only_proxy   | boolean  | true   | none   | ä»ä»Łç          | none      |
| Â»Â»Â» no_cache     | boolean  | true   | none   | æ çŒć­          | none      |
| Â»Â»Â» no_upload    | boolean  | true   | none   | æ äžäŒ           | none      |
| Â»Â»Â» need_ms      | boolean  | true   | none   |                    | none      |
| Â»Â»Â» default_root | string   | true   | none   | é»èź€ćșæŹè·ŻćŸ | none      |
| Â»Â»Â» alert        | string   | true   | none   | è­ŠćäżĄæŻ       | none      |

# admin/setting

## GET ććșèźŸçœź

GET /api/admin/setting/list

ćæŹæ°žäčä»€ç

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ                                                     |
| ------------- | ------ | ------ | ------ | --------- | ---------------------------------------------------------- |
| groups        | query  | string | ćŠ    |           | 5,0-ć¶ćźèźŸçœźïŒćæŹaria2ćä»€çç­                 |
| group         | query  | string | ćŠ    |           | 1-ç«çčïŒ2-æ ·ćŒïŒ3-éąè§ïŒ4-ćšć±ïŒ7-ćçčç»ćœ |
| Authorization | header | string | ćŠ    |           | none                                                       |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°       | ç±»ć   | ćżé | çșŠæ | äž­æć    | èŻŽæ                                                |
| ------------ | -------- | ------ | ------ | ------------ | ----------------------------------------------------- |
| Â» code      | integer  | true   | none   | ç¶æç     | none                                                  |
| Â» message   | string   | true   | none   | äżĄæŻ       | none                                                  |
| Â» data      | [object] | true   | none   |              | none                                                  |
| Â»Â» key     | string   | true   | none   | éź          | none                                                  |
| Â»Â» value   | string   | true   | none   | ćŒ          | none                                                  |
| Â»Â» help    | string   | true   | none   | ćžźć©äżĄæŻ | none                                                  |
| Â»Â» type    | string   | true   | none   | ç±»ć       | string, number, bool, select                          |
| Â»Â» options | string   | true   | none   | ééĄč       | none                                                  |
| Â»Â» group   | integer  | true   | none   | ćç»       | çšäșćç«Żćç»                                    |
| Â»Â» flag    | integer  | true   | none   | æ ćż       | 0 = public, 1 = private, 2 = readonly, 3 = deprecated |

## GET è·ćæéĄčèźŸçœź

GET /api/admin/setting/get

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| keys          | query  | string | ćŠ    |           | none   |
| key           | query  | string | ćŠ    |           | none   |
| Authorization | header | string | ćŠ    |           | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°       | ç±»ć  | ćżé | çșŠæ | äž­æć    | èŻŽæ                                                |
| ------------ | ------- | ------ | ------ | ------------ | ----------------------------------------------------- |
| Â» code      | integer | true   | none   | ç¶æç     | none                                                  |
| Â» message   | string  | true   | none   | äżĄæŻ       | none                                                  |
| Â» data      | object  | true   | none   |              | none                                                  |
| Â»Â» key     | string  | true   | none   | éź          | none                                                  |
| Â»Â» value   | string  | true   | none   | ćŒ          | none                                                  |
| Â»Â» help    | string  | true   | none   | ćžźć©äżĄæŻ | none                                                  |
| Â»Â» type    | string  | true   | none   | ç±»ć       | string, number, bool, select                          |
| Â»Â» options | string  | true   | none   | ééĄč       | none                                                  |
| Â»Â» group   | integer | true   | none   | ćç»       | none                                                  |
| Â»Â» flag    | integer | true   | none   | æ ćż       | 0 = public, 1 = private, 2 = readonly, 3 = deprecated |

## POST äżć­èźŸçœź

POST /api/admin/setting/save

> Body èŻ·æ±ćæ°

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

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć        | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------------- | ------ | --------- | ------ |
| Authorization | header | string        | æŻ    |           | none   |
| body          | body   | array[object] | ćŠ    | æ°ç»    | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ć é€èźŸçœź

POST /api/admin/setting/delete

ä»çšäșćŒçšçèźŸçœź

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| key           | query  | string | æŻ    |           | none   |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

## POST éçœźä»€ç

POST /api/admin/setting/reset_token

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | ćŠ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": "alist-9d"
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | string  | true   | none   | æ°ä»€ç | none   |

## POST èźŸçœźaria2

POST /api/admin/setting/set_aria2

> Body èŻ·æ±ćæ°

```json
{
  "uri": "string",
  "secret": "string"
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć   | èŻŽæ |
| ------------- | ------ | ------ | ------ | ----------- | ------ |
| Authorization | header | string | æŻ    |             | none   |
| body          | body   | object | ćŠ    |             | none   |
| Â» uri        | body   | string | æŻ    | aria2ć°ć | none   |
| Â» secret     | body   | string | æŻ    | aria2ćŻé„ | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": "1.36.0"
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć   | èŻŽæ |
| ---------- | ------- | ------ | ------ | ----------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç    | none   |
| Â» message | string  | true   | none   | äżĄæŻ      | none   |
| Â» data    | string  | true   | none   | aria2çæŹ | none   |

## POST èźŸçœźqBittorrent

POST /api/admin/setting/set_qbit

> Body èŻ·æ±ćæ°

```json
{
  "url": "string",
  "seedtime": "string"
}
```

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć         | èŻŽæ |
| ------------- | ------ | ------ | ------ | ----------------- | ------ |
| Authorization | header | string | æŻ    |                   | none   |
| body          | body   | object | ćŠ    |                   | none   |
| Â» url        | body   | string | æŻ    | qBittorrentéŸæ„ | none   |
| Â» seedtime   | body   | string | æŻ    | ćç§æ¶éŽ      | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": "1.36.0"
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | string  | true   | none   |           | none   |

# admin/task/upload

## GET è·ćć·Čćźæä»»ćĄ

GET /api/admin/task/upload/done

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°        | ç±»ć   | ćżé | çșŠæ | äž­æć          | èŻŽæ |
| ------------- | -------- | ------ | ------ | ------------------ | ------ |
| Â» code       | integer  | true   | none   | ç¶æç           | none   |
| Â» message    | string   | true   | none   | äżĄæŻ             | none   |
| Â» data       | [object] | true   | none   |                    | none   |
| Â»Â» id       | string   | false  | none   | id                 | none   |
| Â»Â» name     | string   | false  | none   | ä»»ćĄć          | none   |
| Â»Â» state    | string   | false  | none   | ä»»ćĄćźæç¶æ | none   |
| Â»Â» status   | string   | false  | none   |                    | none   |
| Â»Â» progress | integer  | false  | none   | èżćșŠ             | none   |
| Â»Â» error    | string   | false  | none   | éèŻŻäżĄæŻ       | none   |

## POST è·ćä»»ćĄäżĄæŻ

POST /api/admin/task/upload/info

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ   |
| ------------- | ------ | ------ | ------ | --------- | -------- |
| tid           | query  | string | ćŠ    |           | ä»»ćĄid |
| Authorization | header | string | æŻ    |           | none     |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°        | ç±»ć   | ćżé | çșŠæ | äž­æć          | èŻŽæ |
| ------------- | -------- | ------ | ------ | ------------------ | ------ |
| Â» code       | integer  | true   | none   | ç¶æç           | none   |
| Â» message    | string   | true   | none   | äżĄæŻ             | none   |
| Â» data       | [object] | true   | none   |                    | none   |
| Â»Â» id       | string   | false  | none   | id                 | none   |
| Â»Â» name     | string   | false  | none   | ä»»ćĄć          | none   |
| Â»Â» state    | string   | false  | none   | ä»»ćĄćźæç¶æ | none   |
| Â»Â» status   | string   | false  | none   |                    | none   |
| Â»Â» progress | integer  | false  | none   | èżćșŠ             | none   |
| Â»Â» error    | string   | false  | none   | éèŻŻäżĄæŻ       | none   |

## GET è·ćæȘćźæä»»ćĄ

GET /api/admin/task/upload/undone

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

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

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°        | ç±»ć   | ćżé | çșŠæ | äž­æć          | èŻŽæ |
| ------------- | -------- | ------ | ------ | ------------------ | ------ |
| Â» code       | integer  | true   | none   | ç¶æç           | none   |
| Â» message    | string   | true   | none   | äżĄæŻ             | none   |
| Â» data       | [object] | true   | none   |                    | none   |
| Â»Â» id       | string   | false  | none   | id                 | none   |
| Â»Â» name     | string   | false  | none   | ä»»ćĄć          | none   |
| Â»Â» state    | string   | false  | none   | ä»»ćĄćźæç¶æ | none   |
| Â»Â» status   | string   | false  | none   |                    | none   |
| Â»Â» progress | integer  | false  | none   | èżćșŠ             | none   |
| Â»Â» error    | string   | false  | none   | éèŻŻäżĄæŻ       | none   |

## POST ć é€ä»»ćĄ

POST /api/admin/task/upload/delete

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ   |
| ------------- | ------ | ------ | ------ | --------- | -------- |
| tid           | query  | string | æŻ    |           | ä»»ćĄid |
| Authorization | header | string | æŻ    |           | none     |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST ćæ¶ä»»ćĄ

POST /api/admin/task/upload/cancel

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ   |
| ------------- | ------ | ------ | ------ | --------- | -------- |
| tid           | query  | string | æŻ    |           | ä»»ćĄid |
| Authorization | header | string | æŻ    |           | none     |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST éèŻä»»ćĄ

POST /api/admin/task/upload/retry

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ   |
| ------------- | ------ | ------ | ------ | --------- | -------- |
| tid           | query  | string | æŻ    |           | ä»»ćĄid |
| Authorization | header | string | æŻ    |           | none     |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST æžé€ć·Čćźæä»»ćĄ

POST /api/admin/task/upload/clear_done

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

## POST æžé€ć·Čæćä»»ćĄ

POST /api/admin/task/upload/clear_succeeded

### èŻ·æ±ćæ°

| ćç§°        | äœçœź | ç±»ć | ćżé | äž­æć | èŻŽæ |
| ------------- | ------ | ------ | ------ | --------- | ------ |
| Authorization | header | string | æŻ    |           | none   |

> èżćç€șäŸ

> 200 Response

```json
{
    "code": 200,
    "message": "success",
    "data": null
}
```

### èżćç»æ

| ç¶æç  | ç¶æç ć«äč                                         | èŻŽæ | æ°æźæšĄć |
| --------- | ------------------------------------------------------- | ------ | ------------ |
| 200       | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none   | Inline       |

### èżćæ°æźç»æ

ç¶æç  **200**

| ćç§°     | ç±»ć  | ćżé | çșŠæ | äž­æć | èŻŽæ |
| ---------- | ------- | ------ | ------ | --------- | ------ |
| Â» code    | integer | true   | none   | ç¶æç  | none   |
| Â» message | string  | true   | none   | äżĄæŻ    | none   |
| Â» data    | null    | true   | none   |           | none   |

# æ°æźæšĄć

