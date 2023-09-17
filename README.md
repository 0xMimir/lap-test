```sql
WITH cte AS (
SELECT
	"users"."id",
	"users"."username",
	"users"."email"
FROM
	"users" )
SELECT
	*
FROM
		(
	SELECT
		*,
		FALSE
	FROM
		cte
	LIMIT 10 offset 10
) sub
UNION (
SELECT
				*,
				TRUE
FROM
		cte
LIMIT 1
)
```