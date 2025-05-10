This project is dedicated to the **Onyx Core Builders** construction company website.

# Technological Stack

* **Programming language**: Rust
* **Database**: PostgreSQL
* **Frontend**:
  * HTMX and Templates (using askama) - for dynamic updates
  * Tailwind-CSS - for styling
* **Rust Crates**:
  * tokio - async runtime
  * axum - http server
  * sqlx - database driver
  * anyhow - error handling
  * askama - templating
  * log and env_logger - logging
  * serde - serialization/deserialization
  * chrono - time-related types and functions

# Description

The following is the description of the company structure that should be reflected in the database architecture and the informational system itself.

A construction company is engaged in the construction of various types of sites (power plants, roads, housing, 
bridges, parks etc.) under contracts with clients (city administration, private firms, etc.). Each of the listed categories of objects has
characteristics peculiar only to this or several categories.

Structurally, a construction organization consists of construction departments, each construction
department conducts work on one or more construction areas, headed
by supervisors, whose subordinates are foremen, masters and technicians. 

Each category of technical personnel (engineers, technologists, technicians) 
and workers (electriricans, plumbers, welders, drivers, masons, etc.) also
has attributes specific only to this group. 

The workers are organized into brigades, for which the brigadiers are in charge. 
Brigadiers are selected from among the workers. Foremen, masters, supervisors
of areas and departments are appointed from among the engineering and technical staff.

One or more sites are being built in each construction area, and one or more brigades are working on each site. 
After completing the work, the brigade moves on to another site in this or
another construction area. Construction equipmment (cranes, excavators, bulldozers, etc.) is assigned to construction departments, 
which is distributed among the sites.

Construction of a site requires specific types of construction tasks. For example, for an apartment
building, this includes foundation construction, brick work, water supply installation, etc. Each
task at the site is performed by one brigade. For the organization of work at the site, 
work schedules are drawn up, indicating in what order and at what time
certain tasks are performed, as well as estimates determining which building materials and in what quantities
are needed for the construction of the site. 

Based on the results of the task, a report is prepared with
indicating the time frame for this task and the actual cost of materials.

# Queries

The informational systems should support at least the following queries:

1. Get a list of construction departments and/or areas and their supervisors.
2. Get a list of technical personnel staff of the designated construction area or construction department, indicating their positions.
3. Get a list of sites being built by the specified construction department and/or area, and schedules for their construction.
4. Get the list of workers of the brigades that have worked (are working) on the construction of the specified site.
5. Get a list of construction equipment assigned to the specified construction department.
6. Get a list of construction equipment allocated to the specified site or that worked there during the specified time period.
7. Get a schedule and estimate (used materials) for the construction of the specified site.
8. Get a report on the construction of the specified site.
9. Get a list of sites under construction in a certain construction department or in the organization as a whole, where the specified type of construction task was performed during the specified time period.
10. Get a list of types of construction tasks for which there was an excess deadlines for completion at the specified site, the construction department, or the organization as a whole.
11. Get a list of building materials for which there was an excess in estimates at the specified site, the construction department or the organization as a whole.
12. Get a list of the types of construction tasks performed by the specified brigade during the designated time period, indicating the sites where these works were working.
13. Get a list of the brigades that performed the specified type of construction tasks during the specified period of time, indicating the sites where these works were performed.

# Requirements

The informational system must support all queries listed in `Queries` section, but also
support editing and viewing all the information in the database.

The user interface for viewing and editing data should be ergonomic (for example, a page per query is a bad decision).
The editing and viewing process should be efficient, extensible, intuitive.
The UI should be easy to understand and navigate.

# Database Architecture

The database should have the following tables and table fields:

* site (A single construction site)
  * id (primary key)
  * name (name of the site, text)
  * area_id (references area)
  * client_id (references client)
  * type (one of power_plant, road, housing, bridge, park)
  * location (latitude and longitude)
  * risk_level (one of high, medium, low)
  * description (optional)

* power_plant (subclass of construction site)
  * site_id (references site)
  * energy_output (in MW)
  * energy_source (the name of source used to get energy)
  * is_grid_connected

