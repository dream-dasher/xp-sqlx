SELECT StudentID as id, 
       FirstName as first_name, 
       LastName as last_name, 
       DateOfBirth as dob, 
       School as school, 
       Email as email
FROM students 
WHERE StudentID > ?
AND FirstName LIKE CONCAT ('%', ?, '%');
