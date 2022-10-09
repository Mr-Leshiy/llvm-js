#ifndef C_OBJECT_H
#define C_OBJECT_H

typedef struct Object Object;
typedef struct Variable Variable;

Object *object_allocate();

void add_property(Object &object, std::string key, Variable *val);
Variable *get_property(Object &object, std::string key);
void remove_property(Object &object, std::string key);

#endif