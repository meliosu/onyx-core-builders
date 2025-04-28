-- Fill database with synthetic data

-- Insert clients
INSERT INTO client (name, inn, address, contact_person_email, contact_person_name, is_vip) VALUES
('City Administration', '1234567890', '123 Main St, Cityville', 'mayor@cityville.gov', 'John Mayor', TRUE),
('Eco Power Corp', '2345678901', '456 Green Ave, Powertown', 'ceo@ecopower.com', 'Sarah Green', TRUE),
('Urban Development LLC', '3456789012', '789 Build St, Constructia', 'director@urbdev.com', 'Mike Builder', FALSE),
('Bridge Masters Inc', '4567890123', '101 Bridge Rd, Spanville', 'contact@bridgemasters.com', 'Robert Span', FALSE),
('Parks & Recreation Dept', '5678901234', '202 Park Ave, Greenfield', 'director@parksrec.gov', 'Leslie Knope', TRUE),
('Residential Builders Ltd', '6789012345', '303 House St, Hometon', 'info@resbuild.com', 'David Home', FALSE),
('Green Energy Solutions', '7890123456', '404 Solar Rd, Cleanville', 'contact@greenenergy.com', 'Alice Sun', TRUE),
('Highway Authority', '8901234567', '505 Highway Blvd, Roadcity', 'info@highways.gov', 'Paul Road', FALSE);

-- Insert employees (Technical personnel)
INSERT INTO employee (id, class, first_name, last_name, middle_name, gender, photo, salary, phone_number) VALUES
(1, 'technical_personnel', 'Dmitri', 'Petrov', 'Nikolaevich', 'male', 'photos/dmitri.jpg', 90000, '+7-900-123-4567'),
(2, 'technical_personnel', 'Elena', 'Ivanova', NULL, 'female', 'photos/elena.jpg', 85000, '+7-900-234-5678'),
(3, 'technical_personnel', 'Sergei', 'Smirnov', 'Aleksandrovich', 'male', NULL, 95000, '+7-900-345-6789'),
(4, 'technical_personnel', 'Olga', 'Kuznetsova', 'Ivanovna', 'female', 'photos/olga.jpg', 88000, '+7-900-456-7890'),
(5, 'technical_personnel', 'Mikhail', 'Volkov', 'Petrovich', 'male', 'photos/mikhail.jpg', 92000, '+7-900-567-8901'),
(6, 'technical_personnel', 'Natalia', 'Sokolova', NULL, 'female', NULL, 87000, '+7-900-678-9012'),
(7, 'technical_personnel', 'Vladimir', 'Novikov', 'Sergeevich', 'male', 'photos/vladimir.jpg', 94000, '+7-900-789-0123'),
(8, 'technical_personnel', 'Tatiana', 'Morozova', 'Dmitrievna', 'female', 'photos/tatiana.jpg', 86000, '+7-900-890-1234'),
(9, 'technical_personnel', 'Alexander', 'Lebedev', 'Mikhailovich', 'male', NULL, 91000, '+7-900-901-2345'),
(10, 'technical_personnel', 'Ekaterina', 'Kozlova', 'Vladimirovna', 'female', 'photos/ekaterina.jpg', 89000, '+7-900-012-3456');

-- Insert technical personnel details
INSERT INTO technical_personnel (id, qualification, position, education_level, software_skills, is_project_manager) VALUES
(1, 'engineer', 'foreman', 'Masters Degree', ARRAY['AutoCAD', 'Revit', 'MS Project'], TRUE),
(2, 'technologist', 'master', 'Bachelors Degree', ARRAY['Primavera', 'Excel'], FALSE),
(3, 'engineer', NULL, 'Masters Degree', ARRAY['AutoCAD', 'STAAD Pro'], FALSE),
(4, 'technician', 'master', 'Associates Degree', ARRAY['AutoCAD'], FALSE),
(5, 'engineer', 'foreman', 'PhD', ARRAY['Revit', 'Navisworks', 'SAP'], TRUE),
(6, 'technologist', NULL, 'Bachelors Degree', ARRAY['MS Project', 'Excel', 'PowerBI'], FALSE),
(7, 'engineer', 'foreman', 'Masters Degree', ARRAY['AutoCAD', 'Civil 3D', 'Revit'], TRUE),
(8, 'technician', 'master', 'Associates Degree', ARRAY['SketchUp', 'AutoCAD'], FALSE),
(9, 'engineer', NULL, 'Masters Degree', ARRAY['Revit', 'BIM 360'], FALSE),
(10, 'technologist', 'master', 'Bachelors Degree', ARRAY['MS Project', 'Primavera', 'JIRA'], TRUE);

