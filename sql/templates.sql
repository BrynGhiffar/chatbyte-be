select * from "User";

select
    first_value("Message"."id"),
    -- "Message"."senderId",
    -- "Message"."receiverId", 
    MAX("Message"."sentAt") 
from "Message" 
group by greatest("Message"."senderId", "Message"."receiverId"), least("Message"."senderId", "Message"."receiverId");

select
    "Message"."senderId",
    "Message"."receiverId", 
    MAX("Message"."sentAt") 
from "Message" 
group by "Message"."senderId", "Message"."receiverId";

select * 
from "Message"
where "Message"."senderId" = 2 or "Message"."receiverId" = 2
order by "Message"."sentAt" desc
limit 1;

select Max(tbl."id") as "messageId"
from (
    select
        "Message"."id",
        case when "Message"."senderId" < "Message"."receiverId" then "Message"."senderId" else "Message"."receiverId" end as c1,
        case when "Message"."senderId" < "Message"."receiverId" then "Message"."receiverId" else "Message"."senderId" end as c2
    from "Message"
) as tbl
group by c1, c2

select * from "Message"
join (
    select Max(tbl."id") as "messageId"
    from (
        select
            "Message"."id",
            case when "Message"."senderId" < "Message"."receiverId" then "Message"."senderId" else "Message"."receiverId" end as c1,
            case when "Message"."senderId" < "Message"."receiverId" then "Message"."receiverId" else "Message"."senderId" end as c2
        from "Message"
    ) as tbl
    group by c1, c2
) as lasts on "Message"."id" = lasts."messageId";

select * from "Message"
where "Message"."id" in (
    select Max(tbl."id") as "messageId"
    from (
        select
            "Message"."id",
            case when "Message"."senderId" < "Message"."receiverId" then "Message"."senderId" else "Message"."receiverId" end as c1,
            case when "Message"."senderId" < "Message"."receiverId" then "Message"."receiverId" else "Message"."senderId" end as c2
        from "Message"
    ) as tbl
    group by c1, c2
);

select
    "Message"."id",
    "Message"."sentAt",
    "Message"."receiverId",
    "Message"."senderId",
    "Message"."content",
    lasts."unreadCount",
    "User"."username"
from "Message" 
join "User" on
    (case when "Message"."senderId" = 2 then "Message"."receiverId" else "Message"."senderId" end) = "User"."id"
join (
    select 
        max(tbl."id") as "messageId",
        sum(case when (tbl."read" = false and tbl."senderId" != 2) then 1 else 0 end) as "unreadCount"
    from (
        select
            "Message"."id",
            "Message"."read",
            "Message"."senderId",
            case when "Message"."senderId" < "Message"."receiverId" then "Message"."senderId" else "Message"."receiverId" end as c1,
            case when "Message"."senderId" < "Message"."receiverId" then "Message"."receiverId" else "Message"."senderId" end as c2
        from "Message"
    ) as tbl
    group by c1, c2
) as lasts on "Message"."id" = lasts."messageId"
where "Message"."senderId" = 2 or "Message"."receiverId" = 2;
;

select 
    sum(case when tbl."read" = false then 1 else 0 end) as "unreadCount",
    max(tbl."id"), 
    c1, 
    c2
from (
    select
        "Message"."id",
        case when "Message"."senderId" < "Message"."receiverId" then "Message"."senderId" else "Message"."receiverId" end as c1,
        case when "Message"."senderId" < "Message"."receiverId" then "Message"."receiverId" else "Message"."senderId" end as c2,
        "Message"."read"
    from "Message"
) as tbl
group by c1, c2

update "Message" set "read" = false;

select * from "Message" where "Message"."id" = 112;