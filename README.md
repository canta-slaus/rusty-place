# r(usty)/place
A very basic "clone" of [r/place](https://www.reddit.com/r/place/) written in Rust using [warp](https://github.com/seanmonstar/warp).

Instead of rendering the image on the client side, the image is encoded as PNG and sent to the user. This doesn't scale well as that image is constructed every time someone sends a GET requests to `/image`. Ideally, you would render the image client-side and e.g. using WebSockets, send them notifications when a pixel was edited.

# Features
There aren't that many:
- When the server shuts down, the image data is stored in a file `image` and loaded back up when the server starts again.

# API
<details>
 <summary>`GET` `/` `basic welcoming message`</summary>

#### Responses
| HTTP code | content-type       | response                                                   |
|-----------|--------------------|------------------------------------------------------------|
| `200`     | `application/json` | `{"code":200,"message":"Welcome to r/place but in Rust!"}` |

#### Example
```
$ curl -X GET http://localhost:3030/
```
</details>

<details>
 <summary>`GET` `/image` `sends the current canvas`</summary>

#### Responses
| HTTP code | content-type | response |
|-----------|--------------|----------|
| `200`     | `image/png`  |          |

#### Example
```
$ curl -X GET http://localhost:3030/image > image.png
```
</details>

<details>
 <summary>`PUT` `/set-pixel/:x/:y` `set the color of a pixel`</summary>

#### Parameters
| name |  type    | data type | description                   |
|------|----------|-----------|-------------------------------|
| `x`  | required | `usize`   | The `x` position of the pixel |
| `y`  | required | `usize`   | The `y` position of the pixel |

#### Body
- max. `Content-Length` is 32 bytes
- `hex_color` has to be string of length 6

```json
{
    "color": hex_color
}
```

Example:
```json
{"color":"FFFFFF"}
```

### Header
| name      |  type    | data type | description                               |
|-----------|----------|-----------|-------------------------------------------|
| `X-Token` | required | `String`  | The "auth" token (has to be set to `abc`) |

#### Responses
| HTTP code | content-type       | response                                             | description                     |
|-----------|--------------------|------------------------------------------------------|---------------------------------|
| `200`     | `application/json` | `{"code":200,"message":"Successfully edited pixel"}` |                                 |
| `400`     | `application/json` | `{"code":400,"message":"MISSING_HEADER: X-Token"}`   |                                 |
| `401`     | `application/json` | `{"code":401,"message":"BAD_AUTH"}`                  | `X-Token` wasn't `abc`          |
| `411`     | `application/json` | `{"code":411,"message":"MISSING_CONTENT_LENGTH"}`    | Missing body                    |
| `413`     | `application/json` | `{"code":413,"message":"PAYLOAD_TOO_LARGE"}`         | Body too big (max. 32 bytes)    |
| `422`     | `application/json` | `{"code":422,"message":"MALFORMED_BODY"}`            | Body couldn't be parsed to JSON |
| `422`     | `application/json` | `{"code":422,"message":"OUT_OF_BOUNDS"}`             | Pixel out of bounds             |
| `422`     | `application/json` | `{"code":422,"message":"INVALID_COLOR"}`             | Invalid hex color               |

#### Example
```
$ curl -X PUT -H "X-Token: abc" -H "Content-Type: application/json" -d "{\"color\":\"ffffff\"}" http://localhost:3030/set-pixel/10/10
```
</details>

# Future ideas
- Add "proper" authentication i.e. creating accounts with unique auth tokens
- Add cooldowns to users to prevent spamming
