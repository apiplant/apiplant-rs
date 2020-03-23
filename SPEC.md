# APIPlant

## Config
Config is loaded parsing `config.toml` in the working directory.
Plugins (.so, .dll) are loaded from the `plugins` directory in the working directory.
Native Hooks (.so, .dll) are loaded from the `hooks` directory in the working directory.

### server
Configuration for initialising the server
```
[server]
mount-url = '' # default, any host, port 8080 or first free, instance mounted on /
mount-url = ':80/api' # any host, port 80, instance mounted on /
mount-url = 'mydomain.com/api' # mydomain.com only, port 8080, instance mounted on /api
mount-url = 'localhost:1337' # localhost only, port 1337, instance mounted on /
```

### driver-secure-storage
Configuration for storing secure information, as specified by plugin.

### driver-event-storage
Configuration for storing events, as specified by plugin.

### driver-model-storage
Configuration for storing models, as specified by plugin.

### driver-error-storage
Configuration for logging errors, as specified by plugin.


## Logic

### Init application

 - Parse config
 - Load plugins and get informations on them
 - Create shared context for sharing connections between drivers
 - Load plugins
 - Initialise Driver Secure Storage with config and shared context
 - Initialise Driver Event Storage with config and shared context
 - Initialise Driver Model Storage with config and shared context
 - Initialise Driver Error Storage with config and shared context
 - Initialise server with config and shared context
 - Load current models definition
 - Listen for model definition changes

### Models
Business logic is split in CRUD entities called Models.

Models at minimum have an identifier (recommended a URL friendly identifier) and a version number starting at 1.

A model can have several hooks:
 - preCreate, postCreate
 - preUpdate, postUpdate
 - preDelete, postDelete

Models definitions and their instances are stored by `driver-model-storage`.

### Hooks context
Hooks can be native rust libraries, loaded at runtime or they can be written in JS and will be run in a V8 Isolate.

PreHooks and PostHooks can: 
 - Read the model instance being updated
 - Read the model instance being updated
 - Read other model instances

PostHooks can:
 - Write other model istances
 - Make HTTP requests
 - Spawn a process

PreHooks can:
 - Fail and prevent the event to be added to the Event Storage and changes stored in the model storage

PostHooks failures will be handled by the `driver-error-storage`.

### Event Log
Creating or updating a Model definition or one of his instance is an event.

Every event is written to the Event storage.

Every event has a reference to a model identifier and a version number.

Events and their instances are stored by `driver-event-storage`.

The `driver-model-storage` can be recreated at any time 

### Server requests

Mounted at `mount-path`.

#### Model definitions

##### GET /models
Returns a list of models and their properties

##### GET/models/:modelId
Returns a model's properties

##### POST /models
 - Add an event in the Event Log with the payload
 - Publish the event for every instance listening
 - Creates new model definition
 - Perform necessary side effects (like creating a schema in a database)

##### PUT /models/:modelId
 - Add an event in the Event Log with the payload
 - Publish the event for every instance listening
 - Updates a model definition, incrementing its version
 - Perform necessary side effects (like updating a schema in a database, migrating columns)

##### DEL /models/:modelId
 - Add an event in the Event Log with the payload
 - Publish the event for every instance listening
 - Listen on that event
 - Delete a model definition
 - Perform necessary side effects (like deleting a schema in a database)

#### Model hooks

##### GET/models/:modelId/hooks
Returns a model's hooks

##### GET /models/:modelId/hooks/:hook
Returns a model's hook

##### PUT /models/:modelId/hooks/:hook
Update a model's hook

#### Model Instances

##### GET /models/:modelId/instances
Returns a list of model instances

##### GET/models/:modelId/instances/:id
Returns a model instance's properties

##### POST /models
 - Add an event in the Event Log with the payload
 - Run preCreate hook
 - Creates new model instance
 - Run postCreate hook

##### PUT /models/:modelId/instances/:id
 - Add an event in the Event Log with the payload
 - Run preUpdate hook
 - Updates a model instance
 - Run postUpdate hook

##### DEL /models/:modelId/instances/:id
 - Add an event in the Event Log with the payload
 - Run preDelete hook
 - Delete a model instance
 - Run postDelete hook

### Plugins Idea:

Driver Secure Storage = needs private key
  - Memory
  - File
  - JSON / PROTOBUF request
  - any key value store

Driver Event Log = 
  - Memory
  - File
  - JSON / PROTOBUF request
  - any appendable db

Driver Model Storage
  - Memory
  - File
  - CRUD API
  - any db

Driver Error Storage
  - STDOUT
  - HTTP Request
  - any db / logging system