* road (subclass of construction site)
  * site_id (references site)
  * length (length of the road, in meters)
  * lanes (number of lanes)
  * surface (type of surface, text)

* housing (subclass of construction site)
  * site_id (references site)
  * number_of_floors
  * number_of_entrances
  * type (type of housing, text)
  * energy_efficiency (one of high, low, medium)

* bridge (subclass of construction site)
  * site_id (references site)
  * length (length of the bridge, in meters)
  * road_material (type of road material, text)
  * max_load (max load of the bridge, in metric tons)

* park (subclass of construction site)
  * site_id (references site)
  * area (park area, in square km)
  * has_playground (boolean)
  * has_lighting (boolean)

* client (Client of Onyx Horizons)
  * id (primary key)
  * name (the name of the company)
  * inn (unique number assigned to companies, unique among all of them)
  * address (address of company headquarters)
  * contact_person_email
  * contact_person_name
  * is_vip 

* area (Construction Company Area)
  * id (primary key)
  * department_id (references department)
  * supervisor_id (references technical_personnel)
  * name (name of this area, text)

* department (Construction Company Department)
  * id (primary key)
  * supervisor_id (references technical_personnel)
  * name (name of this department, text)
  
* equipment (Equipment that is allocated to departments and sites)
  * id (primary key)
  * name 
  * amount (integer)
  * purchase_date (date)
  * purchase_cost 
  * fuel_type (optional)

* equipment_allocation (Equipment allocation, if site_id is null, then the row describes allocation for specific department)
  * equipment_id (references equipment)
  * department_id (references department)
  * site_id (references site, optional)
  * amount (amount of equipment allocated, integer)
  * period_start (date)
  * period_end (date)

* employee (Comapny Employee)
  * id (primary key)
  * class (one of worker or technical_personnel)
  * first_name
  * last_name
  * middle_name (optional)
  * gender (one of male, female)
  * photo (url to worker photo, optional)
  * salary (integer)
  * phone_number

* worker (subclass of employee)
  * id (references employee)
  * profession (one of electrician, plumber, welder, driver, mason)
  * union_name (name of union that worker is in, optional)

* electrican (subclass of worker)
  * id (references worker)
  * voltage_specialization (text)

* plumber (subclass of worker)
  * id (references worker)
  * pipe_specialization (text)

* welder (subclass of worker)
  * id (references worker)
  * welding_machine (text)

* driver (subclass of worker)
  * id (references worker)
  * vehicle_type (text)
  * number_of_accidents (integer)

* mason (subclass of worker)
  * id (references worker)
  * hq_restoration_skills (boolean)

* brigade
  * id (primary key)
  * brigadier_id (references worker)

* assignment
  * brigade_id (references brigade)
  * worker_id (references worker)

* technical_personnel (subclass of employee)
  * id (references employee)
  * qualification (one of technician, technologist, engineer)
  * position (one of master, foreman, optional)
  * education_level 
  * software_skills (list of software, optional)
  * is_project_manager (boolean)

* technician (subclass of technical personnel)
  * id (references technical_personnel)
  * safety_training_level 

* technologist (subclass of technical personnel)
  * id (references technical_personnel)
  * management_tools (list of tools used for management)

* engineer (subclass of technical personnel)
  * id (references technical_personnel)
  * pe_license_id (integer)

* material (Construction Material)
  * id (primary key)
  * name
  * cost
  * units (of measurement)

* expenditure (Amount of material spent on a task, actual_amount is set after the task is complete)
  * task_id (references task)
  * material_id (references material)
  * expected_amount (amount that is expected to be spent)
  * actual_amount (amount that was actually spent, optional)

* task (Single task on a construction site, brigade_id is null if no brigade has been assigned to it yet, actual_period_end is set after the task is complete)
  * id (primary key)
  * site_id (references site)
  * brigade_id (references brigade, optional)
  * period_start (date)
  * expected_period_end (date)
  * actual_period_end (date, optional)
  * name
  * description (optional)
