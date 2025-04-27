# Onyx Core Builders Information System UI/UX Specification

This document outlines the user UI/UX design for the Onyx Horizons construction company information system.

## Design Principles

- **Minimalistic**: Clean, uncluttered interfaces with essential elements only
- **Intuitive**: Self-explanatory navigation and workflows
- **Efficient**: Quick access to frequently used functions
- **Consistent**: Uniform design patterns across all pages

## Color Scheme & Visual Identity

- **Primary Color**: Deep onyx (#0F1621) 
- **Secondary Color**: Gold/amber (#FFC107) for highlighting and CTAs
- **Background**: Light gray (#F5F7FA)
- **Regular Text**: Dark gray (#333333)
- **Headings**: Black (#000000)
- **Success**: Green (#4CAF50) for positive actions
- **Warning**: Amber (#FFBF00) for caution
- **Error**: Red (#F44336) for errors or destructive actions

## Typography

- **Headings**: Sans-serif font (Inter or similar)
- **Body text**: Sans-serif font (Inter or similar)
- **Font sizes**: 
  - H1: 24px
  - H2: 20px
  - H3: 18px
  - Body: 16px
  - Small text: 14px

## Common UI Components

### Navigation

A left sidebar navigation with sections for:

1. Dashboard
2. Departmens
3. Areas
4. Sites
5. Workers
6. Technical Personnel
7. Equipment
8. Clients
9. Brigades
10. Tasks
11. Materials
12. Reports

A company logo appears at the top of the sidebar. Each section has an icon next to it.

### Tables

Used for listing entities with the following features:
- Sortable by any column (using HTMX to reload sorted data)
- Filterable (using HTMX to reload filtered data)
- Pagination (using HTMX to load next/previous pages)
- Action buttons for each row (View, Edit, Delete)

### Cards

Used to display entity details:
- Clean borders with subtle shadows
- Sectioned content with clear headings
- Edit buttons on each section that transform the section into an editable form
- Delete button for deleting the entity

### Forms

- Grouped by logical sections
- Inline validation using HTMX
- Clear labeling and placeholder text
- Required fields marked with asterisk

### Buttons

- **Primary**: Gold/amber with dark text for main actions
- **Secondary**: Light gray with dark text for secondary actions
- **Destructive**: Red for delete actions
- **Icon buttons**: For common actions like edit, delete, view

## Page Specifications

### 1. Dashboard

TODO...

### 2. Departments

#### 2.1 Departments Listing

**Content:**
- Table of departments with columns:
  - Department Name
  - Supervisor

**Actions:**
- Create New Department button
- Filter by name or supervisor
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #1: Get a list of construction departments and their supervisors

#### 2.2 Department Details

**Content:**
- Department information card (Name, Supervisor)
- Areas tab showing areas in this department
- Equipment tab showing equipment assigned to this department
- Sites tab showing all sites across all areas
- Technical personnel tab showing technical personnel in this department

**Actions:**
- Edit department information, including the supervisor
- Add/remove areas
- Assign/unassign equipment

**Queries Supported:**
- #1: Get list of construction departments and supervisors
- #2: Get list of technical personnel staff of designated department
- #3: Get list of sites being built by the specified department
- #5: Get list of construction equipment assigned to the department

#### 2.3 Department Edit

**Content**:
- Editable form fields with current values for the department
- Button to submit the changes

**Actions**:
- Edit department information (name, supervisor)

#### 2.4 Department New

**Content**:
- Required form fields

**Actions**:
- Create new department (with name, supervisor)

### 3. Areas

#### 3.1 Area Listing

**Content:**
- Table of areas with columns:
  - Area Name
  - Department
  - Supervisor

**Actions:**
- Create New Area button
- Filter by name, department, or supervisor
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #1: Get a list of construction areas and their supervisors

#### 3.2 Area Details

**Content:**
- Area information card (Name, Department, Supervisor)
- Sites tab showing sites in this area
- Technical personnel tab showing staff in this area

**Actions:**
- Edit area information
- Add/remove sites
- Assign/unassign supervisor

**Queries Supported:**
- #1: Get list of construction areas and supervisors
- #2: Get list of technical personnel staff of designated area
- #3: Get list of sites being built by the specified area

#### 3.3 Area Edit

**Content**:
- Editable form fields with current values for the area
- Button to submit the changes

**Actions**:
- Edit area information (name, department, supervisor)

#### 3.4 Area New

**Content**:
- Required form fields

**Actions**:
- Create new department (with name, department, supervisor)

### 4. Sites

#### 4.1 Sites Listing

**Content:**
- Table of sites with columns:
  - Name
  - Type (with icon for each type)
  - Area/Department
  - Client
  - Status (Planned/In Progress/Completed)

**Actions:**
- Create New Site button
- Advanced filtering by area, department
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #3: Get a list of sites being built by specified department/area

#### 4.2 Site Details

**Content:**
- Site information card, including type-specific details
- Schedule tab showing timeline of tasks
- Materials tab showing estimates and actual usage
- Equipment tab showing allocated equipment
- Brigades tab showing currently/previously assigned brigades
- Reports tab showing completion reports

**Actions:**
- Edit site information
- Manage construction schedule
- Update material estimates/usage
- Allocate/deallocate equipment
- Assign/unassign brigades

**Queries Supported:**
- #4: Get list of workers of brigades working on the site
- #6: Get list of construction equipment allocated to the site
- #7: Get schedule and estimate for the construction
- #8: Get report on the construction of the site
- #10: Get list of construction tasks with exceeded deadlines
- #11: Get list of building materials with estimate excesses

#### 4.3 Site Edit

**Content**:
- Form fields with values from existing site
- When the site type is changed, type-specific form fileds are loaded using htmx

**Actions**:
- Edit site information

#### 4.4 Site New

**Content**:
- Form fields for creating a new site
- When the site type is chosen, type-specific form fileds are loaded using htmx

**Actions**:
- Create new site

### 5. Technical Personnel

#### 5.1 Technical Personnel Listing

**Content:**
- Table of technical personnel with columns:
  - Name
  - Qualification (Technician/Technologist/Engineer)
  - Position
  - Department/Area

**Actions:**
- Create New Technical Personnel button
- Filter by qualification, position, department
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #2: Get list of technical personnel staff of designated area/department

#### 5.2 Technical Personnel Details

**Content:**
- Personal information card (Name, Photo if present, Contact Info, other fields)
- Professional details card (Qualification, Position, Education)
- Assignments card (Department/Area supervision)
- Type-specific details section based on qualification

**Actions:**
- Edit personal information
- Edit professional details
- Change assignments
- Upload new photo

#### 5.3 Technical Personnel Edit

**Content**:
- Form fields with values for an exising techincal personnel
- When the qualification is chosen, type-specific form fileds are loaded using htmx

**Actions**:
- Edit technical personnel information

#### 5.4 Technical Personnel New

**Content**:
- Form fields for creating a new technical personnel
- When the qualification is chosen, type-specific form fileds are loaded using htmx

**Actions**:
- Create new technical employee

### 6. Workers

#### 6.1 Workers Listing

**Content:**
- Table of workers with columns:
  - Name
  - Profession
  - Brigade (if assigned)
  - Is Brigadier

**Actions:**
- Create New Worker button
- Filter by profession, brigade, is brigadier
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #4: Get list of workers of brigades (when filtered)

#### 6.2 Worker Details

**Content:**
- Personal information card (Name, Photo, Contact)
- Professional details card (Profession, Salary, Union)
- Brigade assignment card
- Profession-specific details section

**Actions:**
- Edit personal information
- Edit professional details
- Change brigade assignment
- Upload new photo

#### 6.3 Worker Edit

**Content**:
- Form fields with values for an exising worker
- When the profession is chosen, type-specific form fileds are loaded using htmx

**Actions**:
- Edit technical personnel information

#### 6.4 Worker New

**Content**:
- Form fields for creating a new technical personnel
- When the qualification is chosen, type-specific form fileds are loaded using htmx

**Actions**:
- Create new technical employee

### 7. Equipment

#### 7.1 Equipment Listing

**Content:**
- Table of equipment with columns:
  - Name
  - Total Amount
  - Available Amount
  - Purchase Date
  - Purchase Cost

**Actions:**
- Create New Equipment button
- Filter by availability
- Seach by name
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #5: Get list of construction equipment (when filtered by department)
- #6: Get list of construction equipment (when filtered by site)

#### 7.2 Equipment Details

**Content:**
- Equipment information card
- Allocation history showing where equipment was used and where it is currently used

**Actions:**
- Edit equipment information
- Create new allocation

**Queries Supported:**
- #5: Get list of construction equipment assigned to department
- #6: Get list of equipment allocated to site or worked during period

#### 7.3 Equipment Edit

**Content**:
- Form fields with values for an exising equipment

**Actions**:
- Edit equipment information (name, amount, purchase date, purchase cost)

#### 7.4 Equipment New

**Content**:
- Form fields for creating a new equipment (name, amount, purchase date, purchase cost)

**Actions**:
- Create new equipment

### 8. Client Management

#### 8.1 Client Listing

**Content:**
- Table of clients with columns:
  - Name
  - INN
  - Is VIP

**Actions:**
- Create New Client button
- Filter by VIP status
- Search by name, INN
- Sort by any column
- View/Edit/Delete buttons for each row

#### 8.2 Client Details

**Content:**
- Client information card

**Actions:**
- Edit client information

#### 8.3 Client Edit

**Content**:
- Form fields with values for an exising client

**Actions**:
- Edit client information

#### 8.4 Client New

**Content**:
- Form fields for creating a new client

**Actions**:
- Create new client

### 9. Brigades

#### 9.1 Brigade Listing

**Content:**
- Table of brigades with columns:
  - Brigadier Name
  - Number of Workers
  - Current Site Assignment

**Actions:**
- Create New Brigade button
- Filter by brigadier, site assignment
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #13: Get list of brigades that performed specified construction tasks

#### 9.2 Brigade Details

**Content:**
- Brigade information card
- Workers tab showing assigned workers
- Task history tab showing completed tasks
- Current assignment tab

**Actions:**
- Change brigadier
- Add/remove workers
- Assign to task

**Queries Supported:**
- #12: Get list of construction tasks performed by brigade
- #13: Get list of sites where brigade performed tasks

### 10. Tasks

#### 10.1 Task Listing

**Content:**
- Table of tasks with columns:
  - Name
  - Site
  - Brigadier Name of the brigade that is currently assigned to task
  - Start Date
  - Expected End Date (or Actual End Date, if it is present)
  - Status (Planned, In Progress, Completed)

**Actions:**
- Create New Task button
- Filter by site, brigade, status, date range
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #9: Get list of sites where specific task type was performed
- #10: Get list of task types with exceeded deadlines
- #12: Get list of task types performed by brigade

#### 10.2 Task Details

**Content:**
- Task information card
- Materials tab showing estimates and actual usage

**Actions:**
- Edit task information
- Add/edit material estimates
- Update actual material usage
- Mark task as completed

**Queries Supported:**
- #11: Get list of building materials with estimate excesses

### 11. Materials

#### 11.1 Material Listing

**Content:**
- Table of materials with columns:
  - Name
  - Cost per Unit
  - Units
  - Estimated spendings
  - Actual spendings

**Actions:**
- Create New Material button
- Filter by cost range, excess usage
- Search by name
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #11: Get list of building materials with estimate excesses (when filtered)

#### 11.2 Material Details

**Content:**
- Material information card

**Actions:**
- Edit material information

### 12. Reports

TODO...

## HTMX Integration

The system uses HTMX for dynamic updates without JavaScript:

- Tables use `hx-get` for sorting, filtering, and pagination
- Forms use `hx-post` for submission
- Dropdowns use `hx-get` to populate dependent fields (e.g., areas based on department)
- Detail pages use `hx-get` to switch between tabs
- Edit buttons use `hx-get` to replace view sections with editable forms
- Save buttons use `hx-post` to submit changes and revert to view mode
- Edit forms use `hx-put` to change an exising entity
- Forms use `hx-get` to get form fields needed for an entity subtype

This UI design provides a comprehensive solution for managing all aspects of the Onyx Core Builders construction company while satisfying all the specified query requirements in an intuitive and efficient manner.