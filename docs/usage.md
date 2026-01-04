
# Usage 

## Generate new UUID

```shell
uuid gen # by default v4
uuid gen v4 # to generate UUIDv4
uuid gen v7 # to generate UUIDv7
```

## Format UUID

```shell
$ uuid fmt 550e8400-e29b-41d4-a716-446655440000 --canonical
550e8400-e29b-41d4-a716-446655440000

$ uuid fmt 550e8400-e29b-41d4-a716-446655440000 --compact
550e8400e29b41d4a716446655440000 

$ uuid fmt 550e8400-e29b-41d4-a716-446655440000 --hex
0x550e8400e29b41d4a716446655440000 

$ uuid fmt 550e8400-e29b-41d4-a716-446655440000 --uppercase
550E8400E29B41D4A716446655440000

$ echo "550e8400-e29b-41d4-a716-446655440000" | uuid fmt --canonical
550e8400-e29b-41d4-a716-446655440000
```

### Combaning with `gen`

```shell
$ uuid gen v7 --uppercase
019B89BC0FB873E4B9F92EED6E6AB8BE
```

## Info

```shell
$ uuid info 550e8400-e29b-41d4-a716-446655440000
version: 4
... # other info specific to version like timestamp, node ID or name 
```

## Normalize UUID

Normalizes UUID with mixed byteorder (like .NET`s System.Guid)
to canonical format:

```shell
$ uuid normalize D004A78FD44C1B4E8213324AE10814DC
8FA704D04CD44E1B8213324AE10814DC
```

.NET System.Guid has mixed byteorder so it's incompatible with RFC4122:
- First 4 bytes (Data1): Reversed (little-endian)
- Next 2 bytes (Data2): Reversed (little-endian)
- Next 2 bytes (Data3): Reversed (little-endian)
- Last 8 bytes: Stay in original order (big-endian)