-- Insert specialized technical personnel
INSERT INTO technician (id, safety_training_level) VALUES
(4, 'Advanced'),
(8, 'Intermediate');

INSERT INTO technologist (id, management_tools) VALUES
(2, ARRAY['Agile', 'Lean', 'Six Sigma']),
(6, ARRAY['Kanban', 'Scrum', 'TQM']),
(10, ARRAY['Lean', 'Prince2', 'PMBOK']);

INSERT INTO engineer (id, pe_license_id) VALUES
(1, 12345),
(3, 23456),
(5, 34567),
(7, 45678),
(9, 56789);

-- Insert employees (Workers)
INSERT INTO employee (id, class, first_name, last_name, middle_name, gender, photo, salary, phone_number) VALUES
(11, 'worker', 'Ivan', 'Kuznetsov', 'Petrovich', 'male', 'photos/ivan.jpg', 50000, '+7-900-456-7890'),
(12, 'worker', 'Anna', 'Sokolova', NULL, 'female', 'photos/anna.jpg', 48000, '+7-900-567-8901'),
(13, 'worker', 'Maxim', 'Volkov', 'Sergeevich', 'male', NULL, 52000, '+7-900-678-9012'),
(14, 'worker', 'Maria', 'Vasilyeva', 'Alexandrovna', 'female', 'photos/maria.jpg', 49000, '+7-900-789-0123'),
(15, 'worker', 'Andrey', 'Fedorov', 'Ivanovich', 'male', 'photos/andrey.jpg', 51000, '+7-900-890-1234'),
(16, 'worker', 'Sofia', 'Bogdanova', NULL, 'female', NULL, 48500, '+7-900-901-2345'),
(17, 'worker', 'Nikolai', 'Semyonov', 'Dmitrievich', 'male', 'photos/nikolai.jpg', 53000, '+7-900-012-3456'),
(18, 'worker', 'Anastasia', 'Petrova', 'Mikhailovna', 'female', 'photos/anastasia.jpg', 49500, '+7-900-123-4567'),
(19, 'worker', 'Pavel', 'Mikhailov', 'Sergeevich', 'male', NULL, 52500, '+7-900-234-5678'),
(20, 'worker', 'Yulia', 'Orlova', 'Andreevna', 'female', 'photos/yulia.jpg', 48000, '+7-900-345-6789'),
(21, 'worker', 'Igor', 'Popov', 'Nikolaevich', 'male', 'photos/igor.jpg', 51500, '+7-900-456-7890'),
(22, 'worker', 'Alina', 'Nikitina', NULL, 'female', NULL, 49000, '+7-900-567-8901'),
(23, 'worker', 'Roman', 'Zaitsev', 'Vladimirovich', 'male', 'photos/roman.jpg', 54000, '+7-900-678-9012'),
(24, 'worker', 'Ksenia', 'Volkova', 'Petrovna', 'female', 'photos/ksenia.jpg', 50000, '+7-900-789-0123'),
(25, 'worker', 'Denis', 'Stepanov', 'Alexandrovich', 'male', NULL, 53500, '+7-900-890-1234');

-- Insert worker details
INSERT INTO worker (id, profession, union_name) VALUES
(11, 'electrician', 'Electrical Workers Union'),
(12, 'plumber', 'Plumbers Union'),
(13, 'welder', 'Welders Association'),
(14, 'driver', NULL),
(15, 'mason', 'Masons Guild'),
(16, 'electrician', 'Electrical Workers Union'),
(17, 'plumber', NULL),
(18, 'welder', 'Welders Association'),
(19, 'driver', 'Drivers Union'),
(20, 'mason', 'Masons Guild'),
(21, 'electrician', NULL),
(22, 'plumber', 'Plumbers Union'),
(23, 'welder', 'Welders Association'),
(24, 'driver', 'Drivers Union'),
(25, 'mason', NULL);

