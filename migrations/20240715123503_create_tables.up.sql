CREATE TABLE class_groups(
	name varchar(50) PRIMARY KEY
);

CREATE TABLE courses(
	name varchar(50) PRIMARY KEY
);

CREATE TABLE group_to_courses(
	group_name varchar(50),
	course_name varchar(50),

	PRIMARY KEY (group_name, course_name),

	FOREIGN KEY (group_name) REFERENCES class_groups(name)
		ON DELETE CASCADE
		ON UPDATE CASCADE,

	FOREIGN KEY (course_name) REFERENCES courses(name)
		ON DELETE CASCADE
		ON UPDATE CASCADE

);

CREATE TABLE classes (
	id int AUTO_INCREMENT PRIMARY KEY,	
	course_name varchar(50),
	location varchar(50),
	professor varchar(50),
	type varchar(1),
	group_name varchar(50),
	slot int,
	day varchar(50),
	
	FOREIGN KEY (course_name) REFERENCES courses(name)
		ON DELETE CASCADE
		ON UPDATE CASCADE,

	FOREIGN KEY (group_name) REFERENCES class_groups(name)
		ON DELETE CASCADE
		ON UPDATE CASCADE
)
