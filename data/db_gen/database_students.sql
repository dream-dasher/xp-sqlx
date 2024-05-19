-- Create the university database
CREATE DATABASE university;
USE university;

-- Create the Students table
CREATE TABLE students (
    StudentID INT NOT NULL AUTO_INCREMENT,
    FirstName VARCHAR(100) NOT NULL,
    LastName VARCHAR(100) NOT NULL,
    DateOfBirth DATE,
    School ENUM('Sciences', 'Humanities', 'Other'),
    Email VARCHAR(255),
    PRIMARY KEY (StudentID)
);

-- Insert sample data into the Students table
INSERT INTO students (FirstName, LastName, DateOfBirth, School, Email)
VALUES 
    ('John', 'Doe', '1998-04-23', 'Sciences', 'john.doe@example.com'),
    ('Jane', 'Smith', '1999-08-14', 'Humanities', 'jane.smith@example.com'),
    ('Max', 'Smale', '1999-09-14', 'Humanities', 'maxxx@example.com'),
    ('Alex', 'Johnson', '2000-11-30', 'Other', 'alex.johnson@example.com'),
    ('Emily', 'Brown', '2000-01-05', 'Sciences', 'emily.brown@example.com'),
    ('Michael', 'Green', '2001-02-18', 'Humanities', 'michael.green@example.com'),
    ('Sarah', 'White', '1998-07-23', 'Other', 'sarah.white@example.com'),
    ('David', 'Black', '1999-03-15', 'Sciences', 'david.black@example.com'),
    ('Laura', 'Wilson', '1997-12-22', 'Humanities', 'laura.wilson@example.com'),
    ('James', 'Taylor', '1999-11-05', 'Other', 'james.taylor@example.com'),
    ('Emma', 'Moore', '2001-05-12', 'Sciences', 'emma.moore@example.com'),
    ('Chris', 'Anderson', '1998-09-30', 'Humanities', 'chris.anderson@example.com'),
    ('Olivia', 'Thomas', '2002-03-19', 'Other', 'olivia.thomas@example.com'),
    ('Daniel', 'Jackson', '1999-06-11', 'Sciences', 'daniel.jackson@example.com'),
    ('Sophia', 'Harris', '2000-10-08', 'Humanities', 'sophia.harris@example.com'),
    ('Andrew', 'Martinez', '1997-04-21', 'Sciences', 'andrew.martinez@example.com'),
    ('Ava', 'Garcia', '2001-02-25', 'Humanities', 'ava.garcia@example.com'),
    ('Ethan', 'Clark', '1998-05-30', 'Other', 'ethan.clark@example.com'),
    ('Isabella', 'Rodriguez', '1997-11-20', 'Sciences', 'isabella.rodriguez@example.com'),
    ('Noah', 'Lewis', '2000-08-14', 'Humanities', 'noah.lewis@example.com'),
    ('Mia', 'Lee', '1999-09-16', 'Other', 'mia.lee@example.com'),
    ('Lucas', 'Walker', '2002-12-03', 'Sciences', 'lucas.walker@example.com'),
    ('Charlotte', 'Hall', '1998-06-22', 'Humanities', 'charlotte.hall@example.com'),
    ('Henry', 'Allen', '1997-08-05', 'Sciences', 'henry.allen@example.com'),
    ('Amelia', 'Young', '2000-10-10', 'Humanities', 'amelia.young@example.com'),
    ('Mason', 'Hernandez', '1999-04-12', 'Other', 'mason.hernandez@example.com'),
    ('Ella', 'King', '2001-11-08', 'Sciences', 'ella.king@example.com'),
    ('Logan', 'Wright', '1998-03-07', 'Humanities', 'logan.wright@example.com'),
    ('Chloe', 'Lopez', '2002-09-25', 'Other', 'chloe.lopez@example.com'),
    ('Benjamin', 'Scott', '2000-06-18', 'Sciences', 'benjamin.scott@example.com'),
    ('Zoe', 'Adams', '1997-12-30', 'Humanities', 'zoe.adams@example.com'),
    ('Jack', 'Nelson', '1998-02-11', 'Sciences', 'jack.nelson@example.com'),
    ('Lily', 'Hill', '1999-07-21', 'Humanities', 'lily.hill@example.com'),
    ('Alexander', 'Ramirez', '2001-04-14', 'Other', 'alexander.ramirez@example.com'),
    ('Grace', 'Campbell', '1998-11-19', 'Sciences', 'grace.campbell@example.com'),
    ('Sebastian', 'Mitchell', '1997-05-27', 'Humanities', 'sebastian.mitchell@example.com'),
    ('Hannah', 'Roberts', '2002-01-09', 'Other', 'hannah.roberts@example.com'),
    ('Matthew', 'Carter', '1999-10-15', 'Sciences', 'matthew.carter@example.com'),
    ('Liam', 'Phillips', '2000-07-24', 'Humanities', 'liam.phillips@example.com'),
    ('Victoria', 'Evans', '1997-08-18', 'Sciences', 'victoria.evans@example.com'),
    ('Abigail', 'Turner', '2001-03-28', 'Humanities', 'abigail.turner@example.com'),
    ('William', 'Torres', '1998-04-17', 'Other', 'william.torres@example.com'),
    ('Natalie', 'Parker', '1997-09-01', 'Sciences', 'natalie.parker@example.com'),
    ('Dylan', 'Collins', '2000-11-06', 'Humanities', 'dylan.collins@example.com'),
    ('Madison', 'Edwards', '2002-05-30', 'Other', 'madison.edwards@example.com'),
    ('Jackson', 'Stewart', '1999-02-22', 'Sciences', 'jackson.stewart@example.com'),
    ('Leah', 'Morris', '2000-12-27', 'Humanities', 'leah.morris@example.com'),
    ('Jacob', 'Rogers', '1997-10-29', 'Sciences', 'jacob.rogers@example.com'),
    ('Avery', 'Reed', '1998-07-13', 'Humanities', 'avery.reed@example.com'),
    ('Samuel', 'Cook', '2001-01-20', 'Other', 'samuel.cook@example.com'),
    ('Elizabeth', 'Morgan', '1997-03-03', 'Sciences', 'elizabeth.morgan@example.com'),
    ('Eli', 'Bell', '2000-09-09', 'Humanities', 'eli.bell@example.com'),
    ('Ella', 'Bailey', '2002-11-05', 'Other', 'ella.bailey@example.com'),
    ('Owen', 'Cooper', '1998-08-02', 'Sciences', 'owen.cooper@example.com'),
    ('Savannah', 'Richardson', '2000-03-16', 'Humanities', 'savannah.richardson@example.com'),
    ('Caleb', 'Cox', '1999-12-26', 'Sciences', 'caleb.cox@example.com'),
    ('Scarlett', 'Howard', '1997-06-11', 'Humanities', 'scarlett.howard@example.com'),
    ('Aiden', 'Ward', '2001-10-07', 'Other', 'aiden.ward@example.com'),
    ('Victoria', 'Peterson', '1998-01-30', 'Sciences', 'victoria.peterson@example.com'),
    ('Evan', 'Gray', '2000-04-26', 'Humanities', 'evan.gray@example.com'),
    ('Zoe', 'Brooks', '2002-07-14', 'Other', 'zoe.brooks@example.com'),
    ('Connor', 'James', '1999-05-21', 'Sciences', 'connor.james@example.com'),
    ('Violet', 'Bennett', '2001-08-31', 'Humanities', 'violet.bennett@example.com');