-- Insert specialized worker details
INSERT INTO electrirican (id, voltage_specializaition) VALUES
(11, 'High Voltage'),
(16, 'Low Voltage'),
(21, 'Industrial');

INSERT INTO plumber (id, pipe_specialization) VALUES
(12, 'PVC'),
(17, 'Metal'),
(22, 'Copper');

INSERT INTO welder (id, welding_machine) VALUES
(13, 'MIG'),
(18, 'TIG'),
(23, 'Arc');

INSERT INTO driver (id, vehicle_type, number_of_accidents) VALUES
(14, 'Dump Truck', 0),
(19, 'Crane', 1),
(24, 'Concrete Mixer', 0);

INSERT INTO mason (id, hq_restoration_skills) VALUES
(15, TRUE),
(20, FALSE),
(25, TRUE);

-- Insert departments
INSERT INTO department (id, supervisor_id, name) VALUES
(1, 1, 'Residential Construction'),
(2, 5, 'Infrastructure Development'),
(3, 7, 'Energy Systems'),
(4, 10, 'Urban Planning');

-- Insert areas
INSERT INTO area (id, department_id, supervisor_id, name) VALUES
(1, 1, 2, 'North District Housing'),
(2, 1, 3, 'South District Housing'),
(3, 2, 4, 'Highway Development'),
(4, 2, 6, 'Bridge Construction'),
(5, 3, 8, 'Power Plant Development'),
(6, 3, 9, 'Renewable Energy'),
(7, 4, 3, 'Urban Parks'),
(8, 4, 6, 'City Infrastructure');

-- Insert brigades
INSERT INTO brigade (id, brigadier_id) VALUES
(1, 11),
(2, 13),
(3, 15),
(4, 18),
(5, 21);

-- Insert worker assignments to brigades
INSERT INTO assignment (brigade_id, worker_id) VALUES
(1, 11), -- brigadier
(1, 12),
(1, 14),
(1, 16),
(2, 13), -- brigadier
(2, 17),
(2, 19),
(2, 22),
(3, 15), -- brigadier
(3, 20),
(3, 23),
(3, 25),
(4, 18), -- brigadier
(4, 11),
(4, 24),
(5, 21), -- brigadier
(5, 12),
(5, 17),
(5, 25);

-- Insert sites
INSERT INTO site (id, name, area_id, client_id, type, location, risk_level, description) VALUES
(1, 'Green Valley Residences', 1, 6, 'housing', POINT(55.7558, 37.6173), 'low', 'Modern residential complex with 4 buildings'),
(2, 'Sunset Heights Apartments', 2, 3, 'housing', POINT(55.7517, 37.6256), 'medium', 'Luxury apartment complex with amenities'),
(3, 'City Bypass Highway', 3, 7, 'road', POINT(55.7439, 37.6317), 'medium', '15km bypass around the northern part of the city'),
(4, 'River Cross Bridge', 4, 4, 'bridge', POINT(55.7522, 37.6156), 'high', 'Cable-stayed bridge across the main river'),
(5, 'Solar Farm Alpha', 6, 2, 'power_plant', POINT(55.7553, 37.6215), 'medium', '50 MW solar power plant'),
(6, 'Natural Gas Power Station', 5, 2, 'power_plant', POINT(55.7485, 37.6236), 'high', '200 MW gas-fired power station'),
(7, 'Central City Park', 7, 5, 'park', POINT(55.7539, 37.6208), 'low', 'Urban park with recreational facilities'),
(8, 'Highway 95 Extension', 3, 8, 'road', POINT(55.7578, 37.6196), 'medium', 'Extension of Highway 95 with 4 lanes'),
(9, 'Riverside Apartments', 1, 6, 'housing', POINT(55.7456, 37.6184), 'low', 'Apartment complex near the river'),
(10, 'Pedestrian Bridge', 4, 1, 'bridge', POINT(55.7612, 37.6236), 'medium', 'Pedestrian bridge connecting two districts');

