# terrible api design document

client polls server every so often and caches posts, feeds etc

state updates are cached when offline and updated at once when online again

# Auth

tbd lol (maybe just manually generated JWTs?)

# Endpoints

## GET /unread

returns array of unread post objects, with body

## GET /all

returns array of all post objects, no body

## GET /posts/:id

returns full post with body

## PATCH /status

this will often be done in bulk after the client comes back online

body:
```
[
    {
        "id": postId,
        "progress": number, // i'm not sure exactly how this'll work
        "read": boolean
    }
]
```

## GET /feeds

returns array of feed objects

## POST /feeds/add

body:
```
{
    "url": url
}
```

response: feed object

## DELETE /feeds/:id


# Errors

http status code with plaintext description in body

# Objects

timestamps are always unix seconds

## Feed
```
{
    "name": string,
    "description": string,
    "link": string,
    "lastUpdated": timestamp,
    "items": FeedItem[]
}
```

## FeedItem
```
{
    "id": string?,
    "title": string?,
    "link": string?,
    "description": string?,
    "pubDate": timestamp?,
    "content": string?
}
```

postId is the post ID or the title with a number for deduplication if absent? this seems like an edge case