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
    ('Alex', 'Johnson', '2000-11-30', 'Other', 'alex.johnson@example.com');

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
