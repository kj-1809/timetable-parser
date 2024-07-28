CREATE TABLE class_group(
	name varchar(50) PRIMARY KEY
);

CREATE TABLE course(
	name varchar(100) PRIMARY KEY
);

CREATE TABLE group_to_courses(
	group_name varchar(50),
	course_name varchar(100),

	PRIMARY KEY (group_name, course_name),

	FOREIGN KEY (group_name) REFERENCES class_group(name)
		ON UPDATE CASCADE,

	FOREIGN KEY (course_name) REFERENCES course(name)
		ON UPDATE CASCADE
);

CREATE TABLE detail_class (
	id int AUTO_INCREMENT PRIMARY KEY,	
	course_name varchar(100),
	location varchar(100),
	professor varchar(100),
	type varchar(1),
	group_name varchar(50),
	slot int,
	day varchar(50),
	
	FOREIGN KEY (course_name) REFERENCES course(name)
		ON UPDATE CASCADE,

	FOREIGN KEY (group_name) REFERENCES class_group(name)
		ON UPDATE CASCADE
)
