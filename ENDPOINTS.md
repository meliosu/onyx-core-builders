# Onyx Core Builders Information System API Specification

# Format

Each endpoint is specified in the following format:

### METHOD /path
- **Description**: description of the endpoint
- **Returns**: does the endpoint return the page, or just html fragment (using HTMX)
- **Query**: query params in json format, optional
- **Form**: form data in json format, optional
- **Template**: template params for the returned HTML template

Each endpoint is either a Full Page Endpoint, or an HTMX endpoint (starts with /api).
HTMX endpoints are listed after Full Page Endpoints.

## Helper Endpoints

### HTMX Endpoints

### GET /api/selectors/departments
- **Description**: Get filtered list of departments
- **Returns**: HTML Fragment - list of departments
- **Form**:
```json
{
    "name": "Filter departments by name"
}
```
- **Template**:
```json
{
    "departments": [
        {
            "id": "Dept. ID",
            "name": "Dept. Name"
        }
    ]
}
```

### GET /api/selectors/areas
- **Description**: Get filtered list of areas
- **Returns**: HTML Fragment - list of areas
- **Query**:
```json
{
    "department_id": "Dept. ID"
}
```
- **Form**:
```json
{
    "name": "Filter areas by name"
}
```
- **Template**:
```json
{
    "areas": [
        {
            "id": "Area ID",
            "name": "Area Name"
        }
    ]
}
```

### GET /api/selectors/clients
- **Description**: Get filtered list of clients
- **Returns**: HTML Fragment - list of clients
- **Form**:
```json
{
    "name": "Filter clients by name"
}
```
- **Template**:
```json
{
    "clients": [
        {
            "id": "Client ID",
            "name": "Client Name"
        }
    ]
}
```

### GET /api/selectors/technical-personnel
- **Description**: Get filtered list of technical personnel
- **Returns**: HTML Fragment - list of technical personnel
- **Query**:
```json
{
    "qualification": "Filter by qualification (technician|technologist|engineer)",
    "position": "Filter by position (master|foreman)",
    "department_id": "Filter by department",
    "area_id": "Filter by area"
}
```
- **Form**:
```json
{
    "name": "Filter technical personnel by name"
}
```
- **Template**:
```json
{
    "personnel": [
        {
            "id": "Personnel ID",
            "name": "Full Name",
            "qualification": "Qualification"
        }
    ]
}
```

### GET /api/selectors/workers
- **Description**: Get filtered list of workers
- **Returns**: HTML Fragment - list of workers
- **Query**:
```json
{
    "profession": "Filter by profession (electrician|plumber|welder|driver|mason)",
    "brigade_id": "Filter by brigade",
    "is_brigadier": "Filter brigadiers only (true|false)"
}
```
- **Form**:
```json
{
    "name": "Filter workers by name"
}
```
- **Template**:
```json
{
    "workers": [
        {
            "id": "Worker ID",
            "name": "Full Name",
            "profession": "Profession"
        }
    ]
}
```

### GET /api/selectors/brigades
- **Description**: Get filtered list of brigades
- **Returns**: HTML Fragment - list of brigades
- **Query**:
```json
{
    "site_id": "Filter by site assignment",
    "available": "Filter available brigades only (true|false)"
}
```
- **Form**:
```json
{
    "brigadier_name": "Filter by brigadier name"
}
```
- **Template**:
```json
{
    "brigades": [
        {
            "id": "Brigade ID",
            "brigadier_name": "Brigadier Name",
            "worker_count": "Number of Workers"
        }
    ]
}
```

### GET /api/selectors/sites
- **Description**: Get filtered list of sites
- **Returns**: HTML Fragment - list of sites
- **Query**:
```json
{
    "area_id": "Filter by area",
    "department_id": "Filter by department",
    "client_id": "Filter by client",
    "type": "Filter by site type"
}
```
- **Form**:
```json
{
    "name": "Filter sites by name"
}
```
- **Template**:
```json
{
    "sites": [
        {
            "id": "Site ID",
            "name": "Site Name",
            "type": "Site Type"
        }
    ]
}
```

### GET /api/selectors/equipment
- **Description**: Get filtered list of equipment
- **Returns**: HTML Fragment - list of equipment
- **Query**:
```json
{
    "available": "Filter by availability (true|false)"
}
```
- **Form**:
```json
{
    "name": "Filter equipment by name"
}
```
- **Template**:
```json
{
    "equipment": [
        {
            "id": "Equipment ID",
            "name": "Equipment Name",
            "available_amount": "Available Amount"
        }
    ]
}
```

### GET /api/selectors/materials
- **Description**: Get filtered list of materials
- **Returns**: HTML Fragment - list of materials
- **Form**:
```json
{
    "name": "Filter materials by name"
}
```
- **Template**:
```json
{
    "materials": [
        {
            "id": "Material ID",
            "name": "Material Name",
            "units": "Units"
        }
    ]
}
```

