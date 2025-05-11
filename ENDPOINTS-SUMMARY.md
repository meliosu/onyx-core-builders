# API Endpoints Summary

## Helper Endpoints - HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/selectors/departments | Get filtered list of departments | HTML Fragment |
| GET /api/selectors/areas | Get filtered list of areas | HTML Fragment |
| GET /api/selectors/clients | Get filtered list of clients | HTML Fragment |
| GET /api/selectors/technical-personnel | Get filtered list of technical personnel | HTML Fragment |
| GET /api/selectors/workers | Get filtered list of workers | HTML Fragment |
| GET /api/selectors/brigades | Get filtered list of brigades | HTML Fragment |
| GET /api/selectors/sites | Get filtered list of sites | HTML Fragment |
| GET /api/selectors/equipment | Get filtered list of equipment | HTML Fragment |
| GET /api/selectors/materials | Get filtered list of materials | HTML Fragment |
| GET /api/selectors/tasks | Get filtered list of tasks | HTML Fragment |

## 1. General Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET / | Return empty page (for now) | Full page |
| GET /404 | Return 404 Page | Full page |
| GET /500 | Return 500 Page | Full page |

## 3. Department Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /departments | List all departments | Full page |
| GET /departments/{id} | Get department details | Full page |
| GET /departments/new | Get department creation page | Full page |
| GET /departments/{id}/edit | Get department edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/departments/{id} | Get department details | HTML Fragment |
| PUT /api/departments/{id} | Change an existing department | HTML Fragment |
| DELETE /api/departments/{id} | Delete an existing department | HTML Fragment |
| GET /api/departments | List all departments | HTML Fragment |
| POST /api/departments | Create new department | HTML Fragment |
| GET /api/departments/{id}/areas | Get areas for this specific department | HTML Fragment |
| GET /api/departments/{id}/equipment | Get equipment for this specific department | HTML Fragment |
| GET /api/departments/{id}/sites | Get sites for this specific department | HTML Fragment |
| GET /api/departments/{id}/personnel | Get personnel for this specific department | HTML Fragment |

## 4. Area Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /areas | List all areas | Full page |
| GET /areas/{id} | Get area details | Full page |
| GET /areas/new | Get area creation page | Full page |
| GET /areas/{id}/edit | Get area edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/areas/{id} | Get area details | HTML Fragment |
| PUT /api/areas/{id} | Change an existing area | HTML Fragment |
| DELETE /api/areas/{id} | Delete an existing area | HTML Fragment |
| GET /api/areas | List all areas | HTML Fragment |
| POST /api/areas | Create new area | HTML Fragment |
| GET /api/areas/{id}/sites | Get sites for this specific area | HTML Fragment |
| GET /api/areas/{id}/personnel | Get personnel for this specific area | HTML Fragment |

## 5. Site Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /sites | List all sites | Full page |
| GET /sites/{id} | Get site details | Full page |
| GET /sites/new | Get site creation page | Full page |
| GET /sites/{id}/edit | Get site edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/sites/{id} | Get site details | HTML Fragment |
| GET /api/sites/type-fields | Get type-specific form fields for chosen site type | HTML Fragment |
| PUT /api/sites/{id} | Change an existing site | HTML Fragment |
| DELETE /api/sites/{id} | Delete an existing site | HTML Fragment |
| GET /api/sites | List all sites | HTML Fragment |
| POST /api/sites | Create new site | HTML Fragment |
| GET /api/sites/{id}/schedule | Get schedule (tasks) for this specific site | HTML Fragment |
| GET /api/sites/{id}/materials | Get material estimates and usage for this site | HTML Fragment |
| GET /api/sites/{id}/equipment | Get equipment allocated to this site | HTML Fragment |
| GET /api/sites/{id}/brigades | Get brigades assigned to this site | HTML Fragment |
| GET /api/sites/{id}/reports | Get reports for this site | HTML Fragment |

## 6. Worker Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /workers | List all workers | Full page |
| GET /workers/{id} | Get worker details | Full page |
| GET /workers/new | Get worker creation page | Full page |
| GET /workers/{id}/edit | Get worker edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/workers/{id} | Get worker details | HTML Fragment |
| GET /api/workers/profession-fields | Get profession-specific form fields | HTML Fragment |
| PUT /api/workers/{id} | Change an existing worker | HTML Fragment |
| DELETE /api/workers/{id} | Delete an existing worker | HTML Fragment |
| GET /api/workers | List all workers | HTML Fragment |
| POST /api/workers | Create new worker | HTML Fragment |

