CREATE TABLE class_groups(
	Name varchar(50) PRIMARY KEY
);

CREATE TABLE courses(
	Name varchar(50) PRIMARY KEY
);

CREATE TABLE group_to_courses(
	GroupName varchar(50),
	CourseName varchar(50),
	PRIMARY KEY (GroupName, CourseName),
	FOREIGN KEY (GroupName) REFERENCES class_groups(Name),
	FOREIGN KEY (CourseName) REFERENCES courses(Name)
);


