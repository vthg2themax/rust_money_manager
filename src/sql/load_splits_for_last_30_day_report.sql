
SELECT  
    (SELECT t.Post_date FROM transactions AS t WHERE t.guid=splits.tx_guid) AS 'PostDate', 
    splits.guid,splits.tx_guid,  
    (Select t.description FROM transactions As t WHERE t.guid=splits.tx_guid) As 'Description', 
    (SELECT c.mnemonic FROM commodities as c WHERE  
    c.guid=(SELECT t.currency_guid FROM transactions as t WHERE t.guid=splits.tx_guid)) as 'Currency',  
    (SELECT a.name FROM accounts as a WHERE a.guid=splits.account_guid) AS 'account_name',  
    (SELECT a.account_type FROM accounts as a WHERE a.guid=splits.account_guid) AS 'AccountType',  
    splits.account_guid,  
    (SELECT CAST (value_num AS Real) / value_denom) as 'Amount',  
    splits.memo,splits.action,splits.reconcile_state,splits.reconcile_date,splits.value_num, 
    splits.value_denom,splits.quantity_num,splits.quantity_denom,splits.lot_guid FROM splits  
    WHERE splits.tx_guid IN ( 
    Select t.guid  
        FROM transactions as t  
    WHERE datetime(substr(t.post_date,1,4)||'-'||substr(t.post_date,5,2)||'-'||substr(t.post_date,7,2)||' '|| 
    substr(t.post_date,9,2)||':'||substr(t.post_date,11,2)||':'||substr(t.post_date,13,2)) >=  
    Datetime(?) AND datetime(substr(t.post_date,1,4)||'-'||substr(t.post_date,5,2)||'-'|| 
    substr(t.post_date,7,2)||' '||substr(t.post_date,9,2)||':'||substr(t.post_date,11,2)||':'||substr(t.post_date,13,2))  
        <= Datetime(?)  
    ) AND AccountType=?
ORDER BY Description;