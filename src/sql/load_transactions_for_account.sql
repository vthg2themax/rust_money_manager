SELECT (SELECT ?) AS 'excluded_account_guid',
(SELECT a.name FROM accounts AS a WHERE a.guid=?) AS 'excluded_account_name',
(SELECT commodities.mnemonic FROM commodities WHERE commodities.guid=(
		SELECT t.currency_guid FROM transactions AS t WHERE t.guid=splits.tx_guid
	)
) AS 'excluded_account_mnemonic',
splits.tx_guid AS 'guid',
(SELECT t.currency_guid FROM transactions AS t WHERE t.guid=splits.tx_guid) AS 'currency_guid', 
(SELECT t.num FROM transactions AS t WHERE t.guid=splits.tx_guid) AS 'num', 
(SELECT t.post_date FROM transactions AS t WHERE t.guid=splits.tx_guid) AS 'post_date', 
(SELECT t.enter_date FROM transactions AS t WHERE t.guid=splits.tx_guid) AS 'enter_date', 
(SELECT t.description FROM transactions AS t WHERE t.guid=splits.tx_guid) As 'description', 
splits.value_num, splits.value_denom, 
(SELECT a.name FROM accounts AS a WHERE a.guid=splits.account_guid) As 'account_name', 
splits.account_guid,
(COALESCE((SELECT slots.string_val FROM slots WHERE slots.obj_guid=splits.tx_guid),'')) AS 'memo' 

FROM splits WHERE splits.tx_guid In (
	SELECT transactions.guid FROM transactions where transactions.guid IN (
		SELECT splits.tx_guid FROM splits where splits.account_guid=?
	) 
) AND splits.account_guid NOT IN (?)
ORDER BY (
    SELECT substr(post_date,0,5)||"-"||
           substr(post_date,5,2)||"-"||
           substr(post_date,7,2)||" "||
           substr(post_date,9,2)||":"||
           substr(post_date,11,2)||":"||
           substr(post_date,13,2)
    ) ASC;
--20150312050000 = 2015-03-12 05:00:00 we need to get to an SQLite sortable date value with the substr above