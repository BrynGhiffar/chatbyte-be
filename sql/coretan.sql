SELECT * FROM PUBLIC.GROUP;

SELECT * FROM PUBLIC.MESSAGE;

SELECT 
    public.group.name as name, 
    public.group.id as group_id, 
    a.id as message_id, 
    a.content as content,
    a.sent_at as sent_at,
    a.sender_id as sender_id 
FROM (
    SELECT 
        group_message.group_id, 
        group_message.id, 
        group_message.content, 
        group_message.sent_at, 
        group_message.sender_id 
    FROM (
        SELECT MAX(id) id,
            public.group_message.group_id
        FROM public.group_message
        GROUP BY group_id
    ) as last_msg
    JOIN public.group_message 
    ON last_msg.id = group_message.id
) AS a JOIN public.group ON public.group.id = a.group_id;

SELECT * FROM PUBLIC.USER;

SELECT 
    G.ID GROUP_ID,
    G.NAME GROUP_NAME,
    U.USERNAME USERNAME,
    M.SENT_AT SENT_AT,
    M.CONTENT CONTENT,
    M.DELETED DELETED
FROM PUBLIC.GROUP_MESSAGE M
    FULL JOIN PUBLIC.GROUP G ON G.ID = M.GROUP_ID
    FULL JOIN PUBLIC.USER U ON U.ID = M.SENDER_ID
WHERE G.ID IS NOT NULL
AND 
    CASE
        WHEN M.SENT_AT IS NOT NULL THEN (M.SENT_AT IN (SELECT MAX(sent_at) as id FROM PUBLIC.GROUP_MESSAGE GROUP BY PUBLIC.GROUP_MESSAGE.GROUP_ID)) ELSE TRUE
    END
-- AND EXISTS (SELECT 1 FROM PUBLIC.GROUP_MEMBER GMEM WHERE GMEM.GROUP_ID = G.ID AND GMEM.USER_ID = 1)
;

SELECT * FROM PUBLIC.USER_AVATAR LIMIT 1;

SELECT 
    GMEM.GROUP_ID, 
    GMEM.USER_ID, 
    GM.CONTENT, 
    GM.SENT_AT 
FROM GROUP_MEMBER GMEM
    LEFT JOIN GROUP_MESSAGE GM ON GM.GROUP_ID = GMEM.GROUP_ID
WHERE GMEM.USER_ID = 1
AND 
    CASE
        WHEN GM.SENT_AT IS NOT NULL THEN (GM.SENT_AT IN (SELECT MAX(sent_at) as id FROM PUBLIC.GROUP_MESSAGE GROUP BY PUBLIC.GROUP_MESSAGE.GROUP_ID)) ELSE TRUE
    END
;

SELECT 
    M.ID MESSAGE_ID,
    G.ID GROUP_ID,
    G.NAME GROUP_NAME,
    U.USERNAME USERNAME,
    M.SENT_AT SENT_AT,
    M.CONTENT CONTENT,
    M.DELETED DELETED
FROM PUBLIC.GROUP_MESSAGE M
    JOIN PUBLIC.GROUP G ON G.ID = M.GROUP_ID
    JOIN PUBLIC.USER U ON U.ID = M.SENDER_ID
WHERE 
    M.SENT_AT IN (SELECT MAX(sent_at) as id FROM PUBLIC.GROUP_MESSAGE GROUP BY PUBLIC.GROUP_MESSAGE.GROUP_ID)
AND EXISTS (SELECT 1 FROM PUBLIC.GROUP_MEMBER GMEM WHERE GMEM.GROUP_ID = G.ID AND GMEM.USER_ID = 3)
ORDER BY M.ID ASC
;

SELECT 
    GM.ID as ID,
    GM.GROUP_ID,
    GM.sender_id, 
    U.username, 
    GM.sent_at,
    GM.content
FROM PUBLIC.GROUP_MESSAGE GM
    JOIN PUBLIC.USER U ON GM.SENDER_ID = U.ID
WHERE GM.GROUP_ID = 1 AND DELETED = FALSE
ORDER BY GM.SENT_AT
;

select * FROM PUBLIC.GROUP_MESSAGE WHERE GROUP_ID = 1;

INSERT INTO PUBLIC.GROUP_MESSAGE_READ(group_id, message_id, reader_id) VALUES (3, 2, 3), (3, 3, 3), (3, 4, 3), (3, 5, 3);

insert into public.group_message_read(message_id, reader_id, group_id) select id as message_id, 3 as reader_id, 1 as group_id from group_message where id > 12;

select gm.group_id, sum(case when reader_id is not null then 1 else 0 end)
    from group_member gm
    left join group_message_read gmr on gm.group_id = gmr.group_id
where gm.user_id = 1 
-- and 
-- case
--     when gmr.reader_id is not null then gmr.reader_id = 1 else true
-- end
group by gm.group_id
;

select gm.group_id, sum(case when reader_id is not null then 1 else 0 end)
    from group_member gm
    left join group_message_read gmr on gm.group_id = gmr.group_id
where gm.user_id = 1 and 
case
    when gmr.reader_id is not null then gmr.reader_id = 1 else true