### GET /api/selectors/tasks
- **Description**: Get filtered list of tasks
- **Returns**: HTML Fragment - list of tasks
- **Query**:
```json
{
    "site_id": "Filter by site",
    "brigade_id": "Filter by brigade",
    "status": "Filter by status (Planned|In Progress|Completed)"
}
```
- **Form**:
```json
{
    "name": "Filter tasks by name"
}
```
- **Template**:
```json
{
    "tasks": [
        {
            "id": "Task ID",
            "name": "Task Name",
            "site_name": "Site Name"
        }
    ]
}
```

... Other selectors

## 1. General Endpoints

### Page Endpoints

#### GET /
- **Description**: Return empty page (for now)
- **Returns**: Full page
- **Template**: null

#### GET /404
- **Description**: Return 404 Page
- **Returns**: Full page
- **Template**: null

#### GET /500
- **Description**: Return 500 Page
- **Returns**: Full page
- **Template**:
```json
{
    "message": "Error message"
}
```

## 3. Department Endpoints

### Page Endpoints

#### GET /departments
- **Description**: List all departments, retrieve the actual list using GET /api/departments
- **Returns**: Full page
- **Template**: null

#### GET /departments/{id}
- **Description**: Get department details, retrieve the actual details using GET /api/departments/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Dept. ID"
}
```

#### GET /departments/new
- **Description**: Get department creation page, the department is created using POST /api/departments
- **Returns**: Full page
- **Template**: null

#### GET /departments/{id}/edit
- **Description**: Get department edit page with forms prefilled with the existing department data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Dept. ID",
    "name": "Dept. Name",
    "supervisor_id": "Supervisor ID",
    "supervisor_name": "Supervisor Name"
}
```

### HTMX Endpoints

#### GET /api/departments/{id}
- **Description**: Get department details
- **Returns**: HTML Fragment - department details
- **Query**:
```json
{
    "tab": "Tab (areas|equipment|sites|personnel)"
}
```
- **Template**:
```json
{
    "id": "Dept. ID",
    "name": "Dept. Name",
    "supervisor_id": "Supervisor ID",
    "supervisor_name": "Supervisor Name",
    "tab": "Tab (areas|equipment|sites|personnel)"
}
```

#### PUT /api/departments/{id}
- **Description**: Change an existing department
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "name": "Dept. Name",
    "supervisor_id": "Supervisor ID"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/departments/{id}