-- Insert specific site details
INSERT INTO housing (site_id, number_of_floors, number_of_entrances, type, energy_efficiency) VALUES
(1, 12, 4, 'Apartment Complex', 'high'),
(2, 18, 2, 'Luxury Condominiums', 'high'),
(9, 8, 3, 'Standard Apartments', 'medium');

INSERT INTO road (site_id, length, lanes, surface) VALUES
(3, 15000.0, 6, 'Asphalt'),
(8, 7500.0, 4, 'Concrete');

INSERT INTO bridge (site_id, length, road_material, max_load) VALUES
(4, 520.0, 'Asphalt', 120.0),
(10, 150.0, 'Composite', 10.0);

INSERT INTO power_plant (site_id, energy_output, energy_source, is_grid_connected) VALUES
(5, 50.0, 'Solar', TRUE),
(6, 200.0, 'Natural Gas', TRUE);

INSERT INTO park (site_id, area, has_playground, has_lighting) VALUES
(7, 2.5, TRUE, TRUE);

-- Insert equipment
INSERT INTO equipment (id, name, amount, purchase_date, purchase_cost, fuel_type) VALUES
(1, 'Tower Crane', 5, '2020-03-15', 1200000.00, NULL),
(2, 'Excavator', 8, '2019-07-22', 450000.00, 'diesel'),
(3, 'Bulldozer', 6, '2021-01-10', 380000.00, 'diesel'),
(4, 'Concrete Mixer', 12, '2020-11-05', 95000.00, 'electric'),
(5, 'Dump Truck', 15, '2019-05-18', 250000.00, 'diesel'),
(6, 'Forklift', 10, '2021-08-30', 120000.00, 'diesel'),
(7, 'Backhoe Loader', 7, '2020-02-12', 280000.00, 'diesel'),
(8, 'Scaffolding Set', 25, '2019-12-03', 45000.00, NULL),
(9, 'Generator', 20, '2021-04-25', 35000.00, 'gasoline'),
(10, 'Welding Machine', 15, '2020-09-17', 18000.00, 'electric');

-- Insert equipment allocations
INSERT INTO equipment_allocation (equipment_id, department_id, site_id, amount, period_start, period_end) VALUES
(1, 1, 1, 2, '2022-01-10', '2022-12-20'),
(1, 1, 2, 1, '2022-02-15', '2022-11-30'),
(2, 2, 3, 3, '2022-03-01', '2022-09-15'),
(2, 2, 4, 2, '2022-01-20', '2022-08-10'),
(3, 2, 3, 2, '2022-03-10', '2022-07-25'),
(4, 1, 1, 4, '2022-01-15', '2022-10-30'),
(4, 1, 2, 3, '2022-02-20', '2022-09-20'),
(5, 2, 3, 5, '2022-03-05', '2022-08-15'),
(6, 3, 5, 3, '2022-04-10', '2022-12-15'),
(6, 3, 6, 4, '2022-05-15', '2022-11-20'),
(7, 2, 4, 2, '2022-01-25', '2022-07-30'),
(8, 1, 1, 8, '2022-01-12', '2022-12-10'),
(8, 1, 2, 6, '2022-02-18', '2022-11-25'),
(9, 3, 5, 6, '2022-04-12', '2022-10-25'),
(9, 3, 6, 8, '2022-05-18', '2022-12-05'),
(10, 1, 1, 4, '2022-01-18', '2022-10-15'),
(10, 3, 6, 5, '2022-05-20', '2022-11-15');

