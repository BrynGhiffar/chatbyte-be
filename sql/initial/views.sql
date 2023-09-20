
CREATE VIEW message_sender as 
    select
        message.id,
        message.sent_at,
        message.read,
        message.content,
        case when message.sender_id < message.receiver_id then message.sender_id else message.receiver_id end as min_sender,
        case when message.sender_id < Message.receiver_id then message.receiver_id else message.sender_id end as max_sender
    from message;

CREATE VIEW last_message as 
    select
        max(id) as last_msg_id, 
        min_sender, 
        max_sender 
    from message_sender
    group by min_sender, max_sender;

CREATE VIEW unread_message_count as
    select count(*) as unread_count, receiver_id, sender_id from message where read = false group by receiver_id, sender_id;

CREATE VIEW unread_message_content as
    select
        message.id,
        message.receiver_id,
        message.sender_id,
        message.sent_at,
        (case when unread_message_count.unread_count is null then 0 else unread_message_count.unread_count end) as unread_count,
        message.content as last_message
    from message
    join last_message on message.id = last_message.last_msg_id
    left join unread_message_count on message.sender_id = unread_message_count.sender_id and message.receiver_id = unread_message_count.receiver_id
    ;