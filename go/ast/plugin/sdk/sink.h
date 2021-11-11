//typedef struct PluginInfo {
//    char *name;
//    char *language;
//    char *plugin_type;
//    char *version;
//} PluginInfo;

//const char *Info();
//const char *Initilize(const char*);
//const char *Finalize();

#include "context.h"

extern "C" {
    const char* name();
    const char* open(Context*);
}