-- Department-only allocations (no site specified)
-- INSERT INTO equipment_allocation (equipment_id, department_id, amount, period_start, period_end) VALUES
-- (1, 1, 2, '2022-01-01', '2022-12-31'),
-- (2, 2, 3, '2022-01-01', '2022-12-31'),
-- (3, 2, 4, '2022-01-01', '2022-12-31'),
-- (4, 1, 5, '2022-01-01', '2022-12-31'),
-- (5, 2, 5, '2022-01-01', '2022-12-31'),
-- (6, 3, 3, '2022-01-01', '2022-12-31'),
-- (7, 2, 5, '2022-01-01', '2022-12-31'),
-- (8, 1, 11, '2022-01-01', '2022-12-31'),
-- (9, 3, 6, '2022-01-01', '2022-12-31'),
-- (10, 4, 6, '2022-01-01', '2022-12-31');

-- Insert materials
INSERT INTO material (id, name, cost, units) VALUES
(1, 'Concrete', 120.00, 'cubic meter'),
(2, 'Steel Rebar', 950.00, 'ton'),
(3, 'Bricks', 0.75, 'piece'),
(4, 'Sand', 40.00, 'cubic meter'),
(5, 'Gravel', 45.00, 'cubic meter'),
(6, 'Wood Lumber', 600.00, 'cubic meter'),
(7, 'Glass', 85.00, 'square meter'),
(8, 'PVC Pipes', 12.50, 'meter'),
(9, 'Copper Wire', 8.75, 'meter'),
(10, 'Asphalt', 110.00, 'ton'),
(11, 'Paint', 15.00, 'liter'),
(12, 'Insulation', 18.50, 'square meter'),
(13, 'Tiles', 25.00, 'square meter'),
(14, 'Cement', 7.50, 'kilogram'),
(15, 'Solar Panels', 250.00, 'piece');

-- Insert tasks
INSERT INTO task (id, site_id, brigade_id, period_start, expected_period_end, actual_period_end, name, description) VALUES
-- Housing site tasks
(1, 1, 1, '2022-01-15', '2022-03-30', '2022-04-05', 'Foundation Construction', 'Excavation and concrete foundation work'),
(2, 1, 2, '2022-04-10', '2022-06-30', '2022-06-25', 'Structural Framework', 'Steel framework and concrete pouring'),
(3, 1, 3, '2022-07-05', '2022-09-15', NULL, 'Interior Work', 'Plumbing, electrical, and interior finishing'),
(4, 2, 4, '2022-02-20', '2022-05-10', '2022-05-20', 'Foundation Construction', 'Excavation and concrete foundation work'),
(5, 2, 5, '2022-06-01', '2022-08-20', NULL, 'Structural Framework', 'Steel framework and concrete pouring'),
-- Road site tasks
(6, 3, 2, '2022-03-10', '2022-05-25', '2022-06-10', 'Ground Preparation', 'Clearing and grading work'),
(7, 3, 3, '2022-06-15', '2022-08-30', NULL, 'Asphalt Laying', 'Asphalt paving for highway'),
-- Bridge site tasks
(8, 4, 1, '2022-01-25', '2022-04-15', '2022-04-20', 'Foundation Work', 'Pier and abutment foundation construction'),
(9, 4, 4, '2022-05-01', '2022-07-30', NULL, 'Steel Structure', 'Steel beam and cable installation'),
-- Power plant site tasks
(10, 5, 5, '2022-04-15', '2022-07-10', '2022-07-15', 'Solar Panel Installation', 'Installation of solar panel arrays'),
(11, 6, 3, '2022-05-20', '2022-08-15', '2022-09-01', 'Generator Installation', 'Installation of gas turbine generators'),
-- Park site tasks
(12, 7, 1, '2022-01-15', '2022-03-15', '2022-03-10', 'Land Preparation', 'Clearing and landscaping'),
(13, 7, 5, '2022-03-20', '2022-05-30', '2022-06-05', 'Path Construction', 'Construction of walking paths and installations'),
-- More tasks
(14, 8, 2, '2022-03-15', '2022-06-10', NULL, 'Road Bed Preparation', 'Ground preparation and drainage installation'),
(15, 9, 4, '2022-02-20', '2022-05-15', '2022-05-30', 'Foundation Construction', 'Excavation and foundation work'),
(16, 9, 1, '2022-06-05', '2022-09-20', NULL, 'Building Construction', 'Main structure construction'),
(17, 10, 5, '2022-01-25', '2022-04-10', '2022-04-25', 'Foundation Work', 'Foundation construction for bridge supports'),
(18, 10, 3, '2022-05-01', '2022-07-20', '2022-08-05', 'Bridge Structure', 'Main structure and surface construction');