end
group by gm.group_id
;

SELECT * FROM 
(
    SELECT GMEM.GROUP_ID,
        (SUM(
            CASE WHEN GM.ID IS NOT NULL THEN 1 ELSE 0 END
        ) - 
        SUM(
            CASE WHEN GMR.READER_ID IS NOT NULL THEN 1 ELSE 0 END
        )) AS UNREAD_MESSAGE
    FROM PUBLIC.GROUP_MEMBER GMEM
        LEFT JOIN PUBLIC.GROUP_MESSAGE GM ON GM.GROUP_ID = GMEM.GROUP_ID
        LEFT JOIN PUBLIC.GROUP_MESSAGE_READ GMR 
            ON GMR.GROUP_ID = GMEM.GROUP_ID 
                AND GMR.READER_ID = GMEM.USER_ID
                AND GMR.MESSAGE_ID = GM.ID
    WHERE GMEM.USER_ID = 2
    GROUP BY GMEM.GROUP_ID
) UM
    JOIN PUBLIC.LAST_MESSAGE_GROUP LMG ON UM.GROUP_ID = LMG.GROUP_ID
;

            SELECT * FROM 
            (
                SELECT GMEM.GROUP_ID,
                    (SUM(
                        CASE WHEN GM.ID IS NOT NULL THEN 1 ELSE 0 END
                    ) - 
                    SUM(
                        CASE WHEN GMR.READER_ID IS NOT NULL THEN 1 ELSE 0 END
                    )) AS UNREAD_MESSAGE
                FROM PUBLIC.GROUP_MEMBER GMEM
                    LEFT JOIN PUBLIC.GROUP_MESSAGE GM ON GM.GROUP_ID = GMEM.GROUP_ID
                    LEFT JOIN PUBLIC.GROUP_MESSAGE_READ GMR 
                        ON GMR.GROUP_ID = GMEM.GROUP_ID 
                            AND GMR.READER_ID = GMEM.USER_ID
                            AND GMR.MESSAGE_ID = GM.ID
                WHERE GMEM.USER_ID = 1
                GROUP BY GMEM.GROUP_ID
            ) UM
                JOIN PUBLIC.LAST_MESSAGE_GROUP LMG ON UM.GROUP_ID = LMG.GROUP_ID;

select * from group_member where group_id = 1;

SELECT GMEM.GROUP_ID,
    (SUM(
        CASE WHEN GM.ID IS NOT NULL THEN 1 ELSE 0 END
    ) - 
    SUM(
        CASE WHEN GMR.READER_ID IS NOT NULL THEN 1 ELSE 0 END
    )) AS UNREAD_MESSAGE
    FROM PUBLIC.GROUP_MEMBER GMEM
    LEFT JOIN PUBLIC.GROUP_MESSAGE GM ON GM.GROUP_ID = GMEM.GROUP_ID
    LEFT JOIN PUBLIC.GROUP_MESSAGE_READ GMR ON GMR.GROUP_ID = GMEM.GROUP_ID AND GMR.READER_ID = GMEM.USER_ID AND GMR.MESSAGE_ID = GM.ID
WHERE GMEM.USER_ID = 2
GROUP BY GMEM.GROUP_ID

INSERT INTO PUBLIC.GROUP_MESSAGE_READ(message_id, reader_id, group_id)
SELECT GM.id as message_id, 1 as reader_id, GM.GROUP_ID FROM PUBLIC.GROUP_MESSAGE GM
WHERE 
GM.GROUP_ID = 1
AND
NOT EXISTS (
    SELECT 1 FROM PUBLIC.GROUP_MESSAGE_READ GMR 
        WHERE GMR.message_id = GM.id 
            AND GMR.reader_id = 1
            AND GMR.group_id = 1
);

SELECT * FROM PUBLIC.GROUP_MEMBER WHERE USER_ID = 2;

select group_id, count(distinct message_id) from public.group_message_read group by group_id;

SELECT 
    GM.ID as ID,
    GM.SENDER_ID AS SENDER_ID,
    U.USERNAME AS USERNAME,
    GM.CONTENT AS CONTENT,
    GM.GROUP_ID AS GROUP_ID,
    GM.DELETED AS DELETED,
    GM.SENT_AT AS SENT_AT
FROM PUBLIC.GROUP_MESSAGE  GM
    JOIN PUBLIC.USER U ON U.ID = GM.SENDER_ID
    WHERE GROUP_ID = 1;



SELECT     
    GM.ID as ID,
    GM.SENDER_ID AS SENDER_ID,
    U.USERNAME AS USERNAME,
    GM.CONTENT AS CONTENT,
    GM.GROUP_ID AS GROUP_ID,
    GM.DELETED AS DELETED,
    GM.SENT_AT AS SENT_AT
FROM (INSERT INTO PUBLIC.GROUP_MESSAGE (GROUP_ID, SENDER_ID, CONTENT) VALUES(1, 2, 'TESTING...') RETURNING *) GM
    JOIN PUBLIC.USER U ON U.ID = GM.SENDER_ID
    WHERE GROUP_ID = 1;
;

