-- Enum Types

CREATE TYPE site_type AS ENUM ('power_plant', 'road', 'housing', 'bridge', 'park');
CREATE TYPE risk_level AS ENUM ('high', 'medium', 'low');
CREATE TYPE gender AS ENUM ('male', 'female');
CREATE TYPE worker_profession AS ENUM ('electrician', 'plumber', 'welder', 'driver', 'mason');
CREATE TYPE personnel_qualification AS ENUM ('technician', 'technologist', 'engineer');
CREATE TYPE personnel_position AS ENUM ('master', 'foreman');
CREATE TYPE employee_class AS ENUM ('worker', 'technical_personnel');

-- Main Tables

CREATE TABLE client (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    inn TEXT NOT NULL UNIQUE,
    address TEXT NOT NULL,
    contact_person_email TEXT NOT NULL,
    contact_person_name TEXT NOT NULL,
    is_vip BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE employee (
    id SERIAL PRIMARY KEY,
    class employee_class NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    middle_name TEXT,
    gender gender NOT NULL,
    photo TEXT,
    salary INTEGER NOT NULL,
    phone_number TEXT NOT NULL
);

CREATE TABLE worker (
    id INTEGER PRIMARY KEY REFERENCES employee(id),
    profession worker_profession NOT NULL,
    union_name TEXT
);

CREATE TABLE technical_personnel (
    id INTEGER PRIMARY KEY REFERENCES employee(id),
    qualification personnel_qualification NOT NULL,
    position personnel_position,
    education_level TEXT NOT NULL,
    software_skills TEXT[],
    is_project_manager BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE department (
    id SERIAL PRIMARY KEY,
    supervisor_id INTEGER REFERENCES technical_personnel(id),  
    name TEXT NOT NULL
);

CREATE TABLE area (
    id SERIAL PRIMARY KEY,
    department_id INTEGER NOT NULL REFERENCES department(id),
    supervisor_id INTEGER NOT NULL REFERENCES technical_personnel(id),
    name TEXT NOT NULL
);

CREATE TABLE site (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    area_id INTEGER NOT NULL REFERENCES area(id),
    client_id INTEGER NOT NULL REFERENCES client(id),
    type site_type NOT NULL,
    location POINT NOT NULL,
    risk_level risk_level NOT NULL,
    description TEXT
);

CREATE TABLE brigade (
    id SERIAL PRIMARY KEY,
    brigadier_id INTEGER NOT NULL REFERENCES worker(id)
);

CREATE TABLE equipment (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    amount INTEGER NOT NULL CHECK (amount >= 0),
    purchase_date DATE NOT NULL,
    purchase_cost NUMERIC(10, 2) NOT NULL,
    fuel_type TEXT
);

CREATE TABLE material (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    cost NUMERIC(10, 2) NOT NULL,
    units TEXT NOT NULL
);

-- Subclass Tables

CREATE TABLE power_plant (
    site_id INTEGER PRIMARY KEY REFERENCES site(id),
    energy_output NUMERIC(10, 2) NOT NULL, 
    energy_source TEXT NOT NULL,
    is_grid_connected BOOLEAN NOT NULL
);

CREATE TABLE road (
    site_id INTEGER PRIMARY KEY REFERENCES site(id),
    length NUMERIC(10, 2) NOT NULL, 
    lanes INTEGER NOT NULL,
    surface TEXT NOT NULL
);

CREATE TABLE housing (
    site_id INTEGER PRIMARY KEY REFERENCES site(id),
    number_of_floors INTEGER NOT NULL,
    number_of_entrances INTEGER NOT NULL,
    type TEXT NOT NULL,
    energy_efficiency TEXT NOT NULL
);

CREATE TABLE bridge (
    site_id INTEGER PRIMARY KEY REFERENCES site(id),
    length NUMERIC(10, 2) NOT NULL, 
    road_material TEXT NOT NULL,
    max_load NUMERIC(10, 2) NOT NULL 
);

CREATE TABLE park (
    site_id INTEGER PRIMARY KEY REFERENCES site(id),
    area NUMERIC(10, 2) NOT NULL, 
    has_playground BOOLEAN NOT NULL,
    has_lighting BOOLEAN NOT NULL
);

CREATE TABLE electrirican (
    id INTEGER PRIMARY KEY REFERENCES worker(id),
    voltage_specializaition TEXT NOT NULL
);

CREATE TABLE plumber (
    id INTEGER PRIMARY KEY REFERENCES worker(id),
    pipe_specialization TEXT NOT NULL
);

CREATE TABLE welder (
    id INTEGER PRIMARY KEY REFERENCES worker(id),
    welding_machine TEXT NOT NULL
);

CREATE TABLE driver (
    id INTEGER PRIMARY KEY REFERENCES worker(id),
    vehicle_type TEXT NOT NULL,
    number_of_accidents INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE mason (
    id INTEGER PRIMARY KEY REFERENCES worker(id),
    hq_restoration_skills BOOLEAN NOT NULL
);

CREATE TABLE technician (
    id INTEGER PRIMARY KEY REFERENCES technical_personnel(id),
    safety_training_level TEXT NOT NULL
);

CREATE TABLE technologist (
    id INTEGER PRIMARY KEY REFERENCES technical_personnel(id),
    management_tools TEXT[] NOT NULL
);

CREATE TABLE engineer (
    id INTEGER PRIMARY KEY REFERENCES technical_personnel(id),
    pe_license_id INTEGER NOT NULL
);

-- Association Tables

CREATE TABLE task (
    id SERIAL PRIMARY KEY,
    site_id INTEGER NOT NULL REFERENCES site(id),
    brigade_id INTEGER REFERENCES brigade(id),
    period_start DATE NOT NULL,
    expected_period_end DATE NOT NULL,
    actual_period_end DATE,
    name TEXT NOT NULL,
    description TEXT
);

CREATE TABLE assignment (
    brigade_id INTEGER NOT NULL REFERENCES brigade(id),
    worker_id INTEGER NOT NULL REFERENCES worker(id),
    PRIMARY KEY (brigade_id, worker_id)
);

CREATE TABLE equipment_allocation (
    equipment_id INTEGER NOT NULL REFERENCES equipment(id),
    department_id INTEGER NOT NULL REFERENCES department(id),
    site_id INTEGER REFERENCES site(id),
    amount INTEGER NOT NULL CHECK (amount > 0),
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    PRIMARY KEY (equipment_id, department_id, site_id, period_start)
);

CREATE TABLE expenditure (
    task_id INTEGER NOT NULL REFERENCES task(id),
    material_id INTEGER NOT NULL REFERENCES material(id),
    expected_amount NUMERIC(10, 2) NOT NULL,
    actuial_amount NUMERIC(10, 2),
    PRIMARY KEY (task_id, material_id)
);
