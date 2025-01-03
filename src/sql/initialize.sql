-- Drop the table if it exists
DROP TABLE IF EXISTS responses;

-- Create the table
CREATE TABLE responses (
    id INTEGER PRIMARY KEY,
    Text_Input TEXT NOT NULL
);

-- Insert initial data
INSERT INTO responses (Text_Input) VALUES (" blood cancer");
INSERT INTO responses (Text_Input) VALUES ("2 sisters with breast cancer");
INSERT INTO responses (Text_Input) VALUES ("5 different cancer/20 years");
INSERT INTO responses (Text_Input) VALUES ("age 46/  acute lymphocytic leukemia");
INSERT INTO responses (Text_Input) VALUES ("both sisters");
INSERT INTO responses (Text_Input) VALUES ("breast ca , three of them");
INSERT INTO responses (Text_Input) VALUES ("breast cancer 34");
INSERT INTO responses (Text_Input) VALUES ("colon ca");
INSERT INTO responses (Text_Input) VALUES ("dad passed away from hodgkin's lymphoma");