WITH GM AS (
    INSERT INTO PUBLIC.GROUP_MESSAGE (GROUP_ID, SENDER_ID, CONTENT) VALUES(1, 2, 'TESTING...') RETURNING *
) SELECT     
    GM.ID as ID,
    GM.SENDER_ID AS SENDER_ID,
    U.USERNAME AS USERNAME,
    GM.CONTENT AS CONTENT,
    GM.GROUP_ID AS GROUP_ID,
    GM.DELETED AS DELETED,
    GM.SENT_AT AS SENT_AT
FROM GM JOIN PUBLIC.USER U ON U.ID = GM.SENDER_ID;

SELECT
    UNREAD_MESSAGE_CONTENT.ID,
    CASE WHEN SENDER_ID = 3 THEN RECEIVER_ID ELSE SENDER_ID END AS CONTACT_ID,
    SENT_AT,
    LAST_MESSAGE,
    CASE WHEN RECEIVER_ID != 3 THEN 0 ELSE UNREAD_COUNT END,
    PUBLIC.USER.USERNAME
FROM PUBLIC.UNREAD_MESSAGE_CONTENT
    JOIN PUBLIC.USER ON 
        CASE WHEN LEAST(RECEIVER_ID, SENDER_ID) = 3 THEN
            GREATEST(RECEIVER_ID, SENDER_ID) = PUBLIC.USER.ID
        ELSE
            LEAST(RECEIVER_ID, SENDER_ID) = PUBLIC.USER.ID
        END
WHERE 3 IN (SENDER_ID, RECEIVER_ID)

SELECT * FROM PUBLIC.MESSAGE;

SELECT
    UNREAD_MESSAGE_CONTENT.ID,
    CASE WHEN SENDER_ID = 2 THEN RECEIVER_ID ELSE SENDER_ID END AS CONTACT_ID,
    SENT_AT,
    LAST_MESSAGE,
    CASE WHEN RECEIVER_ID != 2 THEN 0 ELSE UNREAD_COUNT END,
    PUBLIC.USER.USERNAME,
    DELETED
FROM PUBLIC.UNREAD_MESSAGE_CONTENT
    JOIN PUBLIC.USER ON 
        CASE WHEN LEAST(RECEIVER_ID, SENDER_ID) = 2 THEN
            GREATEST(RECEIVER_ID, SENDER_ID) = PUBLIC.USER.ID
        ELSE
            LEAST(RECEIVER_ID, SENDER_ID) = PUBLIC.USER.ID
        END
WHERE 2 IN (SENDER_ID, RECEIVER_ID)
ORDER BY SENT_AT DESC

select * from public.unread_message_content;

SELECT * FROM 
(
    SELECT GMEM.GROUP_ID,
        (SUM(
            CASE WHEN GM.ID IS NOT NULL THEN 1 ELSE 0 END
        ) - 
        SUM(
            CASE WHEN GMR.READER_ID IS NOT NULL THEN 1 ELSE 0 END
        )) AS UNREAD_MESSAGE
    FROM PUBLIC.GROUP_MEMBER GMEM
        LEFT JOIN PUBLIC.GROUP_MESSAGE GM ON GM.GROUP_ID = GMEM.GROUP_ID
        LEFT JOIN PUBLIC.GROUP_MESSAGE_READ GMR 
            ON GMR.GROUP_ID = GMEM.GROUP_ID 
                AND GMR.READER_ID = GMEM.USER_ID
                AND GMR.MESSAGE_ID = GM.ID
    WHERE GMEM.USER_ID = 1
    GROUP BY GMEM.GROUP_ID
) UM
    JOIN PUBLIC.LAST_MESSAGE_GROUP LMG ON UM.GROUP_ID = LMG.GROUP_ID
ORDER BY LMG.SENT_AT DESC NULLS LAST

SELECT * FROM PUBLIC.GROUP_MESSAGE WHERE CONTENT LIKE '%' || 'it was' || '%';

SELECT     
    GM.ID as ID,
    GM.SENDER_ID AS SENDER_ID,
    U.USERNAME AS USERNAME,
    GM.CONTENT AS CONTENT,
    GM.GROUP_ID AS GROUP_ID,
    GM.DELETED AS DELETED,
    GM.SENT_AT AS SENT_AT
FROM PUBLIC.GROUP_MESSAGE GM 
    JOIN PUBLIC.USER U ON U.ID = GM.SENDER_ID
WHERE GM.ID = 91;

-- SELECT * FROM PUBLIC.MESSAGE WHERE ID = 68;
SELECT 
    A.ID AS ID,
    A.NAME AS NAME,
    A.CONTENT AS CONTENT
FROM PUBLIC.ATTACHMENT_MESSAGE AM
    JOIN PUBLIC.ATTACHMENT A ON A.ID = AM.ATTACHMENT_ID
WHERE AM.DIRECT_MESSAGE_ID = 68;

SELECT * FROM PUBLIC.ATTACHMENT_MESSAGE ORDER BY ATTACHMENT_ID DESC LIMIT 5;

SELECT * FROM PUBLIC.ATTACHMENT ORDER BY ID DESC LIMIT 5;