CREATE or ALTER PROCEDURE SP_Persons
@LastName varchar(255)
AS
	select * from Persons
	where LastName = @LastName
GO

exec SP_Persons @LastName = 'Ayoung'