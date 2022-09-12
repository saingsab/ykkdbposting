declare @usd decimal
set @usd = (select SALE_CONV_RATE
from MST_CURRENCY
where CURRENCY_ID='USD')

select ps_bill.bill_no as "TransactionID",
 ps_bill.bill_no as "InvoiceID",
 fc_bill_setl.setl_no as "ReceiptID",
 ps_bill.bill_DT as "Date",
 ps_bill.C_DT as "Start Time",
 fc_bill_setl.c_DT as "End Time",
 PS_BILL.PAX as "NumberOfCustomer",
 ps_bill.bill_value as "SubTotal",
 ps_bill.DISC_AMT/@usd as "DiscountDollar",
 ps_bill.DISC_AMT as "DiscountRiel",
 ps_bill.DISC_NARRATION as "DiscountTypeID",
 ps_bill.TAXES as "VAT",
 ps_bill.bill_value-ps_bill.DISC_AMT as "Net",
 ps_bill.BILL_AMT as "GrandTotal",
 isnull (FC_BILL_SETL.FORN_AMT,0) as "PaymentDollar",
 isnull (FC_BILL_SETL.SETL_AMT,0) as "PaymentRiel",
 isnull (fc_bill_setl.EXTRA_AMT/@usd,0) as "ChangeDollar",
 isnull (fc_bill_setl.EXTRA_AMT,0) as "ChangeRiel",
 fc_bill_setl.SETL_MODE_ID as "PaymentMethodID",
 fc_bill_setl.C_BY as "Cashier",
 ps_bill.BILL_STATUS_IND as "StatusID",
 ps_bill.BILL_NARRATION as "Comment",
 CAST (isnull (fc_bill_setl.CURRENCY_ID,'KHR') as nvarchar) as Currency,
 isnull (fc_bill_setl.CONV_RATE,1) as "ExchangeRate"
into #T1
from fc_bill_setl, PS_BILL
where FC_BILL_SETL.BILL_NO=ps_bill.bill_no and fc_bill_setl.SETL_SNO=1

update #T1 set Currency='USD Dollar' where Currency='USD'

select ps_bill.bill_no as "TransactionID",
 FC_BILL_SETL.SETL_AMT as "PaymentRiel",
 fc_bill_setl.EXTRA_AMT as "ChangeRiel"
 into #T2
from fc_bill_setl, PS_BILL
where FC_BILL_SETL.BILL_NO=ps_bill.bill_no and fc_bill_setl.SETL_SNO=2

Merge #T1 Using #T2
On (#T1.TransactionID=#T2.TransactionID)
When Matched then update set 
#T1.paymentriel=#T1.paymentriel + #T2.paymentriel,
#T1.ChangeRiel=#T1.ChangeRiel + #T2.ChangeRiel;
Select * from #T1 order by #T1.TransactionID asc

drop table #T1
drop table #T2
