# todo: bitpack
# basic type
type
    vec8* = object
        x*: int8
        y*: int8
# short-hand constructor
proc v8*(x, y: int8): vec8 =
    vec8(x: x, y: y)

# field arithmetic
proc `+`*(u1, u2: vec8): vec8 =
    v8(u1.x + u2.x, u1.y + u2.y)
proc `-`*(u1, u2: vec8): vec8 =
    v8(u1.x - u2.x, u1.y - u2.y)
proc `-`*(u: vec8): vec8 =
    v8(-u.x, -u.y)
proc `*`*(a: int8, u: vec8): vec8 =
    v8(a * u.x, a * u.y)

# inner product
proc `*`*(u1, u2: vec8): int8 =
    u1.x * u2.x  +  u1.y * u2.y

# incremental updates
proc `+=`*(u1 : var vec8, u2: vec8) =
    u1.x += u2.x
    u1.y += u2.y
proc `-=`*(u1 : var vec8, u2: vec8) =
    u1.x -= u2.x
    u1.y -= u2.y
proc `*=`*(u : var vec8, a: int8) =
    u.x *= a
    u.y *= a

# rendering
proc `$`*(u: vec8): string =
    "<" & $u.x & " " & $u.y & ">"
proc `$$`*(u: vec8): string =
    "v(" & $u.x & "," & $u.y & ")"

# CONSTANTS
const
    v8_zero*  = v8(0, 0)
    v8_up*    = v8(0,-1)
    v8_down*  = v8(0,+1)
    v8_left*  = v8(-1,0)
    v8_right* = v8(+1,0)
