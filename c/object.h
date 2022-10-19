#ifndef C_OBJECT_H
#define C_OBJECT_H

#include <stdint.h>

typedef struct Variable Variable;
typedef struct Object Object;

// object
void init_object(Variable *self);
void add_property(Variable *self, const char *key, Variable *val);
Variable *get_property_by_str(Variable *self, const char *key);
Variable *get_property_by_var(Variable *self, Variable *key);
void remove_property(Variable *self, const char *key);

#endif