-- Create the Courses table
CREATE TABLE courses (
    CourseID INT NOT NULL AUTO_INCREMENT,
    CourseName VARCHAR(255) NOT NULL,
    CourseCode VARCHAR(10) NOT NULL,
    Credits INT NOT NULL,
    PRIMARY KEY (CourseID)
);

-- Insert sample data into the Courses table
INSERT INTO courses (CourseName, CourseCode, Credits)
VALUES 
    ('Introduction to Computer Science', 'CS101', 4),
    ('Calculus I', 'MATH101', 3),
    ('English Literature', 'ENG101', 3);

-- Create the Professors table
CREATE TABLE professors (
    ProfessorID INT NOT NULL AUTO_INCREMENT,
    FirstName VARCHAR(100) NOT NULL,
    LastName VARCHAR(100) NOT NULL,
    Department VARCHAR(100),
    Email VARCHAR(255),
    PRIMARY KEY (ProfessorID)
);

-- Insert sample data into the Professors table
INSERT INTO professors (FirstName, LastName, Department, Email)
VALUES 
    ('Alan', 'Turing', 'Computer Science', 'alan.turing@university.edu'),
    ('Isaac', 'Newton', 'Mathematics', 'isaac.newton@university.edu'),
    ('William', 'Shakespeare', 'English', 'william.shakespeare@university.edu');

-- Create the Enrollments table to link Students and Courses
CREATE TABLE enrollments (
    EnrollmentID INT NOT NULL AUTO_INCREMENT,
    StudentID INT NOT NULL,
    CourseID INT NOT NULL,
    EnrollmentDate DATE,
    Grade ENUM('A', 'B', 'C', 'D', 'F', 'Incomplete'),
    PRIMARY KEY (EnrollmentID),
    FOREIGN KEY (StudentID) REFERENCES students(StudentID),
    FOREIGN KEY (CourseID) REFERENCES courses(CourseID)
);

-- Insert sample data into the Enrollments table
INSERT INTO enrollments (StudentID, CourseID, EnrollmentDate, Grade)
VALUES 
    (1, 1, '2023-09-01', 'A'),
    (1, 2, '2023-09-01', 'B'),
    (2, 1, '2023-09-01', 'A'),
    (2, 3, '2023-09-01', 'B'),
    (3, 3, '2023-09-01', 'A'),
    (4, 2, '2023-09-01', 'C');