- **Description**: Delete an existing department
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/departments
- **Description**: List all departments
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page",
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "supervisor_id": "Filter by supervisor ID",
    "name": "Search by department name"
}
```
- **Template**:
```json
{
    "departments": [
        {
            "id": "Dept. ID",
            "name": "Dept. Name",
            "supervisor_id": "Supervisor ID",
            "supervisor_name": "Supervisor Name"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/departments
- **Description**: Create new department
- **Returns**: HTML Fragment - Succes or Error message
- **Form Data**:
```json
{
    "name": "Dept. Name",
    "supervisor_id": "Supervisor ID"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/departments/{id}/areas
- **Description**: get areas for this specific department
- **Returns**: HTML Fragment - list of areas
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "areas": [
        {
            "id": "Area ID",
            "name": "Area Name",
            "supervisor_id": "Supervisor ID",
            "supervisor_name": "Supervisor Name"
        }
    ]
}
```

#### GET /api/departments/{id}/equipment
- **Description**: get equipment for this specific department
- **Returns**: HTML Fragment - list of equipment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
    "equipment": [
        {
            "id": "Equipment ID",
            "name": "Equipment Name",
            "amount": "Amount of Equipment"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
```

#### GET /api/departments/{id}/sites
- **Description**: get sites for this specific department
- **Returns**: HTML Fragment - list of sites
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
    "sites": [
        {
            "id": "Site ID",
            "name": "Site Name",
            "type": "Site Type"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
```

#### GET /api/departments/{id}/personnel
- **Description**: get personnel for this specific department
- **Returns**: HTML Fragment - list of employees
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**: 
```json
    "personnel": [
        {
            "id": "Employee ID",
            "name": "Employee Name",
            "qualification": "Qualification"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
```

## 4. Area Endpoints

### Page Endpoints

#### GET /areas
- **Description**: List all areas, retrieve the actual list using GET /api/areas
- **Returns**: Full page
- **Template**: null

#### GET /areas/{id}
- **Description**: Get area details, retrieve the actual details using GET /api/areas/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Area ID"
}
```

#### GET /areas/new
- **Description**: Get area creation page, the area is created using POST /api/areas
- **Returns**: Full page
- **Template**: null

#### GET /areas/{id}/edit
- **Description**: Get area edit page with forms prefilled with the existing area data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Area ID",
    "name": "Area Name",
    "department_id": "Department ID",
    "department_name": "Department Name",
    "supervisor_id": "Supervisor ID",
    "supervisor_name": "Supervisor Name"
}
```

### HTMX Endpoints

#### GET /api/areas/{id}
- **Description**: Get area details
- **Returns**: HTML Fragment - area details
- **Query**:
```json
{
    "tab": "Tab (sites|personnel)"
}
```
- **Template**:
```json
{
    "id": "Area ID",
    "name": "Area Name",
    "department_id": "Department ID",
    "department_name": "Department Name",
    "supervisor_id": "Supervisor ID",
    "supervisor_name": "Supervisor Name",
    "tab": "Tab (sites|personnel)"
}
```

#### PUT /api/areas/{id}
- **Description**: Change an existing area
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "name": "Area Name",
    "department_id": "Department ID",
    "supervisor_id": "Supervisor ID"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/areas/{id}
- **Description**: Delete an existing area
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/areas
- **Description**: List all areas
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "department_id": "Filter by department ID",
    "supervisor_id": "Filter by supervisor ID",
    "name": "Search by area name"
}
```
- **Template**:
```json
{
    "areas": [
        {
            "id": "Area ID",
            "name": "Area Name",
            "department_id": "Department ID",
            "department_name": "Department Name",
            "supervisor_id": "Supervisor ID",
            "supervisor_name": "Supervisor Name"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/areas
- **Description**: Create new area
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "name": "Area Name",
    "department_id": "Department ID",
    "supervisor_id": "Supervisor ID"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/areas/{id}/sites
- **Description**: Get sites for this specific area
- **Returns**: HTML Fragment - list of sites
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "sites": [
        {
            "id": "Site ID",
            "name": "Site Name",
            "type": "Site Type",
            "client_id": "Client ID",
            "client_name": "Client Name"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

#### GET /api/areas/{id}/personnel
- **Description**: Get personnel for this specific area
- **Returns**: HTML Fragment - list of technical personnel
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "personnel": [
        {
            "id": "Employee ID",
            "name": "Employee Name",
            "qualification": "Qualification",
            "position": "Position"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

## 5. Site Endpoints

### Page Endpoints

#### GET /sites
- **Description**: List all sites, retrieve the actual list using GET /api/sites
- **Returns**: Full page
- **Template**: null

#### GET /sites/{id}
- **Description**: Get site details, retrieve the actual details using GET /api/sites/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Site ID"
}
```

#### GET /sites/new
- **Description**: Get site creation page, the site is created using POST /api/sites
- **Returns**: Full page
- **Template**: null

#### GET /sites/{id}/edit
- **Description**: Get site edit page with forms prefilled with the existing site data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Site ID",
    "name": "Site Name",
    "type": "Site Type",
    "area_id": "Area ID",
    "area_name": "Area Name",
    "client_id": "Client ID",
    "client_name": "Client Name"
}
```

### HTMX Endpoints

#### GET /api/sites/{id}
- **Description**: Get site details
- **Returns**: HTML Fragment - site details
- **Query**:
```json
{
    "tab": "Tab (schedule|materials|equipment|brigades|reports)"
}
```
- **Template**:
```json
{
    "id": "Site ID",
    "name": "Site Name",
    "type": "Site Type",
    "area_id": "Area ID",
    "area_name": "Area Name",
    "client_id": "Client ID",
    "client_name": "Client Name",
    "location": "Site Location",
    "risk_level": "Site Risk Level",
    "description": "Site Description",
    "type_specific_fields": "Fields specific to site type",
    "tab": "Tab (schedule|materials|equipment|brigades|reports)"
}
```

#### GET /api/sites/type-fields
- **Description**: Get type-specific form fields for the chosen site type
- **Returns**: HTML Fragment - form fields specific to site type
- **Query**:
```json
{
    "type": "Site Type (power_plant|road|housing|bridge|park)"
}
```
- **Template**:
```json
{
    "type": "Site Type"
}
```

#### PUT /api/sites/{id}
- **Description**: Change an existing site
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "name": "Site Name",
    "area_id": "Area ID",
    "client_id": "Client ID",
    "type": "Site Type",
    "location": "Site Location",
    "risk_level": "Site Risk Level",
    "description": "Site Description",
    "type_specific_fields": "Fields specific to site type"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/sites/{id}
- **Description**: Delete an existing site
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/sites
- **Description**: List all sites
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "area_id": "Filter by area ID",
    "department_id": "Filter by department ID",
    "client_id": "Filter by client ID",
    "type": "Filter by site type",
    "name": "Search by site name",
    "status": "Filter by status"
}
```
- **Template**:
```json
{
    "sites": [
        {
            "id": "Site ID",
            "name": "Site Name",
            "type": "Site Type",
            "area_id": "Area ID",
            "area_name": "Area Name",
            "department_id": "Department ID",
            "department_name": "Department Name",
            "client_id": "Client ID",
            "client_name": "Client Name",
            "status": "Site Status"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/sites
- **Description**: Create new site
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "name": "Site Name",
    "area_id": "Area ID",
    "client_id": "Client ID",
    "type": "Site Type",
    "location": "Site Location",
    "risk_level": "Site Risk Level",
    "description": "Site Description",
    "type_specific_fields": "Fields specific to site type"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/sites/{id}/schedule
- **Description**: Get schedule (tasks) for this specific site
- **Returns**: HTML Fragment - list of tasks
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "tasks": [
        {
            "id": "Task ID",
            "name": "Task Name",
            "brigade_id": "Brigade ID",
            "brigade_name": "Brigade Name",
            "period_start": "Start Date",
            "expected_period_end": "Expected End Date",
            "actual_period_end": "Actual End Date",
            "status": "Task Status"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

#### GET /api/sites/{id}/materials
- **Description**: Get material estimates and usage for this site
- **Returns**: HTML Fragment - list of materials
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "materials": [
        {
            "id": "Material ID",
            "name": "Material Name",
            "expected_amount": "Expected Amount",
            "actual_amount": "Actual Amount",
            "units": "Units",
            "cost": "Cost per Unit",
            "total_cost": "Total Cost"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

#### GET /api/sites/{id}/equipment
- **Description**: Get equipment allocated to this site
- **Returns**: HTML Fragment - list of equipment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "equipment": [
        {
            "id": "Equipment ID",
            "name": "Equipment Name",
            "amount": "Amount",
            "period_start": "Allocation Start",
            "period_end": "Allocation End"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

#### GET /api/sites/{id}/brigades
- **Description**: Get brigades assigned to this site
- **Returns**: HTML Fragment - list of brigades
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "brigades": [
        {
            "id": "Brigade ID",
            "brigadier_id": "Brigadier ID",
            "brigadier_name": "Brigadier Name",
            "worker_count": "Number of Workers",
            "current_task": "Current Task"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

#### GET /api/sites/{id}/reports
- **Description**: Get reports for this site
- **Returns**: HTML Fragment - site construction reports
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "reports": [
        {
            "task_id": "Task ID",
            "task_name": "Task Name",
            "period_start": "Start Date",
            "expected_period_end": "Expected End Date",
            "actual_period_end": "Actual End Date",
            "delay": "Delay in Days",
            "material_excesses": "List of exceeded materials"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

## 6. Worker Endpoints

### Page Endpoints

#### GET /workers
- **Description**: List all workers, retrieve the actual list using GET /api/workers
- **Returns**: Full page
- **Template**: null

#### GET /workers/{id}
- **Description**: Get worker details, retrieve the actual details using GET /api/workers/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Worker ID"
}
```

#### GET /workers/new
- **Description**: Get worker creation page, the worker is created using POST /api/workers
- **Returns**: Full page
- **Template**: null

#### GET /workers/{id}/edit
- **Description**: Get worker edit page with forms prefilled with the existing worker data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Worker ID",
    "first_name": "First Name",
    "last_name": "Last Name",
    "profession": "Profession"
}
```

### HTMX Endpoints

#### GET /api/workers/{id}
- **Description**: Get worker details
- **Returns**: HTML Fragment - worker details
- **Template**:
```json
{
    "id": "Worker ID",
    "first_name": "First Name",
    "last_name": "Last Name",
    "middle_name": "Middle Name",
    "gender": "Gender",
    "photo": "Photo URL",
    "phone_number": "Phone Number",
    "salary": "Salary",
    "profession": "Profession",
    "union_name": "Union Name",
    "brigade_id": "Brigade ID",
    "brigade_name": "Brigade Name",
    "is_brigadier": "Is Brigadier",
    "profession_specific_fields": "Fields specific to profession"
}
```

#### GET /api/workers/profession-fields
- **Description**: Get profession-specific form fields for the chosen worker profession
- **Returns**: HTML Fragment - form fields specific to profession
- **Query**:
```json
{
    "profession": "Profession (electrician|plumber|welder|driver|mason)"
}
```
- **Template**:
```json
{
    "profession": "Profession"
}
```

#### PUT /api/workers/{id}
- **Description**: Change an existing worker
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "first_name": "First Name",
    "last_name": "Last Name",
    "middle_name": "Middle Name",
    "gender": "Gender",
    "phone_number": "Phone Number",
    "salary": "Salary",
    "profession": "Profession",
    "union_name": "Union Name",
    "brigade_id": "Brigade ID",
    "profession_specific_fields": "Fields specific to profession"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/workers/{id}
- **Description**: Delete an existing worker
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/workers
- **Description**: List all workers
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "profession": "Filter by profession",
    "brigade_id": "Filter by brigade",
    "is_brigadier": "Filter by brigadier status",
    "name": "Search by name"
}
```
- **Template**:
```json
{
    "workers": [
        {
            "id": "Worker ID",
            "first_name": "First Name",
            "last_name": "Last Name",
            "profession": "Profession",
            "brigade_id": "Brigade ID",
            "brigade_name": "Brigade Name",
            "is_brigadier": "Is Brigadier"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/workers
- **Description**: Create new worker
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "first_name": "First Name",
    "last_name": "Last Name",
    "middle_name": "Middle Name",
    "gender": "Gender",
    "phone_number": "Phone Number",
    "salary": "Salary",
    "profession": "Profession",
    "union_name": "Union Name",
    "brigade_id": "Brigade ID",
    "profession_specific_fields": "Fields specific to profession"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

## 7. Technical Personnel Endpoints

### Page Endpoints

#### GET /technical-personnel
- **Description**: List all technical personnel, retrieve the actual list using GET /api/technical-personnel
- **Returns**: Full page
- **Template**: null

#### GET /technical-personnel/{id}
- **Description**: Get technical personnel details, retrieve the actual details using GET /api/technical-personnel/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Technical Personnel ID"
}
```

#### GET /technical-personnel/new
- **Description**: Get technical personnel creation page, created using POST /api/technical-personnel
- **Returns**: Full page
- **Template**: null

#### GET /technical-personnel/{id}/edit
- **Description**: Get technical personnel edit page with forms prefilled with existing data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Technical Personnel ID",
    "first_name": "First Name",
    "last_name": "Last Name",
    "qualification": "Qualification",
    "position": "Position"
}
```

### HTMX Endpoints

#### GET /api/technical-personnel/{id}
- **Description**: Get technical personnel details
- **Returns**: HTML Fragment - technical personnel details
- **Template**:
```json
{
    "id": "Technical Personnel ID",
    "first_name": "First Name",
    "last_name": "Last Name",
    "middle_name": "Middle Name",
    "gender": "Gender",
    "photo": "Photo URL",
    "phone_number": "Phone Number",
    "salary": "Salary",
    "qualification": "Qualification",
    "position": "Position",
    "education_level": "Education Level",
    "software_skills": "Software Skills",
    "is_project_manager": "Is Project Manager",
    "qualification_specific_fields": "Fields specific to qualification",
    "supervising_department_id": "Department ID being supervised",
    "supervising_department_name": "Department Name being supervised",
    "supervising_area_id": "Area ID being supervised",
    "supervising_area_name": "Area Name being supervised"
}
```

#### GET /api/technical-personnel/qualification-fields
- **Description**: Get qualification-specific form fields for the chosen qualification
- **Returns**: HTML Fragment - form fields specific to qualification
- **Query**:
```json
{
    "qualification": "Qualification (technician|technologist|engineer)"
}
```
- **Template**:
```json
{
    "qualification": "Qualification"
}
```

#### PUT /api/technical-personnel/{id}
- **Description**: Change an existing technical personnel
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "first_name": "First Name",
    "last_name": "Last Name",
    "middle_name": "Middle Name",
    "gender": "Gender",
    "phone_number": "Phone Number",
    "salary": "Salary",
    "qualification": "Qualification",
    "position": "Position",
    "education_level": "Education Level",
    "software_skills": "Software Skills",
    "is_project_manager": "Is Project Manager",
    "qualification_specific_fields": "Fields specific to qualification"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/technical-personnel/{id}
- **Description**: Delete an existing technical personnel
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/technical-personnel
- **Description**: List all technical personnel
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "qualification": "Filter by qualification",
    "position": "Filter by position",
    "department_id": "Filter by department",
    "area_id": "Filter by area",
    "name": "Search by name"
}
```
- **Template**:
```json
{
    "technical_personnel": [
        {
            "id": "Technical Personnel ID",
            "first_name": "First Name",
            "last_name": "Last Name",
            "qualification": "Qualification",
            "position": "Position",
            "department_id": "Department ID",
            "department_name": "Department Name",
            "area_id": "Area ID",
            "area_name": "Area Name"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/technical-personnel
- **Description**: Create new technical personnel
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "first_name": "First Name",
    "last_name": "Last Name",
    "middle_name": "Middle Name",
    "gender": "Gender",
    "phone_number": "Phone Number",
    "salary": "Salary",
    "qualification": "Qualification",
    "position": "Position",
    "education_level": "Education Level",
    "software_skills": "Software Skills",
    "is_project_manager": "Is Project Manager",
    "qualification_specific_fields": "Fields specific to qualification"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

## 8. Equipment Endpoints

### Page Endpoints

#### GET /equipment
- **Description**: List all equipment, retrieve the actual list using GET /api/equipment
- **Returns**: Full page
- **Template**: null

#### GET /equipment/{id}
- **Description**: Get equipment details, retrieve the actual details using GET /api/equipment/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Equipment ID"
}
```

#### GET /equipment/new
- **Description**: Get equipment creation page, the equipment is created using POST /api/equipment
- **Returns**: Full page
- **Template**: null

#### GET /equipment/{id}/edit
- **Description**: Get equipment edit page with forms prefilled with the existing equipment data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Equipment ID",
    "name": "Equipment Name",
    "amount": "Total Amount",
    "purchase_date": "Purchase Date",
    "purchase_cost": "Purchase Cost",
    "fuel_type": "Fuel Type"
}
```

### HTMX Endpoints

#### GET /api/equipment/{id}
- **Description**: Get equipment details
- **Returns**: HTML Fragment - equipment details
- **Template**:
```json
{
    "id": "Equipment ID",
    "name": "Equipment Name",
    "amount": "Total Amount",
    "available_amount": "Available Amount",
    "purchase_date": "Purchase Date",
    "purchase_cost": "Purchase Cost",
    "fuel_type": "Fuel Type"
}
```

#### PUT /api/equipment/{id}
- **Description**: Change existing equipment
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "name": "Equipment Name",
    "amount": "Total Amount",
    "purchase_date": "Purchase Date",
    "purchase_cost": "Purchase Cost",
    "fuel_type": "Fuel Type"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/equipment/{id}
- **Description**: Delete existing equipment
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/equipment
- **Description**: List all equipment
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "department_id": "Filter by department ID",
    "site_id": "Filter by site ID",
    "name": "Search by equipment name",
    "available": "Filter by availability"
}
```
- **Template**:
```json
{
    "equipment": [
        {
            "id": "Equipment ID",
            "name": "Equipment Name",
            "total_amount": "Total Amount",
            "available_amount": "Available Amount",
            "purchase_date": "Purchase Date",
            "purchase_cost": "Purchase Cost"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/equipment
- **Description**: Create new equipment
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "name": "Equipment Name",
    "amount": "Total Amount",
    "purchase_date": "Purchase Date",
    "purchase_cost": "Purchase Cost",
    "fuel_type": "Fuel Type"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/equipment/{id}/allocations
- **Description**: Get allocation history for this specific equipment
- **Returns**: HTML Fragment - list of allocations
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "allocations": [
        {
            "department_id": "Department ID",
            "department_name": "Department Name",
            "site_id": "Site ID",
            "site_name": "Site Name",
            "amount": "Allocated Amount",
            "period_start": "Start Date",
            "period_end": "End Date",
            "is_current": "Is Currently Allocated"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

#### POST /api/equipment/{id}/allocations
- **Description**: Create new equipment allocation
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "department_id": "Department ID",
    "site_id": "Site ID (optional)",
    "amount": "Amount to Allocate",
    "period_start": "Start Date",
    "period_end": "End Date"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

## 9. Client Endpoints

### Page Endpoints

#### GET /clients
- **Description**: List all clients, retrieve the actual list using GET /api/clients
- **Returns**: Full page
- **Template**: null

#### GET /clients/{id}
- **Description**: Get client details, retrieve the actual details using GET /api/clients/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Client ID"
}
```

#### GET /clients/new
- **Description**: Get client creation page, the client is created using POST /api/clients
- **Returns**: Full page
- **Template**: null

#### GET /clients/{id}/edit
- **Description**: Get client edit page with forms prefilled with the existing client data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Client ID",
    "name": "Client Name",
    "inn": "INN",
    "is_vip": "VIP Status"
}
```

### HTMX Endpoints

#### GET /api/clients/{id}
- **Description**: Get client details
- **Returns**: HTML Fragment - client details
- **Template**:
```json
{
    "id": "Client ID",
    "name": "Client Name",
    "inn": "INN",
    "address": "Address",
    "contact_person_email": "Contact Email",
    "contact_person_name": "Contact Name",
    "is_vip": "VIP Status",
    "sites": [
        {
            "id": "Site ID",
            "name": "Site Name",
            "type": "Site Type",
            "status": "Site Status"
        }
    ]
}
```

#### PUT /api/clients/{id}
- **Description**: Change an existing client
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "name": "Client Name",
    "inn": "INN",
    "address": "Address",
    "contact_person_email": "Contact Email",
    "contact_person_name": "Contact Name",
    "is_vip": "VIP Status"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/clients/{id}
- **Description**: Delete an existing client
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/clients
- **Description**: List all clients
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "name": "Search by client name",
    "inn": "Search by INN",
    "is_vip": "Filter by VIP status"
}
```
- **Template**:
```json
{
    "clients": [
        {
            "id": "Client ID",
            "name": "Client Name",
            "inn": "INN",
            "is_vip": "VIP Status"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/clients
- **Description**: Create new client
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "name": "Client Name",
    "inn": "INN",
    "address": "Address",
    "contact_person_email": "Contact Email",
    "contact_person_name": "Contact Name",
    "is_vip": "VIP Status"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

## 10. Brigade Endpoints

### Page Endpoints

#### GET /brigades
- **Description**: List all brigades, retrieve the actual list using GET /api/brigades
- **Returns**: Full page
- **Template**: null

#### GET /brigades/{id}
- **Description**: Get brigade details, retrieve the actual details using GET /api/brigades/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Brigade ID"
}
```

#### GET /brigades/new
- **Description**: Get brigade creation page, the brigade is created using POST /api/brigades
- **Returns**: Full page
- **Template**: null

#### GET /brigades/{id}/edit
- **Description**: Get brigade edit page with forms prefilled with the existing brigade data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Brigade ID",
    "brigadier_id": "Brigadier ID",
    "brigadier_name": "Brigadier Name"
}
```

### HTMX Endpoints

#### GET /api/brigades/{id}
- **Description**: Get brigade details
- **Returns**: HTML Fragment - brigade details
- **Query**:
```json
{
    "tab": "Tab (workers|tasks|current)"
}
```
- **Template**:
```json
{
    "id": "Brigade ID",
    "brigadier_id": "Brigadier ID",
    "brigadier_name": "Brigadier Name",
    "worker_count": "Number of Workers",
    "current_task_id": "Current Task ID",
    "current_task_name": "Current Task Name",
    "current_site_id": "Current Site ID",
    "current_site_name": "Current Site Name",
    "tab": "Tab (workers|tasks|current)"
}
```

#### PUT /api/brigades/{id}
- **Description**: Change an existing brigade
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "brigadier_id": "Brigadier ID"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/brigades/{id}
- **Description**: Delete an existing brigade
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/brigades
- **Description**: List all brigades
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "brigadier_id": "Filter by brigadier ID",
    "site_id": "Filter by current site assignment",
    "task_type": "Filter by task type performed"
}
```
- **Template**:
```json
{
    "brigades": [
        {
            "id": "Brigade ID",
            "brigadier_id": "Brigadier ID",
            "brigadier_name": "Brigadier Name",
            "worker_count": "Number of Workers",
            "current_site_id": "Current Site ID",
            "current_site_name": "Current Site Name"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages", 
    "num_items": "Number of items"
}
```

#### POST /api/brigades
- **Description**: Create new brigade
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "brigadier_id": "Brigadier ID"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/brigades/{id}/workers
- **Description**: Get workers assigned to this specific brigade
- **Returns**: HTML Fragment - list of workers
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "workers": [
        {
            "id": "Worker ID",
            "first_name": "First Name",
            "last_name": "Last Name",
            "profession": "Profession",
            "is_brigadier": "Is Brigadier"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

#### POST /api/brigades/{id}/workers
- **Description**: Add worker to brigade
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "worker_id": "Worker ID"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/brigades/{id}/workers/{worker_id}
- **Description**: Remove worker from brigade
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/brigades/{id}/tasks
- **Description**: Get task history for this specific brigade
- **Returns**: HTML Fragment - list of tasks
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page",
    "time_from": "Start date for filtering",
    "time_to": "End date for filtering"
}
```
- **Template**:
```json
{
    "tasks": [
        {
            "id": "Task ID",
            "name": "Task Name",
            "site_id": "Site ID",
            "site_name": "Site Name",
            "period_start": "Start Date",
            "period_end": "End Date",
            "status": "Task Status"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

## 11. Task Endpoints

### Page Endpoints

#### GET /tasks
- **Description**: List all tasks, retrieve the actual list using GET /api/tasks
- **Returns**: Full page
- **Template**: null

#### GET /tasks/{id}
- **Description**: Get task details, retrieve the actual details using GET /api/tasks/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Task ID"
}
```

#### GET /tasks/new
- **Description**: Get task creation page, the task is created using POST /api/tasks
- **Returns**: Full page
- **Template**: null

#### GET /tasks/{id}/edit
- **Description**: Get task edit page with forms prefilled with the existing task data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Task ID",
    "name": "Task Name",
    "site_id": "Site ID",
    "site_name": "Site Name",
    "brigade_id": "Brigade ID",
    "brigade_name": "Brigade Name"
}
```

### HTMX Endpoints

#### GET /api/tasks/{id}
- **Description**: Get task details
- **Returns**: HTML Fragment - task details
- **Query**:
```json
{
    "tab": "Tab (materials|progress)"
}
```
- **Template**:
```json
{
    "id": "Task ID",
    "name": "Task Name",
    "description": "Task Description",
    "site_id": "Site ID",
    "site_name": "Site Name",
    "brigade_id": "Brigade ID",
    "brigade_name": "Brigade Name",
    "period_start": "Start Date",
    "expected_period_end": "Expected End Date",
    "actual_period_end": "Actual End Date",
    "status": "Task Status",
    "tab": "Tab (materials|progress)"
}
```

#### PUT /api/tasks/{id}
- **Description**: Change an existing task
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "name": "Task Name", 
    "description": "Task Description",
    "site_id": "Site ID",
    "brigade_id": "Brigade ID",
    "period_start": "Start Date",
    "expected_period_end": "Expected End Date"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/tasks/{id}
- **Description**: Delete an existing task
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/tasks
- **Description**: List all tasks
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "site_id": "Filter by site ID",
    "brigade_id": "Filter by brigade ID",
    "status": "Filter by status",
    "date_from": "Filter by start date",
    "date_to": "Filter by end date",
    "name": "Search by task name",
    "exceeded_deadline": "Filter by exceeded deadline"
}
```
- **Template**:
```json
{
    "tasks": [
        {
            "id": "Task ID",
            "name": "Task Name",
            "site_id": "Site ID",
            "site_name": "Site Name",
            "brigade_id": "Brigade ID",
            "brigade_name": "Brigade Name",
            "period_start": "Start Date",
            "expected_period_end": "Expected End Date",
            "actual_period_end": "Actual End Date",
            "status": "Task Status",
            "deadline_exceeded": "Is Deadline Exceeded"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/tasks
- **Description**: Create new task
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "name": "Task Name",
    "description": "Task Description",
    "site_id": "Site ID",
    "brigade_id": "Brigade ID",
    "period_start": "Start Date",
    "expected_period_end": "Expected End Date"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/tasks/{id}/materials
- **Description**: Get materials for this specific task
- **Returns**: HTML Fragment - list of materials with estimates and usage
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "materials": [
        {
            "material_id": "Material ID",
            "name": "Material Name",
            "expected_amount": "Expected Amount",
            "actual_amount": "Actual Amount",
            "units": "Units",
            "cost": "Cost per Unit",
            "total_cost": "Total Cost",
            "excess": "Excess Amount"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

#### POST /api/tasks/{id}/materials
- **Description**: Add material to task or update estimate
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "material_id": "Material ID",
    "expected_amount": "Expected Amount"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### PUT /api/tasks/{id}/materials/{material_id}
- **Description**: Update actual material usage
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "actual_amount": "Actual Amount"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### PUT /api/tasks/{id}/complete
- **Description**: Mark task as completed
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "actual_period_end": "Actual End Date"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

## 12. Material Endpoints

### Page Endpoints

#### GET /materials
- **Description**: List all materials, retrieve the actual list using GET /api/materials
- **Returns**: Full page
- **Template**: null

#### GET /materials/{id}
- **Description**: Get material details, retrieve the actual details using GET /api/materials/{id}
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Material ID"
}
```

#### GET /materials/new
- **Description**: Get material creation page, the material is created using POST /api/materials
- **Returns**: Full page
- **Template**: null

#### GET /materials/{id}/edit
- **Description**: Get material edit page with forms prefilled with the existing material data
- **Returns**: Full page
- **Template**:
```json
{
    "id": "Material ID",
    "name": "Material Name",
    "cost": "Cost per Unit",
    "units": "Units"
}
```

### HTMX Endpoints

#### GET /api/materials/{id}
- **Description**: Get material details
- **Returns**: HTML Fragment - material details
- **Template**:
```json
{
    "id": "Material ID",
    "name": "Material Name",
    "cost": "Cost per Unit",
    "units": "Units",
    "total_estimated": "Total Estimated Usage",
    "total_actual": "Total Actual Usage",
    "total_cost": "Total Cost of Usage"
}
```

#### PUT /api/materials/{id}
- **Description**: Change an existing material
- **Returns**: HTML Fragment - Success or Error notification
- **Form Data**:
```json
{
    "name": "Material Name",
    "cost": "Cost per Unit",
    "units": "Units"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### DELETE /api/materials/{id}
- **Description**: Delete an existing material
- **Returns**: HTML Fragment - Success or Error notification
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/materials
- **Description**: List all materials
- **Returns**: HTML Fragment
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Items Per Page"
}
```
- **Form Data**:
```json
{
    "sort_by": "Column to sort by",
    "sort_direction": "Sort direction",
    "name": "Search by material name",
    "cost_min": "Filter by minimum cost",
    "cost_max": "Filter by maximum cost",
    "excess_usage": "Filter by excess usage"
}
```
- **Template**:
```json
{
    "materials": [
        {
            "id": "Material ID",
            "name": "Material Name",
            "cost": "Cost per Unit",
            "units": "Units",
            "estimated_spendings": "Total Estimated Spendings",
            "actual_spendings": "Total Actual Spendings",
            "excess": "Has Excess Usage"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Items Per Page",
    "num_pages": "Number of pages",
    "num_items": "Number of items"
}
```

#### POST /api/materials
- **Description**: Create new material
- **Returns**: HTML Fragment - Success or Error message
- **Form Data**:
```json
{
    "name": "Material Name",
    "cost": "Cost per Unit",
    "units": "Units"
}
```
- **Template**:
```json
{
    "result": "Success or Error",
    "message": "Success or Error Message",
    "redirect": "URL to redirect to after 3s"
}
```

#### GET /api/materials/{id}/usage
- **Description**: Get usage history for this specific material
- **Returns**: HTML Fragment - list of tasks using this material
- **Query**:
```json
{
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```
- **Template**:
```json
{
    "usage": [
        {
            "task_id": "Task ID",
            "task_name": "Task Name",
            "site_id": "Site ID",
            "site_name": "Site Name",
            "expected_amount": "Expected Amount",
            "actual_amount": "Actual Amount",
            "excess_amount": "Excess Amount",
            "total_cost": "Total Cost"
        }
    ],
    "page_number": "Page Number",
    "page_size": "Number of items per page"
}
```

## 13. Report Endpoints

TODO...

**This document describes the http endpoints used by the Onyx Core Builders Informational System**