-- Insert material expenditures
INSERT INTO expenditure (task_id, material_id, expected_amount, actuial_amount) VALUES
-- Foundation Construction (Housing)
(1, 1, 250.0, 275.5), -- Concrete
(1, 2, 15.0, 16.2),   -- Steel Rebar
(1, 4, 80.0, 85.0),   -- Sand
(1, 5, 120.0, 118.5), -- Gravel

-- Structural Framework (Housing)
(2, 1, 320.0, 315.5), -- Concrete
(2, 2, 45.0, 47.8),   -- Steel Rebar
(2, 6, 25.0, 26.2),   -- Wood Lumber

-- Interior Work (Housing)
(3, 8, 450.0, NULL),  -- PVC Pipes
(3, 9, 1200.0, NULL), -- Copper Wire
(3, 11, 350.0, NULL), -- Paint
(3, 12, 800.0, NULL), -- Insulation
(3, 13, 650.0, NULL), -- Tiles

-- Foundation Construction (Housing)
(4, 1, 380.0, 390.5), -- Concrete
(4, 2, 22.0, 21.8),   -- Steel Rebar
(4, 4, 95.0, 98.5),   -- Sand
(4, 5, 140.0, 145.2), -- Gravel

-- Structural Framework (Housing)
(5, 1, 420.0, NULL),  -- Concrete
(5, 2, 60.0, NULL),   -- Steel Rebar
(5, 6, 35.0, NULL),   -- Wood Lumber

-- Ground Preparation (Road)
(6, 4, 850.0, 880.5), -- Sand
(6, 5, 920.0, 950.0), -- Gravel

-- Asphalt Laying (Road)
(7, 10, 950.0, NULL), -- Asphalt

-- Foundation Work (Bridge)
(8, 1, 580.0, 595.5), -- Concrete
(8, 2, 85.0, 88.2),   -- Steel Rebar
(8, 4, 210.0, 205.5), -- Sand
(8, 5, 250.0, 260.0), -- Gravel

-- Steel Structure (Bridge)
(9, 2, 120.0, NULL),  -- Steel Rebar

-- Solar Panel Installation
(10, 15, 200.0, 198.0), -- Solar Panels
(10, 2, 12.0, 12.5),    -- Steel Rebar

-- Generator Installation
(11, 2, 35.0, 38.5),    -- Steel Rebar
(11, 1, 85.0, 90.0),    -- Concrete

-- Land Preparation (Park)
(12, 4, 150.0, 145.5),  -- Sand
(12, 5, 120.0, 118.0),  -- Gravel

-- Path Construction (Park)
(13, 1, 95.0, 100.5),   -- Concrete
(13, 10, 25.0, 27.5),   -- Asphalt

-- Road Bed Preparation
(14, 4, 780.0, NULL),   -- Sand
(14, 5, 850.0, NULL),   -- Gravel
(14, 1, 120.0, NULL),   -- Concrete

-- Foundation Construction (Housing)
(15, 1, 280.0, 290.5),  -- Concrete
(15, 2, 18.0, 17.5),    -- Steel Rebar
(15, 4, 85.0, 88.0),    -- Sand

-- Building Construction (Housing)
(16, 1, 350.0, NULL),   -- Concrete
(16, 2, 50.0, NULL),    -- Steel Rebar
(16, 3, 25000.0, NULL), -- Bricks
(16, 9, 950.0, NULL),   -- Copper Wire

-- Foundation Work (Bridge)
(17, 1, 120.0, 125.5),  -- Concrete
(17, 2, 15.0, 16.2),    -- Steel Rebar
(17, 4, 45.0, 43.8),    -- Sand

-- Bridge Structure
(18, 2, 25.0, 26.5),    -- Steel Rebar
(18, 6, 15.0, 16.0),    -- Wood Lumber
(18, 1, 65.0, 68.0);    -- Concrete