## 7. Technical Personnel Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /technical-personnel | List all technical personnel | Full page |
| GET /technical-personnel/{id} | Get technical personnel details | Full page |
| GET /technical-personnel/new | Get technical personnel creation page | Full page |
| GET /technical-personnel/{id}/edit | Get technical personnel edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/technical-personnel/{id} | Get technical personnel details | HTML Fragment |
| GET /api/technical-personnel/qualification-fields | Get qualification-specific form fields | HTML Fragment |
| PUT /api/technical-personnel/{id} | Change an existing technical personnel | HTML Fragment |
| DELETE /api/technical-personnel/{id} | Delete an existing technical personnel | HTML Fragment |
| GET /api/technical-personnel | List all technical personnel | HTML Fragment |
| POST /api/technical-personnel | Create new technical personnel | HTML Fragment |

## 8. Equipment Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /equipment | List all equipment | Full page |
| GET /equipment/{id} | Get equipment details | Full page |
| GET /equipment/new | Get equipment creation page | Full page |
| GET /equipment/{id}/edit | Get equipment edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/equipment/{id} | Get equipment details | HTML Fragment |
| PUT /api/equipment/{id} | Change existing equipment | HTML Fragment |
| DELETE /api/equipment/{id} | Delete existing equipment | HTML Fragment |
| GET /api/equipment | List all equipment | HTML Fragment |
| POST /api/equipment | Create new equipment | HTML Fragment |
| GET /api/equipment/{id}/allocations | Get allocation history for this equipment | HTML Fragment |
| POST /api/equipment/{id}/allocations | Create new equipment allocation | HTML Fragment |

## 9. Client Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /clients | List all clients | Full page |
| GET /clients/{id} | Get client details | Full page |
| GET /clients/new | Get client creation page | Full page |
| GET /clients/{id}/edit | Get client edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/clients/{id} | Get client details | HTML Fragment |
| PUT /api/clients/{id} | Change an existing client | HTML Fragment |
| DELETE /api/clients/{id} | Delete an existing client | HTML Fragment |
| GET /api/clients | List all clients | HTML Fragment |
| POST /api/clients | Create new client | HTML Fragment |

## 10. Brigade Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /brigades | List all brigades | Full page |
| GET /brigades/{id} | Get brigade details | Full page |
| GET /brigades/new | Get brigade creation page | Full page |
| GET /brigades/{id}/edit | Get brigade edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/brigades/{id} | Get brigade details | HTML Fragment |
| PUT /api/brigades/{id} | Change an existing brigade | HTML Fragment |
| DELETE /api/brigades/{id} | Delete an existing brigade | HTML Fragment |
| GET /api/brigades | List all brigades | HTML Fragment |
| POST /api/brigades | Create new brigade | HTML Fragment |
| GET /api/brigades/{id}/workers | Get workers assigned to this brigade | HTML Fragment |
| POST /api/brigades/{id}/workers | Add worker to brigade | HTML Fragment |
| DELETE /api/brigades/{id}/workers/{worker_id} | Remove worker from brigade | HTML Fragment |
| GET /api/brigades/{id}/tasks | Get task history for this brigade | HTML Fragment |

## 11. Task Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /tasks | List all tasks | Full page |
| GET /tasks/{id} | Get task details | Full page |
| GET /tasks/new | Get task creation page | Full page |
| GET /tasks/{id}/edit | Get task edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/tasks/{id} | Get task details | HTML Fragment |
| PUT /api/tasks/{id} | Change an existing task | HTML Fragment |
| DELETE /api/tasks/{id} | Delete an existing task | HTML Fragment |
| GET /api/tasks | List all tasks | HTML Fragment |
| POST /api/tasks | Create new task | HTML Fragment |
| GET /api/tasks/{id}/materials | Get materials for this specific task | HTML Fragment |
| POST /api/tasks/{id}/materials | Add material to task or update estimate | HTML Fragment |
| PUT /api/tasks/{id}/materials/{material_id} | Update actual material usage | HTML Fragment |
| PUT /api/tasks/{id}/complete | Mark task as completed | HTML Fragment |

## 12. Material Endpoints

### Page Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /materials | List all materials | Full page |
| GET /materials/{id} | Get material details | Full page |
| GET /materials/new | Get material creation page | Full page |
| GET /materials/{id}/edit | Get material edit page | Full page |

### HTMX Endpoints

| Endpoint | Description | Returns |
| --- | --- | --- |
| GET /api/materials/{id} | Get material details | HTML Fragment |
| PUT /api/materials/{id} | Change an existing material | HTML Fragment |
| DELETE /api/materials/{id} | Delete an existing material | HTML Fragment |
| GET /api/materials | List all materials | HTML Fragment |
| POST /api/materials | Create new material | HTML Fragment |
| GET /api/materials/{id}/usage | Get usage history for this material | HTML Fragment |